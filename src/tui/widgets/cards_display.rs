use crate::cards::Card;
use crate::tui::state::CardState;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct CardsDisplay<'a> {
    state: &'a CardState,
    focused: bool,
}

impl<'a> CardsDisplay<'a> {
    pub fn new(state: &'a CardState) -> Self {
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

impl Widget for CardsDisplay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Cards ");

        let inner_area = block.inner(area);
        block.render(area, buf);

        // Build lines for all cards
        let mut lines = Vec::new();

        if self.state.cards.is_empty() {
            lines.push(Line::from(Span::styled(
                "No cards to display",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for (i, card) in self.state.cards.iter().enumerate() {
                let is_selected = i == self.state.selected_index;
                let card_text = card.display_line();

                // Style based on selection and card type
                let style = if is_selected {
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    match card {
                        Card::Transaction { .. } => Style::default().fg(Color::Yellow),
                        Card::Call { .. } => Style::default().fg(Color::Cyan),
                        Card::Log { .. } => Style::default().fg(Color::Green),
                    }
                };

                // Add selection indicator
                let prefix = if is_selected { "> " } else { "  " };
                let full_text = format!("{}{}", prefix, card_text);

                lines.push(Line::from(Span::styled(full_text, style)));
            }

            // Add card count indicator
            let count_text = format!("  [ Card {} of {} ]", self.state.selected_index + 1, self.state.cards.len());
            lines.push(Line::from(Span::styled(
                count_text,
                Style::default().fg(Color::DarkGray),
            )));
        }

        let paragraph = Paragraph::new(lines);
        paragraph.render(inner_area, buf);
    }
}
