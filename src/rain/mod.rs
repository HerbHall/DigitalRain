//! Rain simulation: manages all falling columns of characters.

pub mod chars;
pub mod column;

use rand::Rng;

use self::chars::{CharacterPool, charset_by_name};
use self::column::RainColumn;
use crate::buffer::ScreenBuffer;
use crate::color::gradient::trail_color;
use crate::color::palette::{Palette, palette_by_name};
use crate::config::Config;

/// Manages the full rain simulation across all columns of the screen.
pub struct RainField {
    columns: Vec<RainColumn>,
    char_pool: CharacterPool,
    palette: Palette,
    width: u16,
    height: u16,
    /// Base spawn rate before density multiplier
    spawn_rate: f64,
    /// Speed multiplier applied to all column speeds
    speed_multiplier: f64,
}

impl RainField {
    /// Create a new rain field with default settings.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            columns: Vec::new(),
            char_pool: CharacterPool::matrix(),
            palette: Palette::classic(),
            width,
            height,
            spawn_rate: 0.15,
            speed_multiplier: 1.0,
        }
    }

    /// Create a new rain field from a Config.
    pub fn with_config(width: u16, height: u16, config: &Config) -> Self {
        Self {
            columns: Vec::new(),
            char_pool: charset_by_name(&config.charset_name),
            palette: palette_by_name(&config.palette_name),
            width,
            height,
            spawn_rate: 0.15 * config.density_multiplier,
            speed_multiplier: config.speed_multiplier,
        }
    }

    /// Set the speed multiplier (affects how fast columns fall).
    pub fn set_speed(&mut self, multiplier: f64) {
        self.speed_multiplier = multiplier;
    }

    /// Get the current speed multiplier.
    pub fn speed(&self) -> f64 {
        self.speed_multiplier
    }

    /// Set the density (spawn rate). Higher = more columns at once.
    pub fn set_density(&mut self, multiplier: f64) {
        self.spawn_rate = 0.15 * multiplier;
    }

    /// Get the current density multiplier.
    pub fn density(&self) -> f64 {
        self.spawn_rate / 0.15
    }

    /// Resize the field (e.g., when terminal is resized).
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.columns.retain(|c| c.x < width);
    }

    /// Advance the simulation by one frame.
    pub fn update(&mut self, delta_time: f64) {
        let mut rng = rand::rng();

        // Apply speed multiplier to the effective delta time
        let effective_dt = delta_time * self.speed_multiplier;

        // Update existing columns, removing any that have fully scrolled off
        self.columns.retain_mut(|col| {
            col.update(effective_dt, self.height, &self.char_pool, &mut rng);
            !col.is_dead(self.height)
        });

        // Spawn new columns randomly
        for x in 0..self.width {
            let has_column = self.columns.iter().any(|c| c.x == x && !c.is_fading());
            if !has_column && rng.random_bool((self.spawn_rate * delta_time).min(1.0)) {
                self.columns
                    .push(RainColumn::spawn(x, self.height, &mut rng));
            }
        }
    }

    /// Render all columns into the screen buffer.
    pub fn render(&self, buffer: &mut ScreenBuffer) {
        for col in &self.columns {
            self.render_column(col, buffer);
        }
    }

    /// Render a single rain column with gradient trail.
    fn render_column(&self, col: &RainColumn, buffer: &mut ScreenBuffer) {
        let trail_len = col.trail.len();
        if trail_len == 0 {
            return;
        }

        for (i, &(y, ch)) in col.trail.iter().enumerate() {
            if y >= self.height {
                continue;
            }

            // Position in trail: 0.0 = head (newest), 1.0 = tail (oldest)
            let position = i as f32 / trail_len.max(1) as f32;

            let fg = if col.highlight_positions.contains(&i) {
                self.palette.highlight
            } else {
                trail_color(
                    self.palette.head,
                    self.palette.body_bright,
                    self.palette.body_mid,
                    self.palette.tail,
                    position,
                )
            };

            buffer.set_cell(col.x, y, ch, fg, self.palette.background);
        }
    }
}
