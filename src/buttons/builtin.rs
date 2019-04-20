//! Access to the micro:bit's built-in buttons.
//!
//! This module defines:
//!  - [`Button`] implementations for the micro:bit's buttons A and B
//!  - Monitor implementations specialised to those buttons.
//!
//! The normal way to access these is via the re-exports in the [`buttons`]
//! module.
//!
//! [`buttons`]: crate::buttons
//! [`Button`]: crate::buttons::core::Button

use microbit::hal::gpio::{Floating, Input};
use microbit::hal::gpio::gpio::{PIN17, PIN26};
use crate::gpio::ButtonPins;
use crate::buttons::core::Button;
use crate::buttons::debouncing::TrivialDebouncer;
use crate::buttons::monitors::holding::DefaultHoldDescriptor;
use crate::buttons::monitors::single;
use crate::buttons::monitors::dual;
use crate::buttons::monitors::single_with_hold;
use crate::buttons::monitors::dual_with_hold;

/// The micro:bit's 'A' (left) button, with no debouncing.
pub type ButtonA = Button<PIN17<Input<Floating>>, TrivialDebouncer>;
/// The micro:bit's 'B' (right) button, with no debouncing.
pub type ButtonB = Button<PIN26<Input<Floating>>, TrivialDebouncer>;

/// Make [`ButtonA`] and [`ButtonB`] from the GPIO pins.
pub fn from_pins(pins: ButtonPins) -> (ButtonA, ButtonB) {
    // See https://github.com/nrf-rs/nrf51-hal/issues/20
    (ButtonA::new(pins.pin17.into_floating_input()),
     ButtonB::new(pins.pin26.into_floating_input()))
}

/// Wrapper for the micro:bit's 'A' (left) button generating click events on
/// release.
pub type LazyButtonAMonitor = single::LazyMonitor<ButtonA>;
/// Wrapper for the micro:bit's 'B' (right) button generating click events on
/// release.
pub type LazyButtonBMonitor = single::LazyMonitor<ButtonB>;
/// Wrapper for the micro:bit's 'A' (left) button generating click events on
/// press.
pub type EagerButtonAMonitor = single::EagerMonitor<ButtonA>;
/// Wrapper for the micro:bit's 'B' (right) button generating click events on
/// press.
pub type EagerButtonBMonitor = single::EagerMonitor<ButtonB>;


/// Wrapper for the micro:bit's 'A' (left) button generating click and hold
/// events.
pub type ButtonAMonitorWithHold =
    single_with_hold::Monitor<ButtonA, DefaultHoldDescriptor>;
/// Wrapper for the micro:bit's 'B' (right) button generating click and hold
/// events.
pub type ButtonBMonitorWithHold =
    single_with_hold::Monitor<ButtonB, DefaultHoldDescriptor>;

/// Wrapper for the micro:bit's two buttons generating click events on
/// release.
pub type ABMonitor = dual::Monitor<ButtonA, ButtonB>;

/// Wrapper for the micro:bit's two buttons generating click and hold events.
pub type ABMonitorWithHold =
    dual_with_hold::Monitor<ButtonA, ButtonB, DefaultHoldDescriptor>;

