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
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "yellow".to_string(),
            popup_border: "yellow".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Solarized Dark theme
    pub fn solarized_dark() -> Self {
        Self {
            selection_bg: "blue".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "cyan".to_string(),
            border_unfocused: "darkgray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "cyan".to_string(),
            popup_border: "cyan".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Solarized Light theme
    pub fn solarized_light() -> Self {
        Self {
            selection_bg: "cyan".to_string(),
            selection_fg: "black".to_string(),
            border_focused: "blue".to_string(),
            border_unfocused: "gray".to_string(),
            text_normal: "black".to_string(),
            text_played: "darkgray".to_string(),
            text_unplayed: "black".to_string(),
            status_bar_bg: "white".to_string(),
            status_bar_fg: "blue".to_string(),
            popup_border: "blue".to_string(),
            popup_bg: "white".to_string(),
            popup_fg: "black".to_string(),
            episode_title: "black".to_string(),
            episode_published: "darkgray".to_string(),
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        Self {
            selection_bg: "magenta".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "lightmagenta".to_string(),
            border_unfocused: "darkgray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "lightmagenta".to_string(),
            popup_border: "lightmagenta".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Nord theme
    pub fn nord() -> Self {
        Self {
            selection_bg: "blue".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "lightcyan".to_string(),
            border_unfocused: "darkgray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "lightcyan".to_string(),
            popup_border: "lightcyan".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Monokai theme
    pub fn monokai() -> Self {
        Self {
            selection_bg: "magenta".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "lightyellow".to_string(),
            border_unfocused: "darkgray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "lightyellow".to_string(),
            popup_border: "lightyellow".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Tokyo Night theme
    pub fn tokyo_night() -> Self {
        Self {
            selection_bg: "blue".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "lightmagenta".to_string(),
            border_unfocused: "darkgray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "lightmagenta".to_string(),
            popup_border: "lightmagenta".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Catppuccin Mocha theme
    pub fn catppuccin() -> Self {
        Self {
            selection_bg: "lightmagenta".to_string(),
            selection_fg: "white".to_string(),
            border_focused: "lightblue".to_string(),
            border_unfocused: "darkgray".to_string(),
            text_normal: "white".to_string(),
            text_played: "gray".to_string(),
            text_unplayed: "white".to_string(),
            status_bar_bg: "black".to_string(),
            status_bar_fg: "lightblue".to_string(),
            popup_border: "lightblue".to_string(),
            popup_bg: "black".to_string(),
            popup_fg: "white".to_string(),
            episode_title: "white".to_string(),
            episode_published: "gray".to_string(),
        }
    }

    /// Load theme by name
    pub fn by_name(name: &str) -> Self {
        match name {
            "dark" => Self::dark(),
            "gruvbox" => Self::gruvbox(),
            "solarized" | "solarized-dark" => Self::solarized_dark(),
            "solarized-light" => Self::solarized_light(),
            "dracula" => Self::dracula(),
            "nord" => Self::nord(),
            "monokai" => Self::monokai(),
            "tokyo-night" | "tokyonight" => Self::tokyo_night(),
            "catppuccin" => Self::catppuccin(),
            _ => Self::default(),
        }
    }

    /// Parse color string to ratatui Color
    /// Supports:
    /// - Named colors: "black", "red", "green", etc.
    /// - Hex colors: "#ff0000", "#f00", "ff0000", "f00"
    /// - RGB: "rgb(255, 0, 0)"
    /// - Terminal palette indices: "0" through "255"
    /// - Reset to terminal default: "reset"
    pub fn parse_color(&self, color_str: &str) -> Color {
        let s = color_str.trim().to_lowercase();

        // Check for hex color (#ff0000 or #f00)
        if s.starts_with('#') {
            return Self::parse_hex(&s[1..]).unwrap_or(Color::White);
        } else if s.len() == 6 || s.len() == 3 {
            // Also try parsing without # prefix
            if let Some(color) = Self::parse_hex(&s) {
                return color;
            }
        }

        // Check for RGB format: rgb(r, g, b)
        if s.starts_with("rgb(") && s.ends_with(')') {
            let rgb_str = &s[4..s.len() - 1];
            let parts: Vec<&str> = rgb_str.split(',').map(|s| s.trim()).collect();
            if parts.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].parse::<u8>(),
                    parts[1].parse::<u8>(),
                    parts[2].parse::<u8>(),
                ) {
                    return Color::Rgb(r, g, b);
                }
            }
        }

        // Check for terminal palette index (0-255)
        if let Ok(index) = s.parse::<u8>() {
            return Color::Indexed(index);
        }

        // Named colors and special values
        match s.as_str() {
            "reset" => Color::Reset,
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

    /// Parse hex color string (without #)
    /// Supports both 6-digit (#rrggbb) and 3-digit (#rgb) formats
    fn parse_hex(hex: &str) -> Option<Color> {
        match hex.len() {
            6 => {
                // Parse #rrggbb
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color::Rgb(r, g, b))
            }
            3 => {
                // Parse #rgb (expand to #rrggbb)
                let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                Some(Color::Rgb(r * 17, g * 17, b * 17)) // 0xF -> 0xFF
            }
            _ => None,
        }
    }

    /// Apply an accent color override to accent-related fields
    /// This allows customizing built-in themes with a single color change
    pub fn apply_accent(&mut self, accent: &str) {
        let accent_str = accent.to_string();
        self.selection_bg = accent_str.clone();
        self.border_focused = accent_str.clone();
        self.status_bar_fg = accent_str.clone();
        self.popup_border = accent_str;
        // Keep text_unplayed and episode_title as defined by the theme
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
