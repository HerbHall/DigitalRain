//! Glitch effect: rain with periodic digital corruption events.
//!
//! Three types of glitch events occur randomly:
//! - Horizontal tear: a band of rows shifts left/right
//! - Block corruption: a rectangular region fills with noise characters
//! - Color separation: RGB channel offset in a region
//!
//! Inspired by digital signal corruption and VHS tracking errors.

use crossterm::style::Color;
use rand::Rng;

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::color::gradient::{color_to_rgb, scale_color};
use crate::config::Config;
use crate::rain::RainField;

/// Characters used for block corruption glitches.
const GLITCH_CHARS: &[char] = &[
    '#', '%', '&', '@', '!', '/', '\\', '|', '.', ':', '<', '>', '~', '^', '*', '=',
];

/// A single active glitch event with a lifetime.
enum GlitchEvent {
    /// Horizontal tear: rows shift sideways
    HorizontalTear {
        y_start: u16,
        y_end: u16,
        offset: i16,
        ttl: f64,
    },
    /// Block corruption: rectangular noise region
    BlockCorrupt {
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        ttl: f64,
    },
    /// Color separation: RGB channel offset in a band of rows
    ColorSep {
        y_start: u16,
        y_end: u16,
        r_offset: i16,
        ttl: f64,
    },
}

/// Glitch rain: standard rain with periodic digital corruption.
pub struct GlitchRain {
    rain: RainField,
    /// Time until next glitch spawns
    glitch_timer: f64,
    /// Active glitch events
    active_glitches: Vec<GlitchEvent>,
    width: u16,
    height: u16,
    speed_multiplier: f64,
}

impl GlitchRain {
    pub fn with_config(width: u16, height: u16, config: &Config) -> Self {
        Self {
            rain: RainField::with_config(width, height, config),
            glitch_timer: 0.5,
            active_glitches: Vec::new(),
            width,
            height,
            speed_multiplier: config.speed_multiplier,
        }
    }

    /// Spawn a random glitch event.
    fn spawn_glitch(&mut self, rng: &mut impl Rng) {
        let glitch_type = rng.random_range(0..3);
        match glitch_type {
            0 => {
                // Horizontal tear
                let y_start = rng.random_range(0..self.height);
                let band = rng.random_range(1..=4);
                let y_end = (y_start + band).min(self.height);
                let offset = rng.random_range(-8..=8_i16);
                self.active_glitches.push(GlitchEvent::HorizontalTear {
                    y_start,
                    y_end,
                    offset,
                    ttl: rng.random_range(0.05..0.2),
                });
            }
            1 => {
                // Block corruption
                let x = rng.random_range(0..self.width);
                let y = rng.random_range(0..self.height);
                let w = rng.random_range(3..=12).min(self.width - x);
                let h = rng.random_range(2..=5).min(self.height - y);
                self.active_glitches.push(GlitchEvent::BlockCorrupt {
                    x,
                    y,
                    w,
                    h,
                    ttl: rng.random_range(0.03..0.15),
                });
            }
            _ => {
                // Color separation
                let y_start = rng.random_range(0..self.height);
                let band = rng.random_range(2..=6);
                let y_end = (y_start + band).min(self.height);
                let r_offset = rng.random_range(-3..=3_i16);
                self.active_glitches.push(GlitchEvent::ColorSep {
                    y_start,
                    y_end,
                    r_offset,
                    ttl: rng.random_range(0.05..0.15),
                });
            }
        }
    }
}

impl Effect for GlitchRain {
    fn name(&self) -> &str {
        "glitch"
    }

