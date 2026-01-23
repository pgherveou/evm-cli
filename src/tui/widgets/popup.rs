use crate::tui::layout::centered_popup;
use crate::tui::theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Clear, Widget},
};

pub struct Popup<'a> {
    title: &'a str,
    width_percent: u16,
    height_percent: u16,
}

impl<'a> Popup<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            width_percent: 60,
            height_percent: 50,
        }
    }

    pub fn width_percent(mut self, percent: u16) -> Self {
        self.width_percent = percent;
        self
    }

    pub fn height_percent(mut self, percent: u16) -> Self {
        self.height_percent = percent;
        self
    }

    pub fn render_frame(&self, area: Rect, buf: &mut Buffer) -> Rect {
        let popup_area = centered_popup(area, self.width_percent, self.height_percent);

        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(format!(" {} ", self.title));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        inner
    }
}
