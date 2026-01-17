use alloy::dyn_abi::{DynSolValue, FunctionExt, JsonAbiExt};
use alloy::network::TransactionBuilder;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::context_menu;
use crate::filter_ui::{FilterItem, FilterResult, FilterUI};
use crate::method_list::{self, MethodSelection};
use crate::prompts;
use crate::solc::CompiledContract;
use crate::store::DeploymentStore;
use crate::ui::{self, StatusBar};

#[derive(Clone)]
enum AppAction {
    Deploy,
    CallMethod(alloy::json_abi::Function),
    NewContract,
    EnterAddress,
    SwitchContract(PathBuf),
    SwitchAddress(Address),
    Reset,
}

pub struct App<P> {
    pub provider: P,
    pub store: DeploymentStore,
    pub status: StatusBar,
    pub contract: Option<CompiledContract>,
    pub contract_path: Option<PathBuf>,
    pub address: Option<Address>,
}

impl<P: Provider + Clone> App<P> {
    pub fn new(provider: P, store: DeploymentStore) -> Self {
        Self {
            provider,
            store,
            status: StatusBar::new(),
            contract: None,
            contract_path: None,
            address: None,
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        // Get chain ID for status bar
        let chain_id = self.provider.get_chain_id().await?;
        self.status.set_chain_id(chain_id);

        Ok(())
    }

    pub fn set_contract(&mut self, contract: CompiledContract, path: PathBuf) {
        self.status.set_contract(&contract.name, &path);
        self.store.set_last_contract(&path);
        self.contract = Some(contract);
        self.contract_path = Some(path);
        self.address = None;
        self.status.clear_address();
        let _ = self.store.save();
    }

    pub fn set_address(&mut self, address: Address) {
        self.address = Some(address);
        self.status.set_address(address);
        self.store.set_last_address(address);
        let _ = self.store.save();
    }

    pub fn clear_state(&mut self) {
        self.contract = None;
        self.contract_path = None;
        self.address = None;
        self.status = StatusBar::new();
        self.store.clear_last();
        let _ = self.store.save();
    }

    pub async fn run_interactive(&mut self) -> Result<()> {
        loop {
            // Recalculate mode each iteration based on current state
            let start_mode = if self.contract.is_some() { '/' } else { '@' };

            match self.handle_filter_ui(start_mode).await {
                Ok(()) => continue,
                Err(e) => {
                    if prompts::is_cancelled(&e) {
                        continue;
                    }
                    ui::print_error(&e.to_string());
                }
            }
        }
    }

    async fn handle_filter_ui(&mut self, mut mode: char) -> Result<()> {
        loop {
            let result = match mode {
                '/' => self.show_method_filter()?,
                '@' => self.show_context_filter()?,
                _ => return Ok(()),
            };

            match result {
                FilterResult::Selected(action) => {
                    if let Err(e) = self.execute_action(action).await {
                        if !prompts::is_cancelled(&e) {
                            return Err(e);
                        }
                        // Cancelled - just return to main loop
                    } else {
                        // Print separator after successful operation
                        println!("{}", "─".repeat(65));
                    }
                    return Ok(());
                }
                FilterResult::SwitchTo(new_mode) => {
                    mode = new_mode;
                    continue;
                }
                FilterResult::Cancelled => {
                    return Ok(());
                }
                FilterResult::Exit => {
                    std::process::exit(0);
                }
            }
        }
    }

    fn show_method_filter(&self) -> Result<FilterResult<AppAction>> {
        let contract = self
            .contract
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No contract loaded"))?;

        let methods = method_list::list_methods(&contract.abi, self.address.is_none());
        let items: Vec<_> = methods
            .into_iter()
            .map(|m| {
                let item = FilterItem::new(&m.label).with_tag(m.tag);
                let action = match m.selection {
                    MethodSelection::Constructor => AppAction::Deploy,
                    MethodSelection::Function(f) => AppAction::CallMethod(f),
                };
                (item, action)
            })
            .collect();

        if items.is_empty() {
            anyhow::bail!("No methods available");
        }

        let filter = FilterUI::new('/', items).with_footer(self.status.footer_lines());
        Ok(filter.run()?)
    }

    fn show_context_filter(&self) -> Result<FilterResult<AppAction>> {
        let mut items = vec![
            (FilterItem::new("Load new contract (.sol file)"), AppAction::NewContract),
            (FilterItem::new("Enter address manually"), AppAction::EnterAddress),
            (FilterItem::new("Reset (clear saved state)"), AppAction::Reset),
        ];

        // Add existing contracts
        for path in self.store.all_contracts() {
            let label = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());
            items.push((
                FilterItem::new(format!("Contract: {}", label)),
                AppAction::SwitchContract(path),
            ));
        }

        // Add existing addresses for current contract
        if let Some(contract_path) = &self.contract_path {
            for addr in self.store.get_deployments(contract_path) {
                items.push((
                    FilterItem::new(format!("Address: {:?}", addr)),
                    AppAction::SwitchAddress(addr),
                ));
            }
        }

        let filter = FilterUI::new('@', items).with_footer(self.status.footer_lines());
        Ok(filter.run()?)
    }

