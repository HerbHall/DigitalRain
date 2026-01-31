//! Named color palettes for different visual themes.
//!
//! Two-tier system:
//! 1. Hand-tuned "featured" palettes (best quality, manually crafted gradients)
//! 2. Auto-generated palettes for all 148 CSS Level 4 named colors
//!
//! Hand-tuned names always take priority over CSS auto-generation.

use crossterm::style::Color;

use super::css_colors;
use super::hsl;

/// Hand-tuned palette names, in display order.
/// These always win over CSS auto-generated palettes.
const HAND_TUNED_NAMES: &[&str] = &[
    "classic",
    "gold",
    "cyan",
    "red",
    "silver",
    "purple",
    "fire",
    "ocean",
    "synthwave",
];

/// Returns the list of hand-tuned (featured) palette names.
pub fn hand_tuned_names() -> &'static [&'static str] {
    HAND_TUNED_NAMES
}

/// Returns the full list of available palette names.
/// Hand-tuned palettes first, then CSS colors (deduped).
pub fn palette_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = HAND_TUNED_NAMES.to_vec();
    for css_name in css_colors::css_color_names() {
        if !names.contains(&css_name) {
            names.push(css_name);
        }
    }
    names
}

/// Look up a palette by name. Returns classic if the name is unknown.
///
/// Priority: hand-tuned match -> "monochrome" alias -> CSS auto-gen -> fallback.
pub fn palette_by_name(name: &str) -> Palette {
    let lower = name.to_ascii_lowercase();

    // Hand-tuned palettes
    match lower.as_str() {
        "classic" => return Palette::classic(),
        "gold" => return Palette::gold(),
        "cyan" => return Palette::cyan(),
        "red" => return Palette::red(),
        "silver" | "monochrome" => return Palette::silver(),
        "purple" => return Palette::purple(),
        "fire" => return Palette::fire(),
        "ocean" => return Palette::ocean(),
        "synthwave" => return Palette::synthwave(),
        _ => {}
    }

    // CSS auto-generated palette
    if let Some(css) = css_colors::css_color_by_name(&lower) {
        return generate_from_rgb(css.r, css.g, css.b);
    }

    eprintln!("Unknown palette '{}', using classic", name);
    Palette::classic()
}

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

    /// Silver palette -- white/grey on black (formerly "monochrome").
    pub fn silver() -> Self {
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

    /// Fire palette -- red/orange/yellow heat gradient.
    pub fn fire() -> Self {
        Self {
            head: Color::Rgb {
                r: 255,
                g: 255,
                b: 200,
            },
            body_bright: Color::Rgb {
                r: 255,
                g: 120,
                b: 0,
            },
            body_mid: Color::Rgb {
                r: 200,
                g: 40,
                b: 0,
            },
            tail: Color::Rgb { r: 80, g: 10, b: 0 },
            highlight: Color::Rgb {
                r: 255,
                g: 255,
                b: 100,
            },
            background: Color::Reset,
        }
    }

    /// Ocean palette -- deep blue/teal aquatic feel.
    pub fn ocean() -> Self {
        Self {
            head: Color::Rgb {
                r: 200,
                g: 240,
                b: 255,
            },
            body_bright: Color::Rgb {
                r: 0,
                g: 120,
                b: 220,
            },
            body_mid: Color::Rgb {
                r: 0,
                g: 60,
                b: 140,
            },
            tail: Color::Rgb { r: 0, g: 20, b: 60 },
            highlight: Color::Rgb {
                r: 100,
                g: 255,
                b: 220,
            },
            background: Color::Reset,
        }
    }

    /// Synthwave palette -- pink/purple/cyan retro neon.
    pub fn synthwave() -> Self {
        Self {
            head: Color::Rgb {
                r: 255,
                g: 220,
                b: 255,
            },
            body_bright: Color::Rgb {
                r: 255,
                g: 50,
                b: 150,
            },
            body_mid: Color::Rgb {
                r: 160,
                g: 20,
                b: 100,
            },
            tail: Color::Rgb { r: 60, g: 5, b: 40 },
            highlight: Color::Rgb {
                r: 0,
                g: 255,
                b: 255,
            },
            background: Color::Reset,
        }
    }
}

/// Auto-generate a palette from an RGB base color using HSL math.
///
/// For chromatic colors: derives head, body, tail, and highlight from the hue.
/// For achromatic colors (greys): uses a neutral grey gradient.
fn generate_from_rgb(r: u8, g: u8, b: u8) -> Palette {
    let base = hsl::rgb_to_hsl(r, g, b);

    // Achromatic detection (very low saturation)
    if base.s < 0.05 {
        return generate_achromatic(base.l);
    }

    let h = base.h;

    // Head: near-white with hue tint
    let head_hsl = hsl::Hsl {
        h,
        s: 0.15,
        l: 0.92,
    };
    let (hr, hg, hb) = hsl::hsl_to_rgb(&head_hsl);

    // Body bright: vivid signature color
    let bright_l = base.l.clamp(0.40, 0.60);
    let bright_s = base.s.max(0.6);
    let bright_hsl = hsl::Hsl {
        h,
        s: bright_s,
        l: bright_l,
    };
    let (br, bg, bb) = hsl::hsl_to_rgb(&bright_hsl);

    // Body mid: darker version
    let mid_hsl = hsl::Hsl {
        h,
        s: bright_s,
        l: bright_l * 0.55,
    };
    let (mr, mg, mb) = hsl::hsl_to_rgb(&mid_hsl);

    // Tail: very dark
    let tail_hsl = hsl::Hsl {
        h,
        s: bright_s * 0.8,
        l: bright_l * 0.2,
    };
    let (tr, tg, tb) = hsl::hsl_to_rgb(&tail_hsl);

    // Highlight: complementary hue (180 degrees opposite)
    let comp_h = (h + 180.0) % 360.0;
    let highlight_hsl = hsl::Hsl {
        h: comp_h,
        s: 0.8,
        l: 0.65,
    };
    let (hlr, hlg, hlb) = hsl::hsl_to_rgb(&highlight_hsl);

    Palette {
        head: Color::Rgb {
            r: hr,
            g: hg,
            b: hb,
        },
        body_bright: Color::Rgb {
            r: br,
            g: bg,
            b: bb,
        },
        body_mid: Color::Rgb {
            r: mr,
            g: mg,
            b: mb,
        },
        tail: Color::Rgb {
            r: tr,
            g: tg,
            b: tb,
        },
        highlight: Color::Rgb {
            r: hlr,
            g: hlg,
            b: hlb,
        },
        background: Color::Reset,
    }
}

