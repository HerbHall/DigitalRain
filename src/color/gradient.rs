//! Color gradient interpolation for smooth trail fading.
//!
//! Each rain column has a "trail" of characters that fade from bright (head)
//! to dark (tail). We interpolate between colors based on position in the trail.

use crossterm::style::Color;

/// Linearly interpolate between two RGB colors.
///
/// `t` ranges from 0.0 (returns `from`) to 1.0 (returns `to`).
/// Values outside 0..1 are clamped.
pub fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    // Extract RGB components, defaulting to black for non-RGB colors
    let (r1, g1, b1) = color_to_rgb(from);
    let (r2, g2, b2) = color_to_rgb(to);

    Color::Rgb {
        r: lerp_u8(r1, r2, t),
        g: lerp_u8(g1, g2, t),
        b: lerp_u8(b1, b2, t),
    }
}

/// Compute a trail color given position within the trail.
///
/// `position` is 0.0 at the head (brightest) and 1.0 at the tail (dimmest).
/// The gradient goes: head -> body_bright -> body_mid -> tail
pub fn trail_color(
    head: Color,
    body_bright: Color,
    body_mid: Color,
    tail: Color,
    position: f32,
) -> Color {
    let position = position.clamp(0.0, 1.0);

    if position < 0.15 {
        // Head zone: white-hot to bright
        lerp_color(head, body_bright, position / 0.15)
    } else if position < 0.5 {
        // Upper body: bright to mid
        lerp_color(body_bright, body_mid, (position - 0.15) / 0.35)
    } else {
        // Lower body to tail: mid to dark
        lerp_color(body_mid, tail, (position - 0.5) / 0.5)
    }
}

/// Extract RGB components from a Color, defaulting to black.
fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb { r, g, b } => (r, g, b),
        _ => (0, 0, 0),
    }
}

/// Linearly interpolate between two u8 values.
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    let result = (a as f32) * (1.0 - t) + (b as f32) * t;
    result.round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb { r, g, b }
    }

    fn unwrap_rgb(c: Color) -> (u8, u8, u8) {
        match c {
            Color::Rgb { r, g, b } => (r, g, b),
            _ => panic!("expected Rgb color"),
        }
    }

    #[test]
    fn lerp_color_at_zero_returns_from() {
        let result = lerp_color(rgb(255, 0, 0), rgb(0, 255, 0), 0.0);
        assert_eq!(unwrap_rgb(result), (255, 0, 0));
    }

    #[test]
    fn lerp_color_at_one_returns_to() {
        let result = lerp_color(rgb(255, 0, 0), rgb(0, 255, 0), 1.0);
        assert_eq!(unwrap_rgb(result), (0, 255, 0));
    }

    #[test]
    fn lerp_color_at_half_returns_midpoint() {
        let result = lerp_color(rgb(0, 0, 0), rgb(200, 100, 50), 0.5);
        assert_eq!(unwrap_rgb(result), (100, 50, 25));
    }

    #[test]
    fn lerp_color_clamps_below_zero() {
        let result = lerp_color(rgb(100, 100, 100), rgb(200, 200, 200), -5.0);
        assert_eq!(unwrap_rgb(result), (100, 100, 100));
    }

    #[test]
    fn lerp_color_clamps_above_one() {
        let result = lerp_color(rgb(100, 100, 100), rgb(200, 200, 200), 10.0);
        assert_eq!(unwrap_rgb(result), (200, 200, 200));
    }

    #[test]
    fn lerp_color_non_rgb_defaults_to_black() {
        let result = lerp_color(Color::Reset, rgb(100, 100, 100), 0.5);
        assert_eq!(unwrap_rgb(result), (50, 50, 50));
    }

    #[test]
    fn trail_color_at_head_is_close_to_head_color() {
        let head = rgb(220, 255, 220);
        let bright = rgb(0, 230, 50);
        let mid = rgb(0, 150, 30);
        let tail = rgb(0, 60, 15);
        let result = trail_color(head, bright, mid, tail, 0.0);
        assert_eq!(unwrap_rgb(result), (220, 255, 220));
    }

    #[test]
    fn trail_color_at_tail_is_close_to_tail_color() {
        let head = rgb(220, 255, 220);
        let bright = rgb(0, 230, 50);
        let mid = rgb(0, 150, 30);
        let tail = rgb(0, 60, 15);
        let result = trail_color(head, bright, mid, tail, 1.0);
        assert_eq!(unwrap_rgb(result), (0, 60, 15));
    }

    #[test]
    fn trail_color_monotonically_decreases_green() {
        let head = rgb(220, 255, 220);
        let bright = rgb(0, 230, 50);
        let mid = rgb(0, 150, 30);
        let tail = rgb(0, 60, 15);

        let mut prev_g = 255u8;
        for i in 0..=10 {
            let pos = i as f32 / 10.0;
            let (_, g, _) = unwrap_rgb(trail_color(head, bright, mid, tail, pos));
            assert!(
                g <= prev_g,
                "green should decrease along trail: {} > {} at pos {}",
                g,
                prev_g,
                pos
            );
            prev_g = g;
        }
    }
}
