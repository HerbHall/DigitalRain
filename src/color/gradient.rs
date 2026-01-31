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
