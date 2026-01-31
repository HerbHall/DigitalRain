//! Named color palettes for different visual themes.

use crossterm::style::Color;

/// Returns the list of available palette names.
pub fn palette_names() -> &'static [&'static str] {
    &["classic", "gold", "cyan", "red", "monochrome", "purple"]
}

/// Look up a palette by name. Returns classic if the name is unknown.
pub fn palette_by_name(name: &str) -> Palette {
    match name {
        "classic" => Palette::classic(),
        "gold" => Palette::gold(),
        "cyan" => Palette::cyan(),
        "red" => Palette::red(),
        "monochrome" => Palette::monochrome(),
        "purple" => Palette::purple(),
        _ => {
            eprintln!("Unknown palette '{}', using classic", name);
            Palette::classic()
        }
    }
}

/// A color palette defines the colors used for a rain effect.
#[derive(Clone)]
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
    pub fn classic() -> Self {
        Self {
            head: Color::Rgb {
                r: 220,
                g: 255,
                b: 220,
            },
            body_bright: Color::Rgb {
                r: 0,
                g: 230,
                b: 50,
            },
            body_mid: Color::Rgb {
                r: 0,
                g: 150,
                b: 30,
            },
            tail: Color::Rgb { r: 0, g: 60, b: 15 },
            highlight: Color::Rgb {
                r: 255,
                g: 215,
                b: 0,
            },
            background: Color::Reset,
        }
    }

    /// Gold/amber palette -- warm phosphor CRT feel.
    pub fn gold() -> Self {
        Self {
            head: Color::Rgb {
                r: 255,
                g: 255,
                b: 200,
            },
            body_bright: Color::Rgb {
                r: 255,
                g: 200,
                b: 50,
            },
            body_mid: Color::Rgb {
                r: 180,
                g: 130,
                b: 20,
            },
            tail: Color::Rgb { r: 80, g: 50, b: 5 },
            highlight: Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
            background: Color::Reset,
        }
    }

    /// Cyan/ice palette -- cold digital feel.
    pub fn cyan() -> Self {
        Self {
            head: Color::Rgb {
                r: 220,
                g: 255,
                b: 255,
            },
            body_bright: Color::Rgb {
                r: 0,
                g: 200,
                b: 230,
            },
            body_mid: Color::Rgb {
                r: 0,
                g: 120,
                b: 160,
            },
            tail: Color::Rgb { r: 0, g: 40, b: 60 },
            highlight: Color::Rgb {
                r: 180,
                g: 255,
                b: 255,
            },
            background: Color::Reset,
        }
    }

    /// Red/crimson palette -- danger/alert feel.
    pub fn red() -> Self {
        Self {
            head: Color::Rgb {
                r: 255,
                g: 220,
                b: 220,
            },
            body_bright: Color::Rgb {
                r: 230,
                g: 30,
                b: 30,
            },
            body_mid: Color::Rgb {
                r: 150,
                g: 15,
                b: 15,
            },
            tail: Color::Rgb { r: 60, g: 5, b: 5 },
            highlight: Color::Rgb {
                r: 255,
                g: 180,
                b: 50,
            },
            background: Color::Reset,
        }
    }

    /// Monochrome -- white/grey on black.
    pub fn monochrome() -> Self {
        Self {
            head: Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
            body_bright: Color::Rgb {
                r: 180,
                g: 180,
                b: 180,
            },
            body_mid: Color::Rgb {
                r: 100,
                g: 100,
                b: 100,
            },
            tail: Color::Rgb {
                r: 40,
                g: 40,
                b: 40,
            },
            highlight: Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
            background: Color::Reset,
        }
    }

    /// Purple/violet palette -- synthwave aesthetic.
    pub fn purple() -> Self {
        Self {
            head: Color::Rgb {
                r: 240,
                g: 220,
                b: 255,
            },
            body_bright: Color::Rgb {
                r: 180,
                g: 50,
                b: 230,
            },
            body_mid: Color::Rgb {
                r: 110,
                g: 20,
                b: 160,
            },
            tail: Color::Rgb { r: 40, g: 5, b: 60 },
            highlight: Color::Rgb {
                r: 255,
                g: 100,
                b: 200,
            },
            background: Color::Reset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_names_not_empty() {
        assert!(!palette_names().is_empty());
    }

    #[test]
    fn all_named_palettes_resolve() {
        for name in palette_names() {
            let p = palette_by_name(name);
            // Verify head is an RGB color (not Reset/default)
            assert!(
                matches!(p.head, Color::Rgb { .. }),
                "palette '{}' head should be RGB",
                name
            );
        }
    }

    #[test]
    fn unknown_palette_falls_back_to_classic() {
        let unknown = palette_by_name("nonexistent");
        let classic = Palette::classic();
        // Both should produce the same head color
        assert!(matches!(
            (unknown.head, classic.head),
            (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 })
            if r1 == r2 && g1 == g2 && b1 == b2
        ));
    }
}
