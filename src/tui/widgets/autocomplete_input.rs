use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PathSuggestion {
    pub display_name: String,
    pub full_path: PathBuf,
    pub is_directory: bool,
    pub is_sol_file: bool,
}

impl PathSuggestion {
    pub fn new(display_name: String, full_path: PathBuf, is_directory: bool) -> Self {
        let is_sol_file = !is_directory && full_path.extension().is_some_and(|ext| ext == "sol");
        Self {
            display_name,
            full_path,
            is_directory,
            is_sol_file,
        }
    }
}

/// Parses user input to determine which directory to scan and what prefix to match
///
/// Examples:
/// - "./cont" → (Path("."), "cont")
/// - "./contracts/" → (Path("./contracts"), "")
/// - "src/hello" → (Path("src"), "hello")
/// - "" → (Path("."), "")
/// - "~/foo" → (Path("/home/user"), "foo")
pub fn parse_path_for_autocomplete(input: &str) -> (PathBuf, String) {
    if input.is_empty() {
        return (PathBuf::from("."), String::new());
    }

    // Expand ~ to home directory
    let expanded_input = expand_home_dir(input);
    let path = Path::new(&expanded_input);

    // If input ends with separator, scan that directory with empty prefix
    if input.ends_with('/') || input.ends_with('\\') {
        return (path.to_path_buf(), String::new());
    }

    // Split into parent dir and file prefix
    let Some(parent) = path.parent() else {
        return (PathBuf::from("."), input.to_string());
    };

    let prefix = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let dir = if parent.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        parent.to_path_buf()
    };

    (dir, prefix)
}

fn expand_home_dir(input: &str) -> String {
    let home = std::env::var_os("HOME").map(PathBuf::from);

    match (input, home) {
        ("~", Some(home)) => home.to_string_lossy().to_string(),
        (s, Some(home)) if s.starts_with("~/") => home.join(&s[2..]).to_string_lossy().to_string(),
        _ => input.to_string(),
    }
}

/// Scans filesystem for matching files and directories
///
/// Only shows: .sol files and directories (for navigation)
/// Filters by prefix (case-insensitive)
/// Skips hidden files unless prefix starts with '.'
/// Limits to 50 suggestions for performance
pub fn scan_path_suggestions(dir: &Path, prefix: &str) -> Vec<PathSuggestion> {
    let mut suggestions = Vec::new();
    let show_hidden = prefix.starts_with('.');
    let prefix_lower = prefix.to_lowercase();

    // Try to read directory, fail silently on permission errors
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return suggestions,
    };

    let mut sol_files = Vec::new();
    let mut directories = Vec::new();

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        // Skip hidden files unless prefix starts with '.'
        if name_str.starts_with('.') && !show_hidden {
            continue;
        }

        // Filter by prefix (case-insensitive)
        if !prefix.is_empty() && !name_str.to_lowercase().starts_with(&prefix_lower) {
            continue;
        }

        let path = entry.path();
        let is_dir = path.is_dir();

        let suggestion = PathSuggestion::new(name_str.to_string(), path, is_dir);

        // Only include .sol files and directories
        if suggestion.is_sol_file {
            sol_files.push(suggestion);
        } else if suggestion.is_directory {
            directories.push(suggestion);
        }
        // Skip all other file types

        // Limit total suggestions for performance
        if sol_files.len() + directories.len() >= 50 {
            break;
        }
    }

    // Sort each category alphabetically
    sol_files.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    directories.sort_by(|a, b| a.display_name.cmp(&b.display_name));

    // Combine in priority order: .sol files → directories
    suggestions.extend(sol_files);
    suggestions.extend(directories);

    suggestions
}

pub struct AutocompleteInput<'a> {
    label: &'a str,
    value: &'a str,
    placeholder: Option<&'a str>,
    error: Option<&'a str>,
    focused: bool,
    cursor_position: usize,
    suggestions: &'a [PathSuggestion],
    selected_suggestion: usize,
    show_suggestions: bool,
}

