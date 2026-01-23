use alloy::dyn_abi::{DynSolType, DynSolValue, EventExt, FunctionExt, JsonAbiExt};
use alloy::json_abi::{Function, JsonAbi};
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use ratatui::widgets::Widget;
use separator::Separatable;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::compile::{BytecodeTarget, CompiledContract};

type AbiCache = RefCell<HashMap<PathBuf, Vec<(String, Arc<JsonAbi>)>>>;

use crate::prompts;
use crate::store::{ContractId, DeploymentStore};
use crate::tui::layout::AppLayout;
use crate::tui::state::{
    AppState, ConnectionStatus, FieldState, Focus, OutputStyle, PopupState,
};
use crate::tui::widgets::{
    AutocompleteInput, CommandPalette, ContractTree, DebugBarWidget, OutputArea, ParameterPopup,
    StatusBarWidget, parse_path_for_autocomplete, scan_path_suggestions,
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
    pub signer_address: Address,
    running: bool,
    pending_action: PendingAction,
    edit_config_requested: bool,
    /// Content to display in external editor. Set this field and the main loop
    /// will handle terminal restore, editor launch, and terminal re-setup.
    pending_editor_content: Option<String>,
    /// Cache of loaded ABIs to avoid re-parsing files on every render
    abi_cache: AbiCache,
}

impl<P: Provider + Clone> App<P> {
    pub fn new(provider: P, store: DeploymentStore, signer_address: Address) -> Self {
        let state = AppState {
            account: Some(signer_address),
            ..Default::default()
        };

        Self {
            provider,
            store,
            state,
            contract: None,
            contract_path: None,
            address: None,
            signer_address,
            running: true,
            pending_action: PendingAction::None,
            edit_config_requested: false,
            pending_editor_content: None,
            abi_cache: RefCell::new(HashMap::new()),
        }
    }

    /// Attempt to connect to RPC, returns true if successful.
    /// On success, updates connection status, chain_id, and balance.
    /// On failure, stores the error message for display.
    pub async fn try_connect(&mut self) -> bool {
        match self.provider.get_chain_id().await {
            Ok(chain_id) => {
                self.state.chain_id = Some(chain_id);
                self.state.connection = ConnectionStatus::Connected;
                self.state.connection_error = None;
                
                // Fetch balance
                if let Ok(balance) = self.provider.get_balance(self.signer_address).await {
                    self.state.balance = Some(format_ether(balance));
                }
                
                log::info!("Connected to chain ID: {chain_id}");
                
                // Update connection card if it exists
                self.update_connection_card();
                true
            }
            Err(e) => {
                let error_msg = e.to_string();
                log::warn!("Connection failed: {error_msg}");
                self.state.connection = ConnectionStatus::Disconnected;
                self.state.connection_error = Some(error_msg);
                false
            }
        }
    }

    /// Add the connection card (should be called once at startup)
    pub fn add_connection_card(&mut self) {
        let card = crate::cards::Card::Connection {
            connected: matches!(self.state.connection, ConnectionStatus::Connected),
            account: self.signer_address,
            balance: self.state.balance.clone(),
            chain_id: self.state.chain_id,
            error: self.state.connection_error.clone(),
        };
        self.state.cards.cards.insert(0, card);
    }

    /// Update the connection card with current state (called after reconnection)
    fn update_connection_card(&mut self) {
        if let Some(crate::cards::Card::Connection { connected, balance, chain_id, error, .. }) = 
            self.state.cards.cards.first_mut() 
        {
            *connected = matches!(self.state.connection, ConnectionStatus::Connected);
            *balance = self.state.balance.clone();
            *chain_id = self.state.chain_id;
            *error = self.state.connection_error.clone();
        }
    }

    pub fn set_contract(&mut self, contract: CompiledContract, path: PathBuf) {
        let contract_name = contract.name.clone();
        self.contract = Some(contract);
        self.contract_path = Some(path.clone());
        self.address = None;

        // Expand the contract (use canonicalized path + name for per-contract expansion)
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());
        self.state.sidebar.expanded_contracts.insert((canonical_path, contract_name.clone()));

        // Select the contract in the sidebar (must match both path AND name for multi-contract files)
        self.select_contract_in_sidebar(&path, &contract_name);