    async fn execute_action(&mut self, action: AppAction) -> Result<()> {
        match action {
            AppAction::Deploy => {
                self.deploy_contract().await?;
            }
            AppAction::CallMethod(func) => {
                let address = self.address.ok_or_else(|| {
                    anyhow::anyhow!("No contract address set. Deploy first or set with @")
                })?;
                self.call_function(&func, address).await?;
            }
            AppAction::NewContract => {
                let path = context_menu::prompt_sol_path()?;
                let contracts = crate::solc::compile_solidity(&path)?;
                let contract = crate::solc::select_contract(contracts)?;
                self.set_contract(contract, path);
            }
            AppAction::EnterAddress => {
                let addr_str = inquire::Text::new("Contract address:")
                    .with_placeholder("0x...")
                    .prompt()
                    .context("Failed to get address")?;
                let address: Address = addr_str.parse().context("Invalid address")?;
                self.set_address(address);
            }
            AppAction::SwitchContract(path) => {
                let contracts = crate::solc::compile_solidity(&path)?;
                let contract = crate::solc::select_contract(contracts)?;
                self.set_contract(contract, path.clone());

                let deployments = self.store.get_deployments(&path);
                if !deployments.is_empty() {
                    if let Some(addr) = crate::store::select_address(&deployments, true)? {
                        self.set_address(addr);
                    }
                }
            }
            AppAction::SwitchAddress(addr) => {
                self.set_address(addr);
            }
            AppAction::Reset => {
                self.clear_state();
                println!("State cleared. Press @ to load a contract.");
            }
        }
        Ok(())
    }

    async fn deploy_contract(&mut self) -> Result<()> {
        let contract = self
            .contract
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No contract loaded"))?;

        println!("\nDeploying {}...", contract.name);

        // Get constructor arguments if any
        let args = if let Some(ctor) = &contract.abi.constructor {
            if !ctor.inputs.is_empty() {
                prompts::prompt_for_params(&ctor.inputs)?
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        // Encode constructor call
        let mut deploy_data = contract.bytecode.clone();

        if !args.is_empty() {
            // Encode constructor arguments
            let encoded = DynSolValue::Tuple(args).abi_encode_params();
            deploy_data.extend(encoded);
        }

        // Create deployment transaction
        let tx = TransactionRequest::default()
            .with_deploy_code(deploy_data);

        ui::print_waiting("Sending deployment transaction...");

        let pending = self
            .provider
            .send_transaction(tx)
            .await
            .context("Failed to send deployment transaction")?;

        let tx_hash = *pending.tx_hash();
        println!();
        ui::print_tx_hash(&format!("{:?}", tx_hash));

        ui::print_waiting("Waiting for confirmation...");

        let receipt = pending
            .get_receipt()
            .await
            .context("Failed to get transaction receipt")?;

        println!();

        let address = receipt
            .contract_address
            .ok_or_else(|| anyhow::anyhow!("No contract address in receipt"))?;

        ui::print_result("Deployed at", &format!("{:?}", address));

        // Update state
        self.set_address(address);

        // Save to store
        if let Some(path) = &self.contract_path {
            self.store.add_deployment(path, address);
            self.store.save()?;
        }

        Ok(())
    }

    async fn call_function(
        &self,
        func: &alloy::json_abi::Function,
        address: Address,
    ) -> Result<()> {
        println!("\nCalling {}...", func.name);

        // Get function arguments
        let args = if !func.inputs.is_empty() {
            prompts::prompt_for_params(&func.inputs)?
        } else {
            vec![]
        };

        // Encode function call
        let calldata = func.abi_encode_input(&args)
            .context("Failed to encode function call")?;

        let is_view = matches!(
            func.state_mutability,
            alloy::json_abi::StateMutability::View | alloy::json_abi::StateMutability::Pure
        );

        if is_view {
            // Read-only call
            let tx = TransactionRequest::default()
                .to(address)
                .input(calldata.into());

            let result = self
                .provider
                .call(tx)
                .await
                .context("Call failed")?;

            // Decode result
            let decoded = func
                .abi_decode_output(&result)
                .context("Failed to decode return value")?;

            if decoded.is_empty() {
                ui::print_result("Result", "(no return value)");
            } else if decoded.len() == 1 {
                ui::print_result("Result", &prompts::format_return_value(&decoded[0]));
            } else {
                let formatted: Vec<_> = decoded
                    .iter()
                    .map(prompts::format_return_value)
                    .collect();
                ui::print_result("Result", &format!("({})", formatted.join(", ")));
            }
        } else {
            // State-changing transaction
            let tx = TransactionRequest::default()
                .to(address)
                .input(calldata.into());

            ui::print_waiting("Sending transaction...");

            let pending = self
                .provider
                .send_transaction(tx)
                .await
                .context("Failed to send transaction")?;

            let tx_hash = *pending.tx_hash();
            println!();
            ui::print_tx_hash(&format!("{:?}", tx_hash));
            ui::print_method_call(&prompts::format_method_call(&func.name, &func.inputs, &args));

            ui::print_waiting("Waiting for confirmation...");

            let receipt = pending
                .get_receipt()
                .await
                .context("Failed to get transaction receipt")?;

            println!();

            if receipt.status() {
                ui::print_result("Status", "Success");
            } else {
                ui::print_error("Transaction reverted");
            }

            ui::print_info(&format!("Gas used: {}", receipt.gas_used));
        }

        Ok(())
    }
}
