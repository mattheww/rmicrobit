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
//! In particular, the `pub use` above makes `frame.set()` work.

#[doc(hidden)]
// Hidden from docs to prevent rustdoc choosing this as the main page
// for Frame.
pub use tiny_led_matrix::Frame as _tiny_led_matrix_frame;
