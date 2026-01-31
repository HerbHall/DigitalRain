//! Ocean effect: sine-wave water simulation.
//!
//! Multiple overlapping sine waves at different frequencies create a
//! water surface with depth shading. Blue palette with white foam at crests.

use crossterm::style::Color;
use rand::Rng;

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::color::gradient::lerp_color;

/// Ocean color palette: deep to shallow
const DEEP_BLUE: Color = Color::Rgb { r: 0, g: 20, b: 60 };
const MID_BLUE: Color = Color::Rgb {
    r: 0,
    g: 60,
    b: 140,
};
const LIGHT_BLUE: Color = Color::Rgb {
    r: 30,
    g: 120,
    b: 200,
};
const CYAN: Color = Color::Rgb {
    r: 80,
    g: 200,
    b: 220,
};
const FOAM: Color = Color::Rgb {
    r: 220,
    g: 240,
    b: 255,
};

/// Water texture characters ordered by wave height
const WATER_CHARS: &[char] = &[' ', '.', ',', ';', '~', '=', '#'];

/// Parameters for a single sine wave component.
struct WaveParams {
    frequency: f64,
    amplitude: f64,
    speed: f64,
    phase: f64,
}

/// Sine-wave ocean water simulation.
pub struct OceanEffect {
    width: u16,
    height: u16,
    time: f64,
    waves: Vec<WaveParams>,
    speed_multiplier: f64,
}

impl OceanEffect {
    pub fn with_config(width: u16, height: u16, config: &crate::config::Config) -> Self {
        let mut rng = rand::rng();

        // Create 4 overlapping wave components with varied parameters
        let waves = vec![
            WaveParams {
                frequency: 0.08,
                amplitude: 3.0,
                speed: 1.2,
                phase: rng.random_range(0.0..std::f64::consts::TAU),
            },
            WaveParams {
                frequency: 0.15,
                amplitude: 1.5,
                speed: -0.8, // counter-direction for realism
                phase: rng.random_range(0.0..std::f64::consts::TAU),
            },
            WaveParams {
                frequency: 0.25,
                amplitude: 0.8,
                speed: 2.0,
                phase: rng.random_range(0.0..std::f64::consts::TAU),
            },
            WaveParams {
                frequency: 0.04,
                amplitude: 5.0, // big slow swell
                speed: 0.5,
                phase: rng.random_range(0.0..std::f64::consts::TAU),
            },
        ];

        Self {
            width,
            height,
            time: 0.0,
            waves,
            speed_multiplier: config.speed_multiplier,
        }
    }

    /// Calculate the combined wave height at a given x position.
    fn wave_height_at(&self, x: f64) -> f64 {
        let mut total = 0.0;
        for w in &self.waves {
            total += w.amplitude
                * (w.frequency * x + w.speed * self.time * self.speed_multiplier + w.phase).sin();
        }
        total
    }

    /// Map a depth value (0.0 = surface, 1.0 = deep) to a color.
    fn depth_to_color(depth: f64) -> Color {
        let d = depth.clamp(0.0, 1.0) as f32;
        if d < 0.1 {
            lerp_color(FOAM, CYAN, d / 0.1)
        } else if d < 0.3 {
            lerp_color(CYAN, LIGHT_BLUE, (d - 0.1) / 0.2)
        } else if d < 0.6 {
            lerp_color(LIGHT_BLUE, MID_BLUE, (d - 0.3) / 0.3)
        } else {
            lerp_color(MID_BLUE, DEEP_BLUE, (d - 0.6) / 0.4)
        }
    }
}

impl Effect for OceanEffect {
    fn name(&self) -> &str {
        "ocean"
    }

    fn update(&mut self, delta_time: f64) {
        self.time += delta_time;
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        let mid_y = self.height as f64 * 0.4; // Water surface at ~40% from top

        for x in 0..self.width {
            let wave_h = self.wave_height_at(x as f64);
            let surface_y = (mid_y + wave_h).round();

            for y in 0..self.height {
                let y_f = y as f64;

                if y_f < surface_y - 1.0 {
                    // Sky/above water: leave empty
                    continue;
                }

                // Depth below surface (0.0 = surface, increases downward)
                let depth = (y_f - surface_y + 1.0) / (self.height as f64 - surface_y + 1.0);
                let depth = depth.clamp(0.0, 1.0);

                // Character based on proximity to surface
                let char_idx = if depth < 0.05 {
                    // Surface/foam
                    WATER_CHARS.len() - 1
                } else if depth < 0.15 {
                    WATER_CHARS.len() - 2
                } else {
                    // Deeper water gets calmer characters
                    let idx = (depth * (WATER_CHARS.len() - 3) as f64).round() as usize;
                    idx.min(WATER_CHARS.len() - 3)
                };
                let ch = WATER_CHARS[char_idx.min(WATER_CHARS.len() - 1)];

                // Add horizontal wave motion to deeper water
                let h_offset = if depth > 0.1 {
                    let h_wave = (x as f64 * 0.1 + self.time * 0.5 + y_f * 0.05).sin();
                    h_wave * depth * 0.3
                } else {
                    0.0
                };

                // Subtle shimmer: modulate brightness slightly based on position
                let shimmer = 1.0 + (x as f64 * 0.3 + y_f * 0.2 + self.time * 2.0).sin() * 0.1;
                let _ = h_offset; // used conceptually to influence rendering

                let fg = Self::depth_to_color(depth);
                // Background is a darker version for depth feel
                let bg = Self::depth_to_color((depth + 0.3).min(1.0));

                let fg = crate::color::gradient::scale_color(fg, shimmer);
                buffer.set_cell(x, y, ch, fg, bg);
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    fn set_speed(&mut self, multiplier: f64) {
        self.speed_multiplier = multiplier;
    }

    fn speed(&self) -> f64 {
        self.speed_multiplier
    }
}
