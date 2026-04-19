use crate::app::Theme;
use ratatui::style::Color;

pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub border_active: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
    pub search_match_bg: Color,
    pub search_match_fg: Color,
    pub footer_bg: Color,
    pub footer_fg: Color,
}

pub fn get_theme(theme: Theme) -> ThemeColors {
    match theme {
        Theme::Dark => ThemeColors {
            background: Color::Reset, // Inherit terminal bg
            foreground: Color::White,
            border: Color::DarkGray,
            border_active: Color::Yellow, // Making this more distinct
            highlight_bg: Color::DarkGray,
            highlight_fg: Color::White,
            search_match_bg: Color::Yellow,
            search_match_fg: Color::Black,
            footer_bg: Color::Rgb(30, 30, 30),
            footer_fg: Color::White,
        },
        Theme::Light => ThemeColors {
            background: Color::White,
            foreground: Color::Black,
            border: Color::Gray,
            border_active: Color::Magenta, // Making this more distinct
            highlight_bg: Color::Gray,
            highlight_fg: Color::Black,
            search_match_bg: Color::Yellow,
            search_match_fg: Color::Black,
            footer_bg: Color::Rgb(220, 220, 220),
            footer_fg: Color::Black,
        },
    }
}
