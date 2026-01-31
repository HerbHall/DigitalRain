//! CRT monitor simulation post-processing filter.
//!
//! Applies four sub-effects to the screen buffer after the rain effect renders
//! but before overlays, simulating a vintage CRT monitor look:
//!
//! 1. **Phosphor glow** -- bright cells bleed light to cardinal neighbors
//! 2. **Scanlines** -- alternate rows are dimmed
//! 3. **Screen flicker** -- global brightness oscillates via dual-sine wave
//! 4. **Noise** -- random cell corruption for analog feel
//!
//! Application order matters: glow reads original brightness before scanlines
//! modify it, and noise is applied last so corruption looks jarring on purpose.
//!
//! References:
//! - CRT effect techniques inspired by classic demoscene and retro shader posts
//! - Dual-sine flicker approach based on real CRT refresh characteristics

use crossterm::style::Color;
use rand::Rng;

use crate::buffer::ScreenBuffer;
use crate::color::gradient::{color_to_rgb, scale_color};

/// Characters used for noise corruption -- chosen to look like analog glitches.
const NOISE_CHARS: &[char] = &['#', '%', '&', '@', '!', '/', '\\', '|', '.', ':'];

/// CRT monitor simulation filter applied as a post-processing pass.
///
/// Holds all state needed for time-varying effects (flicker phase, frame count).
/// The `intensity` field (0.0-1.0) scales all sub-effects proportionally.
pub struct CrtFilter {
    enabled: bool,
    intensity: f64,
    width: u16,
    height: u16,
    /// Accumulated time in seconds for sine-wave flicker oscillation.
    flicker_phase: f64,
    /// Frame counter for noise RNG seeding.
    frame_count: u64,
}

impl CrtFilter {
    /// Create a new CRT filter with the given dimensions and intensity.
    pub fn new(width: u16, height: u16, enabled: bool, intensity: f64) -> Self {
        Self {
            enabled,
            intensity: intensity.clamp(0.0, 1.0),
            width,
            height,
            flicker_phase: 0.0,
            frame_count: 0,
        }
    }

