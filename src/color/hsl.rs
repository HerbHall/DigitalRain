//! HSL (Hue, Saturation, Lightness) color conversion utilities.
//!
//! Used by the auto-generated palette system to derive gradient palettes
//! from CSS named colors.

/// An HSL color with components in standard ranges:
/// - h: 0.0..360.0 (degrees)
/// - s: 0.0..1.0
/// - l: 0.0..1.0
#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

/// Convert RGB (0-255 each) to HSL.
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> Hsl {
    let r_norm = r as f64 / 255.0;
    let g_norm = g as f64 / 255.0;
    let b_norm = b as f64 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta < 1e-10 {
        // Achromatic (grey)
        return Hsl { h: 0.0, s: 0.0, l };
    }

    let s = if l <= 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if (max - r_norm).abs() < 1e-10 {
        let mut hue = 60.0 * ((g_norm - b_norm) / delta);
        if hue < 0.0 {
            hue += 360.0;
        }
        hue
    } else if (max - g_norm).abs() < 1e-10 {
        60.0 * ((b_norm - r_norm) / delta) + 120.0
    } else {
        60.0 * ((r_norm - g_norm) / delta) + 240.0
    };

    Hsl { h, s, l }
}

/// Convert HSL back to RGB (0-255 each).
pub fn hsl_to_rgb(hsl: &Hsl) -> (u8, u8, u8) {
    let Hsl { h, s, l } = *hsl;

    if s < 1e-10 {
        // Achromatic
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h_norm = h / 360.0;

    let r = hue_to_rgb(p, q, h_norm + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h_norm);
    let b = hue_to_rgb(p, q, h_norm - 1.0 / 3.0);

    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

/// Helper: convert a single hue channel to RGB component.
fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip: RGB -> HSL -> RGB should preserve values.
    #[test]
    fn round_trip_primary_colors() {
        let test_cases: &[(u8, u8, u8)] = &[
            (255, 0, 0),     // red
            (0, 255, 0),     // green
            (0, 0, 255),     // blue
            (255, 255, 0),   // yellow
            (0, 255, 255),   // cyan
            (255, 0, 255),   // magenta
            (255, 255, 255), // white
            (0, 0, 0),       // black
            (128, 128, 128), // grey
        ];
        for &(r, g, b) in test_cases {
            let hsl = rgb_to_hsl(r, g, b);
            let (r2, g2, b2) = hsl_to_rgb(&hsl);
            assert!(
                (r as i16 - r2 as i16).abs() <= 1
                    && (g as i16 - g2 as i16).abs() <= 1
                    && (b as i16 - b2 as i16).abs() <= 1,
                "Round-trip failed for ({r}, {g}, {b}): got ({r2}, {g2}, {b2})"
            );
        }
    }

    /// Known HSL values for primary colors.
    #[test]
    fn known_hsl_values() {
        // Pure red: H=0, S=1.0, L=0.5
        let red = rgb_to_hsl(255, 0, 0);
        assert!((red.h - 0.0).abs() < 1.0);
        assert!((red.s - 1.0).abs() < 0.01);
        assert!((red.l - 0.5).abs() < 0.01);

        // Pure green: H=120, S=1.0, L=0.5
        let green = rgb_to_hsl(0, 255, 0);
        assert!((green.h - 120.0).abs() < 1.0);
        assert!((green.s - 1.0).abs() < 0.01);

        // Pure blue: H=240, S=1.0, L=0.5
        let blue = rgb_to_hsl(0, 0, 255);
        assert!((blue.h - 240.0).abs() < 1.0);
        assert!((blue.s - 1.0).abs() < 0.01);
    }

    /// Achromatic colors should have S=0.
    #[test]
    fn achromatic_colors() {
        for v in [0, 64, 128, 192, 255] {
            let hsl = rgb_to_hsl(v, v, v);
            assert!(
                hsl.s < 0.01,
                "Grey ({v},{v},{v}) should be achromatic, got S={:.3}",
                hsl.s
            );
        }
    }

    /// White and black lightness values.
    #[test]
    fn white_and_black_lightness() {
        let white = rgb_to_hsl(255, 255, 255);
        assert!((white.l - 1.0).abs() < 0.01);

        let black = rgb_to_hsl(0, 0, 0);
        assert!(black.l < 0.01);
    }

    /// Round-trip with various mid-range colors.
    #[test]
    fn round_trip_mid_range_colors() {
        let test_cases: &[(u8, u8, u8)] = &[
            (100, 149, 237), // cornflowerblue
            (255, 127, 80),  // coral
            (220, 20, 60),   // crimson
            (75, 0, 130),    // indigo
            (250, 128, 114), // salmon
        ];
        for &(r, g, b) in test_cases {
            let hsl = rgb_to_hsl(r, g, b);
            let (r2, g2, b2) = hsl_to_rgb(&hsl);
            assert!(
                (r as i16 - r2 as i16).abs() <= 1
                    && (g as i16 - g2 as i16).abs() <= 1
                    && (b as i16 - b2 as i16).abs() <= 1,
                "Round-trip failed for ({r}, {g}, {b}): got ({r2}, {g2}, {b2})"
            );
        }
    }
}
