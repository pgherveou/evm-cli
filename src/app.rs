use alloy::dyn_abi::{DynSolType, DynSolValue, FunctionExt, JsonAbiExt};
use alloy::json_abi::Function;
use alloy::network::TransactionBuilder;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use std::path::PathBuf;

use crate::compile::{BytecodeTarget, CompiledContract};
use crate::prompts;
use crate::store::DeploymentStore;
use crate::tui::layout::AppLayout;
use crate::tui::state::{
    AppState, ConnectionStatus, FieldState, Focus, OutputStyle, PopupState,
};
use crate::tui::widgets::{
    CommandPalette, ContractTree, OutputArea, ParameterPopup, StatusBarWidget,
};
use crate::tui::widgets::command_palette::default_commands;
use crate::tui::widgets::contract_tree::TreeNode;
use crate::tui::InputEvent;

#[derive(Clone)]
enum PendingAction {
    None,
    Deploy,
    CallMethod(Function, Address),
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
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let chain_id = self.provider.get_chain_id().await?;
        self.state.chain_id = Some(chain_id);
        self.state.connection = ConnectionStatus::Connected;
        Ok(())
    }

    pub fn set_contract(&mut self, contract: CompiledContract, path: PathBuf) {
        // Expand this contract in the sidebar
        self.state.sidebar.expanded_contracts.insert(path.clone());

        self.contract = Some(contract);
        self.contract_path = Some(path);
        self.address = None;

        self.state.output.push_success(format!(
            "Loaded contract: {}",
            self.contract.as_ref().unwrap().name
        ));
    }

    pub fn set_address(&mut self, address: Address) {
        self.address = Some(address);

        // Expand this instance in sidebar
        self.state.sidebar.expanded_instances.insert(address);

        self.state.output.push_info(format!("Selected address: {:?}", address));
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
        self.state.output.push_info("State cleared");
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

        // Render sidebar
        let tree = ContractTree::new(&self.state.sidebar)
            .focused(matches!(self.state.focus, Focus::Sidebar))
            .with_data(
                &self.store,
                self.contract.as_ref().zip(self.contract_path.as_ref()),
            );
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
        use crate::tui::widgets::InputField;
        use ratatui::widgets::{Block, Borders, Clear};
        use ratatui::style::{Color, Style};

        let popup_area = centered_popup(frame.area(), 60, 20);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Load Contract ");
        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let input = InputField::new("Path to .sol file", path)
            .placeholder("./contracts/MyContract.sol")
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
        // Build tree to get current nodes
        let nodes: Vec<TreeNode> = {
            let tree = ContractTree::new(&self.state.sidebar).with_data(
                &self.store,
                self.contract.as_ref().zip(self.contract_path.as_ref()),
            );
            tree.nodes().to_vec()
        };

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
                            }
                            self.state.sidebar.expanded_contracts.insert(path.clone());
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
                    match node {
                        TreeNode::DeployedInstance { address, contract_path } => {
                            // Remove this specific deployment
                            if self.store.remove_deployment(contract_path, *address) {
                                self.store.save()?;
                                self.state.output.push_info(format!("Removed deployment: {:?}", address));
                                // Clear current address if it was the one removed
                                if self.address == Some(*address) {
                                    self.address = None;
                                }
                                // Adjust selection if needed
                                if self.state.sidebar.selected >= node_count.saturating_sub(1) {
                                    self.state.sidebar.selected = self.state.sidebar.selected.saturating_sub(1);
                                }
                            }
                        }
                        TreeNode::Contract { path, name } => {
                            // Remove all deployments for this contract (but keep it loadable)
                            if self.store.remove_contract(path) {
                                self.store.save()?;
                                self.state.output.push_info(format!("Removed all deployments for: {}", name));
                                // Clear current address if it was from this contract
                                if self.contract_path.as_ref() == Some(path) {
                                    self.address = None;
                                }
                            }
                        }
                        _ => {}
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
                            PendingAction::Deploy => {
                                // target is Some for deploy operations
                                self.do_deploy(args, target.unwrap_or_default()).await;
                            }
                            PendingAction::CallMethod(func, addr) => {
                                self.do_call_function(&func, addr, args).await;
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
                }
                KeyCode::Enter => {
                    let file_path = PathBuf::from(path.as_str());
                    if file_path.exists() {
                        let path_clone = file_path.clone();
                        self.state.popup = PopupState::None;
                        self.state.focus = Focus::Sidebar;
                        self.load_contract_from_path(path_clone).await?;
                    } else {
                        *error = Some("File does not exist".to_string());
                    }
                }
                KeyCode::Char(c) => {
                    path.push(c);
                    *error = None;
                }
                KeyCode::Backspace => {
                    path.pop();
                    *error = None;
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
            }
            TreeNode::Contract { path, .. } => {
                // Toggle expand or load
                if self.contract_path.as_ref() == Some(&path) {
                    // Toggle expansion
                    if self.state.sidebar.expanded_contracts.contains(&path) {
                        self.state.sidebar.expanded_contracts.remove(&path);
                    } else {
                        self.state.sidebar.expanded_contracts.insert(path);
                    }
                } else {
                    // Load this contract
                    self.load_contract_from_path(path).await?;
                }
            }
            TreeNode::Constructor => {
                self.start_deploy().await;
            }
            TreeNode::LoadExistingInstance => {
                self.state.popup = PopupState::AddressInput {
                    address: String::new(),
                    error: None,
                };
                self.state.focus = Focus::CommandPalette;
            }
            TreeNode::DeployedInstance { address, .. } => {
                // Toggle expand and select
                if self.state.sidebar.expanded_instances.contains(&address) {
                    self.state.sidebar.expanded_instances.remove(&address);
                } else {
                    self.state.sidebar.expanded_instances.insert(address);
                    self.set_address(address);
                }
            }
            TreeNode::Method {
                function,
                instance_address,
                ..
            } => {
                self.set_address(instance_address);
                self.start_call_function(function).await;
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
        self.state.output.push_info(format!("Loading {}...", path.display()));

        match crate::compile::load_contract_abi(&path) {
            Ok(contracts) => {
                if contracts.len() == 1 {
                    let (name, abi) = contracts.into_iter().next().unwrap();
                    // Create a contract with just ABI (bytecode will be compiled on demand)
                    let contract = CompiledContract {
                        name,
                        abi,
                        bytecode: Vec::new(), // Will be filled during deploy
                        target: BytecodeTarget::Evm,
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
                self.state.output.push_error(format!("Failed to load contract: {}", e));
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
                    target: BytecodeTarget::Evm,
                };
                self.set_contract(contract, path);
            }
        }
        Ok(())
    }

    async fn start_deploy(&mut self) {
        let contract = match self.contract.as_ref() {
            Some(c) => c,
            None => {
                self.state.output.push_error("No contract loaded");
                return;
            }
        };

        self.state.output.push_normal(format!("\nPreparing to deploy {}...", contract.name));

        // Get constructor parameters (empty vec if no constructor)
        let params = contract
            .abi
            .constructor
            .as_ref()
            .map(|ctor| ctor.inputs.clone())
            .unwrap_or_default();

        let fields: Vec<FieldState> = params.iter().map(|_| FieldState::default()).collect();

        // Always show popup with target selector for deploy operations
        self.pending_action = PendingAction::Deploy;
        self.state.popup = PopupState::ParameterPopup {
            method_name: "constructor".to_string(),
            params,
            fields,
            current: 0,
            bytecode_target: Some(BytecodeTarget::Evm), // Default to EVM
        };
    }

    async fn start_call_function(&mut self, func: Function) {
        let address = match self.address {
            Some(a) => a,
            None => {
                self.state.output.push_error("No contract address set");
                return;
            }
        };

        self.state.output.push_normal(format!("\nCalling {}...", func.name));

        if !func.inputs.is_empty() {
            let fields: Vec<FieldState> = func
                .inputs
                .iter()
                .map(|_| FieldState::default())
                .collect();

            self.pending_action = PendingAction::CallMethod(func.clone(), address);
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

    async fn do_deploy(&mut self, args: Vec<DynSolValue>, target: BytecodeTarget) {
        let contract = match self.contract.as_ref() {
            Some(c) => c,
            None => {
                self.state.output.push_error("No contract loaded");
                return;
            }
        };

        let contract_name = contract.name.clone();
        let contract_path = match self.contract_path.clone() {
            Some(p) => p,
            None => {
                self.state.output.push_error("No contract path set");
                return;
            }
        };

        // Compile on demand for the selected target
        self.state.output.push_info(format!("Compiling {} for {}...", contract_name, target));

        let compiled = match crate::compile::compile_contract(&contract_path, &contract_name, target) {
            Ok(c) => c,
            Err(e) => {
                self.state.output.push_error(format!("Compilation failed: {}", e));
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

        self.state.output.push("Sending deployment transaction...", OutputStyle::Waiting);

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

        self.state.output.push_separator();
        self.state.output.scroll_to_bottom();
    }

    async fn do_call_function(
        &mut self,
        func: &Function,
        address: Address,
        args: Vec<DynSolValue>,
    ) {
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
                    self.state.output.push_error(format!("Call failed: {}", e));
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

            self.state.output.push_success(format!("Result: {}", result_str));
        } else {
            let tx = TransactionRequest::default()
                .to(address)
                .input(calldata.into());

            self.state.output.push("Sending transaction...", OutputStyle::Waiting);

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
            self.state.output.push(format!(">>> {} <<<", call_str), OutputStyle::Highlight);

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
        }

        self.state.output.push_separator();
        self.state.output.scroll_to_bottom();
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
