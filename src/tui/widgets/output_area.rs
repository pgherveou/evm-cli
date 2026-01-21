use crate::tui::state::{OutputState, OutputStyle, CardState};
use crate::cards::Card;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct OutputArea<'a> {
    output_state: &'a OutputState,
    card_state: &'a CardState,
    focused: bool,
}

impl<'a> OutputArea<'a> {
    pub fn new(output_state: &'a OutputState, card_state: &'a CardState) -> Self {
        Self {
            output_state,
            card_state,
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

        // Add padding to inner area
        let padded_area = Rect {
            x: inner_area.x + 1,
            y: inner_area.y + 1,
            width: inner_area.width.saturating_sub(2),
            height: inner_area.height.saturating_sub(1),
        };

        // If there are cards, display them; otherwise display text lines
        let lines: Vec<Line> = if !self.card_state.cards.is_empty() {
            // Display cards
            self.card_state.cards
                .iter()
                .enumerate()
                .flat_map(|(i, card)| {
                    let is_selected = i == self.card_state.selected_index;
                    let card_lines = self.format_card(card, is_selected);
                    card_lines
                })
                .collect()
        } else {
            // Display text lines (no cards yet)
            self.output_state
                .lines
                .iter()
                .skip(self.output_state.scroll_offset)
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
                .collect()
        };

        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

        paragraph.render(padded_area, buf);
    }
}

impl OutputArea<'_> {
    fn format_card(&self, card: &Card, is_selected: bool) -> Vec<Line<'_>> {
        let mut lines = Vec::new();

        // Select style based on selection and card type
        let (header_style, text_style) = if is_selected {
            (
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
            )
        } else {
            let base_style = match card {
                Card::Transaction { .. } => Style::default().fg(Color::Yellow),
                Card::Call { .. } => Style::default().fg(Color::Cyan),
                Card::Log { .. } => Style::default().fg(Color::Green),
            };
            (base_style, base_style)
        };

        // Top border
        let border = format!("┌{}┐", "─".repeat(70));
        lines.push(Line::from(Span::styled(border, text_style)));

        // Card type header
        let card_type = match card {
            Card::Transaction { .. } => "Transaction",
            Card::Call { .. } => "Call",
            Card::Log { .. } => "Log",
        };
        lines.push(Line::from(Span::styled(
            format!("│ {} │", card_type),
            header_style,
        )));

        // Card content
        let content = match card {
            Card::Transaction { hash, status, function_name, gas_used } => {
                let hash_str = format!("{:?}", hash);
                let mut content = vec![
                    format!("│ Hash: {}... │", &hash_str[..10.min(hash_str.len())]),
                    format!("│ Status: {} │", status),
                    format!("│ Function: {} │", function_name),
                ];
                if let Some(gas) = gas_used {
                    content.push(format!("│ Gas: {} │", gas));
                }
                content
            }
            Card::Call { from, to, function_signature, value } => {
                let from_str = format!("{:?}", from);
                let to_str = format!("{:?}", to);
                vec![
                    format!("│ Function: {} │", function_signature),
                    format!("│ From: {}... │", &from_str[..8.min(from_str.len())]),
                    format!("│ To: {}... │", &to_str[..8.min(to_str.len())]),
                    format!("│ Value: {} │", value),
                ]
            }
            Card::Log { message } => {
                vec![format!("│ {} │", message)]
            }
        };

        for content_line in content {
            lines.push(Line::from(Span::styled(content_line, text_style)));
        }

        // Bottom border
        let border = format!("└{}┘", "─".repeat(70));
        lines.push(Line::from(Span::styled(border, text_style)));

        // Spacing between cards
        lines.push(Line::from(""));

        lines
    }
}
