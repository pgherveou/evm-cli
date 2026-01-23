use crate::tui::state::{AppState, ConnectionStatus, Focus};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

pub struct StatusBarWidget<'a> {
    state: &'a AppState,
}

impl<'a> StatusBarWidget<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    /// Generate keyboard hints based on terminal width
    fn get_keyboard_hints(&self, available_width: u16) -> Vec<Span<'static>> {
        let mut hints = vec![];

        // Context-specific hints (shown first on the right)
        if self.state.focus == Focus::Sidebar {
            // Show delete hint when sidebar is focused
            if available_width > 60 {
                hints.push(Span::styled(
                    "Del: remove  ",
                    Style::default().fg(Color::DarkGray),
                ));
            } else if available_width > 40 {
                hints.push(Span::styled("Del  ", Style::default().fg(Color::DarkGray)));
            }
        }

        // Global hints (always shown after context hints)
        if available_width > 80 {
            // Full text for wide terminals
            hints.extend(vec![
                Span::styled("Tab: switch tab  ", Style::default().fg(Color::DarkGray)),
                Span::styled("Ctrl+P: commands  ", Style::default().fg(Color::DarkGray)),
                Span::styled("Ctrl+C: quit", Style::default().fg(Color::DarkGray)),
            ]);
        } else if available_width > 50 {
            // Abbreviated for medium terminals
            hints.extend(vec![
                Span::styled("Tab: switch  ", Style::default().fg(Color::DarkGray)),
                Span::styled("Ctrl+P: cmds  ", Style::default().fg(Color::DarkGray)),
                Span::styled("Ctrl+C: quit", Style::default().fg(Color::DarkGray)),
            ]);
        } else if available_width > 20 {
            // Ultra-compact for narrow terminals
            hints.extend(vec![
                Span::styled("Tab  ", Style::default().fg(Color::DarkGray)),
                Span::styled("^P  ", Style::default().fg(Color::DarkGray)),
                Span::styled("^C", Style::default().fg(Color::DarkGray)),
            ]);
        }

        hints
    }
}

impl Widget for StatusBarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Connection indicator
        let (conn_symbol, conn_text, conn_style) = match self.state.connection {
            ConnectionStatus::Connected => ("●", " Connected", Style::default().fg(Color::Green)),
            ConnectionStatus::Disconnected => {
                ("○", " Disconnected", Style::default().fg(Color::DarkGray))
            }
        };

        let mut spans = vec![
            Span::styled(conn_symbol, conn_style),
            Span::styled(conn_text, conn_style),
            Span::raw(" | "),
        ];

        // Chain ID
        let chain_text = if let Some(chain_id) = self.state.chain_id {
            format!("Chain: {chain_id} | ")
        } else {
            "Chain: N/A | ".to_string()
        };
        spans.push(Span::raw(chain_text));

        // Account (full address)
        if let Some(account) = self.state.account {
            spans.push(Span::raw(format!("Account: {account:?} | ")));
        } else {
            spans.push(Span::raw("Account: N/A | "));
        }

        // Balance
        if let Some(balance) = &self.state.balance {
            spans.push(Span::raw(format!("Balance: {balance} ETH")));
        } else {
            spans.push(Span::raw("Balance: 0 ETH"));
        }

        // Calculate left content length
        let left_content: String = spans.iter().map(|s| s.content.as_ref()).collect();
        let left_len = left_content.len() as u16;

        // Calculate available width for right-aligned content
        let available_for_right = area.width.saturating_sub(left_len).saturating_sub(4); // 4 spaces padding

        // Get keyboard hints based on available width
        let right_spans = self.get_keyboard_hints(available_for_right);

        // Calculate space needed for right content
        let right_len: u16 = right_spans.iter().map(|s| s.content.len() as u16).sum();

        // Fill space between left and right content
        let space_needed = area.width.saturating_sub(left_len + right_len);
        spans.push(Span::raw(" ".repeat(space_needed as usize)));
        spans.extend(right_spans);

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
