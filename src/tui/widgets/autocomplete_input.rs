use crate::tui::theme;
use crate::tui::widgets::InputField;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
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

pub fn parse_path_for_autocomplete(input: &str) -> (PathBuf, String) {
    if input.is_empty() {
        return (PathBuf::from("."), String::new());
    }

    let expanded_input = expand_home_dir(input);
    let path = Path::new(&expanded_input);

    if input.ends_with('/') || input.ends_with('\\') {
        return (path.to_path_buf(), String::new());
    }

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

pub fn scan_path_suggestions(dir: &Path, prefix: &str) -> Vec<PathSuggestion> {
    let mut suggestions = Vec::new();
    let show_hidden = prefix.starts_with('.');
    let prefix_lower = prefix.to_lowercase();

    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return suggestions,
    };

    let mut sol_files = Vec::new();
    let mut directories = Vec::new();

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        if name_str.starts_with('.') && !show_hidden {
            continue;
        }

        if !prefix.is_empty() && !name_str.to_lowercase().starts_with(&prefix_lower) {
            continue;
        }

        let path = entry.path();
        let is_dir = path.is_dir();

        let suggestion = PathSuggestion::new(name_str.to_string(), path, is_dir);

        if suggestion.is_sol_file {
            sol_files.push(suggestion);
        } else if suggestion.is_directory {
            directories.push(suggestion);
        }

        if sol_files.len() + directories.len() >= 50 {
            break;
        }
    }

    sol_files.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    directories.sort_by(|a, b| a.display_name.cmp(&b.display_name));

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

        let mut input = InputField::new(self.label, self.value)
            .focused(self.focused)
            .cursor_position(self.value.len());

        if let Some(placeholder) = self.placeholder {
            input = input.placeholder(placeholder);
        }
        if let Some(error) = self.error {
            input = input.error(Some(error));
        }

        let input_height = if self.error.is_some() { 2 } else { 1 };
        let input_area = Rect::new(area.x, current_y, area.width, input_height);
        input.render(input_area, buf);
        current_y += input_height;

        if self.show_suggestions && self.focused && !self.suggestions.is_empty() {
            if let Some(selected) = self.suggestions.get(self.selected_suggestion) {
                if current_y < area.y + area.height {
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
                        Span::styled("  → ", theme::muted_style()),
                        Span::styled(preview_path, Style::default().fg(theme::ACCENT)),
                    ]);
                    buf.set_line(area.x, current_y, &preview_line, area.width);
                    current_y += 1;
                }
            }
        }

        if self.show_suggestions && self.focused && current_y < area.y + area.height {
            let separator = Line::from(Span::styled(
                "─".repeat(area.width as usize),
                theme::separator_style(),
            ));
            if current_y < area.y + area.height {
                buf.set_line(area.x, current_y, &separator, area.width);
                current_y += 1;
            }

            let max_suggestions = 8;
            let visible_suggestions = self.suggestions.iter().take(max_suggestions);

            for (idx, suggestion) in visible_suggestions.enumerate() {
                if current_y >= area.y + area.height {
                    break;
                }

                let is_selected = idx == self.selected_suggestion;

                let (name_style, prefix_style) = if is_selected {
                    (theme::selected_style(), theme::selected_style())
                } else if suggestion.is_sol_file {
                    (Style::default().fg(theme::PRIMARY), theme::muted_style())
                } else if suggestion.is_directory {
                    (theme::muted_style(), theme::muted_style())
                } else {
                    (Style::default().fg(theme::TEXT), theme::muted_style())
                };

                let mut line_spans = vec![Span::styled(
                    if is_selected { "> " } else { "  " },
                    prefix_style,
                )];

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
