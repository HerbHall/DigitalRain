//! Fire effect: classic cellular automata fire simulation.
//!
//! Bottom row is seeded with random heat values. Each cell averages
//! its neighbors below with random cooling. Heat maps to a fire gradient
//! (black -> red -> orange -> yellow -> white).
//!
//! Inspired by the classic Doom fire effect algorithm.
//! Reference: https://fabiensanglard.net/doom_fire_psx/

use crossterm::style::Color;
use rand::Rng;

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::color::gradient::lerp_color;

/// Fire gradient stops: black -> dark red -> red -> orange -> yellow -> white
const FIRE_GRADIENT: [(f32, Color); 6] = [
    (0.0, Color::Rgb { r: 0, g: 0, b: 0 }),
    (0.2, Color::Rgb { r: 120, g: 0, b: 0 }),
    (
        0.4,
        Color::Rgb {
            r: 220,
            g: 30,
            b: 0,
        },
    ),
    (
        0.6,
        Color::Rgb {
            r: 255,
            g: 130,
            b: 0,
        },
    ),
    (
        0.8,
        Color::Rgb {
            r: 255,
            g: 220,
            b: 50,
        },
    ),
    (
        1.0,
        Color::Rgb {
            r: 255,
            g: 255,
            b: 200,
        },
    ),
];

/// Characters used for fire rendering, ordered by heat intensity.
const FIRE_CHARS: &[char] = &[' ', '.', ':', '^', '*', '#', '%', '@'];

/// Classic cellular automata fire simulation.
pub struct FireEffect {
    width: u16,
    height: u16,
    /// Heat value (0.0 - 1.0) per cell, stored row-major
    heat_map: Vec<f64>,
    speed_multiplier: f64,
    /// Cooling factor: higher = fire dies faster
    cooling_factor: f64,
}

impl FireEffect {
    pub fn with_config(width: u16, height: u16, config: &crate::config::Config) -> Self {
        let size = width as usize * height as usize;
        Self {
            width,
            height,
            heat_map: vec![0.0; size],
            speed_multiplier: config.speed_multiplier,
            cooling_factor: 0.04,
        }
    }

    /// Map a heat value (0.0-1.0) to a color using the fire gradient.
    fn heat_to_color(heat: f64) -> Color {
        let heat = heat.clamp(0.0, 1.0) as f32;

        // Find the two gradient stops to interpolate between
        for i in 0..FIRE_GRADIENT.len() - 1 {
            let (t0, c0) = FIRE_GRADIENT[i];
            let (t1, c1) = FIRE_GRADIENT[i + 1];
            if heat >= t0 && heat <= t1 {
                let t = (heat - t0) / (t1 - t0);
                return lerp_color(c0, c1, t);
            }
        }
        FIRE_GRADIENT.last().unwrap().1
    }

    /// Map a heat value to a character.
    fn heat_to_char(heat: f64) -> char {
        let idx = (heat.clamp(0.0, 1.0) * (FIRE_CHARS.len() - 1) as f64).round() as usize;
        FIRE_CHARS[idx.min(FIRE_CHARS.len() - 1)]
    }
}

impl Effect for FireEffect {
    fn name(&self) -> &str {
        "fire"
    }

    fn update(&mut self, delta_time: f64) {
        let mut rng = rand::rng();
        let w = self.width as usize;
        let h = self.height as usize;
        if w == 0 || h == 0 {
            return;
        }

        // Number of simulation steps per frame (speed-dependent)
        let steps = ((self.speed_multiplier * delta_time * 60.0).round() as usize).max(1);

        for _ in 0..steps {
            // Seed the bottom row with random heat
            for x in 0..w {
                let idx = (h - 1) * w + x;
                self.heat_map[idx] = rng.random_range(0.6..1.0);
            }

            // Propagate heat upward: each cell averages neighbors below with cooling
            // Process from top to bottom-1 (reading from below)
            for y in 0..h - 1 {
                for x in 0..w {
                    let idx = y * w + x;
                    // Sample from the row below with slight horizontal spread
                    let below = (y + 1) * w + x;
                    let left = if x > 0 { (y + 1) * w + (x - 1) } else { below };
                    let right = if x < w - 1 {
                        (y + 1) * w + (x + 1)
                    } else {
                        below
                    };

                    // Average of 3 neighbors below with random cooling
                    let avg =
                        (self.heat_map[below] + self.heat_map[left] + self.heat_map[right]) / 3.0;
                    let cooling = rng.random_range(0.0..self.cooling_factor);
                    self.heat_map[idx] = (avg - cooling).max(0.0);
                }
            }
        }
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        let w = self.width as usize;
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y as usize * w + x as usize;
                let heat = self.heat_map[idx];

                if heat < 0.01 {
                    continue; // Skip near-zero cells (leave as background)
                }

                let ch = Self::heat_to_char(heat);
                let fg = Self::heat_to_color(heat);
                // Background gets a dimmer version of the fire for glow
                let bg = Self::heat_to_color(heat * 0.3);
                buffer.set_cell(x, y, ch, fg, bg);
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.heat_map = vec![0.0; width as usize * height as usize];
    }

    fn set_speed(&mut self, multiplier: f64) {
        self.speed_multiplier = multiplier;
    }

    fn speed(&self) -> f64 {
        self.speed_multiplier
    }
}
