use crate::tui::state::SidebarState;
use alloy::json_abi::{Function, JsonAbi};
use alloy::primitives::Address;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};
use std::path::PathBuf;
use std::sync::Arc;

/// Tree node types for the sidebar
/// Each node is self-contained with all the data it needs to execute
#[derive(Debug, Clone)]
pub enum TreeNode {
    NewContract,
    Contract {
        name: String,
        path: PathBuf,
    },
    Constructor {
        contract_name: String,
        contract_path: PathBuf,
        abi: Arc<JsonAbi>,
    },
    LoadExistingInstance {
        contract_name: String,
        contract_path: PathBuf,
        abi: Arc<JsonAbi>,
    },
    DeployedInstance {
        address: Address,
        contract_name: String,
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
                format!("{address:?}")
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

    pub fn with_nodes(mut self, nodes: Vec<TreeNode>) -> Self {
        self.nodes = nodes;
        self
    }
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

        // Add 1px left and top padding
        let padded_area = Rect {
            x: inner_area.x + 1,
            y: inner_area.y + 1,
            width: inner_area.width.saturating_sub(1),
            height: inner_area.height.saturating_sub(1),
        };

        // Render tree nodes
        let visible_height = padded_area.height as usize;
        let total_nodes = self.nodes.len();

        // Calculate visible range
        let scroll = self.state.scroll_offset;
        let end = (scroll + visible_height).min(total_nodes);

        for (i, node) in self
            .nodes
            .iter()
            .enumerate()
            .skip(scroll)
            .take(end - scroll)
        {
            let y = padded_area.y + (i - scroll) as u16;
            if y >= padded_area.y + padded_area.height {
                break;
            }

            let is_selected = i == self.state.selected;
            let depth = node.depth();
            let indent = "  ".repeat(depth);

            // Determine prefix based on node type
            let prefix = match node {
                TreeNode::Contract { path, name } => {
                    if self
                        .state
                        .expanded_contracts
                        .contains(&(path.clone(), name.clone()))
                    {
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
                format!("{indent}{prefix}{label}"),
                style,
            )]);

            buf.set_line(padded_area.x, y, &line, padded_area.width);
        }

        // Show scroll indicators if needed
        if scroll > 0 {
            buf.set_string(
                padded_area.x + padded_area.width.saturating_sub(3),
                padded_area.y,
                "↑",
                Style::default().fg(Color::DarkGray),
            );
        }
        if end < total_nodes {
            buf.set_string(
                padded_area.x + padded_area.width.saturating_sub(3),
                padded_area.y + padded_area.height.saturating_sub(1),
                "↓",
                Style::default().fg(Color::DarkGray),
            );
        }
    }
}
