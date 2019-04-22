//! Low-level button drivers.
//!
//! This module provides an interface for reading a button's state, and an
//! implementation of that interface for a GPIO pin.
//!
//! Use this module's [`PollButton`] interface rather than the monitors if you
//! need to react to both presses and releases.
//!
//! Use this module's [`Button`] to access an externally-connected button or
//! switch, or to specify a debouncing algorithm.
//!
//! # Examples
//!
//! ```ignore
//! use microbit_blinkenlights::prelude::*;
//! use microbit_blinkenlights::gpio::PinsByKind;
//! use microbit_blinkenlights::buttons;
//! use microbit_blinkenlights::buttons::core::TransitionEvent;
//! use microbit_blinkenlights::buttons::builtin::{ButtonA, ButtonB};
//! let p: nrf51::Peripherals = _;
//! let PinsByKind {button_pins, ..} = p.GPIO.split_by_kind();
//! let (button_a, button_b) = buttons::from_pins(button_pins);
//! loop {
//!     // every 6ms
//!     match button_a.poll_event() {
//!         Some(TransitionEvent::Press) => {}
//!         Some(TransitionEvent::Release) => {}
//!         None => {}
//!     }
//!     match button_b.poll_event() {
//!         ...
//!     }
//! }
//! ```
//!
//! See `examples/use_core_buttons.rs` for a complete example.
//!
//! [`PollButton`]: crate::buttons::core::PollButton
//! [`Button`]: crate::buttons::core::Button

use crate::embedded_hal::digital::InputPin;
use crate::buttons::debouncing::Debounce;

/// Old and new button states.
///
/// When a button is polled, a returned `Transition` indicates its previous
/// and current state (which may be the same).
#[derive(Debug)]
pub struct Transition {
    pub was_pressed: bool,
    pub is_pressed: bool,
}

/// A press or release event.
#[derive(Debug)]
pub enum TransitionEvent {
    Press,
    Release,
}

/// A button which can be polled.
///
/// A `PollButton` keeps track of its state (pressed or released), updating it
/// when a `poll_` method is called.
///
/// The `poll_transition()` and `poll_event()` methods have the same effects
/// and return equivalent information; you can use whichever form is more
/// convenient.
///
/// The states reported may have had a debouncing algorithm applied to what
/// the underlying device reports.
pub trait PollButton {

    /// Reports whether the button was in pressed state when last polled.
    fn is_pressed(&self) -> bool;

    /// Polls the button and indicates its previous and current state.
    ///
    /// The underlying button is read at this point.
    fn poll_transition(&mut self) -> Transition;

    /// Polls the button and indicates any change in state.
    ///
    /// The underlying button is read at this point.
    fn poll_event(&mut self) -> Option<TransitionEvent> {
        match self.poll_transition() {
            Transition {was_pressed: false, is_pressed: true} => {
                Some(TransitionEvent::Press)
            },
            Transition {was_pressed: true, is_pressed: false} => {
                Some(TransitionEvent::Release)
            },
            _ => None,
        }
    }

}

/// A button based on a GPIO pin.
///
/// Requires an implementation of [`Debounce`] as a type parameter. Use
/// [`TrivialDebouncer`] if you don't want any debouncing.
///
/// The button behaves as if its switch was in released state before the first
/// call to a poll method, so in practice if the button is pressed when
/// `new()` is called then the first `poll_event()` will report `Press`.
///
/// [`TrivialDebouncer`]: crate::buttons::debouncing::TrivialDebouncer
pub struct Button<T: InputPin, D: Debounce> {
    pin: T,
    debouncer: D,
    pressed_state: bool,
}

impl<T: InputPin, D: Debounce> Button<T, D> {

    /// Takes ownership of a GPIO pin and returns a `Button`.
    pub fn new(pin: T) -> Button<T, D> {
        Button {pin, debouncer: D::default(), pressed_state: false}
    }

    /// Gives the underlying `InputPin` instance back.
    pub fn free(self) -> T {
        self.pin
    }

    fn update_state(&mut self) {
        self.pressed_state = self.debouncer.debounce(self.pin.is_low());
    }

}

impl<T: InputPin, D: Debounce> PollButton for Button<T, D> {

    fn is_pressed(&self) -> bool {
        self.pressed_state
    }

    fn poll_transition(&mut self) -> Transition {
        let was_pressed = self.pressed_state;
        self.update_state();
        Transition {was_pressed, is_pressed: self.pressed_state}
    }
}

