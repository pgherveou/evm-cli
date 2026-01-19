use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent, MouseEvent, MouseEventKind};
use std::time::Duration;

/// Input events from the terminal
pub enum InputEvent {
    Key(KeyEvent),
    ScrollUp(u16, u16),  // (column, row)
    ScrollDown(u16, u16),
}

/// Poll for input events with a short timeout.
/// Returns None if no event was received within the timeout.
pub fn poll_event() -> Result<Option<InputEvent>> {
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) => return Ok(Some(InputEvent::Key(key))),
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollUp, column, row, .. }) => {
                return Ok(Some(InputEvent::ScrollUp(column, row)));
            }
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollDown, column, row, .. }) => {
                return Ok(Some(InputEvent::ScrollDown(column, row)));
            }
            _ => {}
        }
    }
    Ok(None)
}
