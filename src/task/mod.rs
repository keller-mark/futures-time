//! Types and Traits for working with asynchronous tasks.

#[cfg(target_arch = "wasm32")]
pub mod web_timer;

mod sleep;
mod sleep_until;

pub use sleep::{sleep, Sleep};
pub use sleep_until::{sleep_until, SleepUntil};
