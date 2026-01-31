//! Character sets for the rain effect.
//!
//! The original Matrix films use half-width katakana characters mixed with
//! Latin letters and digits. We define character pools that effects can
//! draw from randomly.

use rand::Rng;

/// Returns the list of available character set names.
pub fn charset_names() -> &'static [&'static str] {
    &["matrix", "ascii", "binary", "digits", "katakana", "latin"]
}

/// Look up a character pool by name. Returns matrix if the name is unknown.
pub fn charset_by_name(name: &str) -> CharacterPool {
    match name {
        "matrix" => CharacterPool::matrix(),
        "ascii" => CharacterPool::ascii(),
        "binary" => CharacterPool::binary(),
        "digits" => CharacterPool::digits(),
        "katakana" => CharacterPool::katakana(),
        "latin" => CharacterPool::latin(),
        _ => {
            eprintln!("Unknown charset '{}', using matrix", name);
            CharacterPool::matrix()
        }
    }
}

/// A pool of characters that rain columns draw from.
pub struct CharacterPool {
    /// The available characters
    chars: Vec<char>,
}

impl CharacterPool {
    /// The classic Matrix character set: half-width katakana + digits + symbols.
    pub fn matrix() -> Self {
        let mut chars = Vec::new();

        // Half-width katakana (U+FF66 through U+FF9F)
        for c in '\u{FF66}'..='\u{FF9F}' {
            chars.push(c);
        }

        // Digits
        for c in '0'..='9' {
            chars.push(c);
        }

        // Symbolic characters for variety
        for c in &[':', '.', '"', '=', '*', '+', '-', '<', '>', '|', '~', '^'] {
            chars.push(*c);
        }

        Self { chars }
    }

    /// ASCII letters + digits + symbols.
    pub fn ascii() -> Self {
        let mut chars: Vec<char> = ('!'..='~').collect();
        // Remove space (0x20), start from '!' (0x21)
        chars.retain(|c| !c.is_whitespace());
        Self { chars }
    }

    /// Binary: just 0 and 1.
    pub fn binary() -> Self {
        Self {
            chars: vec!['0', '1'],
        }
    }

    /// Digits only: 0-9.
    pub fn digits() -> Self {
        Self {
            chars: ('0'..='9').collect(),
        }
    }

    /// Katakana only: half-width katakana characters.
    pub fn katakana() -> Self {
        Self {
            chars: ('\u{FF66}'..='\u{FF9F}').collect(),
        }
    }

    /// Latin letters: uppercase + lowercase.
    pub fn latin() -> Self {
        let mut chars: Vec<char> = ('A'..='Z').collect();
        chars.extend('a'..='z');
        Self { chars }
    }

    /// Pick a random character from the pool.
    pub fn random_char(&self, rng: &mut impl Rng) -> char {
        let idx = rng.random_range(0..self.chars.len());
        self.chars[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charset_names_not_empty() {
        assert!(!charset_names().is_empty());
    }

    #[test]
    fn all_named_charsets_resolve() {
        for name in charset_names() {
            let pool = charset_by_name(name);
            assert!(
                !pool.chars.is_empty(),
                "charset '{}' should not be empty",
                name
            );
        }
    }

    #[test]
    fn unknown_charset_falls_back_to_matrix() {
        let unknown = charset_by_name("nonexistent");
        let matrix = CharacterPool::matrix();
        assert_eq!(unknown.chars.len(), matrix.chars.len());
    }

    #[test]
    fn binary_charset_has_only_zero_and_one() {
        let pool = CharacterPool::binary();
        assert_eq!(pool.chars.len(), 2);
        assert!(pool.chars.contains(&'0'));
        assert!(pool.chars.contains(&'1'));
    }

    #[test]
    fn matrix_charset_contains_katakana() {
        let pool = CharacterPool::matrix();
        // Half-width katakana U+FF66 (ｦ)
        assert!(pool.chars.contains(&'\u{FF66}'));
    }

    #[test]
    fn random_char_returns_valid_char() {
        let pool = CharacterPool::matrix();
        let mut rng = rand::rng();
        for _ in 0..100 {
            let ch = pool.random_char(&mut rng);
            assert!(pool.chars.contains(&ch));
        }
    }
}
