//! Traits intended to be globally imported.
//!
//! This module is intended to be used as follows:
//! ```
//! use rmicrobit::prelude::*;
//! ```
//!
//! It exports some of this crate's traits without binding them to names, so
//! that their methods become available without otherwise polluting the global
//! namespace.
//!
//! The `pub use` above provides:
//!
//! Trait                   | Example |
//! ----------------------- | ------- |
//! [`Frame`]               | `frame.set()` |
//! [`MicrobitGpioExt`]     | `GPIO.split_by_kind()` |
//! [`PollButton`]          | `button_a.poll_event()` |
//! [`Animate`]             | `scrolling_images.tick()` |
//!
//! [`MicrobitGpioExt`]: crate::gpio::MicrobitGpioExt
//! [`PollButton`]: crate::buttons::core::PollButton
//! [`Frame`]: tiny_led_matrix::Frame
//! [`Animate`]: crate::graphics::scrolling::Animate

// I'm hiding these from rustdoc to prevent it choosing some of them as the
// main page for the traits (eg for Frame). It seems least misleading to hide
// all of them.

#[doc(hidden)]
pub use tiny_led_matrix::Frame as _;

#[doc(hidden)]
pub use crate::gpio::MicrobitGpioExt as _;

#[doc(hidden)]
pub use crate::buttons::core::PollButton as _;

#[doc(hidden)]
pub use crate::graphics::scrolling::Animate as _;
