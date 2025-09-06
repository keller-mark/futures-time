use std::task::{Context, Poll};
use std::future::{Future};
use std::pin::{Pin};
use crate::time::{Duration, Instant};

use gloo_timers::callback::Timeout;

// We need to wrap the gloo_timer in an async_io-compatible Timer
// Reference: https://github.com/smol-rs/async-io/blob/master/src/lib.rs
pub struct Timer {
    /// This timer's ID and last waker that polled it.
    ///
    /// When this field is set to `None`, this timer is not registered in the reactor.
    id: Option<usize>,

    /// The next instant at which this timer fires.
    ///
    /// If this timer is a blank timer, this value is None. If the timer
    /// must be set, this value contains the next instant at which the
    /// timer must fire.
    when: Option<Instant>,

    /// The period.
    period: Duration,
}


impl Timer {

    /// Creates a timer that emits an event once after the given duration of time.
    pub fn after(duration: Duration) -> Timer {
        
    }

    /// Creates a timer that emits an event once at the given time instant.
    pub fn at(instant: Instant) -> Timer {
        
    }

    /// Sets the timer to emit an event once after the given duration of time.
    /// ```
    pub fn set_after(&mut self, duration: Duration) {
        
    }

}


impl Future for Timer {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();


    }
}
