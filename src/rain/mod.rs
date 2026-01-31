//! Rain simulation: manages all falling columns of characters.

pub mod chars;
pub mod column;

use rand::Rng;

use self::chars::CharacterPool;
use self::column::RainColumn;
use crate::buffer::ScreenBuffer;
use crate::color::gradient::trail_color;
use crate::color::palette::Palette;

/// Manages the full rain simulation across all columns of the screen.
pub struct RainField {
    /// One column state per screen column
    columns: Vec<RainColumn>,
    /// Character pool for random character selection
    char_pool: CharacterPool,
    /// Color palette
    palette: Palette,
    /// Screen dimensions
    width: u16,
    height: u16,
    /// Density: probability (0.0-1.0) that a new column spawns per frame
    spawn_rate: f64,
}

impl RainField {
    /// Create a new rain field covering the screen.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            columns: Vec::new(),
            char_pool: CharacterPool::matrix(),
            palette: Palette::classic(),
            width,
            height,
            spawn_rate: 0.15,
        }
    }

    /// Resize the field (e.g., when terminal is resized).
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        // Remove columns that are now out of bounds
        self.columns.retain(|c| c.x < width);
    }

    /// Advance the simulation by one frame.
    pub fn update(&mut self, delta_time: f64) {
        let mut rng = rand::rng();

        // Update existing columns, removing any that have fully scrolled off
        self.columns.retain_mut(|col| {
            col.update(delta_time, self.height, &self.char_pool, &mut rng);
            !col.is_dead(self.height)
        });

        // Spawn new columns randomly
        for x in 0..self.width {
            // Only spawn if there isn't already an active column at this x
            let has_column = self.columns.iter().any(|c| c.x == x && !c.is_fading());
            if !has_column && rng.random_bool(self.spawn_rate * delta_time) {
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

            // Check if this character is a gold highlight
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
