use alloy::dyn_abi::{DynSolType, DynSolValue, EventExt, FunctionExt, JsonAbiExt};
use alloy::json_abi::{Function, JsonAbi};
use alloy::network::TransactionBuilder;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::compile::{BytecodeTarget, CompiledContract};

type AbiCache = RefCell<HashMap<PathBuf, Vec<(String, Arc<JsonAbi>)>>>;

use crate::prompts;
use crate::store::DeploymentStore;
use crate::tui::layout::AppLayout;
use crate::tui::state::{
    AppState, ConnectionStatus, FieldState, Focus, OutputStyle, PopupState,
};
use crate::tui::widgets::{
    AutocompleteInput, CommandPalette, ContractTree, OutputArea, ParameterPopup, StatusBarWidget,
    parse_path_for_autocomplete, scan_path_suggestions,
};
use crate::tui::widgets::command_palette::default_commands;
use crate::tui::widgets::contract_tree::TreeNode;
use crate::tui::InputEvent;

#[derive(Clone)]
enum PendingAction {
    None,
    Deploy {
        contract_name: String,
        contract_path: PathBuf,
        abi: Arc<JsonAbi>,
    },
    CallMethod {
        function: Function,
        address: Address,
    },
}

pub struct App<P> {
    pub provider: P,
    pub store: DeploymentStore,
    pub state: AppState,
    pub contract: Option<CompiledContract>,
    pub contract_path: Option<PathBuf>,
    pub address: Option<Address>,
    running: bool,
    pending_action: PendingAction,
    edit_config_requested: bool,
    /// Cache of loaded ABIs to avoid re-parsing files on every render
    abi_cache: AbiCache,
}

impl<P: Provider + Clone> App<P> {
    pub fn new(provider: P, store: DeploymentStore) -> Self {
        Self {
            provider,
            store,
            state: AppState::default(),
            contract: None,
            contract_path: None,
            address: None,
            running: true,
            pending_action: PendingAction::None,
            edit_config_requested: false,
            abi_cache: RefCell::new(HashMap::new()),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let chain_id = self.provider.get_chain_id().await?;
        self.state.chain_id = Some(chain_id);
        self.state.connection = ConnectionStatus::Connected;
        Ok(())
    }

    pub fn set_contract(&mut self, contract: CompiledContract, path: PathBuf) {
        self.contract = Some(contract);
        self.contract_path = Some(path.clone());
        self.address = None;

        // Expand the contract
        self.state.sidebar.expanded_contracts.insert(path.clone());

        // Select the contract in the sidebar
        self.select_contract_in_sidebar(&path);
    }

    /// Find and select a contract by path in the sidebar
    fn select_contract_in_sidebar(&mut self, path: &PathBuf) {
        let nodes = self.build_tree_nodes();
        for (i, node) in nodes.iter().enumerate() {
            if let TreeNode::Contract { path: node_path, .. } = node {
                if node_path == path {
                    self.state.sidebar.selected = i;
                    // Adjust scroll if needed
                    if self.state.sidebar.selected < self.state.sidebar.scroll_offset {
                        self.state.sidebar.scroll_offset = self.state.sidebar.selected;
                    }
                    break;
                }
            }
        }
    }

    /// Find and select a deployed instance by address in the sidebar
    fn select_instance_in_sidebar(&mut self, address: Address) {
        let nodes = self.build_tree_nodes();
        for (i, node) in nodes.iter().enumerate() {
            if let TreeNode::DeployedInstance { address: node_address, .. } = node {
                if *node_address == address {
                    self.state.sidebar.selected = i;
                    // Adjust scroll if needed
                    if self.state.sidebar.selected < self.state.sidebar.scroll_offset {
                        self.state.sidebar.scroll_offset = self.state.sidebar.selected;
                    }
                    break;
                }
            }
        }
    }

    pub fn set_address(&mut self, address: Address) {
        self.address = Some(address);
    }

    pub fn set_account_info(&mut self, address: Address, balance: String) {
        self.state.account = Some(address);
        self.state.balance = Some(balance);
    }

    pub fn clear_state(&mut self) {
        self.contract = None;
        self.contract_path = None;
        self.address = None;
        self.state.sidebar = Default::default();
        self.store.clear();
        if let Err(e) = self.store.save() {
            self.state.output.push_error(format!("Failed to save after clearing: {}", e));
        }
        self.state.output.push_info("State cleared");
        self.abi_cache.borrow_mut().clear();
    }

    /// Load contract ABIs with caching
    fn load_contract_abi_cached(&self, path: &PathBuf) -> Option<Vec<(String, Arc<JsonAbi>)>> {
        // Check cache first
        if let Some(cached) = self.abi_cache.borrow().get(path) {
            return Some(cached.clone());
        }

        // Load from disk and cache
        match crate::compile::load_contract_abi(path) {
            Ok(contracts) => {
                let abi_list: Vec<(String, Arc<JsonAbi>)> = contracts
                    .into_iter()
                    .map(|(name, abi)| (name, Arc::new(abi)))
                    .collect();

                self.abi_cache.borrow_mut().insert(path.clone(), abi_list.clone());
                Some(abi_list)
            }
            Err(_) => None,
        }
    }

    /// Get name and ABI for a contract path, using current contract or cache
    fn get_contract_name_and_abi(&self, contract_path: &PathBuf) -> (String, Arc<JsonAbi>) {
        // Use current contract if this is the active one
        let is_current = self.contract_path.as_ref() == Some(contract_path);
        if is_current {
            if let Some(contract) = &self.contract {
                return (contract.name.clone(), Arc::new(contract.abi.clone()));
            }
        }

        // Fall back to cached ABI loading
        if let Some(contracts) = self.load_contract_abi_cached(contract_path) {
            if let Some((name, abi)) = contracts.into_iter().next() {
                return (name, abi);
            }
        }

        // Last resort: use filename as name with empty ABI
        let name = contract_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| contract_path.to_string_lossy().to_string());
        (name, Arc::new(JsonAbi::new()))
    }

