//! Titlebar and window chrome styling.
//!
//! Centralizes colors for the titlebar bar, default button background, min/max hover, close hover,
//! container border, icon color, and title alignment.

use iced::widget::button::{self, Status as ButtonStatus};
use iced::widget::container;
use iced::{Color, Theme};

/// Horizontal alignment of the title text inside the titlebar draggable area.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleAlignment {
    /// Title aligned to the left.
    Left,
    /// Title centered (default).
    #[default]
    Center,
    /// Title aligned to the right.
    Right,
}

/// Style for the titlebar and its buttons: bar and border colors, button hover colors, icon color, title alignment.
///
/// - `bar`: Background of the whole titlebar and default background of all three buttons.
/// - `button_hover`: Hover/pressed background for minimize and maximize buttons.
/// - `close_hover`: Hover/pressed background for the close button (typically red).
/// - `icon`: Color used for the SVG window-control icons (minimize, maximize, close) and any button text. SVGs use `currentColor` so they inherit this.
/// - `border_color`: Color of the titlebar container border (when `border_width` > 0).
/// - `border_width`: Width of the titlebar container border; 0 means no border.
/// - `title_alignment`: Placement of the title text in the draggable area (left, center, right).
#[derive(Debug, Clone, Copy)]
pub struct TitlebarStyle {
    /// Background color for the titlebar and for all buttons in their default state.
    pub bar: Color,
    /// Hover/pressed background for minimize and maximize buttons.
    pub button_hover: Color,
    /// Hover/pressed background for the close button.
    pub close_hover: Color,
    /// Color for the SVG icons (minimize, maximize, close) and button text. SVGs use `currentColor` so they inherit this.
    pub icon: Color,
    /// Color of the titlebar container border. Used when `border_width` > 0.
    pub border_color: Color,
    /// Width of the titlebar container border. Default 0 (borderless).
    pub border_width: f32,
    /// Placement of the title text inside the titlebar: left, center, or right.
    pub title_alignment: TitleAlignment,
}

impl Default for TitlebarStyle {
    fn default() -> Self {
        Self {
            bar: Color::from_rgb8(30, 30, 30),
            button_hover: Color::from_rgb8(60, 60, 60),
            close_hover: Color::from_rgb8(232, 17, 35),
            icon: Color::from_rgb8(240, 240, 240),
            border_color: Color::from_rgb8(160, 160, 160),
            border_width: 0.0,
            title_alignment: TitleAlignment::default(),
        }
    }
}

/// Returns the container style for the titlebar (background and optional border).
pub fn bar_container_style(style: &TitlebarStyle) -> container::Style {
    container::Style::default()
        .background(iced::Background::Color(style.bar))
        .border(
            iced::Border::default()
                .width(style.border_width)
                .color(style.border_color),
        )
}

/// Returns the button style for minimize and maximize: bar color by default, `button_hover` when hovered/pressed.
pub fn min_max_button_style(
    style: &TitlebarStyle,
    _theme: &Theme,
    status: ButtonStatus,
) -> button::Style {
    use button::Status::*;

    let mut s = button::Style::default();
    s.background = Some(iced::Background::Color(style.bar));
    s.border = iced::Border::default().width(0.0);
    s.text_color = style.icon;

    if matches!(status, Hovered | Pressed) {
        s.background = Some(iced::Background::Color(style.button_hover));
    }

    s
}

/// Returns the button style for the close button: bar color by default, `close_hover` when hovered/pressed.
pub fn close_button_style(
    style: &TitlebarStyle,
    _theme: &Theme,
    status: ButtonStatus,
) -> button::Style {
    use button::Status::*;

    let mut s = button::Style::default();
    s.background = Some(iced::Background::Color(style.bar));
    s.border = iced::Border::default().width(0.0);
    s.text_color = style.icon;

    if matches!(status, Hovered | Pressed) {
        s.background = Some(iced::Background::Color(style.close_hover));
    }

    s
}
