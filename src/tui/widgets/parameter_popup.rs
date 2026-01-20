use crate::compile::BytecodeTarget;
use crate::tui::layout::centered_popup;
use crate::tui::state::FieldState;
use crate::tui::widgets::InputField;
use alloy::json_abi::Param;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Widget},
};

pub struct ParameterPopup<'a> {
    method_name: &'a str,
    params: &'a [Param],
    fields: &'a [FieldState],
    current: usize,
    bytecode_target: Option<BytecodeTarget>,
}

impl<'a> ParameterPopup<'a> {
    pub fn new(
        method_name: &'a str,
        params: &'a [Param],
        fields: &'a [FieldState],
        current: usize,
    ) -> Self {
        Self {
            method_name,
            params,
            fields,
            current,
            bytecode_target: None,
        }
    }

    pub fn bytecode_target(mut self, target: Option<BytecodeTarget>) -> Self {
        self.bytecode_target = target;
        self
    }
}

impl Widget for ParameterPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate popup height based on number of fields (2 lines per field + header + footer)
        // Add extra space if bytecode target selector is shown
        let extra_height = if self.bytecode_target.is_some() { 2 } else { 0 };
        let height_percent = ((self.fields.len() * 3 + 6 + extra_height) as u16 * 100 / area.height).min(80);
        let popup_area = centered_popup(area, 70, height_percent.max(30));

        // Clear the popup area
        Clear.render(popup_area, buf);

        let title = format!(" {} - Enter Parameters ", self.method_name);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(title);

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let mut y = inner.y + 1;

        // Render bytecode target selector if this is a deploy operation
        if let Some(target) = self.bytecode_target {
            let hint_style = Style::default().fg(Color::DarkGray);
            let selected_style = Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
            let unselected_style = Style::default().fg(Color::DarkGray);

            let (evm_style, pvm_style) = match target {
                BytecodeTarget::Evm => (selected_style, unselected_style),
                BytecodeTarget::Pvm => (unselected_style, selected_style),
            };

            let target_line = Line::from(vec![
                Span::styled("Target: ", Style::default().fg(Color::Cyan)),
                Span::styled(" EVM ", evm_style),
                Span::styled(" ", Style::default()),
                Span::styled(" PVM ", pvm_style),
                Span::styled("  (←/→ to switch)", hint_style),
            ]);
            buf.set_line(inner.x + 1, y, &target_line, inner.width.saturating_sub(2));
            y += 2;
        }

        // Render each field
        for (i, (param, field)) in self.params.iter().zip(self.fields.iter()).enumerate() {
            if y >= inner.y + inner.height.saturating_sub(2) {
                break;
            }

            let is_focused = i == self.current;
            let label = if param.name.is_empty() {
                format!("arg{} ({})", i, param.ty)
            } else {
                format!("{} ({})", param.name, param.ty)
            };

            let input = InputField::new(&label, &field.value)
                .placeholder(get_placeholder(&param.ty))
                .error(field.error.as_deref())
                .focused(is_focused)
                .cursor_position(field.value.len());

            // Render input field (takes 2 lines if error present)
            let field_height = if field.error.is_some() { 2 } else { 1 };
            let field_area = Rect::new(inner.x + 1, y, inner.width.saturating_sub(2), field_height);
            input.render(field_area, buf);

            y += field_height + 1; // spacing between fields
        }

        // Render footer with hints
        let footer_y = inner.y + inner.height.saturating_sub(1);
        let hint_style = Style::default().fg(Color::DarkGray);
        let mut footer_spans = vec![
            Span::styled("Tab", Style::default().fg(Color::Yellow)),
            Span::styled(": next  ", hint_style),
            Span::styled("Shift+Tab", Style::default().fg(Color::Yellow)),
            Span::styled(": prev  ", hint_style),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::styled(": submit  ", hint_style),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::styled(": cancel", hint_style),
        ];

        if self.bytecode_target.is_some() {
            footer_spans.insert(0, Span::styled("  ", hint_style));
            footer_spans.insert(0, Span::styled(": target", hint_style));
            footer_spans.insert(0, Span::styled("←/→", Style::default().fg(Color::Yellow)));
        }

        let footer = Line::from(footer_spans);
        buf.set_line(inner.x + 1, footer_y, &footer, inner.width.saturating_sub(2));
    }
}

fn get_placeholder(ty: &str) -> &'static str {
    if ty.starts_with("address") {
        "0x..."
    } else if ty.starts_with("uint") || ty.starts_with("int") {
        "0"
    } else if ty.starts_with("bool") {
        "true or false"
    } else if ty.starts_with("bytes") {
        "0x..."
    } else if ty == "string" {
        "enter text..."
    } else {
        ""
    }
}
