use crate::cards::Card;
use crate::tui::state::{CardState, OutputState, OutputStyle};
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
        if !self.card_state.cards.is_empty() {
            // Calculate all card lines with tracking
            let mut all_lines = Vec::new();
            let mut card_line_positions = Vec::new(); // (start_line, end_line) for each card

            for (i, card) in self.card_state.cards.iter().enumerate() {
                let is_selected = i == self.card_state.selected_index;
                let start_line = all_lines.len();
                let card_lines = self.format_card(card, is_selected);
                all_lines.extend(card_lines);
                let end_line = all_lines.len();
                card_line_positions.push((start_line, end_line));
            }

            // Apply scroll offset
            let visible_lines: Vec<Line> = all_lines
                .into_iter()
                .skip(self.card_state.scroll_offset)
                .take(padded_area.height as usize)
                .collect();

            let paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });
            paragraph.render(padded_area, buf);
        } else {
            // Display text lines (no cards yet)
            let lines: Vec<Line> = self
                .output_state
                .lines
                .iter()
                .skip(self.output_state.scroll_offset)
                .map(|line| {
                    let style = match line.style {
                        OutputStyle::Normal => Style::default(),
                        OutputStyle::Success => Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                        OutputStyle::Error => {
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                        }
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
            paragraph.render(padded_area, buf);
        }
    }
}

impl OutputArea<'_> {
    fn format_card(&self, card: &Card, is_selected: bool) -> Vec<Line<'_>> {
        let mut lines = Vec::new();

        // Select style based on selection and card type
        // Active: bright colors for selected cards
        // Muted: dim colors for unselected cards
        let (header_style, text_style, border_style) = if is_selected {
            // Active state: bright, bold text
            (
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::White),
                Style::default().fg(Color::Cyan),
            )
        } else {
            // Muted state: darker colors
            (
                Style::default().fg(Color::DarkGray),
                Style::default().fg(Color::DarkGray),
                Style::default().fg(Color::DarkGray),
            )
        };

        // Card type header with left border
        let card_type = match card {
            Card::Transaction { .. } => "Transaction",
            Card::Call { .. } => "Call",
            Card::Log { .. } => "Log",
            Card::Connection { .. } => "Connection",
        };

        // Create left border and header
        let left_border = Span::styled("┃ ", border_style);
        let card_header = Span::styled(card_type, header_style);
        lines.push(Line::from(vec![left_border, card_header]));

        // Card content
        let content = match card {
            Card::Transaction {
                hash,
                status,
                function_name,
                gas_used,
                contract_name,
                contract_address,
                error_message,
            } => {
                let mut content = vec![
                    format!("  Hash: {hash:?}"),
                    format!("  Status: {status}"),
                    format!("  Function: {function_name}"),
                    format!("  Contract: {contract_name}"),
                ];
                if let Some(addr) = contract_address {
                    content.push(format!("  Address: {addr:?}"));
                }
                if let Some(gas) = gas_used {
                    content.push(format!("  Gas: {gas}"));
                }
                if let Some(error) = error_message {
                    content.push(format!("  Error: {error}"));
                }
                content
            }
            Card::Call {
                from,
                to,
                function_signature,
                result,
            } => {
                vec![
                    format!("  Function: {function_signature}"),
                    format!("  To: {to:?}"),
                    format!("  From: {from:?}"),
                    format!(""),
                    format!("  Result: {result}"),
                ]
            }
            Card::Log { message } => {
                // Split message by newlines to support multiline log cards
                message.lines().map(|line| format!("  {line}")).collect()
            }
            Card::Connection {
                connected,
                account,
                balance,
                chain_id,
                error,
            } => {
                let mut content = vec![];

                if *connected {
                    content.push("  Connected".to_string());
                } else {
                    content.push("  Disconnected".to_string());
                }

                content.push(format!("  Account: {account:?}"));

                if let Some(bal) = balance {
                    content.push(format!("  Balance: {bal} ETH"));
                }

                if let Some(chain) = chain_id {
                    content.push(format!("  Chain ID: {chain}"));
                }

                if let Some(err) = error {
                    content.push(format!("  Error: {err}"));
                }

                content
            }
        };

        for content_line in content {
            let border = Span::styled("┃ ", border_style);
            let text = Span::styled(content_line, text_style);
            lines.push(Line::from(vec![border, text]));
        }

        // Add action buttons for selected interactive cards
        if is_selected && card.is_interactive() {
            let actions = crate::cards::get_card_actions(card);
            if !actions.is_empty() {
                // Add empty line before actions
                lines.push(Line::from(Span::styled("┃", border_style)));

                // Build action line with ◇ diamond symbols
                let border = Span::styled("┃ ", border_style);
                let mut action_spans = vec![border];

                for (i, action) in actions.iter().enumerate() {
                    if i > 0 {
                        action_spans.push(Span::styled("   ", text_style));
                    }
                    action_spans.push(Span::styled("◇ ", Style::default().fg(Color::Cyan)));
                    let action_text = match action {
                        crate::cards::CardAction::Copy => "Copy (c)",
                        crate::cards::CardAction::ViewReceipt => "View Receipt (r)",
                        crate::cards::CardAction::DebugTrace => "Debug Trace (d)",
                        crate::cards::CardAction::DebugCall => "Debug Call (d)",
                    };
                    action_spans.push(Span::styled(action_text, text_style));
                }

                lines.push(Line::from(action_spans));
            }
        }

        // Spacing between cards - border line + blank line
        lines.push(Line::from(Span::styled("┃", border_style)));
        lines.push(Line::from("")); // Actual blank line for spacing

        lines
    }
}