impl<'a> AutocompleteInput<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            placeholder: None,
            error: None,
            focused: false,
            cursor_position: value.len(),
            suggestions: &[],
            selected_suggestion: 0,
            show_suggestions: false,
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

    pub fn suggestions(mut self, suggestions: &'a [PathSuggestion]) -> Self {
        self.suggestions = suggestions;
        self.show_suggestions = !suggestions.is_empty();
        self
    }

    pub fn selected_suggestion(mut self, selected: usize) -> Self {
        self.selected_suggestion = selected;
        self
    }
}

impl Widget for AutocompleteInput<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 1 {
            return;
        }

        let mut current_y = area.y;

        // Render input field (same as InputField)
        let label_style = if self.focused {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
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
            let (before, after) = self
                .value
                .split_at(self.cursor_position.min(self.value.len()));
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
                        Style::default().fg(Color::Black).bg(Color::Cyan),
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
        buf.set_line(area.x, current_y, &line, area.width);
        current_y += 1;

        // Error message on second line if present
        if let Some(error) = self.error {
            if current_y < area.y + area.height {
                let error_line = Line::from(Span::styled(
                    format!("  └ {error}"),
                    Style::default().fg(Color::Red),
                ));
                buf.set_line(area.x, current_y, &error_line, area.width);
                current_y += 1;
            }
        }

        // Show preview of selected path
        if self.show_suggestions && self.focused && !self.suggestions.is_empty() {
            if let Some(selected) = self.suggestions.get(self.selected_suggestion) {
                if current_y < area.y + area.height {
                    // Show the canonicalized (fully resolved) path
                    let resolved_path = selected
                        .full_path
                        .canonicalize()
                        .unwrap_or_else(|_| selected.full_path.clone());

                    let preview_path = if selected.is_directory {
                        format!("{}/", resolved_path.to_string_lossy())
                    } else {
                        resolved_path.to_string_lossy().to_string()
                    };

                    let preview_line = Line::from(vec![
                        Span::styled("  → ", Style::default().fg(Color::DarkGray)),
                        Span::styled(preview_path, Style::default().fg(Color::Yellow)),
                    ]);
                    buf.set_line(area.x, current_y, &preview_line, area.width);
                    current_y += 1;
                }
            }
        }

        // Render suggestions dropdown if visible
        if self.show_suggestions && self.focused && current_y < area.y + area.height {
            // Add separator
            let separator = Line::from(Span::styled(
                "─".repeat(area.width as usize),
                Style::default().fg(Color::DarkGray),
            ));
            if current_y < area.y + area.height {
                buf.set_line(area.x, current_y, &separator, area.width);
                current_y += 1;
            }

            // Render up to 8 suggestions
            let max_suggestions = 8;
            let visible_suggestions = self.suggestions.iter().take(max_suggestions);

            for (idx, suggestion) in visible_suggestions.enumerate() {
                if current_y >= area.y + area.height {
                    break;
                }

                let is_selected = idx == self.selected_suggestion;

                // Determine style based on file type and selection
                let (name_style, prefix_style) = if is_selected {
                    (
                        Style::default()
                            .bg(Color::Cyan)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                        Style::default().bg(Color::Cyan).fg(Color::Black),
                    )
                } else if suggestion.is_sol_file {
                    (
                        Style::default().fg(Color::Cyan),
                        Style::default().fg(Color::DarkGray),
                    )
                } else if suggestion.is_directory {
                    (
                        Style::default().fg(Color::DarkGray),
                        Style::default().fg(Color::DarkGray),
                    )
                } else {
                    (
                        Style::default().fg(Color::White),
                        Style::default().fg(Color::DarkGray),
                    )
                };

                let mut line_spans = vec![Span::styled(
                    if is_selected { "> " } else { "  " },
                    prefix_style,
                )];

                // Add directory suffix if applicable
                let display_text = if suggestion.is_directory {
                    format!("{}/", suggestion.display_name)
                } else {
                    suggestion.display_name.clone()
                };

                line_spans.push(Span::styled(display_text, name_style));

                let suggestion_line = Line::from(line_spans);
                buf.set_line(area.x, current_y, &suggestion_line, area.width);
                current_y += 1;
            }
        }
    }
}
