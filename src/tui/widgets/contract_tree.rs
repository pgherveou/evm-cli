use crate::method_list::{self, MethodSelection};
use crate::solc::CompiledContract;
use crate::store::DeploymentStore;
use crate::tui::state::SidebarState;
use alloy::json_abi::Function;
use alloy::primitives::Address;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};
use std::path::PathBuf;

/// Tree node types for the sidebar
#[derive(Debug, Clone)]
pub enum TreeNode {
    NewContract,
    Contract {
        name: String,
        path: PathBuf,
    },
    Constructor {
        contract_path: PathBuf,
    },
    LoadExistingInstance {
        contract_path: PathBuf,
    },
    DeployedInstance {
        address: Address,
        contract_path: PathBuf,
    },
    Method {
        function: Function,
        tag: &'static str,
        instance_address: Address,
    },
}

impl TreeNode {
    pub fn label(&self) -> String {
        match self {
            TreeNode::NewContract => "Load new contract...".to_string(),
            TreeNode::Contract { name, .. } => name.clone(),
            TreeNode::Constructor { .. } => "Deploy new instance".to_string(),
            TreeNode::LoadExistingInstance { .. } => "Load existing instance...".to_string(),
            TreeNode::DeployedInstance { address, .. } => {
                format!("{:?}", address)
            }
            TreeNode::Method { function, tag, .. } => {
                let params: Vec<_> = function
                    .inputs
                    .iter()
                    .map(|p| {
                        if p.name.is_empty() {
                            p.ty.to_string()
                        } else {
                            format!("{}: {}", p.name, p.ty)
                        }
                    })
                    .collect();
                format!("{}({}) [{}]", function.name, params.join(", "), tag)
            }
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            TreeNode::NewContract => 0,
            TreeNode::Contract { .. } => 0,
            TreeNode::Constructor { .. } => 1,
            TreeNode::LoadExistingInstance { .. } => 1,
            TreeNode::DeployedInstance { .. } => 1,
            TreeNode::Method { .. } => 2,
        }
    }
}

pub struct ContractTree<'a> {
    state: &'a SidebarState,
    nodes: Vec<TreeNode>,
    focused: bool,
}

impl<'a> ContractTree<'a> {
    pub fn new(state: &'a SidebarState) -> Self {
        Self {
            state,
            nodes: Vec::new(),
            focused: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn with_data(
        mut self,
        store: &DeploymentStore,
        current_contract: Option<(&CompiledContract, &PathBuf)>,
    ) -> Self {
        self.nodes = build_tree(store, current_contract, self.state);
        self
    }

    pub fn nodes(&self) -> &[TreeNode] {
        &self.nodes
    }
}

fn build_tree(
    store: &DeploymentStore,
    current_contract: Option<(&CompiledContract, &PathBuf)>,
    state: &SidebarState,
) -> Vec<TreeNode> {
    let mut nodes = Vec::new();

    // Always show "New contract" at top
    nodes.push(TreeNode::NewContract);

    // Add current contract if loaded
    if let Some((contract, path)) = current_contract {
        nodes.push(TreeNode::Contract {
            name: contract.name.clone(),
            path: path.clone(),
        });

        // Check if this contract is expanded
        if state.expanded_contracts.contains(path) {
            // Add constructor option
            nodes.push(TreeNode::Constructor {
                contract_path: path.clone(),
            });

            // Add load existing instance option
            nodes.push(TreeNode::LoadExistingInstance {
                contract_path: path.clone(),
            });

            // Add deployed instances
            let deployments = store.get_deployments(path);
            for address in deployments {
                nodes.push(TreeNode::DeployedInstance {
                    address,
                    contract_path: path.clone(),
                });

                // If instance is expanded, show methods
                if state.expanded_instances.contains(&address) {
                    let methods = method_list::list_methods(&contract.abi, false);
                    for method in methods {
                        if let MethodSelection::Function(f) = method.selection {
                            nodes.push(TreeNode::Method {
                                function: f,
                                tag: method.tag,
                                instance_address: address,
                            });
                        }
                    }
                }
            }
        }
    }

    // Add other known contracts (collapsed)
    for contract_path in store.all_contracts() {
        // Skip current contract
        if current_contract
            .map(|(_, p)| p == &contract_path)
            .unwrap_or(false)
        {
            continue;
        }

        let name = contract_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| contract_path.to_string_lossy().to_string());

        nodes.push(TreeNode::Contract {
            name,
            path: contract_path,
        });
    }

    nodes
}

impl Widget for ContractTree<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Contracts ");

        let inner_area = block.inner(area);
        block.render(area, buf);

        // Render tree nodes
        let visible_height = inner_area.height as usize;
        let total_nodes = self.nodes.len();

        // Calculate visible range
        let scroll = self.state.scroll_offset;
        let end = (scroll + visible_height).min(total_nodes);

        for (i, node) in self.nodes.iter().enumerate().skip(scroll).take(end - scroll) {
            let y = inner_area.y + (i - scroll) as u16;
            if y >= inner_area.y + inner_area.height {
                break;
            }

            let is_selected = i == self.state.selected;
            let depth = node.depth();
            let indent = "  ".repeat(depth);

            // Determine prefix based on node type
            let prefix = match node {
                TreeNode::Contract { path, .. } => {
                    if self.state.expanded_contracts.contains(path) {
                        "▾ "
                    } else {
                        "▸ "
                    }
                }
                TreeNode::DeployedInstance { address, .. } => {
                    if self.state.expanded_instances.contains(address) {
                        "▾ "
                    } else {
                        "▸ "
                    }
                }
                TreeNode::NewContract => "+ ",
                TreeNode::Constructor { .. } => "◇ ",
                TreeNode::LoadExistingInstance { .. } => "◇ ",
                TreeNode::Method { .. } => "├ ",
            };

            let label = node.label();

            // Style based on selection and node type
            let style = if is_selected {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                match node {
                    TreeNode::NewContract => Style::default().fg(Color::Yellow),
                    TreeNode::Contract { .. } => Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                    TreeNode::Constructor { .. } => Style::default().fg(Color::Green),
                    TreeNode::LoadExistingInstance { .. } => Style::default().fg(Color::Yellow),
                    TreeNode::DeployedInstance { .. } => Style::default().fg(Color::Green),
                    TreeNode::Method { tag, .. } => {
                        if *tag == "view" {
                            Style::default().fg(Color::Blue)
                        } else {
                            Style::default().fg(Color::Yellow)
                        }
                    }
                }
            };

            let line = Line::from(vec![Span::styled(
                format!("{}{}{}", indent, prefix, label),
                style,
            )]);

            buf.set_line(inner_area.x, y, &line, inner_area.width);
        }

        // Show scroll indicators if needed
        if scroll > 0 {
            buf.set_string(
                inner_area.x + inner_area.width.saturating_sub(3),
                inner_area.y,
                "↑",
                Style::default().fg(Color::DarkGray),
            );
        }
        if end < total_nodes {
            buf.set_string(
                inner_area.x + inner_area.width.saturating_sub(3),
                inner_area.y + inner_area.height.saturating_sub(1),
                "↓",
                Style::default().fg(Color::DarkGray),
            );
        }
    }
}
