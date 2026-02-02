use crate::tui::layout::centered_popup;
use crate::tui::state::RustConfigField;
use crate::tui::theme;
use crate::tui::widgets::KeyboardHints;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Widget},
};
use std::path::PathBuf;

pub struct RustConfigPopup<'a> {
    project_path: &'a PathBuf,
    bin_targets: &'a [String],
    selected_bin: usize,
    sol_files: &'a [PathBuf],
    selected_sol: usize,
    release_mode: bool,
    current_field: RustConfigField,
}

impl<'a> RustConfigPopup<'a> {
    pub fn new(
        project_path: &'a PathBuf,
        bin_targets: &'a [String],
        selected_bin: usize,
        sol_files: &'a [PathBuf],
        selected_sol: usize,
        release_mode: bool,
        current_field: RustConfigField,
    ) -> Self {
        Self {
            project_path,
            bin_targets,
            selected_bin,
            sol_files,
            selected_sol,
            release_mode,
            current_field,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_dropdown(
        &self,
        buf: &mut Buffer,
        x: u16,
        y: u16,
        width: u16,
        label: &str,
        items: &[String],
        selected: usize,
        is_focused: bool,
    ) {
        let label_style = if is_focused {
            theme::focused_label_style()
        } else {
            theme::label_style()
        };

        let label_line = Line::from(Span::styled(label, label_style));
        buf.set_line(x, y, &label_line, width);

        let value = items.get(selected).map(|s| s.as_str()).unwrap_or("(none)");
        let value_style = if is_focused {
            theme::selected_style()
        } else {
            Style::default().fg(theme::TEXT)
        };

        let arrows = if is_focused { "◄ " } else { "  " };
        let arrows_end = if is_focused { " ►" } else { "" };

        let value_line = Line::from(vec![
            Span::styled(arrows, theme::hint_key_style()),
            Span::styled(format!(" {value} "), value_style),
            Span::styled(arrows_end, theme::hint_key_style()),
        ]);
        buf.set_line(x, y + 1, &value_line, width);
    }
}

impl Widget for RustConfigPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_popup(area, 60, 50);

        Clear.render(popup_area, buf);

        let title = " Configure Rust Contract ";
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(title);

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let mut y = inner.y + 1;

        let path_label = Line::from(vec![
            Span::styled("Project: ", theme::muted_style()),
            Span::styled(
                self.project_path.display().to_string(),
                Style::default().fg(theme::TEXT),
            ),
        ]);
        buf.set_line(inner.x + 1, y, &path_label, inner.width.saturating_sub(2));
        y += 2;

        let bin_items: Vec<String> = self.bin_targets.to_vec();
        self.render_dropdown(
            buf,
            inner.x + 1,
            y,
            inner.width.saturating_sub(2),
            "Binary Target:",
            &bin_items,
            self.selected_bin,
            self.current_field == RustConfigField::BinTarget,
        );
        y += 3;

        let sol_items: Vec<String> = self
            .sol_files
            .iter()
            .map(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| p.display().to_string())
            })
            .collect();
        self.render_dropdown(
            buf,
            inner.x + 1,
            y,
            inner.width.saturating_sub(2),
            "Solidity Interface (.sol):",
            &sol_items,
            self.selected_sol,
            self.current_field == RustConfigField::SolInterface,
        );
        y += 3;

        let mode_focused = self.current_field == RustConfigField::BuildMode;
        let mode_label_style = if mode_focused {
            theme::focused_label_style()
        } else {
            theme::label_style()
        };

        let mode_label = Line::from(Span::styled("Build Mode:", mode_label_style));
        buf.set_line(inner.x + 1, y, &mode_label, inner.width.saturating_sub(2));
        y += 1;

        let (debug_style, release_style) = if mode_focused {
            if self.release_mode {
                (theme::muted_style(), theme::selected_style())
            } else {
                (theme::selected_style(), theme::muted_style())
            }
        } else if self.release_mode {
            (theme::muted_style(), Style::default().fg(theme::TEXT))
        } else {
            (Style::default().fg(theme::TEXT), theme::muted_style())
        };

        let mode_line = Line::from(vec![
            Span::styled(" Debug ", debug_style),
            Span::styled(" / ", theme::muted_style()),
            Span::styled(" Release ", release_style),
        ]);
        buf.set_line(inner.x + 1, y, &mode_line, inner.width.saturating_sub(2));
        y += 2;

        let buttons_focused = self.current_field == RustConfigField::Buttons;
        let build_style = if buttons_focused {
            theme::selected_style()
        } else {
            Style::default().fg(theme::SUCCESS)
        };

        let buttons_line = Line::from(vec![
            Span::styled("  [ ", theme::muted_style()),
            Span::styled("Build", build_style),
            Span::styled(" ]  ", theme::muted_style()),
        ]);
        buf.set_line(inner.x + 1, y, &buttons_line, inner.width.saturating_sub(2));

        let footer_y = inner.y + inner.height.saturating_sub(1);
        let hints = vec![
            ("↑/↓", "field"),
            ("←/→", "change"),
            ("Enter", "build"),
            ("Esc", "cancel"),
        ];

        let keyboard_hints = KeyboardHints::new(hints);
        let hints_area = Rect::new(inner.x + 1, footer_y, inner.width.saturating_sub(2), 1);
        keyboard_hints.render(hints_area, buf);
    }
}
