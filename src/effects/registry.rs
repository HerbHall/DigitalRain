//! Effect registry: discovery, listing, and creation of effects by name.

use super::Effect;
use super::classic::ClassicRain;
use crate::config::Config;

/// Returns the list of available effect names.
pub fn effect_names() -> &'static [&'static str] {
    &["classic"]
}

/// Get the next effect name in the cycle after the given name.
pub fn next_effect_name(current: &str) -> &'static str {
    let names = effect_names();
    let current_idx = names.iter().position(|&n| n == current).unwrap_or(0);
    let next_idx = (current_idx + 1) % names.len();
    names[next_idx]
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
    use crate::color::palette;

    // Featured (hand-tuned) palettes with descriptions
    println!("Featured palettes:");
    for name in palette::hand_tuned_names() {
        let desc = match *name {
            "classic" => "Green phosphor (the Matrix default)",
            "gold" => "Warm amber/gold CRT feel",
            "cyan" => "Cold ice-blue digital",
            "red" => "Crimson danger/alert",
            "silver" => "White/grey on black",
            "purple" => "Violet synthwave",
            "fire" => "Red/orange/yellow heat gradient",
            "ocean" => "Deep blue/teal aquatic",
            "synthwave" => "Pink/purple/cyan retro neon",
            _ => "",
        };
        println!("  {:<12} - {}", name, desc);
    }

    // CSS named colors in compact columns
    let css_names: Vec<&str> = palette::palette_names()
        .into_iter()
        .filter(|n| !palette::hand_tuned_names().contains(n))
        .collect();

    println!();
    println!(
        "CSS named colors ({} additional -- use any as --color <name>):",
        css_names.len()
    );

    // Print in columns (4 per row, 20 chars wide)
    const COLS: usize = 4;
    const COL_WIDTH: usize = 22;
    for chunk in css_names.chunks(COLS) {
        print!("  ");
        for name in chunk {
            print!("{:<width$}", name, width = COL_WIDTH);
        }
        println!();
    }

    println!();
    println!("Aliases: monochrome -> silver");
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
