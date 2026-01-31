//! Pulse effect: standard rain with a brightness wave overlay.
//!
//! A sine wave sweeps vertically through the screen, modulating cell
//! brightness based on distance from the wave peak. Creates a pulsing,
//! breathing feel on top of normal rain.

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::color::gradient::{color_to_rgb, scale_color};
use crate::config::Config;
use crate::rain::RainField;

/// Pulse rain: brightness wave sweeping over standard rain.
pub struct PulseRain {
    rain: RainField,
    /// Current phase of the pulse wave (radians)
    pulse_phase: f64,
    /// Distance between wave peaks in rows
    pulse_wavelength: f64,
    /// How deep the modulation goes (0.0 = no effect, 1.0 = full dark-to-bright)
    pulse_amplitude: f64,
    /// Speed of the wave in rows per second
    pulse_speed: f64,
    width: u16,
    height: u16,
}

impl PulseRain {
    pub fn with_config(width: u16, height: u16, config: &Config) -> Self {
        Self {
            rain: RainField::with_config(width, height, config),
            pulse_phase: 0.0,
            pulse_wavelength: height as f64 * 0.6, // ~60% of screen height between peaks
            pulse_amplitude: 0.5,                  // modulate brightness by up to 50%
            pulse_speed: 12.0 * config.speed_multiplier,
            width,
            height,
        }
    }
}

impl Effect for PulseRain {
    fn name(&self) -> &str {
        "pulse"
    }

    fn update(&mut self, delta_time: f64) {
        self.rain.update(delta_time);
        // Advance the pulse wave downward
        self.pulse_phase += self.pulse_speed * delta_time;
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        // First render the base rain
        self.rain.render(buffer);

        // Then apply brightness modulation per row based on sine wave
        for y in 0..self.height {
            let wave_value = ((y as f64 - self.pulse_phase) * std::f64::consts::TAU
                / self.pulse_wavelength)
                .sin();
            // Map sine (-1..1) to brightness factor
            // At wave peak (1.0): full brightness
            // At wave trough (-1.0): dimmed by pulse_amplitude
            let brightness = 1.0 - self.pulse_amplitude * (1.0 - wave_value) * 0.5;

            for x in 0..self.width {
                if let Some(cell) = buffer.get_cell(x, y) {
                    // Skip empty cells
                    if cell.ch == ' ' {
                        continue;
                    }
                    // Only modulate cells that have visible content
                    let (r, g, b) = color_to_rgb(cell.fg);
                    if r == 0 && g == 0 && b == 0 {
                        continue;
                    }
                    let new_fg = scale_color(cell.fg, brightness);
                    let new_bg = scale_color(cell.bg, brightness);
                    buffer.set_cell(x, y, cell.ch, new_fg, new_bg);
                }
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.rain.resize(width, height);
        self.width = width;
        self.height = height;
        self.pulse_wavelength = height as f64 * 0.6;
    }

    fn set_speed(&mut self, multiplier: f64) {
        self.rain.set_speed(multiplier);
        self.pulse_speed = 12.0 * multiplier;
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
