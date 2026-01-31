//! Binary rain effect: dense columns of 0s and 1s.
//!
//! Thin wrapper around RainField that forces the binary character set
//! and uses slightly higher density (1.3x) for a dense data stream look.

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::config::Config;
use crate::rain::RainField;

/// Binary rain: dense columns of 0s and 1s.
pub struct BinaryRain {
    rain: RainField,
}

impl BinaryRain {
    pub fn with_config(width: u16, height: u16, config: &Config) -> Self {
        // Override charset to binary and boost density by 1.3x
        let mut binary_config = Config {
            effect_name: config.effect_name.clone(),
            speed_multiplier: config.speed_multiplier,
            density_multiplier: config.density_multiplier * 1.3,
            palette_name: config.palette_name.clone(),
            charset_name: "binary".to_string(),
            target_fps: config.target_fps,
            auto_cycle_secs: config.auto_cycle_secs,
            forward: config.forward,
            crt_enabled: config.crt_enabled,
            crt_intensity: config.crt_intensity,
        };
        // Clamp density after boosting
        binary_config.density_multiplier = binary_config.density_multiplier.clamp(0.1, 10.0);

        Self {
            rain: RainField::with_config(width, height, &binary_config),
        }
    }
}

impl Effect for BinaryRain {
    fn name(&self) -> &str {
        "binary"
    }

    fn update(&mut self, delta_time: f64) {
        self.rain.update(delta_time);
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        self.rain.render(buffer);
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.rain.resize(width, height);
    }

    fn set_speed(&mut self, multiplier: f64) {
        self.rain.set_speed(multiplier);
    }

    fn speed(&self) -> f64 {
        self.rain.speed()
    }

    fn set_density(&mut self, multiplier: f64) {
        self.rain.set_density(multiplier * 1.3);
    }

    fn density(&self) -> f64 {
        self.rain.density() / 1.3
    }
}
