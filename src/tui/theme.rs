#![allow(dead_code)]

use ratatui::style::{Color, Modifier, Style};

// ============================================================================
// Color Palette
// ============================================================================

/// Primary accent color - used for focused elements, borders, highlights
pub const PRIMARY: Color = Color::Cyan;

/// Muted color - used for secondary text, separators, unfocused elements
pub const MUTED: Color = Color::DarkGray;

/// Accent color - used for shortcuts, special highlights
pub const ACCENT: Color = Color::Yellow;

/// Error color - used for error messages and validation failures
pub const ERROR: Color = Color::Red;

/// Success color - used for success messages and confirmations
pub const SUCCESS: Color = Color::Green;

/// Default text color
pub const TEXT: Color = Color::White;

/// Background color for selected items
pub const SELECTION_BG: Color = Color::Cyan;

/// Foreground color for selected items
pub const SELECTION_FG: Color = Color::Black;

// ============================================================================
// Style Helpers
// ============================================================================

/// Style for selected/highlighted items (e.g., selected list item)
#[inline]
pub fn selected_style() -> Style {
    Style::default()
        .bg(SELECTION_BG)
        .fg(SELECTION_FG)
        .add_modifier(Modifier::BOLD)
}

/// Style for muted/secondary text
#[inline]
pub fn muted_style() -> Style {
    Style::default().fg(MUTED)
}

/// Style for error text
#[inline]
pub fn error_style() -> Style {
    Style::default().fg(ERROR)
}

/// Style for success text
#[inline]
pub fn success_style() -> Style {
    Style::default().fg(SUCCESS)
}

/// Style for popup borders
#[inline]
pub fn border_style() -> Style {
    Style::default().fg(PRIMARY)
}

/// Style for popup titles
#[inline]
pub fn title_style() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

/// Style for focused labels
#[inline]
pub fn focused_label_style() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

/// Style for unfocused labels
#[inline]
pub fn label_style() -> Style {
    Style::default().fg(TEXT)
}

/// Style for keyboard shortcut hints (the key part)
#[inline]
pub fn hint_key_style() -> Style {
    Style::default().fg(ACCENT)
}

/// Style for keyboard shortcut hints (the description part)
#[inline]
pub fn hint_desc_style() -> Style {
    Style::default().fg(MUTED)
}

/// Style for blinking cursor
#[inline]
pub fn cursor_style() -> Style {
    Style::default()
        .fg(PRIMARY)
        .add_modifier(Modifier::SLOW_BLINK)
}

/// Style for cursor when positioned on a character (block cursor)
#[inline]
pub fn cursor_on_char_style() -> Style {
    Style::default().fg(SELECTION_FG).bg(SELECTION_BG)
}

/// Style for placeholder text
#[inline]
pub fn placeholder_style() -> Style {
    Style::default().fg(MUTED)
}

/// Style for separators
#[inline]
pub fn separator_style() -> Style {
    Style::default().fg(MUTED)
}

/// Style for prompt character (e.g., "> ")
#[inline]
pub fn prompt_style() -> Style {
    Style::default().fg(PRIMARY)
}
