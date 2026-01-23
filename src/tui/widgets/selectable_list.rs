use crate::tui::theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

type RenderItemFn<'a, T> = Box<dyn Fn(&T, bool) -> Vec<Span<'static>> + 'a>;

pub struct SelectableList<'a, T> {
    items: &'a [T],
    selected: usize,
    render_item: RenderItemFn<'a, T>,
}

impl<'a, T> SelectableList<'a, T> {
    pub fn new<F>(items: &'a [T], selected: usize, render_item: F) -> Self
    where
        F: Fn(&T, bool) -> Vec<Span<'static>> + 'a,
    {
        Self {
            items,
            selected,
            render_item: Box::new(render_item),
        }
    }
}

impl<'a> SelectableList<'a, String> {
    pub fn simple(items: &'a [String], selected: usize) -> Self {
        SelectableList::new(items, selected, |item, is_selected| {
            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };
            vec![
                Span::styled(if is_selected { "> " } else { "  " }, style),
                Span::styled(item.clone(), style),
            ]
        })
    }
}

impl<T> Widget for SelectableList<'_, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (idx, item) in self.items.iter().enumerate() {
            let y = area.y + idx as u16;
            if y >= area.y + area.height {
                break;
            }

            let is_selected = idx == self.selected;
            let spans = (self.render_item)(item, is_selected);
            let line = Line::from(spans);
            buf.set_line(area.x, y, &line, area.width);
        }
    }
}
