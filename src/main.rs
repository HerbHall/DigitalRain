//! DigitalRain - Terminal-based Matrix digital rain effect.
//!
//! A visual effects application that renders the iconic Matrix falling
//! characters in your terminal. Built with Rust and crossterm for
//! cross-platform compatibility (Windows-first).

mod buffer;
mod color;
mod config;
mod effects;
mod rain;
mod terminal;
mod timing;

use clap::Parser;
use crossterm::event::Event;

use buffer::ScreenBuffer;
use config::{Cli, Config};
use effects::registry;
use terminal::Terminal;
use timing::FrameClock;

fn main() {
    let cli = Cli::parse();

    // Handle list flags (print info and exit, no terminal setup needed)
    if cli.list_effects {
        registry::print_effects();
        return;
    }
    if cli.list_colors {
        registry::print_palettes();
        return;
    }
    if cli.list_charsets {
        registry::print_charsets();
        return;
    }

    // Build config from CLI args (or randomize if --random)
    let config = if cli.random {
        Config::randomized()
    } else {
        Config::from_cli(&cli)
    };

    // If randomized, show what was picked so the user knows
    if cli.random {
        eprintln!(
            "Random: effect={}, color={}, charset={}, speed={:.1}, density={:.1}",
            config.effect_name,
            config.palette_name,
            config.charset_name,
            config.speed_multiplier,
            config.density_multiplier,
        );
    }

    // Initialize the terminal (alternate screen, raw mode, hidden cursor)
    let mut term = Terminal::init().expect("Failed to initialize terminal");

    let mut buffer = ScreenBuffer::new(term.width, term.height);
    let mut clock = FrameClock::new(config.target_fps);

    // Create the selected effect
    let mut effect = registry::create_effect(&config.effect_name, term.width, term.height, &config)
        .unwrap_or_else(|| {
            eprintln!(
                "Unknown effect '{}', using classic. Run --list-effects to see options.",
                config.effect_name
            );
            registry::create_effect("classic", term.width, term.height, &config).unwrap()
        });

    // Main loop: poll events, update, render
    loop {
        match term.poll_event(clock.poll_timeout()) {
            Ok(Some(event)) => {
                if Terminal::should_quit(&event) {
                    break;
                }

                if let Event::Resize(_, _) = event {
                    term.update_size().ok();
                    buffer.resize(term.width, term.height);
                    effect.resize(term.width, term.height);
                }
            }
            Ok(None) => {}
            Err(_) => break,
        }

        if !clock.tick() {
            continue;
        }

        effect.update(clock.delta_time());

        buffer.clear();
        effect.render(&mut buffer);
        if buffer.flush().is_err() {
            break;
        }
    }
}
