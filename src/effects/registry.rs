//! Effect registry: discovery, listing, and creation of effects by name.

use super::Effect;
use super::classic::ClassicRain;
use crate::config::Config;

/// Returns the list of available effect names.
pub fn effect_names() -> &'static [&'static str] {
    &["classic"]
}

/// Create an effect by name, using the provided config and screen dimensions.
/// Returns None if the effect name is unknown.
pub fn create_effect(
    name: &str,
    width: u16,
    height: u16,
    config: &Config,
) -> Option<Box<dyn Effect>> {
    match name {
        "classic" => Some(Box::new(ClassicRain::with_config(width, height, config))),
        _ => None,
    }
}

/// Print available effects to stdout (for --list-effects).
pub fn print_effects() {
    println!("Available effects:");
    println!("  classic    - Classic Matrix digital rain");
    println!();
    println!("More effects coming in Phase 5 (cascade, pulse, glitch, binary, fire, ocean)");
}

/// Print available color palettes to stdout (for --list-colors).
pub fn print_palettes() {
    println!("Available color palettes:");
    for name in crate::color::palette::palette_names() {
        let desc = match *name {
            "classic" => "Green phosphor (the Matrix default)",
            "gold" => "Warm amber/gold CRT feel",
            "cyan" => "Cold ice-blue digital",
            "red" => "Crimson danger/alert",
            "monochrome" => "White/grey on black",
            "purple" => "Violet synthwave",
            _ => "",
        };
        println!("  {:<12} - {}", name, desc);
    }
}

/// Print available character sets to stdout (for --list-charsets).
pub fn print_charsets() {
    println!("Available character sets:");
    for name in crate::rain::chars::charset_names() {
        let desc = match *name {
            "matrix" => "Half-width katakana + digits + symbols (film-authentic)",
            "ascii" => "Full printable ASCII characters",
            "binary" => "0 and 1 only",
            "digits" => "0-9 only",
            "katakana" => "Half-width katakana only",
            "latin" => "A-Z, a-z letters",
            _ => "",
        };
        println!("  {:<12} - {}", name, desc);
    }
}
