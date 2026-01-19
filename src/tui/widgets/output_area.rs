use crate::tui::state::{OutputState, OutputStyle};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct OutputArea<'a> {
    state: &'a OutputState,
    focused: bool,
}

impl<'a> OutputArea<'a> {
    pub fn new(state: &'a OutputState) -> Self {
        Self {
            state,
            focused: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl Widget for OutputArea<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Output ");

        let inner_area = block.inner(area);
        block.render(area, buf);

        // Convert output lines to ratatui Lines
        let lines: Vec<Line> = self
            .state
            .lines
            .iter()
            .skip(self.state.scroll_offset)
            .map(|line| {
                let style = match line.style {
                    OutputStyle::Normal => Style::default(),
                    OutputStyle::Success => Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                    OutputStyle::Error => Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                    OutputStyle::Info => Style::default().fg(Color::DarkGray),
                    OutputStyle::Waiting => Style::default().fg(Color::Yellow),
                    OutputStyle::Highlight => Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                    OutputStyle::Separator => Style::default().fg(Color::DarkGray),
                };
                Line::from(Span::styled(&line.text, style))
            })
            .collect();

        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

        paragraph.render(inner_area, buf);
    }
}
