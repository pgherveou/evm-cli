use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct AppLayout {
    pub sidebar: Rect,
    pub output: Rect,
    pub status_bar: Rect,
    pub debug_bar: Option<Rect>,
}

impl AppLayout {
    pub fn new(area: Rect, debug_mode: bool) -> Self {
        let main_split = if debug_mode {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(2),
                ])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(2)])
                .split(area)
        };

        let (content_area, status_area, debug_area) = if debug_mode {
            (main_split[1], main_split[2], Some(main_split[0]))
        } else {
            (main_split[0], main_split[1], None)
        };

        let content_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(content_area);

        Self {
            sidebar: content_split[0],
            output: content_split[1],
            status_bar: status_area,
            debug_bar: debug_area,
        }
    }
}

/// Calculate centered popup area
pub fn centered_popup(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(popup_layout[1])[1]
}
