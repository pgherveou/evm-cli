use crate::tui::state::{AppState, ConnectionStatus};
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
}

impl Widget for StatusBarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Connection indicator
        let (conn_symbol, conn_style) = match self.state.connection {
            ConnectionStatus::Connected => ("●", Style::default().fg(Color::Green)),
            ConnectionStatus::Disconnected => ("○", Style::default().fg(Color::DarkGray)),
        };

        let mut spans = vec![Span::styled(format!("{} ", conn_symbol), conn_style)];

        // Chain ID
        if let Some(chain_id) = self.state.chain_id {
            spans.push(Span::styled(
                format!("Chain:{} ", chain_id),
                Style::default().fg(Color::Magenta),
            ));
        }

        // Right-aligned hints (styled like popup footers)
        let mut hint_spans = vec![];

        // Show Del hint when sidebar is focused
        if matches!(self.state.focus, crate::tui::state::Focus::Sidebar) {
            hint_spans.push(Span::styled("Del", Style::default().fg(Color::Yellow)));
            hint_spans.push(Span::styled(": remove  ", Style::default().fg(Color::DarkGray)));
        }

        hint_spans.extend(vec![
            Span::styled("Ctrl+P", Style::default().fg(Color::Yellow)),
            Span::styled(": commands  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+C", Style::default().fg(Color::Yellow)),
            Span::styled(": quit", Style::default().fg(Color::DarkGray)),
        ]);
        let hints_len: u16 = hint_spans.iter().map(|s| s.content.len() as u16).sum();
        let left_content: String = spans.iter().map(|s| s.content.as_ref()).collect();
        let left_len = left_content.len() as u16;

        // Fill space between left and right content
        let space_needed = area.width.saturating_sub(left_len + hints_len);
        spans.push(Span::raw(" ".repeat(space_needed as usize)));
        spans.extend(hint_spans);

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
