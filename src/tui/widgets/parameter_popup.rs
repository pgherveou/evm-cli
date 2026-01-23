use crate::compile::BytecodeTarget;
use crate::tui::layout::centered_popup;
use crate::tui::state::FieldState;
use crate::tui::theme;
use crate::tui::widgets::{InputField, KeyboardHints};
use alloy::json_abi::Param;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
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
        let extra_height = if self.bytecode_target.is_some() { 2 } else { 0 };
        let height_percent =
            ((self.fields.len() * 3 + 6 + extra_height) as u16 * 100 / area.height).min(80);
        let popup_area = centered_popup(area, 70, height_percent.max(30));

        Clear.render(popup_area, buf);

        let title = format!(" {} - Enter Parameters ", self.method_name);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(title);

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let mut y = inner.y + 1;

        if let Some(target) = self.bytecode_target {
            let (evm_style, pvm_style) = match target {
                BytecodeTarget::Evm => (theme::selected_style(), theme::muted_style()),
                BytecodeTarget::Pvm => (theme::muted_style(), theme::selected_style()),
            };

            let target_line = Line::from(vec![
                Span::styled("Target: ", Style::default().fg(theme::PRIMARY)),
                Span::styled(" EVM ", evm_style),
                Span::styled(" ", Style::default()),
                Span::styled(" PVM ", pvm_style),
                Span::styled("  (←/→ to switch)", theme::hint_desc_style()),
            ]);
            buf.set_line(inner.x + 1, y, &target_line, inner.width.saturating_sub(2));
            y += 2;
        }

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

            let field_height = if field.error.is_some() { 2 } else { 1 };
            let field_area = Rect::new(inner.x + 1, y, inner.width.saturating_sub(2), field_height);
            input.render(field_area, buf);

            y += field_height + 1;
        }

        let footer_y = inner.y + inner.height.saturating_sub(1);
        let mut hints = vec![
            ("Tab", "next"),
            ("Shift+Tab", "prev"),
            ("Enter", "submit"),
            ("Esc", "cancel"),
        ];

        if self.bytecode_target.is_some() {
            hints.insert(0, ("←/→", "target"));
        }

        let keyboard_hints = KeyboardHints::new(hints);
        let hints_area = Rect::new(inner.x + 1, footer_y, inner.width.saturating_sub(2), 1);
        keyboard_hints.render(hints_area, buf);
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
