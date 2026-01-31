//! Frame timing and FPS control.
//!
//! Provides a simple frame clock that tracks delta time between frames
//! and sleeps to maintain a target frame rate.

use std::time::{Duration, Instant};

/// Controls frame timing for the main loop.
pub struct FrameClock {
    /// Target time per frame (e.g., 33ms for 30fps)
    target_frame_time: Duration,
    /// When the last frame started
    last_frame: Instant,
    /// Time elapsed since the last frame (in seconds)
    delta_time: f64,
}

impl FrameClock {
    /// Create a new FrameClock targeting the given FPS.
    pub fn new(target_fps: u32) -> Self {
        Self {
            target_frame_time: Duration::from_secs_f64(1.0 / target_fps as f64),
            last_frame: Instant::now(),
            delta_time: 0.0,
        }
    }

    /// How long to wait when polling for events.
    /// This is the remaining time until the next frame is due.
    pub fn poll_timeout(&self) -> Duration {
        let elapsed = self.last_frame.elapsed();
        self.target_frame_time.saturating_sub(elapsed)
    }

    /// Call this at the start of each frame. Returns true if enough time
    /// has passed for a new frame (i.e., we've reached the target frame time).
    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame);

        if elapsed >= self.target_frame_time {
            self.delta_time = elapsed.as_secs_f64();
            self.last_frame = now;
            true
        } else {
            false
        }
    }

    /// Time in seconds since the last frame. Use this for animation calculations
    /// so that effects run at the same visual speed regardless of frame rate.
    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}
