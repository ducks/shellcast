use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    // Selection colors
    pub selection_bg: String,
    pub selection_fg: String,

    // Border colors
    pub border_focused: String,
    pub border_unfocused: String,

    // Text colors
    pub text_normal: String,
    pub text_played: String,
    pub text_unplayed: String,

    // Status bar
    pub status_bar_bg: String,
    pub status_bar_fg: String,

    // Popups
    pub popup_border: String,
    pub popup_bg: String,
    pub popup_fg: String,

    // Episode info
    pub episode_title: String,
    pub episode_published: String,
}

impl Theme {
    /// Default theme (current shellcast colors)
    pub fn default() -> Self {
        Self {
            selection_bg: "blue".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "cyan".to_string(),
            border_unfocused: "gray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "white".to_string(),
            popup_border: "cyan".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Dark theme (higher contrast)
    pub fn dark() -> Self {
        Self {
            selection_bg: "magenta".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "yellow".to_string(),
            border_unfocused: "darkgray".to_string(),
            text_normal: "white".to_string(),
            text_played: "darkgray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "yellow".to_string(),
            popup_border: "yellow".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "yellow".to_string(),
            episode_published: "darkgray".to_string(),
        }
    }

    /// Gruvbox theme
    pub fn gruvbox() -> Self {
        Self {
            selection_bg: "yellow".to_string(),
            selection_fg: "black".to_string(),
            border_focused: "yellow".to_string(),
            border_unfocused: "gray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "yellow".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "yellow".to_string(),
            popup_border: "yellow".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "yellow".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Load theme by name
    pub fn by_name(name: &str) -> Self {
        match name {
            "dark" => Self::dark(),
            "gruvbox" => Self::gruvbox(),
            _ => Self::default(),
        }
    }

    /// Parse color string to ratatui Color
    pub fn parse_color(&self, color_str: &str) -> Color {
        match color_str.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" | "grey" => Color::Gray,
            "darkgray" | "darkgrey" => Color::DarkGray,
            "lightred" => Color::LightRed,
            "lightgreen" => Color::LightGreen,
            "lightyellow" => Color::LightYellow,
            "lightblue" => Color::LightBlue,
            "lightmagenta" => Color::LightMagenta,
            "lightcyan" => Color::LightCyan,
            "white" => Color::White,
            _ => Color::White, // Default fallback
        }
    }

    // Convenience methods for getting ratatui Colors
    pub fn selection_bg_color(&self) -> Color {
        self.parse_color(&self.selection_bg)
    }

    pub fn selection_fg_color(&self) -> Color {
        self.parse_color(&self.selection_fg)
    }

    pub fn border_focused_color(&self) -> Color {
        self.parse_color(&self.border_focused)
    }

    pub fn border_unfocused_color(&self) -> Color {
        self.parse_color(&self.border_unfocused)
    }

    pub fn text_normal_color(&self) -> Color {
        self.parse_color(&self.text_normal)
    }

    pub fn text_played_color(&self) -> Color {
        self.parse_color(&self.text_played)
    }

    pub fn text_unplayed_color(&self) -> Color {
        self.parse_color(&self.text_unplayed)
    }

    pub fn status_bar_bg_color(&self) -> Color {
        self.parse_color(&self.status_bar_bg)
    }

    pub fn status_bar_fg_color(&self) -> Color {
        self.parse_color(&self.status_bar_fg)
    }

    pub fn popup_border_color(&self) -> Color {
        self.parse_color(&self.popup_border)
    }

    pub fn popup_bg_color(&self) -> Color {
        self.parse_color(&self.popup_bg)
    }

    pub fn popup_fg_color(&self) -> Color {
        self.parse_color(&self.popup_fg)
    }

    pub fn episode_title_color(&self) -> Color {
        self.parse_color(&self.episode_title)
    }

    pub fn episode_published_color(&self) -> Color {
        self.parse_color(&self.episode_published)
    }
}
