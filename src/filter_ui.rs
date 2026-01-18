use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

pub struct FilterItem {
    pub label: String,
    pub tag: Option<String>,
}

impl FilterItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            tag: None,
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

pub enum FilterResult<T> {
    Selected(T),
    Cancelled,
    SwitchTo(char),
    Exit,
}

pub struct FilterUI<T> {
    prefix: char,
    items: Vec<(FilterItem, T)>,
    input: String,
    selected: usize,
    scroll_offset: usize,
    footer: Vec<String>,
}

impl<T: Clone> FilterUI<T> {
    pub fn new(prefix: char, items: Vec<(FilterItem, T)>) -> Self {
        Self {
            prefix,
            items,
            input: String::new(),
            selected: 0,
            scroll_offset: 0,
            footer: Vec::new(),
        }
    }

    pub fn with_footer(mut self, footer: Vec<String>) -> Self {
        self.footer = footer;
        self
    }

    fn filtered_items(&self) -> Vec<(usize, &FilterItem, &T)> {
        let query = self.input.to_lowercase();
        self.items
            .iter()
            .enumerate()
            .filter(|(_, (item, _))| {
                query.is_empty() || item.label.to_lowercase().contains(&query)
            })
            .map(|(i, (item, data))| (i, item, data))
            .collect()
    }

    fn render(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        let (_, term_height) = terminal::size()?;
        let footer_height = self.footer.len() as u16;

        // Fixed menu height: up to 5 items + 1 "more" indicator + 1 separator + 1 input line + footer
        let max_items = 5usize;
        let menu_height = max_items as u16 + 3 + footer_height; // items + more/separator + input + footer
        let menu_start_row = term_height.saturating_sub(menu_height);
        let input_row = term_height.saturating_sub(footer_height + 1);

        let filtered = self.filtered_items();

        // Clear only the menu area (from menu_start_row to bottom)
        for row in menu_start_row..term_height {
            stdout.queue(MoveTo(0, row))?;
            stdout.queue(Clear(ClearType::CurrentLine))?;
        }

        // Separator row is right above input
        let separator_row = input_row.saturating_sub(1);

        if filtered.is_empty() {
            // Show "No matches" right above separator
            stdout.queue(MoveTo(0, separator_row.saturating_sub(1)))?;
            stdout
                .queue(SetForegroundColor(Color::DarkGrey))?
                .queue(Print("  No matches"))?
                .queue(ResetColor)?;
        } else {
            // Render items bottom-up (max 5, with "..." if more)
            let visible_start = self.scroll_offset;
            let has_more = filtered.len() > visible_start + max_items;
            let display_items = max_items.min(filtered.len() - visible_start);
            let visible_end = visible_start + display_items;

            // Calculate how many rows we need (items + optional "more" line)
            let total_rows = display_items + if has_more { 1 } else { 0 };
            let items_start_row = separator_row.saturating_sub(total_rows as u16);

            for (i, display_idx) in (visible_start..visible_end).enumerate() {
                let (_, item, _) = &filtered[display_idx];
                let is_selected = display_idx == self.selected;

                let row = items_start_row + i as u16;
                stdout.queue(MoveTo(0, row))?;

                if is_selected {
                    stdout
                        .queue(SetForegroundColor(Color::Black))?
                        .queue(Print("\x1b[46m"))?  // Cyan background
                        .queue(Print("> "))?;
                } else {
                    stdout.queue(Print("  "))?;
                }

                // Label
                stdout.queue(Print(&item.label))?;

                // Tag
                if let Some(tag) = &item.tag {
                    stdout
                        .queue(Print(" "))?
                        .queue(SetForegroundColor(if is_selected {
                            Color::Black
                        } else {
                            Color::DarkGrey
                        }))?
                        .queue(Print(format!("[{}]", tag)))?;
                }

                if is_selected {
                    stdout.queue(Print("\x1b[0m"))?;  // Reset
                }

                stdout.queue(ResetColor)?;
            }

            // Show "..." if there are more items (right above separator)
            if has_more {
                let more_count = filtered.len() - visible_end;
                let row = separator_row.saturating_sub(1);
                stdout.queue(MoveTo(0, row))?;
                stdout
                    .queue(SetForegroundColor(Color::DarkGrey))?
                    .queue(Print(format!("  ... {} more", more_count)))?
                    .queue(ResetColor)?;
            }
        }

        // Separator line above input
        stdout.queue(MoveTo(0, separator_row))?;
        stdout
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print("─".repeat(65)))?
            .queue(ResetColor)?;

        // Input line
        stdout.queue(MoveTo(0, input_row))?;
        stdout
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print(self.prefix))?
            .queue(ResetColor)?
            .queue(Print(&self.input))?
            .queue(Print("█"))?;

        // Footer
        for (i, line) in self.footer.iter().enumerate() {
            let row = input_row + 1 + i as u16;
            stdout.queue(MoveTo(0, row))?;
            stdout.queue(Print(line))?;
        }

        stdout.flush()
    }

    pub fn run(mut self) -> io::Result<FilterResult<T>> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        stdout.execute(cursor::Hide)?;

        // Scroll terminal to make room for menu at bottom
        let (_, term_height) = terminal::size()?;
        let footer_height = self.footer.len() as u16;
        let menu_height = 5 + 3 + footer_height; // max_items + more/separator + input + footer
        for _ in 0..menu_height {
            stdout.execute(Print("\n"))?;
        }

        let result = self.run_loop(&mut stdout);

        // Clear only the menu area when exiting
        let menu_start_row = term_height.saturating_sub(menu_height);
        for row in menu_start_row..term_height {
            stdout.queue(MoveTo(0, row))?;
            stdout.queue(Clear(ClearType::CurrentLine))?;
        }
        stdout.flush()?;

        stdout.execute(cursor::Show)?;
        stdout.execute(MoveTo(0, menu_start_row))?;
        terminal::disable_raw_mode()?;

        result
    }

    fn run_loop(&mut self, stdout: &mut io::Stdout) -> io::Result<FilterResult<T>> {
        loop {
            self.render(stdout)?;

            if let Event::Key(key) = event::read()? {
                // Handle Ctrl+C - exit app
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('c')
                {
                    return Ok(FilterResult::Exit);
                }

                match key.code {
                    KeyCode::Enter => {
                        let filtered = self.filtered_items();
                        if let Some((_, _, data)) = filtered.get(self.selected) {
                            return Ok(FilterResult::Selected((*data).clone()));
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(FilterResult::Cancelled);
                    }
                    KeyCode::Backspace => {
                        if !self.input.is_empty() {
                            self.input.pop();
                            self.selected = 0;
                            self.scroll_offset = 0;
                        }
                        // Swallow backspace on empty input
                    }
                    KeyCode::Char(c) => {
                        // If input is empty and user types a different prefix, switch modes
                        if self.input.is_empty() && (c == '/' || c == '@') && c != self.prefix {
                            return Ok(FilterResult::SwitchTo(c));
                        }
                        self.input.push(c);
                        self.selected = 0;
                        self.scroll_offset = 0;
                    }
                    KeyCode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                            if self.selected < self.scroll_offset {
                                self.scroll_offset = self.selected;
                            }
                        }
                    }
                    KeyCode::Down => {
                        let filtered_len = self.filtered_items().len();
                        if self.selected + 1 < filtered_len {
                            self.selected += 1;
                            let (_, term_height) = terminal::size().unwrap_or((80, 24));
                            let max_visible = (term_height as usize).saturating_sub(4);
                            if self.selected >= self.scroll_offset + max_visible {
                                self.scroll_offset = self.selected - max_visible + 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
