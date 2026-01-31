//! Individual rain column: one vertical stream of falling characters.
//!
//! Each column has a head position that moves downward, leaving a trail of
//! characters behind it. The trail has a maximum length; characters at the
//! tail end fade out and disappear.

use rand::Rng;

use super::chars::CharacterPool;

/// A single vertical column of falling rain.
pub struct RainColumn {
    /// X position (screen column)
    pub x: u16,
    /// The trail of characters: (y_position, character).
    /// Index 0 is the tail (oldest), last index is the head (newest).
    pub trail: Vec<(u16, char)>,
    /// Which trail indices are gold highlights
    pub highlight_positions: Vec<usize>,
    /// Maximum trail length before tail characters start disappearing
    max_trail_len: usize,
    /// Vertical speed in rows per second
    speed: f64,
    /// Accumulated fractional row movement (for sub-row-per-frame speeds)
    accumulator: f64,
    /// Current head Y position (where the next character will be placed)
    head_y: f64,
    /// Whether this column has stopped spawning new characters (draining)
    draining: bool,
    /// Probability that a character mutates (changes) each frame
    mutation_rate: f64,
}

impl RainColumn {
    /// Spawn a new rain column at the given x position.
    pub fn spawn(x: u16, screen_height: u16, rng: &mut impl Rng) -> Self {
        // Randomize speed: faster columns feel "closer" to the viewer
        let speed = rng.random_range(8.0..25.0);

        // Trail length proportional to screen height, with some randomness
        let max_trail_len =
            rng.random_range((screen_height as usize / 3)..=(screen_height as usize));

        // Start above the screen so the head "enters" from the top
        let start_y = -(rng.random_range(0..screen_height / 2) as f64);

        Self {
            x,
            trail: Vec::with_capacity(max_trail_len),
            highlight_positions: Vec::new(),
            max_trail_len,
            speed,
            accumulator: 0.0,
            head_y: start_y,
            draining: false,
            mutation_rate: 0.02,
        }
    }

    /// Advance this column by one frame.
    pub fn update(
        &mut self,
        delta_time: f64,
        screen_height: u16,
        char_pool: &CharacterPool,
        rng: &mut impl Rng,
    ) {
        // Move the head down by speed * delta_time rows
        self.accumulator += self.speed * delta_time;

        // For each whole row the head has moved, add a new character
        while self.accumulator >= 1.0 {
            self.accumulator -= 1.0;
            let y = self.head_y as i32;

            if y >= 0 && y < screen_height as i32 {
                let ch = char_pool.random_char(rng);
                self.trail.push((y as u16, ch));

                // Small chance this character is a gold highlight
                if rng.random_bool(0.03) {
                    self.highlight_positions.push(self.trail.len() - 1);
                }
            }

            self.head_y += 1.0;

            // If the head has gone past the bottom, start draining
            if self.head_y >= screen_height as f64 {
                self.draining = true;
            }
        }

        // Trim trail from the tail if it exceeds max length
        while self.trail.len() > self.max_trail_len {
            self.trail.remove(0);
            // Adjust highlight positions
            self.highlight_positions.retain_mut(|pos| {
                if *pos == 0 {
                    false
                } else {
                    *pos -= 1;
                    true
                }
            });
        }

        // If draining, also remove from the tail each frame
        if self.draining && !self.trail.is_empty() {
            self.trail.remove(0);
            self.highlight_positions.retain_mut(|pos| {
                if *pos == 0 {
                    false
                } else {
                    *pos -= 1;
                    true
                }
            });
        }

        // Character mutation: randomly change some characters in the trail
        for (_, ch) in &mut self.trail {
            if rng.random_bool(self.mutation_rate) {
                *ch = char_pool.random_char(rng);
            }
        }
    }

    /// Returns true if this column has no visible characters left.
    pub fn is_dead(&self, _screen_height: u16) -> bool {
        self.draining && self.trail.is_empty()
    }

    /// Returns true if this column is draining (no longer spawning new chars).
    pub fn is_fading(&self) -> bool {
        self.draining
    }
}
