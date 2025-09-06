use std::task::{Context, Poll};
use std::future::{Future};
use std::pin::{Pin};
use crate::time::{Duration, Instant};

use gloo_timers::callback::Timeout;

// We need to wrap the gloo_timer in an async_io-compatible Timer
// Reference: https://github.com/smol-rs/async-io/blob/master/src/lib.rs
#[derive(Debug)]
pub struct Timer {
    /// The underlying timer.
    timer: Option<Timeout>,

    /// The duration.
    duration: Duration,

    /// Whether the timer has fired.
    fired: bool,
}


impl Timer {

    /// Creates a timer that emits an event once after the given duration of time.
    pub fn after(duration: Duration) -> Timer {
        Timer {
            timer: None,
            duration,
            fired: false,
        }
    }

    /// Sets the timer to emit an event once after the given duration of time.
    pub fn set_after(&mut self, duration: Duration) {
        self.duration = duration;
        // Invalidate the existing timer so it's recreated on the next poll.
        self.timer = None;
        self.fired = false;
    }

    /// Creates a timer that emits an event once at the given instant in time.
    pub fn at(instant: Instant) -> Timer {
        Timer {
            timer: None,
            duration: instant.duration_since(*Instant::now()).into(),
            fired: false, // TODO: check against Instant.now to see if at is in the past...
        }
    }

}


impl Future for Timer {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.fired {
            return Poll::Ready(Instant::now());
        }

        if this.timer.is_none() {
            // The timer has not been set yet, so set it.
            let waker = cx.waker().clone();
            let timeout = Timeout::new(this.duration.as_millis() as u32, move || {
                // The waker will be dropped when the closure is called.
                // We don't need to manually set `fired` here because the waker
                // is moved and will be dropped, which is our signal.
                waker.wake();
            });
            this.timer = Some(timeout);
        }

        // Check if the waker from the previous poll is the same as the current one.
        // If the waker is gone, it means the timer has fired and dropped it.
        if cx.waker().will_wake(cx.waker()) {
            Poll::Pending
        } else {
            this.fired = true;
            Poll::Ready(Instant::now())
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if let Some(timeout) = self.timer.take() {
            // The `Drop` implementation of `gloo_timers::callback::Timeout`
            // will handle cleaning up the browser timer.
            drop(timeout);
        }
    }
}