/// Generate a grey-scale palette for achromatic CSS colors.
fn generate_achromatic(base_l: f64) -> Palette {
    // Clamp base lightness to a usable range for gradient visibility
    let l = base_l.clamp(0.15, 0.85);

    let head_hsl = hsl::Hsl {
        h: 0.0,
        s: 0.0,
        l: (l + 0.35).min(0.95),
    };
    let bright_hsl = hsl::Hsl { h: 0.0, s: 0.0, l };
    let mid_hsl = hsl::Hsl {
        h: 0.0,
        s: 0.0,
        l: l * 0.55,
    };
    let tail_hsl = hsl::Hsl {
        h: 0.0,
        s: 0.0,
        l: l * 0.2,
    };

    let (hr, hg, hb) = hsl::hsl_to_rgb(&head_hsl);
    let (br, bg, bb) = hsl::hsl_to_rgb(&bright_hsl);
    let (mr, mg, mb) = hsl::hsl_to_rgb(&mid_hsl);
    let (tr, tg, tb) = hsl::hsl_to_rgb(&tail_hsl);

    Palette {
        head: Color::Rgb {
            r: hr,
            g: hg,
            b: hb,
        },
        body_bright: Color::Rgb {
            r: br,
            g: bg,
            b: bb,
        },
        body_mid: Color::Rgb {
            r: mr,
            g: mg,
            b: mb,
        },
        tail: Color::Rgb {
            r: tr,
            g: tg,
            b: tb,
        },
        highlight: Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
        background: Color::Reset,
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
    fn hand_tuned_palettes_listed_first() {
        let names = palette_names();
        for (i, ht) in HAND_TUNED_NAMES.iter().enumerate() {
            assert_eq!(names[i], *ht);
        }
    }

    #[test]
    fn all_named_palettes_resolve() {
        for name in palette_names() {
            let p = palette_by_name(name);
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
        assert!(matches!(
            (unknown.head, classic.head),
            (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 })
            if r1 == r2 && g1 == g2 && b1 == b2
        ));
    }

    #[test]
    fn monochrome_alias_returns_silver() {
        let mono = palette_by_name("monochrome");
        let silver = palette_by_name("silver");
        assert!(matches!(
            (mono.head, silver.head),
            (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 })
            if r1 == r2 && g1 == g2 && b1 == b2
        ));
    }

    #[test]
    fn hand_tuned_takes_priority_over_css() {
        // "gold" exists both as hand-tuned and CSS. The hand-tuned should win.
        let gold = palette_by_name("gold");
        let hand_tuned = Palette::gold();
        assert!(matches!(
            (gold.body_bright, hand_tuned.body_bright),
            (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 })
            if r1 == r2 && g1 == g2 && b1 == b2
        ));
    }

    #[test]
    fn css_auto_generated_palette_works() {
        // "coral" is not hand-tuned, should auto-generate
        let coral = palette_by_name("coral");
        assert!(matches!(coral.head, Color::Rgb { .. }));
        assert!(matches!(coral.body_bright, Color::Rgb { .. }));
        assert!(matches!(coral.highlight, Color::Rgb { .. }));
    }

    #[test]
    fn achromatic_css_color_works() {
        // "gray" is achromatic (128, 128, 128)
        let gray = palette_by_name("gray");
        assert!(matches!(gray.head, Color::Rgb { .. }));
        assert!(matches!(gray.body_bright, Color::Rgb { .. }));
    }

    #[test]
    fn very_dark_css_color_works() {
        let dark = palette_by_name("darkred");
        assert!(matches!(dark.head, Color::Rgb { .. }));
    }

    #[test]
    fn very_light_css_color_works() {
        let light = palette_by_name("snow");
        assert!(matches!(light.head, Color::Rgb { .. }));
    }

    #[test]
    fn no_duplicate_names() {
        let names = palette_names();
        let mut seen = std::collections::HashSet::new();
        for name in &names {
            assert!(seen.insert(name), "Duplicate palette name: {}", name);
        }
    }

    #[test]
    fn fire_ocean_synthwave_exist() {
        for name in &["fire", "ocean", "synthwave"] {
            let p = palette_by_name(name);
            assert!(
                matches!(p.head, Color::Rgb { .. }),
                "palette '{}' should resolve",
                name
            );
        }
    }

    #[test]
    fn total_palette_count_is_reasonable() {
        let names = palette_names();
        // 9 hand-tuned + ~139 CSS (minus overlaps: cyan, red, gold, purple, silver = 5)
        // = 9 + 143 = 152 (some CSS names like grey/gray duplicates)
        assert!(
            names.len() > 140,
            "Expected 140+ palettes, got {}",
            names.len()
        );
    }
}