    /// Build tree nodes with ABI caching for performance
    fn build_tree_nodes(&self) -> Vec<TreeNode> {
        use crate::method_list::{self, MethodSelection};

        let mut nodes = Vec::new();

        // Always show "New contract" at top
        nodes.push(TreeNode::NewContract);

        // Get all contracts and sort them for stable ordering
        let mut all_contracts: Vec<PathBuf> = self.store.all_contracts().into_iter().collect();

        // Add current contract if it's not already in the store
        if let Some(current_path) = &self.contract_path {
            if !all_contracts.contains(current_path) {
                all_contracts.push(current_path.clone());
            }
        }

        // Sort for stable ordering
        all_contracts.sort();

        // Add each contract in sorted order
        for contract_path in all_contracts {
            let (name, abi) = self.get_contract_name_and_abi(&contract_path);

            nodes.push(TreeNode::Contract {
                name: name.clone(),
                path: contract_path.clone(),
            });

            // Check if this contract is expanded
            if self.state.sidebar.expanded_contracts.contains(&contract_path) {
                // Add constructor and load options for all expanded contracts
                let abi_clone = Arc::clone(&abi);
                nodes.push(TreeNode::Constructor {
                    contract_name: name.clone(),
                    contract_path: contract_path.clone(),
                    abi: abi_clone,
                });

                let abi_clone = Arc::clone(&abi);
                nodes.push(TreeNode::LoadExistingInstance {
                    contract_name: name.clone(),
                    contract_path: contract_path.clone(),
                    abi: abi_clone,
                });

                // Show deployed instances for all expanded contracts
                let deployments = self.store.get_deployments(&contract_path);
                for address in &deployments {
                    nodes.push(TreeNode::DeployedInstance {
                        address: *address,
                        contract_path: contract_path.clone(),
                    });

                    // Show methods if instance is expanded
                    if self.state.sidebar.expanded_instances.contains(address) {
                        let methods = method_list::list_methods(&abi, false);
                        for method in methods {
                            if let MethodSelection::Function(f) = method.selection {
                                nodes.push(TreeNode::Method {
                                    function: f,
                                    tag: method.tag,
                                    instance_address: *address,
                                });
                            }
                        }
                    }
                }
            }
        }

        nodes
    }

    pub async fn run_interactive(&mut self) -> Result<()> {
        let mut terminal = crate::tui::setup()?;
        let mut output_area = ratatui::layout::Rect::default();

        while self.running {
            // Check if we need to open the config editor
            if self.edit_config_requested {
                self.edit_config_requested = false;

                // Restore terminal before launching editor
                crate::tui::restore(&mut terminal)?;

                // Open editor
                self.open_config_in_editor()?;

                // Reload store to pick up changes
                match DeploymentStore::load() {
                    Ok(store) => {
                        self.store = store;
                        // Reset sidebar selection to ensure it's valid after reload
                        self.state.sidebar.selected = 0;
                        self.state.sidebar.scroll_offset = 0;
                        self.state.output.push_success("Config reloaded");
                    }
                    Err(e) => {
                        self.state.output.push_error(format!("Failed to reload config: {}", e));
                    }
                }

                // Re-setup terminal
                terminal = crate::tui::setup()?;
                continue;
            }

            terminal.draw(|f| {
                let layout = AppLayout::new(f.area());
                output_area = layout.output;
                self.render(f);
            })?;

            if let Some(event) = crate::tui::poll_event()? {
                match event {
                    InputEvent::Key(key) => self.handle_key(key).await?,
                    InputEvent::ScrollUp(col, row) => {
                        // Check if scroll is within output area
                        if col >= output_area.x
                            && col < output_area.x + output_area.width
                            && row >= output_area.y
                            && row < output_area.y + output_area.height
                            && self.state.output.scroll_offset > 0 {
                                self.state.output.scroll_offset -= 1;
                            }
                    }
                    InputEvent::ScrollDown(col, row) => {
                        // Check if scroll is within output area
                        if col >= output_area.x
                            && col < output_area.x + output_area.width
                            && row >= output_area.y
                            && row < output_area.y + output_area.height
                        {
                            let max_scroll = self.state.output.lines.len().saturating_sub(10);
                            if self.state.output.scroll_offset < max_scroll {
                                self.state.output.scroll_offset += 1;
                            }
                        }
                    }
                }
            }
        }

        crate::tui::restore(&mut terminal)?;
        Ok(())
    }

    fn open_config_in_editor(&self) -> Result<()> {
        use std::process::Command;

        let config_path = DeploymentStore::config_path();

        // Ensure config file exists
        if !config_path.exists() {
            self.store.save()?;
        }

        // Get editor from environment, fallback to common editors
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| "vi".to_string());

        Command::new(&editor)
            .arg(&config_path)
            .status()
            .with_context(|| format!("Failed to open {} with {}", config_path.display(), editor))?;

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let layout = AppLayout::new(frame.area());

        // Render sidebar with cached tree building
        let nodes = self.build_tree_nodes();
        let tree = ContractTree::new(&self.state.sidebar)
            .focused(matches!(self.state.focus, Focus::Sidebar))
            .with_nodes(nodes);
        frame.render_widget(tree, layout.sidebar);

