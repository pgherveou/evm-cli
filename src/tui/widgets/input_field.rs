use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

pub struct InputField<'a> {
    label: &'a str,
    value: &'a str,
    placeholder: Option<&'a str>,
    error: Option<&'a str>,
    focused: bool,
    cursor_position: usize,
}

impl<'a> InputField<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            placeholder: None,
            error: None,
            focused: false,
            cursor_position: value.len(),
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn error(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn cursor_position(mut self, pos: usize) -> Self {
        self.cursor_position = pos;
        self
    }
}

impl Widget for InputField<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 1 {
            return;
        }

        // Label
        let label_style = if self.focused {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let label_span = Span::styled(format!("{}: ", self.label), label_style);

        // Value or placeholder
        let (value_content, value_style) = if self.value.is_empty() {
            (
                self.placeholder.unwrap_or("").to_string(),
                Style::default().fg(Color::DarkGray),
            )
        } else {
            (self.value.to_string(), Style::default().fg(Color::White))
        };

        let mut spans = vec![label_span];

        if self.focused && !self.value.is_empty() {
            // Show cursor in the middle of text
            let (before, after) = self.value.split_at(self.cursor_position.min(self.value.len()));
            spans.push(Span::styled(before, value_style));
            if after.is_empty() {
                spans.push(Span::styled(
                    "█",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::SLOW_BLINK),
                ));
            } else {
                let mut chars = after.chars();
                if let Some(cursor_char) = chars.next() {
                    spans.push(Span::styled(
                        cursor_char.to_string(),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan),
                    ));
                    let rest: String = chars.collect();
                    if !rest.is_empty() {
                        spans.push(Span::styled(rest, value_style));
                    }
                }
            }
        } else if self.focused && self.value.is_empty() {
            spans.push(Span::styled(value_content, value_style));
            spans.push(Span::styled(
                "█",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::SLOW_BLINK),
            ));
        } else {
            spans.push(Span::styled(value_content, value_style));
        }

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);

        // Error message on second line if present and we have space
        if let Some(error) = self.error {
            if area.height > 1 {
                let error_line = Line::from(Span::styled(
                    format!("  └ {}", error),
                    Style::default().fg(Color::Red),
                ));
                buf.set_line(area.x, area.y + 1, &error_line, area.width);
            }
        }
    }
}