    /// Toggle the CRT filter on/off. Returns the new enabled state.
    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        self.enabled
    }

    /// Whether the CRT filter is currently active.
    #[allow(dead_code)] // available for future use (e.g., status display)
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set the enabled state directly (used when randomizing config).
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Update dimensions after a terminal resize.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Apply all CRT sub-effects to the buffer in order.
    ///
    /// Called once per frame between effect.render() and overlay rendering.
    /// `delta_time` is seconds since last frame (for flicker oscillation).
    pub fn apply(&mut self, buffer: &mut ScreenBuffer, delta_time: f64) {
        if !self.enabled {
            return;
        }

        self.width = buffer.width();
        self.height = buffer.height();
        self.flicker_phase += delta_time;
        self.frame_count = self.frame_count.wrapping_add(1);

        // Order matters: glow reads original brightness, scanlines dim rows,
        // flicker scales everything, noise corrupts last.
        self.apply_glow(buffer);
        self.apply_scanlines(buffer);
        self.apply_flicker(buffer);
        self.apply_noise(buffer);
    }

    /// Phosphor glow: bright cells bleed dimmed color to cardinal neighbors' backgrounds.
    ///
    /// Builds a brightness snapshot first (so glow reads pre-modification values),
    /// then adds glow color to the 4 neighbors of each bright cell.
    fn apply_glow(&self, buffer: &mut ScreenBuffer) {
        let w = self.width as usize;
        let h = self.height as usize;
        if w == 0 || h == 0 {
            return;
        }

        let glow_strength = 0.07 * self.intensity;
        if glow_strength < 0.001 {
            return;
        }

        // Snapshot brightness and fg color for each cell (drops the borrow on buffer
        // so we can mutate it in the second pass).
        let snapshot: Vec<(u8, char, u8, u8, u8)> = buffer
            .cells()
            .iter()
            .map(|cell| {
                let (r, g, b) = color_to_rgb(cell.fg);
                (r.max(g).max(b), cell.ch, r, g, b)
            })
            .collect();

        // For each bright cell, add dimmed fg color to neighbors' bg.
        // High threshold so only the brightest head characters glow,
        // keeping the body/tail crisp with good contrast.
        let threshold: u8 = 170;
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let (bright, ch, fr, fg, fb) = snapshot[idx];
                if bright < threshold || ch == ' ' {
                    continue;
                }

                let glow_r = (fr as f64 * glow_strength) as u8;
                let glow_g = (fg as f64 * glow_strength) as u8;
                let glow_b = (fb as f64 * glow_strength) as u8;

                // Cardinal neighbors: up, down, left, right
                let directions: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                for (dx, dy) in directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0
                        && nx < w as isize
                        && ny >= 0
                        && ny < h as isize
                        && let Some(neighbor) = buffer.get_cell(nx as u16, ny as u16)
                    {
                        let (br, bg_val, bb) = color_to_rgb(neighbor.bg);
                        let new_bg = Color::Rgb {
                            r: br.saturating_add(glow_r),
                            g: bg_val.saturating_add(glow_g),
                            b: bb.saturating_add(glow_b),
                        };
                        buffer.set_cell(nx as u16, ny as u16, neighbor.ch, neighbor.fg, new_bg);
                    }
                }
            }
        }
    }

    /// Scanlines: dim every even row to simulate CRT horizontal scan gaps.
    ///
    /// The dimming factor interpolates between 1.0 (no effect) and 0.45 (heavy)
    /// based on intensity. Empty cells (spaces) are skipped.
    fn apply_scanlines(&self, buffer: &mut ScreenBuffer) {
        let dim_factor = 1.0 - (0.55 * self.intensity); // 1.0 at intensity=0, 0.45 at intensity=1

        for y in (0..self.height).step_by(2) {
            for x in 0..self.width {
                if let Some(cell) = buffer.get_cell(x, y) {
                    if cell.ch == ' ' {
                        continue;
                    }
                    let new_fg = scale_color(cell.fg, dim_factor);
                    let new_bg = scale_color(cell.bg, dim_factor);
                    buffer.set_cell(x, y, cell.ch, new_fg, new_bg);
                }
            }
        }
    }

    /// Screen flicker: oscillate global brightness via dual sine wave.
    ///
    /// Primary wave at 0.3 Hz (slow drift, 60% weight) and secondary at 1.7 Hz
    /// (subtle shimmer, 40% weight). The combined factor maps to a narrow range
    /// around 1.0 so it's noticeable but not nauseating.
    fn apply_flicker(&self, buffer: &mut ScreenBuffer) {
        let max_dip = 0.08 * self.intensity; // max brightness reduction
        if max_dip < 0.001 {
            return;
        }

        // Dual sine: primary slow drift + secondary shimmer
        let primary = (self.flicker_phase * 0.3 * std::f64::consts::TAU).sin(); // -1..1
        let secondary = (self.flicker_phase * 1.7 * std::f64::consts::TAU).sin();
        let combined = primary * 0.6 + secondary * 0.4; // -1..1

        // Map to brightness factor: [1.0 - max_dip, 1.0]
        let factor = 1.0 - max_dip * (combined + 1.0) * 0.5; // normalize to 0..1 range

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = buffer.get_cell(x, y) {
                    if cell.ch == ' ' {
                        continue;
                    }
                    let new_fg = scale_color(cell.fg, factor);
                    let new_bg = scale_color(cell.bg, factor);
                    buffer.set_cell(x, y, cell.ch, new_fg, new_bg);
                }
            }
        }
    }

    /// Noise: randomly corrupt a small fraction of cells for analog feel.
    ///
    /// Per-cell probability is 0.2% at full intensity. Corrupted cells get a
    /// random character and a brightness-shifted foreground color.
    fn apply_noise(&self, buffer: &mut ScreenBuffer) {
        let probability = 0.002 * self.intensity;
        if probability < 0.0001 {
            return;
        }

        let mut rng = rand::rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = buffer.get_cell(x, y) {
                    if cell.ch == ' ' {
                        continue;
                    }
                    if rng.random_range(0.0..1.0) < probability {
                        let noise_ch = NOISE_CHARS[rng.random_range(0..NOISE_CHARS.len())];
                        // Random brightness shift: 0.7x to 1.3x
                        let shift = rng.random_range(0.7..1.3);
                        let new_fg = scale_color(cell.fg, shift);
                        buffer.set_cell(x, y, noise_ch, new_fg, cell.bg);
                    }
                }
            }
        }
    }
}

