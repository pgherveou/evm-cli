use crate::tui::layout::centered_popup;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Widget},
};

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub shortcut: Option<String>,
}

impl Command {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            shortcut: None,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

pub fn default_commands() -> Vec<Command> {
    vec![
        Command::new("Edit config", "Open config file in $EDITOR"),
        Command::new("Reset", "Clear all saved state"),
        Command::new("Clear output", "Clear the output area"),
        Command::new("Quit", "Exit the application").with_shortcut("Ctrl+C"),
    ]
}

pub struct CommandPalette<'a> {
    query: &'a str,
    selected: usize,
    commands: Vec<Command>,
}

impl<'a> CommandPalette<'a> {
    pub fn new(query: &'a str, selected: usize) -> Self {
        Self {
            query,
            selected,
            commands: default_commands(),
        }
    }

    pub fn with_commands(mut self, commands: Vec<Command>) -> Self {
        self.commands = commands;
        self
    }

    pub fn filtered_commands(&self) -> Vec<(usize, &Command)> {
        let query_lower = self.query.to_lowercase();
        self.commands
            .iter()
            .enumerate()
            .filter(|(_, cmd)| {
                query_lower.is_empty()
                    || cmd.name.to_lowercase().contains(&query_lower)
                    || cmd.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

impl Widget for CommandPalette<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_popup(area, 60, 50);

        // Clear the popup area
        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Commands (Ctrl+P) ");

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        // Render search input
        let search_line = Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Cyan)),
            Span::raw(self.query),
            Span::styled(
                "█",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]);
        buf.set_line(inner.x, inner.y, &search_line, inner.width);

        // Separator
        let separator = Line::from(Span::styled(
            "─".repeat(inner.width as usize),
            Style::default().fg(Color::DarkGray),
        ));
        buf.set_line(inner.x, inner.y + 1, &separator, inner.width);

        // Render filtered commands
        let filtered = self.filtered_commands();
        for (display_idx, (_, cmd)) in filtered.iter().enumerate() {
            let y = inner.y + 2 + display_idx as u16;
            if y >= inner.y + inner.height {
                break;
            }

            let is_selected = display_idx == self.selected;
            let style = if is_selected {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let mut spans = vec![
                Span::styled(
                    if is_selected { "> " } else { "  " },
                    style,
                ),
                Span::styled(&cmd.name, style),
            ];

            // Add description in dimmer style
            let desc_style = if is_selected {
                Style::default().bg(Color::Cyan).fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            spans.push(Span::styled(format!(" - {}", cmd.description), desc_style));

            // Add shortcut if present
            if let Some(shortcut) = &cmd.shortcut {
                let shortcut_style = if is_selected {
                    Style::default().bg(Color::Cyan).fg(Color::Black)
                } else {
                    Style::default().fg(Color::Yellow)
                };
                spans.push(Span::styled(format!(" [{}]", shortcut), shortcut_style));
            }

            let line = Line::from(spans);
            buf.set_line(inner.x, y, &line, inner.width);
        }

        // Show "no matches" if empty
        if filtered.is_empty() {
            let no_match = Line::from(Span::styled(
                "  No matching commands",
                Style::default().fg(Color::DarkGray),
            ));
            buf.set_line(inner.x, inner.y + 2, &no_match, inner.width);
        }
    }
}
