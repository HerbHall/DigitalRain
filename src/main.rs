//! DigitalRain - Terminal-based Matrix digital rain effect.
//!
//! A visual effects application that renders the iconic Matrix falling
//! characters in your terminal. Built with Rust and crossterm for
//! cross-platform compatibility (Windows-first).

mod buffer;
mod color;
mod effects;
mod rain;
mod terminal;
mod timing;

use crossterm::event::Event;

use buffer::ScreenBuffer;
use effects::Effect;
use effects::classic::ClassicRain;
use terminal::Terminal;
use timing::FrameClock;

fn main() {
    // Initialize the terminal (alternate screen, raw mode, hidden cursor).
    // If this fails, we can't do anything, so unwrap is appropriate.
    let mut term = Terminal::init().expect("Failed to initialize terminal");

    // Create the screen buffer matching the terminal dimensions
    let mut buffer = ScreenBuffer::new(term.width, term.height);

    // Create the frame clock targeting 30 FPS
    let mut clock = FrameClock::new(30);

    // Create the classic rain effect
    let mut effect: Box<dyn Effect> = Box::new(ClassicRain::new(term.width, term.height));

    // Main loop: poll events, update, render
    loop {
        // Poll for input events (non-blocking, waits only until next frame is due)
        match term.poll_event(clock.poll_timeout()) {
            Ok(Some(event)) => {
                // Check for quit
                if Terminal::should_quit(&event) {
                    break;
                }

                // Handle terminal resize
                if let Event::Resize(_, _) = event {
                    term.update_size().ok();
                    buffer.resize(term.width, term.height);
                    effect.resize(term.width, term.height);
                }
            }
            Ok(None) => {}   // No event, just continue to next frame
            Err(_) => break, // Terminal error, exit
        }

        // Check if it's time for a new frame
        if !clock.tick() {
            continue;
        }

        // Update the effect state
        effect.update(clock.delta_time());

        // Clear the buffer, render the effect, then flush to terminal
        buffer.clear();
        effect.render(&mut buffer);
        if buffer.flush().is_err() {
            break; // Terminal write error, exit
        }
    }

    // Terminal cleanup happens automatically when `term` is dropped
}
