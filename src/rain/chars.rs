//! Character sets for the rain effect.
//!
//! The original Matrix films use half-width katakana characters mixed with
//! Latin letters and digits. We define character pools that effects can
//! draw from randomly.

use rand::Rng;

/// A pool of characters that rain columns draw from.
pub struct CharacterPool {
    /// The available characters
    chars: Vec<char>,
}

impl CharacterPool {
    /// The classic Matrix character set: half-width katakana + digits + some symbols.
    ///
    /// Half-width katakana (U+FF66 - U+FF9F) are single-column-width characters
    /// in terminals, so they align properly with ASCII characters.
    pub fn matrix() -> Self {
        let mut chars = Vec::new();

        // Half-width katakana (the iconic Matrix characters)
        // Range: U+FF66 (ｦ) through U+FF9F (ﾟ)
        for c in '\u{FF66}'..='\u{FF9F}' {
            chars.push(c);
        }

        // Digits
        for c in '0'..='9' {
            chars.push(c);
        }

        // Some symbolic characters for variety
        for c in &[':', '.', '"', '=', '*', '+', '-', '<', '>', '|', '~', '^'] {
            chars.push(*c);
        }

        Self { chars }
    }

    /// Pick a random character from the pool.
    pub fn random_char(&self, rng: &mut impl Rng) -> char {
        let idx = rng.random_range(0..self.chars.len());
        self.chars[idx]
    }
}
