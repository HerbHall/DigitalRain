//! Terminal setup, teardown, and state management.
//!
//! Handles switching to the alternate screen buffer, enabling raw mode,
//! hiding the cursor, and restoring everything on exit (including panics).

use std::io;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};

/// Manages terminal state. On creation, switches to alternate screen and raw mode.
/// On drop, restores the original terminal state.
pub struct Terminal {
    /// Current terminal width in columns.
    pub width: u16,
    /// Current terminal height in rows.
    pub height: u16,
}

impl Terminal {
    /// Initialize the terminal for full-screen rendering.
    ///
    /// This enables raw mode (no line buffering, no echo), switches to the
    /// alternate screen buffer (so we don't clobber the user's scrollback),
    /// and hides the cursor.
    pub fn init() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(ClearType::All)
        )?;

        let (width, height) = terminal::size()?;

        Ok(Self { width, height })
    }

    /// Update stored dimensions. Call this when a resize event is detected.
    pub fn update_size(&mut self) -> io::Result<()> {
        let (width, height) = terminal::size()?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    /// Poll for a terminal event with a timeout.
    /// Returns `None` if no event occurred within the timeout.
    pub fn poll_event(&self, timeout: std::time::Duration) -> io::Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    /// Check if the user pressed 'q', Escape, or Ctrl+C.
    pub fn should_quit(event: &Event) -> bool {
        matches!(
            event,
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) | Event::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            }) | Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            })
        )
    }

}

impl Drop for Terminal {
    /// Restore the terminal to its original state.
    /// This runs even if the program panics, as long as the Terminal is in scope.
    fn drop(&mut self) {
        // Best-effort cleanup — ignore errors since we're in Drop
        let _ = execute!(io::stdout(), cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}
