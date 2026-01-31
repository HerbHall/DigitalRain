//! Parallax effect: multi-layer rain with depth.
//!
//! 2-3 RainField instances run at different speeds and brightness levels,
//! creating a sense of depth. Background layers are slow, dim, and sparse.
//! Foreground layers are fast, bright, and dense.

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::color::gradient::scale_color;
use crate::config::Config;
use crate::rain::RainField;

/// A single depth layer with its own RainField and brightness.
struct ParallaxLayer {
    rain: RainField,
    brightness: f64,
}

/// Multi-layer parallax rain with depth effect.
pub struct ParallaxRain {
    layers: Vec<ParallaxLayer>,
    base_speed: f64,
}

impl ParallaxRain {
    pub fn with_config(width: u16, height: u16, config: &Config) -> Self {
        // Create 3 layers: background, mid-ground, foreground
        let layer_params = [
            // (speed_scale, density_scale, brightness)
            (0.3, 0.4, 0.25), // Background: slow, sparse, dim
            (0.7, 0.7, 0.55), // Mid-ground: medium
            (1.5, 1.2, 1.0),  // Foreground: fast, dense, bright
        ];

        let layers = layer_params
            .iter()
            .map(|&(speed_scale, density_scale, brightness)| {
                let mut layer_config = Config {
                    effect_name: config.effect_name.clone(),
                    speed_multiplier: config.speed_multiplier * speed_scale,
                    density_multiplier: config.density_multiplier * density_scale,
                    palette_name: config.palette_name.clone(),
                    charset_name: config.charset_name.clone(),
                    target_fps: config.target_fps,
                    auto_cycle_secs: config.auto_cycle_secs,
                    forward: config.forward,
                    crt_enabled: config.crt_enabled,
                    crt_intensity: config.crt_intensity,
                };
                // Clamp density
                layer_config.density_multiplier = layer_config.density_multiplier.clamp(0.1, 10.0);

                ParallaxLayer {
                    rain: RainField::with_config(width, height, &layer_config),
                    brightness,
                }
            })
            .collect();

        Self {
            layers,
            base_speed: config.speed_multiplier,
        }
    }
}

impl Effect for ParallaxRain {
    fn name(&self) -> &str {
        "parallax"
    }

    fn update(&mut self, delta_time: f64) {
        for layer in &mut self.layers {
            layer.rain.update(delta_time);
        }
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        // Render back-to-front: background first, foreground overwrites
        for layer in &self.layers {
            if layer.brightness >= 1.0 {
                // Foreground at full brightness: render directly
                layer.rain.render(buffer);
            } else {
                // Snapshot which cells are currently non-empty so we can identify
                // which cells this layer adds (and dim only those).
                let w = buffer.width();
                let h = buffer.height();
                let mut before = vec![false; w as usize * h as usize];
                for y in 0..h {
                    for x in 0..w {
                        let idx = y as usize * w as usize + x as usize;
                        before[idx] = buffer.get_cell(x, y).map(|c| c.ch != ' ').unwrap_or(false);
                    }
                }

                layer.rain.render(buffer);

                // Dim any newly-written cells
                for y in 0..h {
                    for x in 0..w {
                        let idx = y as usize * w as usize + x as usize;
                        if !before[idx]
                            && let Some(cell) = buffer.get_cell(x, y)
                            && cell.ch != ' '
                        {
                            let dimmed_fg = scale_color(cell.fg, layer.brightness);
                            let dimmed_bg = scale_color(cell.bg, layer.brightness);
                            buffer.set_cell(x, y, cell.ch, dimmed_fg, dimmed_bg);
                        }
                    }
                }
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        for layer in &mut self.layers {
            layer.rain.resize(width, height);
        }
    }

    fn set_speed(&mut self, multiplier: f64) {
        self.base_speed = multiplier;
        let scales = [0.3, 0.7, 1.5];
        for (i, layer) in self.layers.iter_mut().enumerate() {
            layer
                .rain
                .set_speed(multiplier * scales[i.min(scales.len() - 1)]);
        }
    }

    fn speed(&self) -> f64 {
        self.base_speed
    }

    fn set_density(&mut self, multiplier: f64) {
        let scales = [0.4, 0.7, 1.2];
        for (i, layer) in self.layers.iter_mut().enumerate() {
            layer
                .rain
                .set_density(multiplier * scales[i.min(scales.len() - 1)]);
        }
    }

    fn density(&self) -> f64 {
        // Return the base density (foreground layer / scale factor)
        self.layers
            .last()
            .map(|l| l.rain.density() / 1.2)
            .unwrap_or(1.0)
    }
}
