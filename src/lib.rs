//! # Embassy `embedded-time`
//!
//! This library provides an [embedded_time::Clock] that can be used with [embassy].
//!
//! The provided [embedded_time::Clock] implementation is based on [embassy_time].
//!
//! # Usage
//!
//! ```rust
//! use embassy_embedded_time::EmbassyClock;
//! use embedded_time::Clock;
//!
//! let clock = EmbassyClock::default();
//!
//! let now = clock.try_now().unwrap();
//! println!("Current time: {:?}", now);
//!
//! ```

#![no_std]

use embedded_time::clock::Error;
use embedded_time::duration::Fraction;
use embedded_time::{Clock, Instant};

/// A clock with at maximum microsecond precision.
/// The actual precision depends on the tick rate configured for the underlying `embassy_time` clock.
///
/// To construct a clock, use [EmbassyClock::default()].
///
/// The clock is "started" when it is constructed.
///
/// # Limitations
/// The clock represents up to ~584542 years worth of time, after which it will roll over.
#[derive(Copy, Clone, Debug)]
pub struct EmbassyClock {
    start: embassy_time::Instant,
}

impl Default for EmbassyClock {
    fn default() -> Self {
        EmbassyClock {
            start: embassy_time::Instant::now(),
        }
    }
}

impl Clock for EmbassyClock {
    /// With a 64-bit tick register, the clock can represent times up to approximately 584542 years in
    /// duration, after which the clock will roll over.
    type T = u64;

    /// Each tick of the clock is equivalent to 1 microsecond.
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);

    /// Get the current time from the clock.
    fn try_now(&self) -> Result<Instant<Self>, Error> {
        let now = embassy_time::Instant::now();

        let elapsed = now.duration_since(self.start);

        Ok(Instant::new(elapsed.as_micros()))
    }
}
