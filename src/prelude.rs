//! Traits intended to be globally imported.
//!
//! This module is intended to be used as follows:
//! ```
//! use microbit_blinkenlights::prelude::*
//! ```
//!
//! It exports some of this crate's traits under 'safe' names, so that their
//! methods become available without otherwise polluting the global namespace.
//!
//! The `pub use` above provides:
//!
//! Trait                   | Example |
//! ----------------------- | ------- |
//! [`Frame`]               | `frame.set()` |
//! [`MicrobitGpioExt`]     | `GPIO.split_by_kind()` |
//!
//! [`MicrobitGpioExt`]: crate::gpio::MicrobitGpioExt
//! [`Frame`]: tiny_led_matrix::Frame

// I'm hiding these from rustdoc to prevent it choosing some of them as the
// main page for the traits (eg for Frame). It seems least misleading to hide
// all of them.

#[doc(hidden)]
pub use tiny_led_matrix::Frame as _tiny_led_matrix_frame;

#[doc(hidden)]
pub use crate::gpio::MicrobitGpioExt as _mb_gpio_microbit_gpio_ext;
