//! Support for detecting button 'hold' events.
//!
//! This is part of the implementation of the [`single_with_hold`] and
//! [`dual_with_hold`] button monitors, public so that it's possible to make
//! variants with different timings.

use core::ops::AddAssign;
use crate::buttons::core::Transition;

/// Description of the number of ticks to treat as a 'hold'.
pub trait HoldDescriptor: {
    /// Integer type wide enough to hold the tick count
    type width: PartialOrd + AddAssign;
    /// Zero of the `width` type
    const HOLD_START: Self::width;
    /// One of the `width` type
    const HOLD_INCREMENT: Self::width;
    /// The number of ticks
    const HOLD_TICKS: Self::width;
}

/// The default `HoldDescriptor`.
///
/// Represents 250 ticks (which is 1.5s for 6ms ticks).
pub struct DefaultHoldDescriptor ();

impl HoldDescriptor for DefaultHoldDescriptor {
    type width = u8;
    const HOLD_START: u8 = 0;
    const HOLD_INCREMENT: u8 = 1;
    const HOLD_TICKS: u8 = 250;
}



/// Variant of [`TransitionEvent`] with an additional `Hold`
/// event.
///
/// [`TransitionEvent`]: crate::buttons::core::TransitionEvent
#[derive(Debug)]
pub enum Event {
    Press,
    Release,
    Hold,
}

/// A hold-detection algorithm and associated state.
#[derive(Debug)]
pub struct HoldAnnotator<T: HoldDescriptor> {
    counter: T::width,
}

impl<T: HoldDescriptor> HoldAnnotator<T> {

    /// Returns a new `HoldAnnotator`.
    pub fn new() -> HoldAnnotator<T> {
        HoldAnnotator { counter: T::HOLD_START }
    }

    /// Convert the result of a button poll to an event.
    ///
    /// Returns [events] similar to those from [`PollButton::poll_event`], but
    /// with `Hold` as possibility as well as `Press` and `Release`.
    ///
    /// If the button has been down for longer than `HOLD_TICKS`, immediately
    /// reports `Hold`, and reports no event when the button is next released.
    ///
    /// See [`DefaultHoldDescriptor`] for the default `HOLD_TICKS`.
    ///
    /// # Example
    /// ```ignore
    /// match hold_annotator.annotate(button.poll_transition()) {
    ///     Some(holding::Event::Press) => ...,
    ///     Some(holding::Event::Release) => ...,
    ///     Some(holding::Event::Hold) => ...,
    ///     None => ...,
    /// }
    /// ```
    /// [events]: Event
    /// [`PollButton::poll_event`]: crate::buttons::core::PollButton::poll_event
    pub fn annotate(&mut self, transition: Transition)
                    -> Option<Event> {
        match transition {
            Transition {was_pressed: false, is_pressed: true} => {
                self.counter = T::HOLD_START;
                Some(Event::Press)
            },
            Transition {was_pressed: true, is_pressed: false} => {
                Some(Event::Release)
            },
            Transition {was_pressed: true, is_pressed: true} => {
                if self.counter <= T::HOLD_TICKS {
                    self.counter += T::HOLD_INCREMENT;
                }
                if self.counter == T::HOLD_TICKS {
                    Some(Event::Hold)
                } else {
                    None
                }
            },
            Transition {was_pressed: false, is_pressed: false} => None,
        }
    }

}

