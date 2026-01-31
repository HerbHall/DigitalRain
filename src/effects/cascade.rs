//! Cascade effect: wave-front spawning across the screen.
//!
//! A sweep moves left-to-right, activating columns as it passes.
//! Creates a coordinated waterfall/curtain effect where columns start
//! in sequence rather than randomly.

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::color::palette::{Palette, palette_by_name};
use crate::config::Config;
use crate::rain::chars::{CharacterPool, charset_by_name};
use crate::rain::column::RainColumn;
use crate::rain::render_rain_column;

/// Cascade rain: columns activate in a wave-front sweep.
pub struct CascadeRain {
    columns: Vec<RainColumn>,
    /// Current wave position (fractional column index)
    wave_x: f64,
    /// Wave speed in columns per second
    wave_speed: f64,
    /// Which x positions have been activated
    activated: Vec<bool>,
    palette: Palette,
    char_pool: CharacterPool,
    width: u16,
    height: u16,
    speed_multiplier: f64,
    forward: bool,
}

impl CascadeRain {
    pub fn with_config(width: u16, height: u16, config: &Config) -> Self {
        // Wave speed scales with screen width so the sweep completes
        // in about 3-5 seconds regardless of terminal size
        let wave_speed = width as f64 / 3.5 * config.speed_multiplier;

        Self {
            columns: Vec::new(),
            wave_x: 0.0,
            wave_speed,
            activated: vec![false; width as usize],
            palette: palette_by_name(&config.palette_name),
            char_pool: charset_by_name(&config.charset_name),
            width,
            height,
            speed_multiplier: config.speed_multiplier,
            forward: config.forward,
        }
    }

    /// Reset the wave to sweep again from the left
    fn reset_wave(&mut self) {
        self.wave_x = 0.0;
        self.activated = vec![false; self.width as usize];
    }
}

impl Effect for CascadeRain {
    fn name(&self) -> &str {
        "cascade"
    }

    fn update(&mut self, delta_time: f64) {
        let mut rng = rand::rng();
        let effective_dt = delta_time * self.speed_multiplier;

        // Advance the wave front
        self.wave_x += self.wave_speed * delta_time;

        // Activate columns the wave has passed over
        let wave_end = (self.wave_x as usize).min(self.width as usize);
        for x in 0..wave_end {
            if x < self.activated.len() && !self.activated[x] {
                self.activated[x] = true;
                self.columns
                    .push(RainColumn::spawn(x as u16, self.height, &mut rng));
            }
        }

        // Update existing columns
        self.columns.retain_mut(|col| {
            col.update(effective_dt, self.height, &self.char_pool, &mut rng);
            !col.is_dead(self.height)
        });

        // If all columns have drained and wave has passed, reset for another sweep
        if self.wave_x > self.width as f64 + 10.0 && self.columns.is_empty() {
            self.reset_wave();
        }
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        for col in &self.columns {
            render_rain_column(col, &self.palette, self.height, self.forward, buffer);
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.columns.retain(|c| c.x < width);
        // Reset wave for new dimensions
        self.reset_wave();
        self.wave_speed = width as f64 / 3.5 * self.speed_multiplier;
    }

    fn set_speed(&mut self, multiplier: f64) {
        self.speed_multiplier = multiplier;
        self.wave_speed = self.width as f64 / 3.5 * multiplier;
    }

    fn speed(&self) -> f64 {
        self.speed_multiplier
    }
}