        // Render output area
        let output = OutputArea::new(&self.state.output)
            .focused(matches!(self.state.focus, Focus::Output));
        frame.render_widget(output, layout.output);

        // Render status bar
        let status = StatusBarWidget::new(&self.state);
        frame.render_widget(status, layout.status_bar);

        // Render popups
        match &self.state.popup {
            PopupState::None => {}
            PopupState::CommandPalette { query, selected } => {
                let palette = CommandPalette::new(query, *selected);
                frame.render_widget(palette, frame.area());
            }
            PopupState::ParameterPopup {
                method_name,
                params,
                fields,
                current,
                bytecode_target,
            } => {
                let popup = ParameterPopup::new(method_name, params, fields, *current)
                    .bytecode_target(*bytecode_target);
                frame.render_widget(popup, frame.area());
            }
            PopupState::FilePicker { path, error } => {
                self.render_file_picker(frame, path, error.as_deref());
            }
            PopupState::AddressInput { address, error } => {
                self.render_address_input(frame, address, error.as_deref());
            }
            PopupState::ContractSelector { contracts, selected } => {
                self.render_contract_selector(frame, contracts, *selected);
            }
        }
    }

    fn render_file_picker(&self, frame: &mut Frame, path: &str, error: Option<&str>) {
        use crate::tui::layout::centered_popup;
        use ratatui::widgets::{Block, Borders, Clear};
        use ratatui::style::{Color, Style, Modifier};
        use ratatui::text::{Line, Span};

        let popup_area = centered_popup(frame.area(), 60, 40);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Load Contract ");
        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let input = AutocompleteInput::new("Path to .sol file", path)
            .placeholder("./contracts/MyContract.sol")
            .error(error)
            .focused(true)
            .suggestions(&self.state.file_picker_suggestions)
            .selected_suggestion(self.state.file_picker_selected_idx);

        let field_area = ratatui::layout::Rect::new(
            inner.x + 1,
            inner.y + 2,
            inner.width.saturating_sub(2),
            inner.height.saturating_sub(4)
        );
        frame.render_widget(input, field_area);

        // Render keyboard hints at bottom
        let hints = Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": navigate | "),
            Span::styled("Tab", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": complete | "),
            Span::styled("Enter", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": accept | "),
            Span::styled("Esc", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": cancel"),
        ]);

        let hints_y = inner.y + inner.height - 1;
        frame.buffer_mut().set_line(inner.x + 1, hints_y, &hints, inner.width.saturating_sub(2));
    }

    fn render_address_input(&self, frame: &mut Frame, address: &str, error: Option<&str>) {
        use crate::tui::layout::centered_popup;
        use crate::tui::widgets::InputField;
        use ratatui::widgets::{Block, Borders, Clear};
        use ratatui::style::{Color, Style};

        let popup_area = centered_popup(frame.area(), 60, 20);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Enter Address ");
        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let input = InputField::new("Contract address", address)
            .placeholder("0x...")
            .error(error)
            .focused(true);

        let field_area = ratatui::layout::Rect::new(
            inner.x + 1,
            inner.y + 2,
            inner.width.saturating_sub(2),
            if error.is_some() { 2 } else { 1 }
        );
        frame.render_widget(input, field_area);
    }

    fn render_contract_selector(&self, frame: &mut Frame, contracts: &[String], selected: usize) {
        use crate::tui::layout::centered_popup;
        use ratatui::widgets::{Block, Borders, Clear};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span};

        let popup_area = centered_popup(frame.area(), 50, 40);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Select Contract ");
        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        for (i, name) in contracts.iter().enumerate() {
            let y = inner.y + i as u16;
            if y >= inner.y + inner.height {
                break;
            }

            let is_selected = i == selected;
            let style = if is_selected {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let line = Line::from(Span::styled(
                format!("{} {}", if is_selected { ">" } else { " " }, name),
                style,
            ));
            frame.buffer_mut().set_line(inner.x, y, &line, inner.width);
        }
    }

    fn update_file_picker_suggestions(&mut self, input: &str) {
        let (dir, prefix) = parse_path_for_autocomplete(input);
        self.state.file_picker_suggestions = scan_path_suggestions(&dir, &prefix);
        self.state.file_picker_selected_idx = 0;
    }

    async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        // Global shortcuts
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') => {
                    self.running = false;
                    return Ok(());
                }
                KeyCode::Char('p') => {
                    self.state.focus = Focus::CommandPalette;
                    self.state.popup = PopupState::CommandPalette {
                        query: String::new(),
                        selected: 0,
                    };
                    return Ok(());
                }
                _ => {}
            }
        }

        // Handle based on focus/popup state
        match &self.state.popup {
            PopupState::None => self.handle_main_key(key).await?,
            PopupState::CommandPalette { .. } => self.handle_command_palette_key(key).await?,
            PopupState::ParameterPopup { .. } => self.handle_parameter_popup_key(key).await?,
            PopupState::FilePicker { .. } => self.handle_file_picker_key(key).await?,
            PopupState::AddressInput { .. } => self.handle_address_input_key(key).await?,
            PopupState::ContractSelector { .. } => self.handle_contract_selector_key(key).await?,
        }

        Ok(())
    }

    async fn handle_main_key(&mut self, key: KeyEvent) -> Result<()> {
        match self.state.focus {
            Focus::Sidebar => self.handle_sidebar_key(key).await?,
            Focus::Output => self.handle_output_key(key),
            _ => {}
        }
        Ok(())
    }

    async fn handle_sidebar_key(&mut self, key: KeyEvent) -> Result<()> {
        // Build tree to get current nodes (with caching)
        let nodes = self.build_tree_nodes();
        let node_count = nodes.len();

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.sidebar.selected > 0 {
                    self.state.sidebar.selected -= 1;
                    // Adjust scroll
                    if self.state.sidebar.selected < self.state.sidebar.scroll_offset {
                        self.state.sidebar.scroll_offset = self.state.sidebar.selected;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.state.sidebar.selected + 1 < node_count {
                    self.state.sidebar.selected += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // Collapse current node
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    match node {
                        TreeNode::Contract { path, .. } => {
                            self.state.sidebar.expanded_contracts.remove(path);
                        }
                        TreeNode::DeployedInstance { address, .. } => {
                            self.state.sidebar.expanded_instances.remove(address);
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                // Expand current node
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    match node {
                        TreeNode::Contract { path, .. } => {
                            // Load and expand this contract
                            if self.contract_path.as_ref() != Some(path) {
                                self.load_contract_from_path(path.clone()).await?;
                            } else {
                                // Already loaded, just expand
                                self.state.sidebar.expanded_contracts.insert(path.clone());
                            }
                        }
                        TreeNode::DeployedInstance { address, .. } => {
                            self.state.sidebar.expanded_instances.insert(*address);
                            self.set_address(*address);
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    self.execute_tree_node(node.clone()).await?;
                }
            }
            KeyCode::Delete | KeyCode::Backspace => {
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    let removed = match node {
                        TreeNode::DeployedInstance { address, contract_path, .. } => {
                            if self.store.remove_deployment(contract_path, *address) {
                                self.state.output.push_info(format!("Removed deployment: {:?}", address));
                                if self.address == Some(*address) {
                                    self.address = None;
                                }
                                true
                            } else {
                                false
                            }
                        }
                        TreeNode::Contract { path, name, .. } => {
                            if self.store.remove_contract(path) {
                                self.state.output.push_info(format!("Removed all deployments for: {}", name));
                                if self.contract_path.as_ref() == Some(path) {
                                    self.address = None;
                                }
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    if removed {
                        self.store.save()?;
                        let new_count = self.build_tree_nodes().len();
                        if new_count > 0 && self.state.sidebar.selected >= new_count {
                            self.state.sidebar.selected = new_count - 1;
                        }
                    }
                }
            }
            KeyCode::Tab => {
                self.state.focus = Focus::Output;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_output_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.output.scroll_offset > 0 {
                    self.state.output.scroll_offset -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max_scroll = self.state.output.lines.len().saturating_sub(10);
                if self.state.output.scroll_offset < max_scroll {
                    self.state.output.scroll_offset += 1;
                }
            }
            KeyCode::Tab => {
                self.state.focus = Focus::Sidebar;
            }
            _ => {}
        }
    }

    async fn handle_command_palette_key(&mut self, key: KeyEvent) -> Result<()> {
        if let PopupState::CommandPalette { query, selected } = &mut self.state.popup {
            match key.code {
                KeyCode::Esc => {
                    self.state.popup = PopupState::None;
                    self.state.focus = Focus::Sidebar;
                }
                KeyCode::Enter => {
                    let commands = default_commands();
                    let query_lower = query.to_lowercase();
                    let filtered: Vec<_> = commands
                        .iter()
                        .enumerate()
                        .filter(|(_, cmd)| {
                            query_lower.is_empty()
                                || cmd.name.to_lowercase().contains(&query_lower)
                                || cmd.description.to_lowercase().contains(&query_lower)
                        })
                        .collect();

                    if let Some((idx, _)) = filtered.get(*selected) {
                        let action = *idx;
                        self.state.popup = PopupState::None;
                        self.state.focus = Focus::Sidebar;
                        self.execute_command(action).await?;
                    }
                }
                KeyCode::Up => {
                    if *selected > 0 {
                        *selected -= 1;
                    }
                }
                KeyCode::Down => {
                    let commands = default_commands();
                    let query_lower = query.to_lowercase();
                    let count = commands
                        .iter()
                        .filter(|cmd| {
                            query_lower.is_empty()
                                || cmd.name.to_lowercase().contains(&query_lower)
                                || cmd.description.to_lowercase().contains(&query_lower)
                        })
                        .count();

                    if *selected + 1 < count {
                        *selected += 1;
                    }
                }
                KeyCode::Char(c) => {
                    query.push(c);
                    *selected = 0;
                }
                KeyCode::Backspace => {
                    query.pop();
                    *selected = 0;
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn handle_parameter_popup_key(&mut self, key: KeyEvent) -> Result<()> {
        // Handle Enter separately to avoid borrow checker issues
        if key.code == KeyCode::Enter {
            if let PopupState::ParameterPopup { params, fields, bytecode_target, .. } = &self.state.popup {
                // Clone data we need for parsing
                let params_clone = params.clone();
                let fields_clone = fields.clone();
                let target = *bytecode_target;

                let values = self.try_parse_params(&params_clone, &fields_clone);
                match values {
                    Ok(args) => {
                        let action = self.pending_action.clone();
                        self.state.popup = PopupState::None;
                        self.state.focus = Focus::Sidebar;
                        self.pending_action = PendingAction::None;

                        match action {
                            PendingAction::Deploy { contract_name, contract_path, abi } => {
                                // target is Some for deploy operations
                                self.do_deploy(contract_name, contract_path, abi, args, target.unwrap_or_default()).await;
                            }
                            PendingAction::CallMethod { function, address } => {
                                self.do_call_function(&function, address, args).await;
                            }
                            PendingAction::None => {}
                        }
                    }
                    Err(errors) => {
                        // Update field errors
                        if let PopupState::ParameterPopup { fields, .. } = &mut self.state.popup {
                            for (i, err) in errors {
                                if let Some(field) = fields.get_mut(i) {
                                    field.error = Some(err);
                                }
                            }
                        }
                    }
                }
            }
            return Ok(());
        }

        // Handle left/right arrows for target switching
        if matches!(key.code, KeyCode::Left | KeyCode::Right) {
            if let PopupState::ParameterPopup { bytecode_target: Some(target), .. } = &mut self.state.popup {
                *target = target.toggle();
            }
            return Ok(());
        }

        if let PopupState::ParameterPopup { fields, current, .. } = &mut self.state.popup {
            match key.code {
                KeyCode::Esc => {
                    self.state.popup = PopupState::None;
                    self.state.focus = Focus::Sidebar;
                    self.pending_action = PendingAction::None;
                }
                KeyCode::Tab => {
                    *current = (*current + 1) % fields.len().max(1);
                }
                KeyCode::BackTab => {
                    if *current > 0 {
                        *current -= 1;
                    } else {
                        *current = fields.len().saturating_sub(1);
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(field) = fields.get_mut(*current) {
                        field.value.push(c);
                        field.error = None;
                    }
                }
                KeyCode::Backspace => {
                    if let Some(field) = fields.get_mut(*current) {
                        field.value.pop();
                        field.error = None;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn handle_file_picker_key(&mut self, key: KeyEvent) -> Result<()> {
        if let PopupState::FilePicker { path, error } = &mut self.state.popup {
            match key.code {
                KeyCode::Esc => {
                    self.state.popup = PopupState::None;
                    self.state.focus = Focus::Sidebar;
                    self.state.file_picker_suggestions.clear();
                    self.state.file_picker_selected_idx = 0;
                }
                KeyCode::Up => {
                    if !self.state.file_picker_suggestions.is_empty() {
                        if self.state.file_picker_selected_idx == 0 {
                            self.state.file_picker_selected_idx = self.state.file_picker_suggestions.len() - 1;
                        } else {
                            self.state.file_picker_selected_idx -= 1;
                        }
                    }
                }
                KeyCode::Down => {
                    if !self.state.file_picker_suggestions.is_empty() {
                        self.state.file_picker_selected_idx =
                            (self.state.file_picker_selected_idx + 1) % self.state.file_picker_suggestions.len();
                    }
                }
                KeyCode::Tab => {
                    if let Some(suggestion) = self.state.file_picker_suggestions.get(self.state.file_picker_selected_idx) {
                        let new_path = suggestion.full_path.to_string_lossy().to_string();
                        *path = if suggestion.is_directory {
                            format!("{}/", new_path)
                        } else {
                            new_path
                        };
                        *error = None;

                        // Update suggestions based on new path
                        let path_clone = path.clone();
                        self.update_file_picker_suggestions(&path_clone);
                    }
                }
                KeyCode::Enter => {
                    let suggestion = self.state.file_picker_suggestions.get(self.state.file_picker_selected_idx);

                    // If selected suggestion is a directory, navigate into it
                    if let Some(s) = suggestion.filter(|s| s.is_directory) {
                        let new_path = s.full_path.to_string_lossy().to_string();
                        *path = format!("{}/", new_path);
                        *error = None;
                        let path_clone = path.clone();
                        self.update_file_picker_suggestions(&path_clone);
                        return Ok(());
                    }

                    // Determine file path: from suggestion or raw input
                    let file_path = suggestion
                        .map(|s| s.full_path.clone())
                        .unwrap_or_else(|| PathBuf::from(path.as_str()));

                    if !file_path.exists() {
                        *error = Some("File does not exist".to_string());
                        return Ok(());
                    }

                    // Load the file
                    self.state.popup = PopupState::None;
                    self.state.focus = Focus::Sidebar;
                    self.state.file_picker_suggestions.clear();
                    self.state.file_picker_selected_idx = 0;
                    self.load_contract_from_path(file_path).await?;
                }
                KeyCode::Char(c) => {
                    path.push(c);
                    *error = None;
                    let path_clone = path.clone();
                    self.update_file_picker_suggestions(&path_clone);
                }
                KeyCode::Backspace => {
                    path.pop();
                    *error = None;
                    let path_clone = path.clone();
                    self.update_file_picker_suggestions(&path_clone);
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn handle_address_input_key(&mut self, key: KeyEvent) -> Result<()> {
        if let PopupState::AddressInput { address, error } = &mut self.state.popup {
            match key.code {
                KeyCode::Esc => {
                    self.state.popup = PopupState::None;
                    self.state.focus = Focus::Sidebar;
                }
                KeyCode::Enter => {
                    match address.parse::<Address>() {
                        Ok(addr) => {
                            self.state.popup = PopupState::None;
                            self.state.focus = Focus::Sidebar;
                            self.set_address(addr);
                        }
                        Err(_) => {
                            *error = Some("Invalid address format".to_string());
                        }
                    }
                }
                KeyCode::Char(c) => {
                    address.push(c);
                    *error = None;
                }
                KeyCode::Backspace => {
                    address.pop();
                    *error = None;
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn handle_contract_selector_key(&mut self, key: KeyEvent) -> Result<()> {
        if let PopupState::ContractSelector { contracts, selected } = &mut self.state.popup {
            match key.code {
                KeyCode::Esc => {
                    self.state.popup = PopupState::None;
                    self.state.focus = Focus::Sidebar;
                }
                KeyCode::Up => {
                    if *selected > 0 {
                        *selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if *selected + 1 < contracts.len() {
                        *selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(name) = contracts.get(*selected).cloned() {
                        self.state.popup = PopupState::None;
                        self.state.focus = Focus::Sidebar;
                        self.select_compiled_contract(&name)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn execute_tree_node(&mut self, node: TreeNode) -> Result<()> {
        match node {
            TreeNode::NewContract => {
                self.state.popup = PopupState::FilePicker {
                    path: String::new(),
                    error: None,
                };
                self.state.focus = Focus::CommandPalette;
                self.update_file_picker_suggestions("");
            }
            TreeNode::Contract { path, .. } => {
                // If this is not the current contract, load it
                // If it is the current contract, toggle expansion
                if self.contract_path.as_ref() == Some(&path) {
                    // Toggle expansion for current contract
                    if self.state.sidebar.expanded_contracts.contains(&path) {
                        self.state.sidebar.expanded_contracts.remove(&path);
                    } else {
                        self.state.sidebar.expanded_contracts.insert(path);
                    }
                } else {
                    // Load a different contract (will auto-expand and select)
                    self.load_contract_from_path(path).await?;
                }
            }
            TreeNode::Constructor { contract_name, contract_path, abi } => {
                // Deploy directly using node data - no need to set app state
                self.start_deploy(contract_name, contract_path, abi).await;
            }
            TreeNode::LoadExistingInstance { contract_name, contract_path, abi } => {
                // Set this as current contract for loading instance
                let compiled = CompiledContract {
                    name: contract_name,
                    abi: (*abi).clone(),
                    bytecode: Vec::new(),
                };
                self.set_contract(compiled, contract_path);
                self.state.popup = PopupState::AddressInput {
                    address: String::new(),
                    error: None,
                };
                self.state.focus = Focus::CommandPalette;
            }
            TreeNode::DeployedInstance { address, .. } => {
                // Toggle expand only
                if self.state.sidebar.expanded_instances.contains(&address) {
                    self.state.sidebar.expanded_instances.remove(&address);
                } else {
                    self.state.sidebar.expanded_instances.insert(address);
                }
            }
            TreeNode::Method {
                function,
                instance_address,
                ..
            } => {
                // Call directly using node data - no need to set app state
                self.start_call_function(function, instance_address).await;
            }
        }
        Ok(())
    }

    async fn execute_command(&mut self, command_idx: usize) -> Result<()> {
        match command_idx {
            0 => {
                // Edit config
                self.edit_config_requested = true;
            }
            1 => {
                // Reset
                self.clear_state();
            }
            2 => {
                // Clear output
                self.state.output.clear();
            }
            3 => {
                // Quit
                self.running = false;
            }
            _ => {}
        }
        Ok(())
    }

    async fn load_contract_from_path(&mut self, path: PathBuf) -> Result<()> {
        match crate::compile::load_contract_abi(&path) {
            Ok(contracts) => {
                if contracts.len() == 1 {
                    let (name, abi) = contracts.into_iter().next().unwrap();
                    // Create a contract with just ABI (bytecode will be compiled on demand)
                    let contract = CompiledContract {
                        name,
                        abi,
                        bytecode: Vec::new(), // Will be filled during deploy
                    };
                    self.set_contract(contract, path);
                } else {
                    // Multiple contracts - show selector
                    let names: Vec<String> = contracts.into_iter().map(|(name, _)| name).collect();
                    // Store path for later selection
                    self.contract_path = Some(path);
                    self.state.popup = PopupState::ContractSelector {
                        contracts: names,
                        selected: 0,
                    };
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to load contract: {}", e);
                log::error!("{}", error_msg);
                self.state.output.push_error(error_msg);
            }
        }
        Ok(())
    }

    fn select_compiled_contract(&mut self, name: &str) -> Result<()> {
        // Load just the ABI for the selected contract
        if let Some(path) = self.contract_path.clone() {
            let contracts = crate::compile::load_contract_abi(&path)?;
            if let Some((_, abi)) = contracts.into_iter().find(|(n, _)| n == name) {
                let contract = CompiledContract {
                    name: name.to_string(),
                    abi,
                    bytecode: Vec::new(), // Will be filled during deploy
                };
                self.set_contract(contract, path);
            }
        }
        Ok(())
    }

    async fn start_deploy(&mut self, contract_name: String, contract_path: PathBuf, abi: Arc<JsonAbi>) {
        self.state.output.push_normal(format!("\nPreparing to deploy {}...", contract_name));

        // Get constructor parameters (empty vec if no constructor)
        let params = abi
            .constructor
            .as_ref()
            .map(|ctor| ctor.inputs.clone())
            .unwrap_or_default();

        let fields: Vec<FieldState> = params.iter().map(|_| FieldState::default()).collect();

        // Always show popup with target selector for deploy operations
        self.pending_action = PendingAction::Deploy {
            contract_name,
            contract_path,
            abi,
        };
        self.state.popup = PopupState::ParameterPopup {
            method_name: "constructor".to_string(),
            params,
            fields,
            current: 0,
            bytecode_target: Some(BytecodeTarget::Evm), // Default to EVM
        };
    }

    async fn start_call_function(&mut self, func: Function, address: Address) {
        if !func.inputs.is_empty() {
            let fields: Vec<FieldState> = func
                .inputs
                .iter()
                .map(|_| FieldState::default())
                .collect();

            self.pending_action = PendingAction::CallMethod {
                function: func.clone(),
                address,
            };
            self.state.popup = PopupState::ParameterPopup {
                method_name: func.name.clone(),
                params: func.inputs.clone(),
                fields,
                current: 0,
                bytecode_target: None, // No target selector for calls
            };
            return;
        }

        // No parameters - call directly
        self.do_call_function(&func, address, vec![]).await;
    }

    fn try_parse_params(
        &self,
        params: &[alloy::json_abi::Param],
        fields: &[FieldState],
    ) -> std::result::Result<Vec<DynSolValue>, Vec<(usize, String)>> {
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for (i, (param, field)) in params.iter().zip(fields.iter()).enumerate() {
            let type_str = param.ty.as_str();
            match parse_value(&field.value, type_str) {
                Ok(value) => values.push(value),
                Err(e) => errors.push((i, e)),
            }
        }

        if errors.is_empty() {
            Ok(values)
        } else {
            Err(errors)
        }
    }

    async fn do_deploy(&mut self, contract_name: String, contract_path: PathBuf, _abi: Arc<JsonAbi>, args: Vec<DynSolValue>, target: BytecodeTarget) {
        // Compile on demand for the selected target
        self.state.output.push_info(format!("Compiling {} for {}...", contract_name, target));

        let compiled = match crate::compile::compile_contract(&contract_path, &contract_name, target) {
            Ok(c) => c,
            Err(e) => {
                let error_msg = format!("Compilation failed: {}", e);
                log::error!("{}", error_msg);
                self.state.output.push_error(error_msg);
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                return;
            }
        };

        self.state.output.push_success(format!("Compilation successful ({})", target));

        let mut deploy_data = compiled.bytecode.clone();

        if !args.is_empty() {
            let encoded = DynSolValue::Tuple(args).abi_encode_params();
            deploy_data.extend(encoded);
        }

        let tx = TransactionRequest::default().with_deploy_code(deploy_data);

        self.state.output.push(
            format!("Deploying {} contract...", contract_name),
            OutputStyle::Waiting
        );

        let pending = match self.provider.send_transaction(tx).await {
            Ok(p) => p,
            Err(e) => {
                self.state.output.push_error(format!("Transaction failed: {}", e));
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                return;
            }
        };

        let tx_hash = *pending.tx_hash();
        self.state.output.push_success(format!("Transaction: {:?}", tx_hash));
        self.state.output.push("Waiting for confirmation...", OutputStyle::Waiting);

        let receipt = match pending.get_receipt().await {
            Ok(r) => r,
            Err(e) => {
                self.state.output.push_error(format!("Failed to get receipt: {}", e));
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                return;
            }
        };

        let address = match receipt.contract_address {
            Some(a) => a,
            None => {
                self.state.output.push_error("No contract address in receipt");
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                return;
            }
        };

        self.state.output.push_success(format!("Deployed at: {:?}", address));

        self.set_address(address);

        self.store.add_deployment(&contract_path, address);
        if let Err(e) = self.store.save() {
            self.state.output.push_error(format!("Failed to save deployment: {}", e));
        }

        // Expand and select the newly deployed instance in the sidebar
        self.state.sidebar.expanded_instances.insert(address);
        self.select_instance_in_sidebar(address);

        self.state.output.push_separator();
        self.state.output.scroll_to_bottom();
    }

    async fn do_call_function(
        &mut self,
        func: &Function,
        address: Address,
        args: Vec<DynSolValue>,
    ) {
        let contract_name = self.contract.as_ref().map(|c| c.name.as_str()).unwrap_or("Unknown");

        let calldata = match func.abi_encode_input(&args) {
            Ok(data) => data,
            Err(e) => {
                self.state.output.push_error(format!("Failed to encode function call: {}", e));
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                return;
            }
        };

        let is_view = matches!(
            func.state_mutability,
            alloy::json_abi::StateMutability::View | alloy::json_abi::StateMutability::Pure
        );

        if is_view {
            let tx = TransactionRequest::default()
                .to(address)
                .input(calldata.into());

            let result = match self.provider.call(tx).await {
                Ok(r) => r,
                Err(e) => {
                    self.state.output.push_error(format!("Call to {} {:?} failed: {}", contract_name, address, e));
                    self.state.output.push_separator();
                    self.state.output.scroll_to_bottom();
                    return;
                }
            };

            let decoded = match func.abi_decode_output(&result) {
                Ok(d) => d,
                Err(e) => {
                    self.state.output.push_error(format!("Failed to decode return value: {}", e));
                    self.state.output.push_separator();
                    self.state.output.scroll_to_bottom();
                    return;
                }
            };

            let result_str = match decoded.as_slice() {
                [] => "(no return value)".to_string(),
                [single] => prompts::format_return_value(single),
                multiple => {
                    let formatted: Vec<_> =
                        multiple.iter().map(prompts::format_return_value).collect();
                    format!("({})", formatted.join(", "))
                }
            };

            let call_str = prompts::format_method_call(&func.name, &func.inputs, &args);
            self.state.output.push(format!("{} @ {:?}", call_str, address), OutputStyle::Highlight);
            self.state.output.push_success(format!("Result: {}", result_str));
        } else {
            let tx = TransactionRequest::default()
                .to(address)
                .input(calldata.into());

            self.state.output.push(
                format!("Sending transaction to {} {:?}...", contract_name, address),
                OutputStyle::Waiting
            );

            let pending = match self.provider.send_transaction(tx).await {
                Ok(p) => p,
                Err(e) => {
                    self.state.output.push_error(format!("Transaction failed: {}", e));
                    self.state.output.push_separator();
                    self.state.output.scroll_to_bottom();
                    return;
                }
            };

            let tx_hash = *pending.tx_hash();
            self.state.output.push_success(format!("Transaction: {:?}", tx_hash));

            let call_str = prompts::format_method_call(&func.name, &func.inputs, &args);
            self.state.output.push(format!("{} @ {:?}", call_str, address), OutputStyle::Highlight);

            self.state.output.push("Waiting for confirmation...", OutputStyle::Waiting);

            let receipt = match pending.get_receipt().await {
                Ok(r) => r,
                Err(e) => {
                    self.state.output.push_error(format!("Failed to get receipt: {}", e));
                    self.state.output.push_separator();
                    self.state.output.scroll_to_bottom();
                    return;
                }
            };

            if receipt.status() {
                self.state.output.push_success("Status: Success");
            } else {
                self.state.output.push_error("Transaction reverted");
            }

            self.state.output.push_info(format!("Gas used: {}", receipt.gas_used));

            // Display logs if any
            let logs = receipt.inner.logs();
            if !logs.is_empty() {
                self.state.output.push_info(format!("Logs ({})", logs.len()));
                for (i, log) in logs.iter().enumerate() {
                    self.display_log(i, log);
                }
            }
        }

        self.state.output.push_separator();
        self.state.output.scroll_to_bottom();
    }

    /// Display a log entry, attempting to decode it with known ABIs
    fn display_log(&mut self, index: usize, log: &alloy::rpc::types::Log) {
        let log_address = log.address();

        // Try to find an ABI for this address
        let abi = self.find_abi_for_address(log_address);

        // Try to decode the log if we have topics and an ABI
        if let (Some(abi), Some(first_topic)) = (abi, log.topics().first()) {
            // Find matching event in ABI
            if let Some(event) = abi.events().find(|e| e.selector() == *first_topic) {
                // Convert Log to LogData
                let log_data = alloy::primitives::LogData::new_unchecked(
                    log.topics().to_vec(),
                    log.data().data.clone().into(),
                );

                // Try to decode the log
                match event.decode_log(&log_data) {
                    Ok(decoded) => {
                        // Successfully decoded
                        self.state.output.push_info(format!("  [{}] {} @ {:?}", index, event.name, log_address));

                        // Display decoded parameters
                        for (param, value) in event.inputs.iter().zip(decoded.indexed.iter().chain(decoded.body.iter())) {
                            let value_str = prompts::format_return_value(value);
                            self.state.output.push_info(format!("      {}: {}", param.name, value_str));
                        }
                        return;
                    }
                    Err(_) => {
                        // Decoding failed, fall through to raw display
                    }
                }
            }
        }

        // Fall back to raw display
        self.state.output.push_info(format!("  [{}] Address: {:?}", index, log_address));
        if !log.topics().is_empty() {
            self.state.output.push_info(format!("      Topics: {}",
                log.topics().iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(", ")));
        }
        if !log.data().data.is_empty() {
            self.state.output.push_info(format!("      Data: 0x{}", hex::encode(&log.data().data)));
        }
    }

    /// Try to find an ABI for a given address by checking known deployments
    fn find_abi_for_address(&self, address: Address) -> Option<Arc<JsonAbi>> {
        // Check all contracts in the store
        for contract_path in self.store.all_contracts() {
            let deployments = self.store.get_deployments(&contract_path);
            if deployments.contains(&address) {
                // This contract has this address
                let (_, abi) = self.get_contract_name_and_abi(&contract_path);
                return Some(abi);
            }
        }
        None
    }
}

fn parse_value(input: &str, type_str: &str) -> std::result::Result<DynSolValue, String> {
    let sol_type: DynSolType = type_str
        .parse()
        .map_err(|_| format!("Unsupported type: {}", type_str))?;

    match sol_type {
        DynSolType::Address => {
            let addr: Address = input.parse().map_err(|_| "Invalid address format")?;
            Ok(DynSolValue::Address(addr))
        }
        DynSolType::Bool => {
            let b = match input.to_lowercase().as_str() {
                "true" | "1" | "yes" => true,
                "false" | "0" | "no" => false,
                _ => return Err("Invalid boolean (use true/false)".to_string()),
            };
            Ok(DynSolValue::Bool(b))
        }
        DynSolType::Uint(bits) => {
            let value: alloy::primitives::U256 = if input.starts_with("0x") {
                alloy::primitives::U256::from_str_radix(input.trim_start_matches("0x"), 16)
                    .map_err(|_| "Invalid hex number")?
            } else {
                input.parse().map_err(|_| "Invalid number")?
            };
            Ok(DynSolValue::Uint(value, bits))
        }
        DynSolType::Int(bits) => {
            let value: alloy::primitives::I256 = input.parse().map_err(|_| "Invalid number")?;
            Ok(DynSolValue::Int(value, bits))
        }
        DynSolType::Bytes => {
            let input = input.strip_prefix("0x").unwrap_or(input);
            let bytes = hex::decode(input).map_err(|_| "Invalid hex string")?;
            Ok(DynSolValue::Bytes(bytes))
        }
        DynSolType::FixedBytes(size) => {
            let input = input.strip_prefix("0x").unwrap_or(input);
            let bytes = hex::decode(input).map_err(|_| "Invalid hex string")?;
            if bytes.len() != size {
                return Err(format!("Expected {} bytes", size));
            }
            Ok(DynSolValue::FixedBytes(
                alloy::primitives::FixedBytes::from_slice(&bytes),
                size,
            ))
        }
        DynSolType::String => Ok(DynSolValue::String(input.to_string())),
        _ => Err(format!("Unsupported type: {}", type_str)),
    }
}
