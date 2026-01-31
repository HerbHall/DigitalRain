//! Named color palettes for different visual themes.

use crossterm::style::Color;

/// A color palette defines the colors used for a rain effect.
pub struct Palette {
    /// The bright leading character color (head of the rain column)
    pub head: Color,
    /// The main body color at full brightness
    pub body_bright: Color,
    /// The body color at medium brightness
    pub body_mid: Color,
    /// The tail color (dimmest, about to fade out)
    pub tail: Color,
    /// Special highlight color (e.g., gold characters in Matrix)
    pub highlight: Color,
    /// Background color (usually black/reset)
    pub background: Color,
}

impl Palette {
    /// Classic Matrix green phosphor palette.
    /// White-hot lead character fading through vivid green to near-black.
    pub fn classic() -> Self {
        Self {
            head: Color::Rgb {
                r: 220,
                g: 255,
                b: 220,
            }, // near-white with green tint
            body_bright: Color::Rgb {
                r: 0,
                g: 230,
                b: 50,
            }, // vivid green
            body_mid: Color::Rgb {
                r: 0,
                g: 150,
                b: 30,
            }, // medium green
            tail: Color::Rgb { r: 0, g: 60, b: 15 }, // dark green
            highlight: Color::Rgb {
                r: 255,
                g: 215,
                b: 0,
            }, // gold
            background: Color::Reset,
        }
    }
}