// scale_color is now imported from crate::color::gradient

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::ScreenBuffer;

    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb { r, g, b }
    }

    fn unwrap_rgb(c: Color) -> (u8, u8, u8) {
        match c {
            Color::Rgb { r, g, b } => (r, g, b),
            _ => panic!("expected Rgb color, got {:?}", c),
        }
    }

    // --- scale_color tests ---

    #[test]
    fn scale_color_dims_correctly() {
        let result = scale_color(rgb(200, 100, 50), 0.5);
        assert_eq!(unwrap_rgb(result), (100, 50, 25));
    }

    #[test]
    fn scale_color_clamps_to_255() {
        let result = scale_color(rgb(200, 200, 200), 2.0);
        // All channels should be clamped to 255 (200 * 2.0 = 400 -> 255)
        assert_eq!(unwrap_rgb(result), (255, 255, 255));
    }

    #[test]
    fn scale_color_passes_through_non_rgb() {
        let result = scale_color(Color::Reset, 0.5);
        assert!(matches!(result, Color::Reset));

        let result2 = scale_color(Color::Green, 0.5);
        assert!(matches!(result2, Color::Green));
    }

    // --- CrtFilter toggle tests ---

    #[test]
    fn toggle_flips_enabled_state() {
        let mut filter = CrtFilter::new(80, 24, false, 0.7);
        assert!(!filter.is_enabled());
        assert!(filter.toggle()); // now enabled
        assert!(filter.is_enabled());
        assert!(!filter.toggle()); // now disabled
        assert!(!filter.is_enabled());
    }

    // --- disabled filter test ---

    #[test]
    fn disabled_filter_does_not_modify_buffer() {
        let mut buffer = ScreenBuffer::new(10, 5);
        buffer.set_cell(5, 2, 'A', rgb(0, 255, 0), Color::Reset);

        let mut filter = CrtFilter::new(10, 5, false, 1.0);
        filter.apply(&mut buffer, 0.033);

        let cell = buffer.get_cell(5, 2).unwrap();
        assert_eq!(cell.ch, 'A');
        assert_eq!(unwrap_rgb(cell.fg), (0, 255, 0));
    }

    // --- resize test ---

    #[test]
    fn resize_updates_dimensions() {
        let mut filter = CrtFilter::new(80, 24, true, 0.7);
        filter.resize(120, 40);

        // Should not panic when applying to a buffer of the new size
        let mut buffer = ScreenBuffer::new(120, 40);
        buffer.set_cell(60, 20, 'X', rgb(0, 200, 0), Color::Reset);
        filter.apply(&mut buffer, 0.033);
    }

    // --- scanlines tests ---

    #[test]
    fn scanlines_dim_even_rows() {
        let mut buffer = ScreenBuffer::new(5, 4);
        // Set cells on even row (0) and odd row (1) with same color
        let bright = rgb(0, 200, 0);
        buffer.set_cell(2, 0, 'A', bright, Color::Reset); // even row - will be dimmed
        buffer.set_cell(2, 1, 'B', bright, Color::Reset); // odd row - untouched by scanlines

        let filter = CrtFilter::new(5, 4, true, 1.0);
        filter.apply_scanlines(&mut buffer);

        let even_cell = buffer.get_cell(2, 0).unwrap();
        let odd_cell = buffer.get_cell(2, 1).unwrap();
        let (_, even_g, _) = unwrap_rgb(even_cell.fg);
        let (_, odd_g, _) = unwrap_rgb(odd_cell.fg);

        // Even row should be dimmer than odd row
        assert!(
            even_g < odd_g,
            "even row green {} should be less than odd row green {}",
            even_g,
            odd_g
        );
    }

    #[test]
    fn scanlines_skip_empty_cells() {
        let mut buffer = ScreenBuffer::new(5, 2);
        // Empty cell (space) on even row
        buffer.set_cell(2, 0, ' ', Color::Reset, Color::Reset);

        let filter = CrtFilter::new(5, 2, true, 1.0);
        filter.apply_scanlines(&mut buffer);

        let cell = buffer.get_cell(2, 0).unwrap();
        assert_eq!(cell.ch, ' ');
        assert!(matches!(cell.fg, Color::Reset));
    }

    // --- glow tests ---

    #[test]
    fn glow_brightens_neighbors_of_bright_cells() {
        let mut buffer = ScreenBuffer::new(5, 5);
        // Place a bright cell in the center
        buffer.set_cell(2, 2, 'X', rgb(0, 255, 0), Color::Reset);

        let filter = CrtFilter::new(5, 5, true, 1.0);
        filter.apply_glow(&mut buffer);

        // Check a cardinal neighbor's background got some glow
        let right = buffer.get_cell(3, 2).unwrap();
        let (r, g, _b) = color_to_rgb(right.bg);
        // Should have green glow added (0.07 * 255 = ~17)
        assert!(
            g > 0,
            "neighbor bg green should be > 0 from glow, got {}",
            g
        );
        assert_eq!(r, 0); // no red in source
    }

    #[test]
    fn glow_does_not_affect_dim_cells() {
        let mut buffer = ScreenBuffer::new(5, 5);
        // Place a dim cell (below threshold of 170)
        buffer.set_cell(2, 2, 'X', rgb(0, 50, 0), Color::Reset);

        let filter = CrtFilter::new(5, 5, true, 1.0);
        filter.apply_glow(&mut buffer);

        // Neighbor bg should still be default (Reset or black)
        let right = buffer.get_cell(3, 2).unwrap();
        // Background should be unchanged (Reset)
        assert!(
            matches!(right.bg, Color::Reset),
            "dim cell should not cause glow"
        );
    }

    // --- flicker test ---

    #[test]
    fn flicker_modulates_brightness() {
        let mut buffer = ScreenBuffer::new(5, 3);
        buffer.set_cell(2, 1, 'A', rgb(0, 200, 0), Color::Reset);

        // Use a phase that produces a non-unity flicker factor
        let mut filter = CrtFilter::new(5, 3, true, 1.0);
        filter.flicker_phase = 0.833; // 0.3Hz * TAU * 0.833 ~= pi -> sin = 0

        filter.apply_flicker(&mut buffer);

        let cell = buffer.get_cell(2, 1).unwrap();
        let (_, g, _) = unwrap_rgb(cell.fg);
        // Flicker should have modified the brightness (may be slightly dimmer)
        // With intensity=1.0, max_dip=0.08, so green should be within ~8% of 200
        assert!(
            g <= 200,
            "flicker should not brighten beyond original, got {}",
            g
        );
    }

    // --- buffer cells() accessor test ---

    #[test]
    fn buffer_cells_accessor_returns_correct_slice() {
        let mut buffer = ScreenBuffer::new(3, 2);
        buffer.set_cell(1, 0, 'A', rgb(255, 0, 0), Color::Reset);

        let cells = buffer.cells();
        assert_eq!(cells.len(), 6); // 3 * 2
        assert_eq!(cells[1].ch, 'A'); // index = 0*3 + 1 = 1
    }
}
