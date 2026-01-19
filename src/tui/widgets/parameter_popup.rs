use crate::tui::layout::centered_popup;
use crate::tui::state::FieldState;
use crate::tui::widgets::InputField;
use alloy::json_abi::Param;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Widget},
};

pub struct ParameterPopup<'a> {
    method_name: &'a str,
    params: &'a [Param],
    fields: &'a [FieldState],
    current: usize,
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
        }
    }
}

impl Widget for ParameterPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate popup height based on number of fields (2 lines per field + header + footer)
        let height_percent = ((self.fields.len() * 3 + 6) as u16 * 100 / area.height).min(80);
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

        // Render each field (with top padding)
        let mut y = inner.y + 1;
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
        let footer = Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Yellow)),
            Span::styled(": next  ", hint_style),
            Span::styled("Shift+Tab", Style::default().fg(Color::Yellow)),
            Span::styled(": prev  ", hint_style),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::styled(": submit  ", hint_style),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::styled(": cancel", hint_style),
        ]);
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
