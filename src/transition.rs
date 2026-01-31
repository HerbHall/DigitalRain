//! Crossfade transition between effects.
//!
//! When switching effects, both old and new effects render simultaneously.
//! Per-cell color blending creates a smooth crossfade over a configurable
//! duration (default ~0.75 seconds).

use crate::buffer::ScreenBuffer;
use crate::color::gradient::lerp_color;
use crate::effects::Effect;

/// Manages a crossfade transition from an outgoing effect to the new current effect.
///
/// The outgoing effect renders into a scratch buffer, then blends with the
/// incoming effect (already rendered into the main buffer) on a per-cell basis.
pub struct Transition {
    /// The outgoing effect being faded out
    outgoing: Box<dyn Effect>,
    /// Scratch buffer for the outgoing effect's render
    scratch: ScreenBuffer,
    /// Total crossfade duration in seconds
    duration: f64,
    /// Time elapsed since the transition started
    elapsed: f64,
}

impl Transition {
    /// Create a new crossfade transition.
    ///
    /// Takes ownership of the outgoing effect. The incoming effect is the
    /// current `effect` variable in the main loop.
    pub fn new(outgoing: Box<dyn Effect>, width: u16, height: u16, duration: f64) -> Self {
        Self {
            outgoing,
            scratch: ScreenBuffer::new(width, height),
            duration: duration.max(0.05), // minimum duration to avoid division by zero
            elapsed: 0.0,
        }
    }

    /// Whether the transition has completed (outgoing fully faded out).
    pub fn is_complete(&self) -> bool {
        self.elapsed >= self.duration
    }

    /// Update the outgoing effect and advance the transition timer.
    pub fn update(&mut self, delta_time: f64) {
        self.outgoing.update(delta_time);
        self.elapsed += delta_time;
    }

    /// Blend the outgoing effect into the main buffer.
    ///
    /// Call this AFTER the incoming effect has already rendered into `buffer`.
    /// The outgoing effect renders into the scratch buffer, then each cell is
    /// blended based on the transition progress.
    pub fn render(&mut self, buffer: &mut ScreenBuffer) {
        // Render outgoing into scratch
        self.scratch.clear();
        self.outgoing.render(&mut self.scratch);

        // Blend factor: 0.0 = all outgoing, 1.0 = all incoming
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0) as f32;

        let w = buffer.width();
        let h = buffer.height();

        for y in 0..h {
            for x in 0..w {
                let out_cell = self.scratch.get_cell(x, y);
                let in_cell = buffer.get_cell(x, y);

                if let (Some(out), Some(inc)) = (out_cell, in_cell) {
                    // Skip if both are empty
                    if out.ch == ' ' && inc.ch == ' ' {
                        continue;
                    }

                    let blended_fg = lerp_color(out.fg, inc.fg, t);
                    let blended_bg = lerp_color(out.bg, inc.bg, t);
                    // Character switches at the midpoint
                    let ch = if t < 0.5 { out.ch } else { inc.ch };

                    buffer.set_cell(x, y, ch, blended_fg, blended_bg);
                }
            }
        }
    }

    /// Handle terminal resize for the outgoing effect and scratch buffer.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.outgoing.resize(width, height);
        self.scratch.resize(width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::Color;

    /// Minimal test effect that fills the buffer with a single character and color.
    struct TestEffect {
        ch: char,
        color: Color,
    }

    impl Effect for TestEffect {
        fn name(&self) -> &str {
            "test"
        }
        fn update(&mut self, _dt: f64) {}
        fn render(&self, buffer: &mut ScreenBuffer) {
            for y in 0..buffer.height() {
                for x in 0..buffer.width() {
                    buffer.set_cell(x, y, self.ch, self.color, Color::Reset);
                }
            }
        }
        fn resize(&mut self, _w: u16, _h: u16) {}
    }

    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb { r, g, b }
    }

    #[test]
    fn transition_starts_incomplete() {
        let effect = Box::new(TestEffect {
            ch: 'A',
            color: rgb(255, 0, 0),
        });
        let t = Transition::new(effect, 10, 5, 1.0);
        assert!(!t.is_complete());
    }

    #[test]
    fn transition_completes_after_duration() {
        let effect = Box::new(TestEffect {
            ch: 'A',
            color: rgb(255, 0, 0),
        });
        let mut t = Transition::new(effect, 10, 5, 0.5);
        t.update(0.6);
        assert!(t.is_complete());
    }

    #[test]
    fn blend_at_start_favors_outgoing() {
        let outgoing = Box::new(TestEffect {
            ch: 'A',
            color: rgb(200, 0, 0),
        });
        let mut t = Transition::new(outgoing, 5, 3, 1.0);
        // Don't advance time (t=0 -> all outgoing)

        let mut buffer = ScreenBuffer::new(5, 3);
        // Incoming: fill with green
        for y in 0..3 {
            for x in 0..5 {
                buffer.set_cell(x, y, 'B', rgb(0, 200, 0), Color::Reset);
            }
        }

        t.render(&mut buffer);

        // At t=0, should be mostly outgoing (red)
        let cell = buffer.get_cell(2, 1).unwrap();
        assert_eq!(cell.ch, 'A'); // outgoing character (t < 0.5)
        let (r, g, _) = crate::color::gradient::color_to_rgb(cell.fg);
        assert!(
            r > g,
            "at t=0, red (outgoing) should dominate: r={} g={}",
            r,
            g
        );
    }

    #[test]
    fn blend_at_end_favors_incoming() {
        let outgoing = Box::new(TestEffect {
            ch: 'A',
            color: rgb(200, 0, 0),
        });
        let mut t = Transition::new(outgoing, 5, 3, 1.0);
        t.update(0.95); // nearly complete

        let mut buffer = ScreenBuffer::new(5, 3);
        for y in 0..3 {
            for x in 0..5 {
                buffer.set_cell(x, y, 'B', rgb(0, 200, 0), Color::Reset);
            }
        }

        t.render(&mut buffer);

        let cell = buffer.get_cell(2, 1).unwrap();
        assert_eq!(cell.ch, 'B'); // incoming character (t > 0.5)
        let (r, g, _) = crate::color::gradient::color_to_rgb(cell.fg);
        assert!(
            g > r,
            "at t≈1, green (incoming) should dominate: r={} g={}",
            r,
            g
        );
    }
}
