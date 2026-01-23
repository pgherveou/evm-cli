use crate::tui::theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

pub struct KeyboardHints {
    hints: Vec<(&'static str, &'static str)>,
}

impl KeyboardHints {
    pub fn new(hints: Vec<(&'static str, &'static str)>) -> Self {
        Self { hints }
    }
}

impl Widget for KeyboardHints {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = Vec::new();

        for (i, (key, desc)) in self.hints.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled("  ", theme::hint_desc_style()));
            }
            spans.push(Span::styled(*key, theme::hint_key_style()));
            spans.push(Span::styled(
                format!(": {}", desc),
                theme::hint_desc_style(),
            ));
        }

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
