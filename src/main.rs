//! DigitalRain - Terminal-based Matrix digital rain effect.
//!
//! A visual effects application that renders the iconic Matrix falling
//! characters in your terminal. Built with Rust and crossterm for
//! cross-platform compatibility (Windows-first).

mod buffer;
mod color;
mod config;
mod effects;
mod overlay;
mod rain;
mod terminal;
mod timing;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEvent};

use buffer::ScreenBuffer;
use config::{Cli, Config};
use effects::registry;
use terminal::Terminal;
use timing::FrameClock;

/// How many frames to show the status message after a parameter change.
const STATUS_DISPLAY_FRAMES: u32 = 60;

/// Speed adjustment step per keypress.
const SPEED_STEP: f64 = 0.2;
/// Density adjustment step per keypress.
const DENSITY_STEP: f64 = 0.2;

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
    let mut config = if cli.random {
        Config::randomized()
    } else {
        Config::from_cli(&cli)
    };

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
            config.effect_name = "classic".to_string();
            registry::create_effect("classic", term.width, term.height, &config).unwrap()
        });

    // Runtime state
    let mut paused = false;
    let mut show_help = false;
    let mut status_message: Option<String> = None;
    let mut status_frames_remaining: u32 = 0;

    // Auto-cycle timer state
    let mut auto_cycle_enabled = config.auto_cycle_secs.is_some();
    let auto_cycle_interval = config.auto_cycle_secs;
    let mut auto_cycle_elapsed: f64 = 0.0;

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

                // Handle interactive key controls
                if let Event::Key(KeyEvent { code, .. }) = event {
                    match code {
                        // Pause / Resume
                        KeyCode::Char(' ') => {
                            paused = !paused;
                            set_status(
                                &mut status_message,
                                &mut status_frames_remaining,
                                if paused { "PAUSED" } else { "RESUMED" },
                            );
                        }

                        // Speed up
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            let new_speed = (effect.speed() + SPEED_STEP).clamp(0.1, 10.0);
                            effect.set_speed(new_speed);
                            set_status(
                                &mut status_message,
                                &mut status_frames_remaining,
                                &format!("Speed: {:.1}x", new_speed),
                            );
                        }

                        // Speed down
                        KeyCode::Char('-') => {
                            let new_speed = (effect.speed() - SPEED_STEP).clamp(0.1, 10.0);
                            effect.set_speed(new_speed);
                            set_status(
                                &mut status_message,
                                &mut status_frames_remaining,
                                &format!("Speed: {:.1}x", new_speed),
                            );
                        }

                        // Density up
                        KeyCode::Char(']') => {
                            let new_density = (effect.density() + DENSITY_STEP).clamp(0.1, 10.0);
                            effect.set_density(new_density);
                            set_status(
                                &mut status_message,
                                &mut status_frames_remaining,
                                &format!("Density: {:.1}x", new_density),
                            );
                        }

                        // Density down
                        KeyCode::Char('[') => {
                            let new_density = (effect.density() - DENSITY_STEP).clamp(0.1, 10.0);
                            effect.set_density(new_density);
                            set_status(
                                &mut status_message,
                                &mut status_frames_remaining,
                                &format!("Density: {:.1}x", new_density),
                            );
                        }

                        // Next effect
                        KeyCode::Char('n') => {
                            let next_name = registry::next_effect_name(&config.effect_name);
                            config.effect_name = next_name.to_string();
                            if let Some(new_effect) =
                                registry::create_effect(next_name, term.width, term.height, &config)
                            {
                                effect = new_effect;
                            }
                            set_status(
                                &mut status_message,
                                &mut status_frames_remaining,
                                &format!("Effect: {}", config.effect_name),
                            );
                        }

                        // Randomize
                        KeyCode::Char('r') => {
                            config = Config::randomized();
                            if let Some(new_effect) = registry::create_effect(
                                &config.effect_name,
                                term.width,
                                term.height,
                                &config,
                            ) {
                                effect = new_effect;
                            }
                            // Reset auto-cycle timer so it counts from the new effect
                            auto_cycle_elapsed = 0.0;
                            set_status(
                                &mut status_message,
                                &mut status_frames_remaining,
                                &format!(
                                    "Random: {} / {} / {:.1}x",
                                    config.effect_name,
                                    config.palette_name,
                                    config.speed_multiplier,
                                ),
                            );
                        }

                        // Toggle auto-cycle timer
                        KeyCode::Char('t') => {
                            if auto_cycle_interval.is_some() {
                                auto_cycle_enabled = !auto_cycle_enabled;
                                auto_cycle_elapsed = 0.0;
                                set_status(
                                    &mut status_message,
                                    &mut status_frames_remaining,
                                    if auto_cycle_enabled {
                                        "Auto-cycle: ON"
                                    } else {
                                        "Auto-cycle: OFF"
                                    },
                                );
                            } else {
                                set_status(
                                    &mut status_message,
                                    &mut status_frames_remaining,
                                    "Auto-cycle: use --timer to enable",
                                );
                            }
                        }

                        // Toggle help overlay
                        KeyCode::Char('?') => {
                            show_help = !show_help;
                        }

                        _ => {}
                    }
                }
            }
            Ok(None) => {}
            Err(_) => break,
        }

        if !clock.tick() {
            continue;
        }

        // Update the effect (skip when paused)
        if !paused {
            effect.update(clock.delta_time());

            // Auto-cycle: accumulate time and randomize when interval reached
            if auto_cycle_enabled
                && let Some(interval) = auto_cycle_interval
            {
                auto_cycle_elapsed += clock.delta_time();
                if auto_cycle_elapsed >= interval {
                    auto_cycle_elapsed = 0.0;
                    config = Config::randomized();
                    if let Some(new_effect) = registry::create_effect(
                        &config.effect_name,
                        term.width,
                        term.height,
                        &config,
                    ) {
                        effect = new_effect;
                    }
                    set_status(
                        &mut status_message,
                        &mut status_frames_remaining,
                        &format!(
                            "Auto: {} / {} / {:.1}x",
                            config.effect_name,
                            config.palette_name,
                            config.speed_multiplier,
                        ),
                    );
                }
            }
        }

        // Render
        buffer.clear();
        effect.render(&mut buffer);

        // Draw overlays on top of the effect
        if show_help {
            overlay::render_help(&mut buffer);
        }

        // Show status message if active
        if status_frames_remaining > 0 {
            if let Some(ref msg) = status_message {
                overlay::render_status(&mut buffer, msg);
            }
            status_frames_remaining -= 1;
        }

        if buffer.flush().is_err() {
            break;
        }
    }
}

/// Set the status message and reset the display timer.
fn set_status(message: &mut Option<String>, frames: &mut u32, text: &str) {
    *message = Some(text.to_string());
    *frames = STATUS_DISPLAY_FRAMES;
}