    fn update(&mut self, delta_time: f64) {
        self.rain.update(delta_time);

        // Count down glitch timer, spawn new glitches
        self.glitch_timer -= delta_time;
        if self.glitch_timer <= 0.0 {
            let mut rng = rand::rng();
            // Spawn 1-3 glitches at once for clusters
            let count = rng.random_range(1..=3);
            for _ in 0..count {
                self.spawn_glitch(&mut rng);
            }
            // Next batch in 0.3-1.5 seconds (faster at higher speeds)
            self.glitch_timer = rng.random_range(0.3..1.5) / self.speed_multiplier.max(0.5);
        }

        // Decay active glitches
        for glitch in &mut self.active_glitches {
            match glitch {
                GlitchEvent::HorizontalTear { ttl, .. }
                | GlitchEvent::BlockCorrupt { ttl, .. }
                | GlitchEvent::ColorSep { ttl, .. } => {
                    *ttl -= delta_time;
                }
            }
        }
        self.active_glitches.retain(|g| match g {
            GlitchEvent::HorizontalTear { ttl, .. }
            | GlitchEvent::BlockCorrupt { ttl, .. }
            | GlitchEvent::ColorSep { ttl, .. } => *ttl > 0.0,
        });
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        // Render base rain
        self.rain.render(buffer);

        let mut rng = rand::rng();

        // Apply glitch post-processing
        for glitch in &self.active_glitches {
            match glitch {
                GlitchEvent::HorizontalTear {
                    y_start,
                    y_end,
                    offset,
                    ..
                } => {
                    // Shift rows horizontally by reading and rewriting cells
                    for y in *y_start..*y_end {
                        // Read entire row into a temp buffer
                        let mut row: Vec<_> = (0..self.width)
                            .map(|x| buffer.get_cell(x, y).copied().unwrap_or_default())
                            .collect();
                        // Shift the row
                        let len = row.len();
                        if *offset > 0 {
                            row.rotate_right((*offset as usize).min(len));
                        } else if *offset < 0 {
                            row.rotate_left(((-*offset) as usize).min(len));
                        }
                        // Write back
                        for (x, cell) in row.iter().enumerate() {
                            buffer.set_cell(x as u16, y, cell.ch, cell.fg, cell.bg);
                        }
                    }
                }
                GlitchEvent::BlockCorrupt { x, y, w, h, .. } => {
                    // Fill block with random noise characters
                    for by in *y..(*y + *h).min(self.height) {
                        for bx in *x..(*x + *w).min(self.width) {
                            let ch = GLITCH_CHARS[rng.random_range(0..GLITCH_CHARS.len())];
                            let brightness = rng.random_range(0.5..1.5);
                            if let Some(cell) = buffer.get_cell(bx, by) {
                                let fg = scale_color(cell.fg, brightness);
                                buffer.set_cell(bx, by, ch, fg, cell.bg);
                            }
                        }
                    }
                }
                GlitchEvent::ColorSep {
                    y_start,
                    y_end,
                    r_offset,
                    ..
                } => {
                    // Shift the red channel by reading from offset position
                    for y in *y_start..*y_end {
                        for x in 0..self.width {
                            if let Some(cell) = buffer.get_cell(x, y) {
                                if cell.ch == ' ' {
                                    continue;
                                }
                                let (r, g, b) = color_to_rgb(cell.fg);
                                // Read red from an offset position
                                let src_x =
                                    (x as i16 + *r_offset).clamp(0, self.width as i16 - 1) as u16;
                                let shifted_r = if let Some(src) = buffer.get_cell(src_x, y) {
                                    let (sr, _, _) = color_to_rgb(src.fg);
                                    sr
                                } else {
                                    r
                                };
                                let new_fg = Color::Rgb { r: shifted_r, g, b };
                                buffer.set_cell(x, y, cell.ch, new_fg, cell.bg);
                            }
                        }
                    }
                }
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.rain.resize(width, height);
        self.width = width;
        self.height = height;
        self.active_glitches.clear();
    }

    fn set_speed(&mut self, multiplier: f64) {
        self.rain.set_speed(multiplier);
        self.speed_multiplier = multiplier;
    }

    fn speed(&self) -> f64 {
        self.rain.speed()
    }

    fn set_density(&mut self, multiplier: f64) {
        self.rain.set_density(multiplier);
    }

    fn density(&self) -> f64 {
        self.rain.density()
    }
}