        // Ensure the contract is in the store (with empty deployments if not yet deployed)
        let contract_id = ContractId::new(path, contract_name);
        self.store.ensure_contract(&contract_id);
        if let Err(e) = self.store.save() {
            self.state.output.push_error(format!("Failed to save contract: {e}"));
        }
    }

    /// Find and select a contract by path and name in the sidebar.
    /// For files with multiple contracts, we must match both path AND name.
    fn select_contract_in_sidebar(&mut self, path: &Path, contract_name: &str) {
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let nodes = self.build_tree_nodes();
        for (i, node) in nodes.iter().enumerate() {
            if let TreeNode::Contract { path: node_path, name: node_name } = node {
                // Match both path AND contract name (node_path is already canonicalized)
                if *node_path == canonical_path && node_name == contract_name {
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

    /// Refresh the account balance from the provider and update connection card
    async fn refresh_balance(&mut self) {
        match self.provider.get_balance(self.signer_address).await {
            Ok(balance) => {
                self.state.balance = Some(format_ether(balance));
                self.update_connection_card();
            }
            Err(e) => {
                log::warn!("Failed to refresh balance: {e}");
            }
        }
    }

    pub fn clear_state(&mut self) {
        self.contract = None;
        self.contract_path = None;
        self.address = None;
        self.state.sidebar = Default::default();
        self.store.clear();
        if let Err(e) = self.store.save() {
            self.state.output.push_error(format!("Failed to save after clearing: {e}"));
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

    /// Build tree nodes with ABI caching for performance
    fn build_tree_nodes(&self) -> Vec<TreeNode> {
        use crate::method_list::{self, MethodSelection};

        let mut nodes = Vec::new();

        // Always show "New contract" at top
        nodes.push(TreeNode::NewContract);

        // Get all contracts and sort them for stable ordering
        let mut all_contracts: Vec<ContractId> = self.store.all_contracts();

        // Add current contract if it's not already in the store
        // We canonicalize for comparison since the store uses canonicalized paths
        if let (Some(current_path), Some(current_contract)) = (&self.contract_path, &self.contract) {
            let current_canonical = current_path.canonicalize().unwrap_or_else(|_| current_path.clone());
            let current_id = ContractId::new(current_canonical.clone(), current_contract.name.clone());
            // Check if this exact contract (path + name) is in the store
            let is_in_store = all_contracts.iter().any(|c| {
                let c_canonical = c.path.canonicalize().unwrap_or_else(|_| c.path.clone());
                c_canonical == current_canonical && c.name == current_contract.name
            });

            if !is_in_store {
                all_contracts.push(current_id);
            }
        }

        // Sort for stable ordering (by path then name)
        all_contracts.sort_by(|a, b| {
            let path_cmp = a.path.cmp(&b.path);
            if path_cmp == std::cmp::Ordering::Equal {
                a.name.cmp(&b.name)
            } else {
                path_cmp
            }
        });

        // Add each contract in sorted order
        for contract_id in all_contracts {
            let contract_path = &contract_id.path;
            let name = &contract_id.name;
            let abi = self.get_abi_for_contract(&contract_id);

            nodes.push(TreeNode::Contract {
                name: name.clone(),
                path: contract_path.clone(),
            });

            // Check if this contract is expanded (by path + name)
            if self.state.sidebar.expanded_contracts.contains(&(contract_path.clone(), name.clone())) {
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

                // Show deployed instances for this contract
                let deployments = self.store.get_deployments(&contract_id);
                for address in &deployments {
                    nodes.push(TreeNode::DeployedInstance {
                        address: *address,
                        contract_name: name.clone(),
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

    /// Get ABI for a contract, using cache or current contract
    fn get_abi_for_contract(&self, contract_id: &ContractId) -> Arc<JsonAbi> {
        // Use current contract if this matches
        if let (Some(current_path), Some(current_contract)) = (&self.contract_path, &self.contract) {
            let current_canonical = current_path.canonicalize().unwrap_or_else(|_| current_path.clone());
            let contract_canonical = contract_id.path.canonicalize().unwrap_or_else(|_| contract_id.path.clone());
            if current_canonical == contract_canonical && current_contract.name == contract_id.name {
                return Arc::new(current_contract.abi.clone());
            }
        }

        // Fall back to cached ABI loading
        if let Some(contracts) = self.load_contract_abi_cached(&contract_id.path) {
            for (name, abi) in contracts {
                if name == contract_id.name {
                    return abi;
                }
            }
        }

        // Last resort: empty ABI
        Arc::new(JsonAbi::new())
    }

    pub async fn run_interactive(&mut self) -> Result<()> {
        let mut terminal = crate::tui::setup()?;
        let mut output_area = ratatui::layout::Rect::default();
        
        // Reconnection polling state
        let mut last_reconnect_attempt = std::time::Instant::now();
        let reconnect_interval = std::time::Duration::from_secs(5);

        while self.running {
            // Poll for reconnection if disconnected
            if matches!(self.state.connection, ConnectionStatus::Disconnected)
                && last_reconnect_attempt.elapsed() >= reconnect_interval
            {
                last_reconnect_attempt = std::time::Instant::now();
                self.try_connect().await;
            }
            // Check if we need to display content in editor
            if let Some(content) = self.pending_editor_content.take() {
                // Restore terminal before launching editor
                crate::tui::restore(&mut terminal)?;

                // Display in editor
                if let Err(e) = self.display_in_editor_impl(&content) {
                    self.state.output.push_error(format!("Editor error: {e}"));
                }

                // Re-setup terminal
                terminal = crate::tui::setup()?;
                continue;
            }

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
                        
                        // Update signer address from new config (if private key changed)
                        if let Ok(new_signer) = self.store.config.private_key
                            .strip_prefix("0x")
                            .unwrap_or(&self.store.config.private_key)
                            .parse::<alloy::signers::local::PrivateKeySigner>()
                        {
                            let new_address = new_signer.address();
                            if new_address != self.signer_address {
                                self.signer_address = new_address;
                                self.state.account = Some(new_address);
                                self.state.output.push_info("Account updated from config");
                            }
                        }
                        
                        self.state.output.push_success("Config reloaded");
                        self.state.output.push_info("Note: RPC URL changes require restart");
                    }
                    Err(e) => {
                        self.state.output.push_error(format!("Failed to reload config: {e}"));
                    }
                }
                
                // Refresh connection status and balance with new account
                self.try_connect().await;

                // Re-setup terminal
                terminal = crate::tui::setup()?;
                continue;
            }

            terminal.draw(|f| {
                let layout = AppLayout::new(f.area(), self.state.debug_mode);
                output_area = layout.output;
                self.state.output_area_height = layout.output.height.saturating_sub(4);
                self.render(f);
            })?;

            if let Some(event) = crate::tui::poll_event()? {
                match event {
                    InputEvent::Key(key) => self.handle_key(key).await?,
                    InputEvent::Resize(width, height) => {
                        // Update terminal size
                        self.state.terminal_size = (width, height);
                        // Check minimum size (80 characters wide)
                        self.state.terminal_too_small = width < 80;
                        // Redraw will happen automatically on next loop iteration
                    }
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

        let config_path = self.store.config_path();

        // Ensure config file exists
        if !config_path.exists() {
            self.store.save()?;
        }

        // Get editor from environment, fallback to common editors
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| "vi".to_string());

        Command::new(&editor)
            .arg(config_path)
            .status()
            .with_context(|| format!("Failed to open {} with {editor}", config_path.display()))?;

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        // Check if terminal is too small
        if self.state.terminal_too_small {
            use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
            use ratatui::style::{Color, Style};
            use ratatui::text::{Line, Span};

            let area = crate::tui::layout::centered_popup(frame.area(), 60, 30);
            let warning_text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "⚠ Terminal Too Small",
                    Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD)
                )),
                Line::from(""),
                Line::from("Please resize your terminal to at least 80 characters wide."),
                Line::from(""),
                Line::from(Span::styled(
                    format!("Current size: {} × {}", self.state.terminal_size.0, self.state.terminal_size.1),
                    Style::default().fg(Color::DarkGray)
                )),
            ];

            let paragraph = Paragraph::new(warning_text)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)))
                .wrap(Wrap { trim: true });

            frame.render_widget(paragraph, area);
            return;
        }

        let layout = AppLayout::new(frame.area(), self.state.debug_mode);

        if let Some(debug_area) = layout.debug_bar {
            let debug = DebugBarWidget::new(&self.state);
            frame.render_widget(debug, debug_area);
        }

        let nodes = self.build_tree_nodes();
        let tree = ContractTree::new(&self.state.sidebar)
            .focused(matches!(self.state.focus, Focus::Sidebar))
            .with_nodes(nodes);
        frame.render_widget(tree, layout.sidebar);

        let output = OutputArea::new(&self.state.output, &self.state.cards)
            .focused(matches!(self.state.focus, Focus::Output));
        frame.render_widget(output, layout.output);

        let status = StatusBarWidget::new(&self.state);
        frame.render_widget(status, layout.status_bar);

        // Render popups
        match &self.state.popup {
            PopupState::None => {}
            PopupState::CommandPalette { query, selected } => {
                let palette = CommandPalette::new(query, *selected, self.state.debug_mode);
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
            PopupState::TracerMenu { card_index: _, tracers, selected } => {
                self.render_tracer_menu(frame, tracers, *selected);
            }
            PopupState::TracerConfig { card_index: _, config, current } => {
                self.render_tracer_config(frame, config, *current);
            }
            PopupState::CopyMenu { card_index: _, options, selected } => {
                self.render_copy_menu(frame, options, *selected);
            }
        }
    }

    fn render_file_picker(&self, frame: &mut Frame, path: &str, error: Option<&str>) {
        use crate::tui::widgets::{KeyboardHints, Popup};

        let area = frame.area();
        let popup = Popup::new("Load Contract")
            .width_percent(60)
            .height_percent(40);
        let inner = popup.render_frame(area, frame.buffer_mut());

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

        let hints = KeyboardHints::new(vec![
            ("↑/↓", "navigate"),
            ("Tab", "complete"),
            ("Enter", "accept"),
            ("Esc", "cancel"),
        ]);
        let hints_y = inner.y + inner.height - 1;
        let hints_area = ratatui::layout::Rect::new(inner.x + 1, hints_y, inner.width.saturating_sub(2), 1);
        frame.render_widget(hints, hints_area);
    }

    fn render_address_input(&self, frame: &mut Frame, address: &str, error: Option<&str>) {
        use crate::tui::widgets::{InputField, Popup};

        let area = frame.area();
        let popup = Popup::new("Enter Address")
            .width_percent(60)
            .height_percent(20);
        let inner = popup.render_frame(area, frame.buffer_mut());

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
        use crate::tui::widgets::{Popup, SelectableList};

        let area = frame.area();
        let popup = Popup::new("Select Contract")
            .width_percent(50)
            .height_percent(40);
        let inner = popup.render_frame(area, frame.buffer_mut());

        let list = SelectableList::simple(contracts, selected);
        frame.render_widget(list, inner);
    }

    fn update_file_picker_suggestions(&mut self, input: &str) {
        let (dir, prefix) = parse_path_for_autocomplete(input);
        self.state.file_picker_suggestions = scan_path_suggestions(&dir, &prefix);
        self.state.file_picker_selected_idx = 0;
    }

    async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        let key_str = format_key_event(&key);
        self.state.last_key = Some(key_str.clone());
        self.state.last_action = None;

        let popup_state = match &self.state.popup {
            PopupState::None => "None",
            PopupState::CommandPalette { .. } => "CommandPalette",
            PopupState::ParameterPopup { .. } => "ParameterPopup",
            PopupState::FilePicker { .. } => "FilePicker",
            PopupState::AddressInput { .. } => "AddressInput",
            PopupState::ContractSelector { .. } => "ContractSelector",
            PopupState::TracerMenu { .. } => "TracerMenu",
            PopupState::TracerConfig { .. } => "TracerConfig",
            PopupState::CopyMenu { .. } => "CopyMenu",
        };
        log::trace!("[KEY] {} | focus={:?} popup={}", key_str, self.state.focus, popup_state);

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') => {
                    self.state.last_action = Some("Quit".into());
                    self.running = false;
                    return Ok(());
                }
                KeyCode::Char('p') => {
                    self.state.last_action = Some("Open CommandPalette".into());
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

        // Global card action shortcuts (work regardless of focus, but only when no popup is open)
        if matches!(self.state.popup, PopupState::None) && !self.state.cards.cards.is_empty() {
            let card_index = self.state.cards.selected_index;
            if card_index < self.state.cards.cards.len() {
                match key.code {
                    KeyCode::Char('r') => {
                        // View Receipt shortcut for Transaction cards
                        if matches!(&self.state.cards.cards[card_index], crate::cards::Card::Transaction { .. }) {
                            self.handle_card_action(card_index, crate::cards::CardAction::ViewReceipt).await?;
                            return Ok(());
                        }
                    }
                    KeyCode::Char('d') => {
                        // Debug Trace/Call shortcut
                        match &self.state.cards.cards[card_index] {
                            crate::cards::Card::Transaction { .. } => {
                                self.handle_card_action(card_index, crate::cards::CardAction::DebugTrace).await?;
                                return Ok(());
                            }
                            crate::cards::Card::Call { .. } => {
                                self.handle_card_action(card_index, crate::cards::CardAction::DebugCall).await?;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char('c') => {
                        // Copy shortcut for Transaction cards
                        if matches!(&self.state.cards.cards[card_index], crate::cards::Card::Transaction { .. }) {
                            self.handle_card_action(card_index, crate::cards::CardAction::Copy).await?;
                            return Ok(());
                        }
                    }
                    _ => {}
                }
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
            PopupState::TracerMenu { .. } => self.handle_tracer_menu_key(key).await?,
            PopupState::TracerConfig { .. } => self.handle_tracer_config_key(key).await?,
            PopupState::CopyMenu { .. } => self.handle_copy_menu_key(key).await?,
        }

        Ok(())
    }

    async fn handle_main_key(&mut self, key: KeyEvent) -> Result<()> {
        match self.state.focus {
            Focus::Sidebar => self.handle_sidebar_key(key).await?,
            Focus::Output => self.handle_output_key(key).await?,
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
                    self.state.last_action = Some("Move up".into());
                    if self.state.sidebar.selected < self.state.sidebar.scroll_offset {
                        self.state.sidebar.scroll_offset = self.state.sidebar.selected;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.state.sidebar.selected + 1 < node_count {
                    self.state.sidebar.selected += 1;
                    self.state.last_action = Some("Move down".into());
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    match node {
                        TreeNode::Contract { path, name } => {
                            let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());
                            self.state.sidebar.expanded_contracts.remove(&(canonical_path, name.clone()));
                            self.state.last_action = Some("Collapse contract".into());
                        }
                        TreeNode::DeployedInstance { address, .. } => {
                            self.state.sidebar.expanded_instances.remove(address);
                            self.state.last_action = Some("Collapse instance".into());
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    match node {
                        TreeNode::Contract { path, name } => {
                            let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());
                            self.state.sidebar.expanded_contracts.insert((canonical_path, name.clone()));
                            self.state.last_action = Some("Expand contract".into());
                        }
                        TreeNode::DeployedInstance { address, .. } => {
                            self.state.sidebar.expanded_instances.insert(*address);
                            self.set_address(*address);
                            self.state.last_action = Some("Expand instance".into());
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    self.state.last_action = Some(format!("Execute: {:?}", node));
                    self.execute_tree_node(node.clone()).await?;
                }
            }
            KeyCode::Delete | KeyCode::Backspace => {
                if let Some(node) = nodes.get(self.state.sidebar.selected) {
                    let removed = match node {
                        TreeNode::DeployedInstance { address, contract_name, contract_path, .. } => {
                            let contract_id = ContractId::new(contract_path.clone(), contract_name.clone());
                            if self.store.remove_deployment(&contract_id, *address) {
                                self.state.output.push_info(format!("Removed deployment: {address:?}"));
                                // Clear expanded state for this instance
                                self.state.sidebar.expanded_instances.remove(address);
                                if self.address == Some(*address) {
                                    self.address = None;
                                }
                                true
                            } else {
                                false
                            }
                        }
                        TreeNode::Contract { path, name, .. } => {
                            let contract_id = ContractId::new(path.clone(), name.clone());
                            if self.store.remove_contract(&contract_id) {
                                self.state.output.push_info(format!("Removed contract: {name}"));
                                // Clear expanded state for this contract
                                self.state.sidebar.expanded_contracts.remove(&(path.clone(), name.clone()));
                                // Check if this is the currently loaded contract (must match both path AND name)
                                let is_current = self.contract_path.as_ref().map(|p| {
                                    p.canonicalize().unwrap_or_else(|_| p.clone()) == *path
                                }).unwrap_or(false) && self.contract.as_ref().map(|c| &c.name == name).unwrap_or(false);
                                if is_current {
                                    // Clear the current contract state so it doesn't get re-added
                                    self.contract = None;
                                    self.contract_path = None;
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
                        if new_count == 0 {
                            self.state.sidebar.selected = 0;
                            self.state.sidebar.scroll_offset = 0;
                        } else if self.state.sidebar.selected >= new_count {
                            self.state.sidebar.selected = new_count - 1;
                        }
                    }
                }
            }
            KeyCode::Tab => {
                self.state.focus = Focus::Output;
                self.state.last_action = Some("Focus Output".into());
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_output_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                // If cards exist, navigate cards; otherwise scroll lines
                if !self.state.cards.cards.is_empty() {
                    if self.state.cards.selected_index > 0 {
                        self.state.cards.selected_index -= 1;
                    } else if !self.state.cards.cards.is_empty() {
                        self.state.cards.selected_index = self.state.cards.cards.len() - 1;
                    }
                    // Update scroll offset to keep selected card visible
                    let viewport_height = self.state.output_area_height as usize;
                    self.state.cards.scroll_offset = self.state.cards.calculate_scroll_offset(viewport_height);
                } else {
                    if self.state.output.scroll_offset > 0 {
                        self.state.output.scroll_offset -= 1;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // If cards exist, navigate cards; otherwise scroll lines
                if !self.state.cards.cards.is_empty() {
                    self.state.cards.selected_index = (self.state.cards.selected_index + 1) % self.state.cards.cards.len();
                    // Update scroll offset to keep selected card visible
                    let viewport_height = self.state.output_area_height as usize;
                    self.state.cards.scroll_offset = self.state.cards.calculate_scroll_offset(viewport_height);
                } else {
                    let max_scroll = self.state.output.lines.len().saturating_sub(10);
                    if self.state.output.scroll_offset < max_scroll {
                        self.state.output.scroll_offset += 1;
                    }
                }
            }
            KeyCode::Tab => {
                self.state.focus = Focus::Sidebar;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_command_palette_key(&mut self, key: KeyEvent) -> Result<()> {
        if let PopupState::CommandPalette { query, selected } = &mut self.state.popup {
            match key.code {
                KeyCode::Esc => {
                    self.state.popup = PopupState::None;
                    self.state.focus = Focus::Sidebar;
                }
                KeyCode::Enter => {
                    let commands = default_commands(self.state.debug_mode);
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
                    let commands = default_commands(self.state.debug_mode);
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
            if let PopupState::ParameterPopup { params, fields, bytecode_target, method_name, .. } = &self.state.popup {
                // Log parameter submission
                let field_values: Vec<_> = fields.iter().map(|f| f.value.as_str()).collect();
                log::info!(
                    "[PARAMS] Submitting {} with values: {:?} (pending_action={:?})",
                    method_name,
                    field_values,
                    match &self.pending_action {
                        PendingAction::None => "None".to_string(),
                        PendingAction::Deploy { contract_name, .. } => format!("Deploy({})", contract_name),
                        PendingAction::CallMethod { function, address } => format!("Call({} @ {:?})", function.name, address),
                    }
                );

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
                            format!("{new_path}/")
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
                        *path = format!("{new_path}/");
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
                            log::info!("[ADDRESS] set_address: {:?} for contract {:?}", addr, self.contract.as_ref().map(|c| &c.name));
                            self.state.popup = PopupState::None;
                            self.state.focus = Focus::Sidebar;
                            self.set_address(addr);
                        }
                        Err(_) => {
                            log::warn!("[ADDRESS] Invalid address format: {}", address);
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
        log::info!("[ACTION] execute_tree_node: {}", node.label());
        match node {
            TreeNode::NewContract => {
                self.state.popup = PopupState::FilePicker {
                    path: String::new(),
                    error: None,
                };
                self.state.focus = Focus::CommandPalette;
                self.update_file_picker_suggestions("");
            }
            TreeNode::Contract { path, name } => {
                // Toggle expansion for this contract (path is already canonicalized from tree)
                let key = (path.clone(), name.clone());
                if self.state.sidebar.expanded_contracts.contains(&key) {
                    self.state.sidebar.expanded_contracts.remove(&key);
                } else {
                    self.state.sidebar.expanded_contracts.insert(key);
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
        let command_names = ["Edit config", "Clear output", "Open Logs", "Clear Logs", "Reconnect", "Toggle Debug", "Reset", "Quit"];
        let cmd_name = command_names.get(command_idx).unwrap_or(&"Unknown");
        log::info!("[COMMAND] execute_command: {} (idx={})", cmd_name, command_idx);
        match command_idx {
            0 => {
                self.edit_config_requested = true;
            }
            1 => {
                self.state.output.clear();
            }
            2 => {
                if let Some(home) = std::env::var_os("HOME") {
                    let log_path = std::path::PathBuf::from(home).join(".evm-cli/output.log");
                    if log_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&log_path) {
                            self.pending_editor_content = Some(content);
                        } else {
                            self.state.output.push_error("Failed to read log file");
                        }
                    } else {
                        self.state.output.push_info("Log file does not exist yet");
                    }
                }
            }
            3 => {
                if let Some(home) = std::env::var_os("HOME") {
                    let log_path = std::path::PathBuf::from(home).join(".evm-cli/output.log");
                    if log_path.exists() {
                        if let Err(e) = std::fs::remove_file(&log_path) {
                            self.state.output.push_error(format!("Failed to clear log file: {e}"));
                        } else {
                            self.state.output.push_success("Log file cleared");
                        }
                    } else {
                        self.state.output.push_info("Log file does not exist");
                    }
                }
            }
            4 => {
                self.try_connect().await;
            }
            5 => {
                self.state.debug_mode = !self.state.debug_mode;
                let status = if self.state.debug_mode { "enabled" } else { "disabled" };
                self.state.last_action = Some(format!("Debug {}", status));
            }
            6 => {
                self.clear_state();
            }
            7 => {
                self.running = false;
            }
            _ => {}
        }
        Ok(())
    }

    async fn load_contract_from_path(&mut self, path: PathBuf) -> Result<()> {
        log::info!("[LOAD] load_contract_from_path: {:?}", path);
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
                let error_msg = format!("Failed to load contract: {e}");
                log::error!("{error_msg}");
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
        self.state.output.push_normal(format!("\nPreparing to deploy {contract_name}..."));

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
        log::info!("[ACTION] start_call_function: {}() at {:?}", func.name, address);
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
        log::info!(
            "[DEPLOY] do_deploy: {} from {:?} with args {:?} target={:?}",
            contract_name,
            contract_path,
            args,
            target
        );
        // Check connection status
        if matches!(self.state.connection, ConnectionStatus::Disconnected) {
            self.add_log_card("Cannot deploy: not connected to RPC".to_string());
            return;
        }
        
        // Compile on demand for the selected target
        self.state.output.push_info(format!("Compiling {contract_name} for {target}..."));

        let compiled = match crate::compile::compile_contract(&contract_path, &contract_name, target) {
            Ok(c) => c,
            Err(e) => {
                let error_msg = format!("Compilation failed: {e}");
                log::error!("{error_msg}");
                self.state.output.push_error(error_msg);
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                return;
            }
        };

        self.state.output.push_success(format!("Compilation successful ({target})"));

        let mut deploy_data = compiled.bytecode.clone();

        if !args.is_empty() {
            let encoded = DynSolValue::Tuple(args).abi_encode_params();
            deploy_data.extend(encoded);
        }

        let mut tx = TransactionRequest::default().with_deploy_code(deploy_data.clone());
        if let Some(chain_id) = self.state.chain_id {
            tx = tx.with_chain_id(chain_id);
        }

        self.state.output.push(
            format!("Deploying {contract_name} contract..."),
            OutputStyle::Waiting
        );

        let pending = match self.provider.send_transaction(tx.clone()).await {
            Ok(p) => p,
            Err(e) => {
                let error_msg = format!("Deployment failed: {e}");
                self.state.output.push_error(&error_msg);
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                // Add a log card so the error is visible in the card view
                self.add_log_card(format!("Failed: Deploy {contract_name}\n\n{error_msg}"));
                return;
            }
        };

        let tx_hash = *pending.tx_hash();
        self.state.output.push_success(format!("Transaction: {tx_hash:?}"));
        self.state.output.push("Waiting for confirmation...", OutputStyle::Waiting);

        let receipt = match pending.get_receipt().await {
            Ok(r) => r,
            Err(e) => {
                let error_msg = format!("Failed to get receipt: {e}");
                self.state.output.push_error(&error_msg);
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                // Add a log card so the error is visible in the card view
                self.add_log_card(format!("Failed: Deploy {contract_name} (tx: {tx_hash:?})\n\n{error_msg}"));
                return;
            }
        };

        let address = match receipt.contract_address {
            Some(a) => a,
            None => {
                let error_msg = "No contract address in receipt";
                self.state.output.push_error(error_msg);
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                // Add a log card so the error is visible in the card view
                self.add_log_card(format!("Failed: Deploy {contract_name} (tx: {tx_hash:?})\n\n{error_msg}"));
                return;
            }
        };

        // Create a transaction card with actual status and gas from receipt
        let (status, error_message) = if receipt.status() {
            (crate::cards::TransactionStatus::Success, None)
        } else {
            // Deployment reverted - try to get the revert reason
            let error_msg = self.get_revert_reason(&tx, receipt.block_number).await;
            (crate::cards::TransactionStatus::Failed, Some(error_msg))
        };
        let gas_used = receipt.gas_used.separated_string();
        self.add_transaction_card(
            tx_hash,
            status,
            format!("Deploy {contract_name}"),
            Some(gas_used.clone()),
            contract_name.to_string(),
            Some(address),
            error_message.clone(),
        );

        if !receipt.status() {
            let error_display = error_message.as_deref().unwrap_or("Unknown reason");
            self.state.output.push_error(format!("Deployment reverted: {error_display}"));
            self.state.output.push_separator();
            self.state.output.scroll_to_bottom();
            return;
        }

        self.state.output.push_success(format!("Deployed at: {address:?}"));

        self.set_address(address);

        let contract_id = ContractId::new(contract_path.clone(), contract_name.to_string());
        self.store.add_deployment(&contract_id, address);
        if let Err(e) = self.store.save() {
            self.state.output.push_error(format!("Failed to save deployment: {e}"));
        }

        // Expand and select the newly deployed instance in the sidebar
        self.state.sidebar.expanded_instances.insert(address);
        self.select_instance_in_sidebar(address);

        self.state.output.push_separator();
        self.state.output.scroll_to_bottom();

        // Refresh balance after transaction
        self.refresh_balance().await;
    }

    async fn do_call_function(
        &mut self,
        func: &Function,
        address: Address,
        args: Vec<DynSolValue>,
    ) {
        let contract_name = self.contract.as_ref().map(|c| c.name.as_str()).unwrap_or("Unknown");
        log::info!(
            "[CALL] do_call_function: {}.{}({:?}) at {:?}",
            contract_name,
            func.name,
            args,
            address
        );

        let is_view = matches!(
            func.state_mutability,
            alloy::json_abi::StateMutability::View | alloy::json_abi::StateMutability::Pure
        );

        // Check connection status for state-changing functions
        if !is_view && matches!(self.state.connection, ConnectionStatus::Disconnected) {
            self.add_log_card(format!("Cannot call {}: not connected to RPC", func.name));
            return;
        }

        let calldata = match func.abi_encode_input(&args) {
            Ok(data) => data,
            Err(e) => {
                self.state.output.push_error(format!("Failed to encode function call: {e}"));
                self.state.output.push_separator();
                self.state.output.scroll_to_bottom();
                return;
            }
        };

        if is_view {
            let tx = TransactionRequest::default()
                .to(address)
                .input(calldata.into());

            let result = match self.provider.call(tx).await {
                Ok(r) => r,
                Err(e) => {
                    self.state.output.push_error(format!("Call to {contract_name} {address:?} failed: {e}"));
                    self.state.output.push_separator();
                    self.state.output.scroll_to_bottom();
                    return;
                }
            };

            let decoded = match func.abi_decode_output(&result) {
                Ok(d) => d,
                Err(e) => {
                    self.state.output.push_error(format!("Failed to decode return value: {e}"));
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
            self.state.output.push(format!("{call_str} @ {address:?}"), OutputStyle::Highlight);
            self.state.output.push_success(format!("Result: {result_str}"));

            // Add a call card for view/pure calls
            if let Some(from_addr) = self.address {
                self.add_call_card(from_addr, address, call_str.clone(), result_str.clone());
            } else {
                // Fallback to log card if no account is available
                self.add_log_card(format!("{} completed successfully", func.name));
            }
        } else {
            let mut tx = TransactionRequest::default()
                .to(address)
                .input(calldata.into());
            if let Some(chain_id) = self.state.chain_id {
                tx = tx.with_chain_id(chain_id);
            }

            self.state.output.push(
                format!("Sending transaction to {contract_name} {address:?}..."),
                OutputStyle::Waiting
            );

            let pending = match self.provider.send_transaction(tx.clone()).await {
                Ok(p) => p,
                Err(e) => {
                    let error_msg = format!("Transaction failed: {e}");
                    self.state.output.push_error(&error_msg);
                    self.state.output.push_separator();
                    self.state.output.scroll_to_bottom();
                    // Add a log card so the error is visible in the card view
                    self.add_log_card(format!("Failed: {} @ {address:?}\n\n{error_msg}", func.name));
                    return;
                }
            };

            let tx_hash = *pending.tx_hash();
            self.state.output.push_success(format!("Transaction: {tx_hash:?}"));

            let call_str = prompts::format_method_call(&func.name, &func.inputs, &args);
            self.state.output.push(format!("{call_str} @ {address:?}"), OutputStyle::Highlight);

            self.state.output.push("Waiting for confirmation...", OutputStyle::Waiting);

            let receipt = match pending.get_receipt().await {
                Ok(r) => r,
                Err(e) => {
                    let error_msg = format!("Failed to get receipt: {e}");
                    self.state.output.push_error(&error_msg);
                    self.state.output.push_separator();
                    self.state.output.scroll_to_bottom();
                    // Add a log card so the error is visible in the card view
                    self.add_log_card(format!("Failed: {} @ {address:?} (tx: {tx_hash:?})\n\n{error_msg}", func.name));
                    return;
                }
            };

            // Create a transaction card with actual status and gas from receipt
            let (status, error_message) = if receipt.status() {
                (crate::cards::TransactionStatus::Success, None)
            } else {
                // Transaction reverted - try to get the revert reason by simulating the call
                let error_msg = self.get_revert_reason(&tx, receipt.block_number).await;
                (crate::cards::TransactionStatus::Failed, Some(error_msg))
            };
            let gas_used = receipt.gas_used.separated_string();
            self.add_transaction_card(
                tx_hash,
                status,
                call_str.clone(),
                Some(gas_used.clone()),
                contract_name.to_string(),
                Some(address),
                error_message.clone(),
            );

            if receipt.status() {
                self.state.output.push_success("Status: Success");
            } else {
                let error_display = error_message.as_deref().unwrap_or("Unknown reason");
                self.state.output.push_error(format!("Transaction reverted: {error_display}"));
            }

            self.state.output.push_info(format!("Gas used: {gas_used}"));

            // Display logs if any
            let logs = receipt.inner.logs();
            if !logs.is_empty() {
                self.state.output.push_info(format!("Logs ({})", logs.len()));
                for (i, log) in logs.iter().enumerate() {
                    self.display_log(i, log);
                }
            }

            // Refresh balance after transaction
            self.refresh_balance().await;
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
                    log.data().data.clone(),
                );

                // Try to decode the log
                match event.decode_log(&log_data) {
                    Ok(decoded) => {
                        // Successfully decoded
                        self.state.output.push_info(format!("  [{index}] {} @ {log_address:?}", event.name));

                        // Display decoded parameters
                        for (param, value) in event.inputs.iter().zip(decoded.indexed.iter().chain(decoded.body.iter())) {
                            let value_str = prompts::format_return_value(value);
                            self.state.output.push_info(format!("      {}: {value_str}", param.name));
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
        self.state.output.push_info(format!("  [{index}] Address: {log_address:?}"));
        if !log.topics().is_empty() {
            self.state.output.push_info(format!("      Topics: {}",
                log.topics().iter()
                    .map(|t| format!("{t:?}"))
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
        for contract_id in self.store.all_contracts() {
            let deployments = self.store.get_deployments(&contract_id);
            if deployments.contains(&address) {
                // This contract has this address
                return Some(self.get_abi_for_contract(&contract_id));
            }
        }
        None
    }

    /// Try to get the revert reason for a failed transaction by simulating the call
    async fn get_revert_reason(&self, tx: &TransactionRequest, block_number: Option<u64>) -> String {
        // Use the block number from the receipt to simulate at the same state
        let block_id = block_number.map(alloy::eips::BlockId::number);
        
        // Try to call the transaction to get the revert reason
        let result = match block_id {
            Some(block) => self.provider.call(tx.clone()).block(block).await,
            None => self.provider.call(tx.clone()).await,
        };
        
        match result {
            Ok(_) => "Transaction reverted (no revert reason available)".to_string(),
            Err(e) => {
                let error_str = e.to_string();
                // Try to extract a meaningful error message
                // Common patterns: "execution reverted: <reason>", "revert: <reason>"
                if let Some(pos) = error_str.find("execution reverted:") {
                    let reason = error_str[pos + 19..].trim();
                    if reason.is_empty() {
                        "Execution reverted".to_string()
                    } else {
                        reason.to_string()
                    }
                } else if let Some(pos) = error_str.find("revert:") {
                    let reason = error_str[pos + 7..].trim();
                    if reason.is_empty() {
                        "Execution reverted".to_string()
                    } else {
                        reason.to_string()
                    }
                } else if error_str.contains("reverted") || error_str.contains("revert") {
                    error_str
                } else {
                    format!("Execution reverted: {error_str}")
                }
            }
        }
    }

    async fn handle_tracer_menu_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.state.popup = PopupState::None;
                self.state.focus = Focus::Output;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if let PopupState::TracerMenu { selected, tracers, .. } = &mut self.state.popup {
                    if *selected > 0 {
                        *selected -= 1;
                    } else {
                        *selected = tracers.len() - 1;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let PopupState::TracerMenu { selected, tracers, .. } = &mut self.state.popup {
                    *selected = (*selected + 1) % tracers.len();
                }
            }
            KeyCode::Enter => {
                if let PopupState::TracerMenu { card_index, tracers, selected } = self.state.popup.clone() {
                    let tracer = tracers[selected];
                    self.state.popup = PopupState::TracerConfig {
                        card_index,
                        config: crate::cards::TracerConfig {
                            tracer_type: tracer,
                            ..Default::default()
                        },
                        current: 0,
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_tracer_config_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.state.popup = PopupState::None;
                self.state.focus = Focus::Output;
            }
            KeyCode::Tab | KeyCode::Down => {
                // Move to next field
                if let PopupState::TracerConfig { config, current, .. } = &mut self.state.popup {
                    let field_count = Self::get_tracer_field_count(config.tracer_type);
                    *current = (*current + 1) % field_count;
                }
            }
            KeyCode::BackTab | KeyCode::Up => {
                // Move to previous field
                if let PopupState::TracerConfig { config, current, .. } = &mut self.state.popup {
                    let field_count = Self::get_tracer_field_count(config.tracer_type);
                    *current = current.checked_sub(1).unwrap_or(field_count - 1);
                }
            }
            KeyCode::Char(' ') => {
                // Toggle the current field
                if let PopupState::TracerConfig { config, current, .. } = &mut self.state.popup {
                    Self::toggle_tracer_field(config, *current);
                }
            }
            KeyCode::Enter => {
                if let PopupState::TracerConfig { card_index, config, .. } = self.state.popup.clone() {
                    self.state.popup = PopupState::None;
                    self.execute_debug_trace(card_index, &config).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_tracer_field_count(tracer_type: crate::cards::TracerType) -> usize {
        match tracer_type {
            crate::cards::TracerType::Call => 2,      // withLogs, onlyTopCall
            crate::cards::TracerType::Prestate => 1,  // diffMode
            crate::cards::TracerType::Execution => 3, // enableMemory, disableStack, disableStorage
        }
    }

    fn toggle_tracer_field(config: &mut crate::cards::TracerConfig, index: usize) {
        match config.tracer_type {
            crate::cards::TracerType::Call => match index {
                0 => config.with_logs = !config.with_logs,
                1 => config.only_top_call = !config.only_top_call,
                _ => {}
            },
            crate::cards::TracerType::Prestate => {
                if index == 0 {
                    config.diff_mode = !config.diff_mode;
                }
            }
            crate::cards::TracerType::Execution => match index {
                0 => config.enable_memory = !config.enable_memory,
                1 => config.disable_stack = !config.disable_stack,
                2 => config.disable_storage = !config.disable_storage,
                _ => {}
            },
        }
    }

    async fn handle_card_action(&mut self, card_index: usize, action: crate::cards::CardAction) -> Result<()> {
        log::info!("[CARD_ACTION] handle_card_action: {:?} on card_index={}", action, card_index);
        if card_index >= self.state.cards.cards.len() {
            return Ok(());
        }

        let card = &self.state.cards.cards[card_index];

        match action {
            crate::cards::CardAction::Copy => {
                let options = crate::cards::get_copy_options(card);
                if options.len() == 1 {
                    // Only one option (hash), copy directly
                    self.execute_copy(card_index, options[0]);
                } else if options.len() > 1 {
                    // Multiple options, show menu
                    self.state.popup = PopupState::CopyMenu {
                        card_index,
                        options,
                        selected: 0,
                    };
                }
            }
            crate::cards::CardAction::ViewReceipt => {
                if let crate::cards::Card::Transaction { hash, .. } = card {
                    self.execute_view_receipt(*hash).await?;
                    // Return to card view with selection preserved
                    self.state.focus = Focus::Output;
                }
            }
            crate::cards::CardAction::DebugTrace => {
                if let crate::cards::Card::Transaction { .. } = card {
                    let tracers = crate::cards::get_tracer_types();
                    self.state.popup = PopupState::TracerMenu {
                        card_index,
                        tracers,
                        selected: 0,
                    };
                }
            }
            crate::cards::CardAction::DebugCall => {
                if let crate::cards::Card::Call { .. } = card {
                    self.execute_debug_call(card_index).await?;
                    // Return to card view with selection preserved
                    self.state.focus = Focus::Output;
                }
            }
        }
        Ok(())
    }

    async fn execute_view_receipt(&mut self, tx_hash: alloy::primitives::TxHash) -> Result<()> {
        self.state.output.push("Fetching transaction receipt...", OutputStyle::Waiting);
        self.state.output.scroll_to_bottom();

        match self.provider.get_transaction_receipt(tx_hash).await? {
            Some(receipt) => {
                let json = serde_json::to_string_pretty(&receipt)?;
                self.display_in_editor(&json)?;
                self.state.output.push_success("Receipt displayed in editor");
            }
            None => {
                self.state.output.push_error("Receipt not found");
            }
        }
        self.state.output.push_separator();
        self.state.output.scroll_to_bottom();
        Ok(())
    }

    fn execute_copy(&mut self, card_index: usize, option: crate::cards::CopyOption) {
        if card_index >= self.state.cards.cards.len() {
            return;
        }

        let card = &self.state.cards.cards[card_index];

        if let crate::cards::Card::Transaction { hash, contract_address, .. } = card {
            let text = match option {
                crate::cards::CopyOption::Hash => format!("{hash:?}"),
                crate::cards::CopyOption::Address => {
                    if let Some(addr) = contract_address {
                        format!("{addr:?}")
                    } else {
                        self.state.output.push_error("No contract address available");
                        return;
                    }
                }
            };

            match copy_to_clipboard(&text) {
                Ok(_) => {
                    log::info!("Copied to clipboard: {text}");
                    self.state.output.push_success(format!("Copied: {text}"));
                }
                Err(e) => {
                    log::error!("Clipboard error: {e}");
                    self.state.output.push_error(format!("Failed to copy: {e}"));
                }
            }
            self.state.output.scroll_to_bottom();
        }
    }

    async fn handle_copy_menu_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.state.popup = PopupState::None;
                self.state.focus = Focus::Output;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if let PopupState::CopyMenu { selected, options, .. } = &mut self.state.popup {
                    if *selected > 0 {
                        *selected -= 1;
                    } else {
                        *selected = options.len() - 1;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let PopupState::CopyMenu { selected, options, .. } = &mut self.state.popup {
                    *selected = (*selected + 1) % options.len();
                }
            }
            KeyCode::Enter => {
                if let PopupState::CopyMenu { card_index, options, selected } = self.state.popup.clone() {
                    let option = options[selected];
                    self.state.popup = PopupState::None;
                    self.execute_copy(card_index, option);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn render_copy_menu(&self, frame: &mut Frame, options: &[crate::cards::CopyOption], selected: usize) {
        use crate::tui::theme;
        use crate::tui::widgets::{Popup, SelectableList};
        use ratatui::style::Style;
        use ratatui::text::Span;

        let area = frame.area();
        let popup = Popup::new("Copy")
            .width_percent(30)
            .height_percent(20);
        let inner = popup.render_frame(area, frame.buffer_mut());

        let list = SelectableList::new(options, selected, |option, is_selected| {
            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default().fg(theme::TEXT)
            };
            vec![Span::styled(format!("  {}  ", option), style)]
        });
        frame.render_widget(list, inner);
    }

    async fn execute_debug_call(&mut self, card_index: usize) -> Result<()> {
        if card_index >= self.state.cards.cards.len() {
            return Ok(());
        }

        let card = &self.state.cards.cards[card_index];

        if let crate::cards::Card::Call {
            to,
            function_signature,
            ..
        } = card
        {
            self.state.output.push(
                format!("Executing debug trace for call: {function_signature} @ {to}..."),
                OutputStyle::Waiting,
            );
            self.state.output.scroll_to_bottom();

            // Try to execute debug_traceCall with call tracer
            // This is a best-effort attempt; may fail if provider doesn't support debug API
            match self.execute_rpc_debug_call(*to).await {
                Ok(trace_json) => {
                    let json = serde_json::to_string_pretty(&trace_json)?;
                    self.display_in_editor(&json)?;
                    self.state.output.push_success("Debug trace displayed in editor");
                }
                Err(e) => {
                    self.state.output.push_error(format!(
                        "Debug trace failed: {e}. Your RPC provider may not support the debug API."
                    ));
                }
            }

            self.state.output.push_separator();
            self.state.output.scroll_to_bottom();
        }

        Ok(())
    }

    async fn execute_debug_trace(&mut self, card_index: usize, config: &crate::cards::TracerConfig) -> Result<()> {
        if card_index >= self.state.cards.cards.len() {
            return Ok(());
        }

        let card = &self.state.cards.cards[card_index];

        if let crate::cards::Card::Transaction { hash, .. } = card {
            self.state.output.push(
                format!("Executing debug_traceTransaction with {} tracer...", config.tracer_name()),
                OutputStyle::Waiting,
            );
            self.state.output.scroll_to_bottom();

            // Try to execute debug_traceTransaction
            // This requires a Geth-compatible RPC endpoint with debug API enabled
            match self.execute_rpc_debug_trace(*hash, config).await {
                Ok(trace_json) => {
                    let json = serde_json::to_string_pretty(&trace_json)?;
                    self.display_in_editor(&json)?;
                    self.state.output.push_success("Debug trace displayed in editor");
                }
                Err(e) => {
                    self.state.output.push_error(format!(
                        "Debug trace failed: {e}. Ensure your RPC provider supports the debug API (Geth, etc.)"
                    ));
                }
            }

            // Return to card view with selection preserved
            self.state.focus = Focus::Output;
            self.state.output.push_separator();
            self.state.output.scroll_to_bottom();
        }

        Ok(())
    }

    fn display_in_editor(&mut self, content: &str) -> Result<()> {
        // Set the content to be displayed; the main loop will handle terminal restore/setup
        self.pending_editor_content = Some(content.to_string());
        Ok(())
    }

    fn display_in_editor_impl(&self, content: &str) -> Result<()> {
        use std::fs;
        use std::io::Write;
        use std::process::Command;

        // Get EDITOR from environment, default to "vim"
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        // Create a temporary file in /tmp
        let temp_path = format!("/tmp/evm-cli-{}.json", chrono::Local::now().timestamp_millis());

        let mut file = fs::File::create(&temp_path)
            .with_context(|| format!("Failed to create temp file: {temp_path}"))?;

        file.write_all(content.as_bytes())?;
        file.flush()?;

        // Open in editor
        let status = Command::new(&editor)
            .arg(&temp_path)
            .status()
            .with_context(|| format!("Failed to open editor: {editor}"))?;

        // Clean up the temporary file
        let _ = fs::remove_file(&temp_path);

        // Return error if editor returned non-zero status
        if !status.success() {
            anyhow::bail!("Editor exited with error status");
        }

        Ok(())
    }

    fn render_tracer_menu(&self, frame: &mut Frame, tracers: &[crate::cards::TracerType], selected: usize) {
        use crate::tui::theme;
        use crate::tui::widgets::{Popup, SelectableList};
        use ratatui::style::Style;
        use ratatui::text::Span;

        let area = frame.area();
        let popup = Popup::new("Select Tracer")
            .width_percent(40)
            .height_percent(30);
        let inner = popup.render_frame(area, frame.buffer_mut());

        let list = SelectableList::new(tracers, selected, |tracer, is_selected| {
            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default().fg(theme::PRIMARY)
            };
            vec![
                Span::styled(if is_selected { "> " } else { "  " }, style),
                Span::styled(format!("{}", tracer), style),
            ]
        });
        frame.render_widget(list, inner);
    }

    fn render_tracer_config(&self, frame: &mut Frame, config: &crate::cards::TracerConfig, current: usize) {
        use crate::tui::theme;
        use crate::tui::widgets::{KeyboardHints, Popup};
        use ratatui::text::{Line, Span};

        let area = frame.area();
        let title = format!("{} Config", config.tracer_type);
        let popup = Popup::new(&title)
            .width_percent(50)
            .height_percent(40);
        let inner = popup.render_frame(area, frame.buffer_mut());

        let y = inner.y + 1;

        let render_toggle = |buf: &mut ratatui::buffer::Buffer, y: u16, label: &str, value: bool, is_focused: bool| {
            let label_style = if is_focused {
                theme::focused_label_style()
            } else {
                theme::label_style()
            };

            let on_style = if value {
                theme::selected_style()
            } else {
                theme::muted_style()
            };

            let off_style = if !value {
                theme::selected_style()
            } else {
                theme::muted_style()
            };

            let line = Line::from(vec![
                Span::styled(format!("{label}: "), label_style),
                Span::styled(" ON ", on_style),
                Span::raw("  "),
                Span::styled(" OFF ", off_style),
            ]);
            buf.set_line(inner.x + 1, y, &line, inner.width.saturating_sub(2));
        };

        match config.tracer_type {
            crate::cards::TracerType::Call => {
                render_toggle(frame.buffer_mut(), y, "withLogs", config.with_logs, current == 0);
                render_toggle(frame.buffer_mut(), y + 2, "onlyTopCall", config.only_top_call, current == 1);
            }
            crate::cards::TracerType::Prestate => {
                render_toggle(frame.buffer_mut(), y, "diffMode", config.diff_mode, current == 0);
            }
            crate::cards::TracerType::Execution => {
                render_toggle(frame.buffer_mut(), y, "enableMemory", config.enable_memory, current == 0);
                render_toggle(frame.buffer_mut(), y + 2, "disableStack", config.disable_stack, current == 1);
                render_toggle(frame.buffer_mut(), y + 4, "disableStorage", config.disable_storage, current == 2);
            }
        }

        let footer_y = inner.y + inner.height.saturating_sub(1);
        let hints = KeyboardHints::new(vec![
            ("↑↓", "navigate"),
            ("Space", "toggle"),
            ("Enter", "execute"),
            ("Esc", "cancel"),
        ]);
        let hints_area = ratatui::layout::Rect::new(inner.x + 1, footer_y, inner.width.saturating_sub(2), 1);
        hints.render(hints_area, frame.buffer_mut());
    }

    async fn execute_rpc_debug_trace(
        &mut self,
        tx_hash: alloy::primitives::TxHash,
        config: &crate::cards::TracerConfig,
    ) -> Result<serde_json::Value> {
        // Build the tracer config JSON matching Polkadot SDK format
        let tracer_config = config.to_json();

        // Get RPC URL from store config
        let rpc_url = &self.store.config.rpc_url;

        // Create the JSON-RPC request for debug_traceTransaction
        let request_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "debug_traceTransaction",
            "params": [
                format!("{tx_hash:?}"),
                tracer_config,  // Pass the full tracer config as second param
            ],
            "id": 1,
        });

        // Log the RPC request for debugging
        log::info!("🔍 Debug RPC Request: {}", serde_json::to_string_pretty(&request_payload).unwrap_or_default());

        // Try to make the actual RPC call
        let trace_result = match self.try_raw_rpc_call(rpc_url, &request_payload).await {
            Ok(result) => {
                log::info!("✓ Debug RPC Response received");
                result
            },
            Err(e) => {
                log::error!("✗ Debug RPC Error: {e}");
                // Fallback: Return informative placeholder with request details
                serde_json::json!({
                    "error": format!("{e}"),
                    "request_sent": request_payload,
                })
            }
        };

        Ok(trace_result)
    }

    async fn execute_rpc_debug_call(
        &mut self,
        contract_address: alloy::primitives::Address,
    ) -> Result<serde_json::Value> {
        // Get RPC URL from config
        let rpc_url = &self.store.config.rpc_url;

        // For debug_traceCall, we would need the actual call data
        // This is informational since we don't have the call data in the Card
        let trace_result = serde_json::json!({
            "type": "debug_traceCall",
            "contract_address": format!("{contract_address:?}"),
            "tracer": "callTracer",
            "rpc_url": rpc_url,
            "status": "debug_traceCall requires Geth-compatible RPC with debug API enabled",
            "instructions": [
                "This would execute debug_traceCall for the contract",
                "Full implementation requires storing call data with call cards",
                format!("RPC URL being used: {rpc_url}"),
                "Ensure your RPC provider supports debug API"
            ],
        });

        Ok(trace_result)
    }

    /// Attempt to make a raw JSON-RPC call to the configured RPC endpoint using Alloy's client
    async fn try_raw_rpc_call(
        &self,
        _rpc_url: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        // Extract method and params from payload
        let method = payload.get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("No method in payload"))?;

        let params = payload.get("params")
            .ok_or_else(|| anyhow::anyhow!("No params in payload"))?;

        // Use Alloy's client to make the raw RPC call
        // Match on method to use 'static str literals
        let result: serde_json::Value = match method {
            "debug_traceTransaction" => {
                self.provider
                    .raw_request::<_, serde_json::Value>("debug_traceTransaction".into(), params.clone())
                    .await
                    .with_context(|| "Failed to execute debug_traceTransaction")?
            }
            "debug_traceCall" => {
                self.provider
                    .raw_request::<_, serde_json::Value>("debug_traceCall".into(), params.clone())
                    .await
                    .with_context(|| "Failed to execute debug_traceCall")?
            }
            _ => {
                anyhow::bail!("Unsupported debug method: {method}");
            }
        };

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn add_transaction_card(
        &mut self,
        hash: alloy::primitives::TxHash,
        status: crate::cards::TransactionStatus,
        function: String,
        gas: Option<String>,
        contract_name: String,
        contract_address: Option<Address>,
        error_message: Option<String>,
    ) {
        let card = crate::cards::Card::Transaction {
            hash,
            status,
            function_name: function,
            gas_used: gas,
            contract_name,
            contract_address,
            error_message,
        };
        self.state.cards.cards.push(card);
        self.state.cards.selected_index = self.state.cards.cards.len() - 1;
        // Auto-scroll to show new card
        let viewport_height = self.state.output_area_height as usize;
        self.state.cards.scroll_offset = self.state.cards.calculate_scroll_offset(viewport_height);
    }

    fn add_call_card(&mut self, from: Address, to: Address, function: String, result: String) {
        let card = crate::cards::Card::Call {
            from,
            to,
            function_signature: function,
            result,
        };
        self.state.cards.cards.push(card);
        self.state.cards.selected_index = self.state.cards.cards.len() - 1;
        // Auto-scroll to show new card
        let viewport_height = self.state.output_area_height as usize;
        self.state.cards.scroll_offset = self.state.cards.calculate_scroll_offset(viewport_height);
    }

    pub fn add_log_card(&mut self, message: String) {
        let card = crate::cards::Card::Log { message };
        self.state.cards.cards.push(card);
        self.state.cards.selected_index = self.state.cards.cards.len() - 1;
        // Auto-scroll to show new card
        let viewport_height = self.state.output_area_height as usize;
        self.state.cards.scroll_offset = self.state.cards.calculate_scroll_offset(viewport_height);
    }
}

/// Copy text to clipboard using system commands on Linux for reliability
fn copy_to_clipboard(text: &str) -> std::result::Result<(), String> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    // Try wl-copy (Wayland) first, then xclip, then xsel, finally fall back to arboard
    let clipboard_commands = [
        ("wl-copy", vec![]),
        ("xclip", vec!["-selection", "clipboard"]),
        ("xsel", vec!["--clipboard", "--input"]),
    ];

    for (cmd, args) in &clipboard_commands {
        if let Ok(mut child) = Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                if stdin.write_all(text.as_bytes()).is_ok() {
                    drop(stdin);
                    if child.wait().map(|s| s.success()).unwrap_or(false) {
                        return Ok(());
                    }
                }
            }
        }
    }

    // Fall back to arboard if no system command works
    arboard::Clipboard::new()
        .and_then(|mut cb| cb.set_text(text))
        .map_err(|e| e.to_string())
}

fn parse_value(input: &str, type_str: &str) -> std::result::Result<DynSolValue, String> {
    let sol_type: DynSolType = type_str
        .parse()
        .map_err(|_| format!("Unsupported type: {type_str}"))?;

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
                return Err(format!("Expected {size} bytes"));
            }
            Ok(DynSolValue::FixedBytes(
                alloy::primitives::FixedBytes::from_slice(&bytes),
                size,
            ))
        }
        DynSolType::String => Ok(DynSolValue::String(input.to_string())),
        _ => Err(format!("Unsupported type: {type_str}")),
    }
}

fn format_ether(wei: U256) -> String {
    const DECIMALS: usize = 18;
    const DISPLAY_DECIMALS: usize = 6;

    let wei_str = wei.to_string();
    let len = wei_str.len();

    let (int_part, frac_part) = if len <= DECIMALS {
        let padded = format!("{wei_str:0>DECIMALS$}");
        ("0".to_string(), padded)
    } else {
        let split = len - DECIMALS;
        (wei_str[..split].to_string(), wei_str[split..].to_string())
    };

    let frac_display = &frac_part[..DISPLAY_DECIMALS.min(frac_part.len())];
    format!("{int_part}.{frac_display}")
}

/// Format a key event for logging
fn format_key_event(key: &KeyEvent) -> String {
    let mut parts = Vec::new();
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        parts.push("Ctrl");
    }
    if key.modifiers.contains(KeyModifiers::ALT) {
        parts.push("Alt");
    }
    if key.modifiers.contains(KeyModifiers::SHIFT) {
        parts.push("Shift");
    }
    let key_str = match key.code {
        KeyCode::Char(c) => format!("'{c}'"),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::F(n) => format!("F{n}"),
        _ => format!("{:?}", key.code),
    };
    parts.push(&key_str);
    parts.join("+")
}
