use crate::tui::theme;
use crate::tui::widgets::{KeyboardHints, Popup};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
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

pub fn default_commands(debug_mode: bool) -> Vec<Command> {
    let debug_label = if debug_mode {
        "Toggle Debug [ON]"
    } else {
        "Toggle Debug [OFF]"
    };
    vec![
        Command::new("Edit config", "Open config file in $EDITOR"),
        Command::new("Clear output", "Clear the output area"),
        Command::new("Open Logs", "Open application log file"),
        Command::new("Clear Logs", "Delete the application log file"),
        Command::new("Reconnect", "Retry connection to RPC server"),
        Command::new(debug_label, "Toggle debug panel visibility"),
        Command::new("Reset", "Clear all saved state"),
        Command::new("Quit", "Exit the application").with_shortcut("Ctrl+C"),
    ]
}

pub struct CommandPalette<'a> {
    query: &'a str,
    selected: usize,
    commands: Vec<Command>,
}

impl<'a> CommandPalette<'a> {
    pub fn new(query: &'a str, selected: usize, debug_mode: bool) -> Self {
        Self {
            query,
            selected,
            commands: default_commands(debug_mode),
        }
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
        let popup = Popup::new("Commands (Ctrl+P)")
            .width_percent(60)
            .height_percent(50);
        let inner = popup.render_frame(area, buf);

        let search_line = Line::from(vec![
            Span::styled("> ", theme::prompt_style()),
            Span::raw(self.query),
            Span::styled("█", theme::cursor_style()),
        ]);
        buf.set_line(inner.x, inner.y, &search_line, inner.width);

        let separator = Line::from(Span::styled(
            "─".repeat(inner.width as usize),
            theme::separator_style(),
        ));
        buf.set_line(inner.x, inner.y + 1, &separator, inner.width);

        let filtered = self.filtered_commands();
        for (display_idx, (_, cmd)) in filtered.iter().enumerate() {
            let y = inner.y + 2 + display_idx as u16;
            if y >= inner.y + inner.height.saturating_sub(2) {
                break;
            }

            let is_selected = display_idx == self.selected;
            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            let mut spans = vec![
                Span::styled(if is_selected { "> " } else { "  " }, style),
                Span::styled(&cmd.name, style),
            ];

            let desc_style = if is_selected {
                Style::default().bg(theme::SELECTION_BG).fg(theme::MUTED)
            } else {
                theme::muted_style()
            };
            spans.push(Span::styled(format!(" - {}", cmd.description), desc_style));

            if let Some(shortcut) = &cmd.shortcut {
                let shortcut_style = if is_selected {
                    Style::default()
                        .bg(theme::SELECTION_BG)
                        .fg(theme::SELECTION_FG)
                } else {
                    Style::default().fg(theme::ACCENT)
                };
                spans.push(Span::styled(format!(" [{shortcut}]"), shortcut_style));
            }

            let line = Line::from(spans);
            buf.set_line(inner.x, y, &line, inner.width);
        }

        if filtered.is_empty() {
            let no_match = Line::from(Span::styled("  No matching commands", theme::muted_style()));
            buf.set_line(inner.x, inner.y + 2, &no_match, inner.width);
        }

        let hints = KeyboardHints::new(vec![
            ("↑↓", "navigate"),
            ("Enter", "select"),
            ("Esc", "close"),
        ]);
        let hints_y = inner.y + inner.height.saturating_sub(1);
        let hints_area = Rect::new(inner.x, hints_y, inner.width, 1);
        hints.render(hints_area, buf);
    }
}
