//! Effect system: defines the Effect trait and manages effect selection.
//!
//! Each visual effect implements the Effect trait. The main loop calls
//! update() and render() on the active effect each frame.

pub mod classic;

use crate::buffer::ScreenBuffer;

/// The core trait that all visual effects implement.
///
/// The main loop calls `update()` with the time delta, then `render()`
/// to draw into the screen buffer.
pub trait Effect {
    /// Human-readable name for display and CLI selection.
    fn name(&self) -> &str;

    /// Advance the effect's state by one frame.
    /// `delta_time` is seconds since the last frame.
    fn update(&mut self, delta_time: f64);

    /// Draw the current state into the screen buffer.
    fn render(&self, buffer: &mut ScreenBuffer);

    /// Handle a terminal resize.
    fn resize(&mut self, width: u16, height: u16);
}
