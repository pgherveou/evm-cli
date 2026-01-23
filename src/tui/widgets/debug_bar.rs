use crate::tui::state::{AppState, Focus, PopupState};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

pub struct DebugBarWidget<'a> {
    state: &'a AppState,
}

impl<'a> DebugBarWidget<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    fn focus_name(&self) -> &'static str {
        match self.state.focus {
            Focus::Sidebar => "Sidebar",
            Focus::Output => "Output",
            Focus::CommandPalette => "CommandPalette",
        }
    }

    fn popup_name(&self) -> &'static str {
        match &self.state.popup {
            PopupState::None => "None",
            PopupState::CommandPalette { .. } => "CommandPalette",
            PopupState::ParameterPopup { .. } => "ParameterPopup",
            PopupState::ContractSelector { .. } => "ContractSelector",
            PopupState::FilePicker { .. } => "FilePicker",
            PopupState::AddressInput { .. } => "AddressInput",
            PopupState::TracerMenu { .. } => "TracerMenu",
            PopupState::TracerConfig { .. } => "TracerConfig",
            PopupState::CopyMenu { .. } => "CopyMenu",
        }
    }
}

impl Widget for DebugBarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label_style = Style::default()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let value_style = Style::default().fg(Color::Yellow);
        let separator_style = Style::default().fg(Color::DarkGray);

        let mut spans = vec![
            Span::styled(" DEBUG ", label_style),
            Span::styled(" ", Style::default()),
        ];

        spans.push(Span::styled("Focus: ", separator_style));
        spans.push(Span::styled(self.focus_name(), value_style));
        spans.push(Span::styled(" │ ", separator_style));

        spans.push(Span::styled("Popup: ", separator_style));
        spans.push(Span::styled(self.popup_name(), value_style));
        spans.push(Span::styled(" │ ", separator_style));

        spans.push(Span::styled("Key: ", separator_style));
        let key_display = self.state.last_key.as_deref().unwrap_or("-");
        spans.push(Span::styled(key_display.to_string(), value_style));
        spans.push(Span::styled(" │ ", separator_style));

        spans.push(Span::styled("Action: ", separator_style));
        let action_display = self.state.last_action.as_deref().unwrap_or("-");
        spans.push(Span::styled(action_display.to_string(), value_style));

        spans.push(Span::styled(" │ ", separator_style));
        spans.push(Span::styled("Sel: ", separator_style));
        spans.push(Span::styled(
            format!("{}", self.state.sidebar.selected),
            value_style,
        ));

        spans.push(Span::styled(" │ ", separator_style));
        spans.push(Span::styled("Cards: ", separator_style));
        spans.push(Span::styled(
            format!("{}", self.state.cards.cards.len()),
            value_style,
        ));

        // Card count
        spans.push(Span::styled(" │ ", separator_style));
        spans.push(Span::styled("Cards: ", separator_style));
        spans.push(Span::styled(
            format!("{}", self.state.cards.cards.len()),
            value_style,
        ));

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
