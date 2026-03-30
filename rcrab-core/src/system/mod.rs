// System utilities

use std::time::{Duration, Instant};

/// Timer for tracking time
pub struct Timer {
    start_time: Instant,
    last_time: Instant,
    frame_count: usize,
    fps_time: f32,
    current_fps: f32,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_time: now,
            frame_count: 0,
            fps_time: 0.0,
            current_fps: 0.0,
        }
    }

    /// Update the timer, returns time per frame
    pub fn update(&mut self) -> f32 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_time);
        self.last_time = now;

        self.fps_time += elapsed.as_secs_f32();
        self.frame_count += 1;

        if self.fps_time >= 1.0 {
            self.current_fps = self.frame_count as f32 / self.fps_time;
            self.frame_count = 0;
            self.fps_time = 0.0;
        }

        elapsed.as_secs_f32()
    }

    /// Get time since start in seconds
    pub fn get_time_since_start(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    /// Get current FPS
    pub fn get_fps(&self) -> f32 {
        self.current_fps
    }

    /// Get time per frame
    pub fn get_tpf(&self) -> f32 {
        if self.current_fps > 0.0 {
            1.0 / self.current_fps
        } else {
            0.0
        }
    }

    /// Get current time
    pub fn now() -> f32 {
        // This is a static method, return 0 as we don't have access to instance
        // Use get_time_since_start() on an instance instead
        0.0
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Debug utilities
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        tracing::debug!($($arg)*)
    };
}

/// Info utilities
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

/// Warn utilities
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

/// Error utilities
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}
