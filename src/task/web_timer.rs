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
}


impl Timer {

    /// Creates a timer that emits an event once after the given duration of time.
    pub fn after(duration: Duration) -> Timer {
        Timer {
            timer: None,
            duration,
        }
    }

    /// Sets the timer to emit an event once after the given duration of time.
    pub fn set_after(&mut self, duration: Duration) {
        self.duration = duration;
        // Invalidate the existing timer so it's recreated on the next poll.
        self.timer = None;
    }

    /// Creates a timer that emits an event once at the given instant in time.
    pub fn at(instant: Instant) -> Timer {
        Timer {
            timer: None,
            duration: instant.duration_since(*Instant::now()).into(),
        }
    }

}


impl Future for Timer {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match this.timer.as_mut() {
            Some(_) => {
                // The timer has been set and has now fired.
                Poll::Ready(Instant::now())
            }
            None => {
                // The timer has not been set yet, so set it.
                let waker = cx.waker().clone();
                let timeout = Timeout::new(this.duration.as_millis() as u32, move || {
                    waker.wake()
                });
                this.timer = Some(timeout);
                Poll::Pending
            }
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