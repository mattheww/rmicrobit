//! High-level driver for two buttons together, with 'hold' support.

use crate::buttons::core::PollButton;
use crate::buttons::monitors::holding::{
    Event as SingleButtonEvent,
    HoldAnnotator,
    HoldDescriptor,
};

#[cfg(test)]
#[path = "../../../unit_tests/buttons/dual_monitor_with_hold_tests.rs"]
mod tests;

/// An event from this module's [`Monitor`].
#[derive(PartialEq, Eq, Debug)]
pub enum Event {
    ClickA,
    ClickB,
    ClickAB,
    HoldA,
    HoldB,
    HoldAB,
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    SeenOne,
    SeenBoth,
    HeldASeenB,
    HeldBSeenA,
    ReportedHold,
}

trait ButtonDescriptor {
    const CLICK_EVENT: Event;
    const HOLD_EVENT: Event;
    const HELD_STATE: State;
    const OTHER_HELD_STATE: State;
}

struct ButtonADescriptor();
struct ButtonBDescriptor();

impl ButtonDescriptor for ButtonADescriptor {
    const CLICK_EVENT: Event = Event::ClickA;
    const HOLD_EVENT: Event = Event::HoldA;
    const HELD_STATE: State = State::HeldASeenB;
    const OTHER_HELD_STATE: State = State::HeldBSeenA;
}

impl ButtonDescriptor for ButtonBDescriptor {
    const CLICK_EVENT: Event = Event::ClickB;
    const HOLD_EVENT: Event = Event::HoldB;
    const HELD_STATE: State = State::HeldBSeenA;
    const OTHER_HELD_STATE: State = State::HeldASeenB;
}

// The event-generation rules and internal state for Monitor.
//
// Promises that consecutive calls to handle_a() and handle_b() don't both
// return Some.
//
// Promises that each 'transaction' generates exactly one event (that is, each
// sequence of presses and releases between states where neither button is
// pressed).
struct MonitorState {
    state: State,
}

impl MonitorState {
    fn new() -> MonitorState {
        MonitorState {
            // the initial state doesn't matter
            state: State::ReportedHold,
        }
    }

    fn handle<T: ButtonDescriptor>(
        &mut self,
        event: SingleButtonEvent,
        other_button_is_pressed: bool,
    ) -> Option<Event> {
        match event {
            SingleButtonEvent::Press if !other_button_is_pressed => {
                // all transactions start here
                self.state = State::SeenOne;
            }
            SingleButtonEvent::Press if self.state == State::SeenOne => {
                self.state = State::SeenBoth;
            }
            SingleButtonEvent::Release if self.state == State::SeenOne => {
                // this transaction ends here
                return Some(T::CLICK_EVENT);
            }
            SingleButtonEvent::Release if self.state != State::ReportedHold => {
                if !other_button_is_pressed {
                    // this transaction ends here
                    return Some(Event::ClickAB);
                }
            }
            SingleButtonEvent::Hold if self.state == State::SeenOne => {
                self.state = State::ReportedHold;
                return Some(T::HOLD_EVENT);
            }
            SingleButtonEvent::Hold if self.state == State::SeenBoth => {
                self.state = T::HELD_STATE;
            }
            SingleButtonEvent::Hold if self.state == T::OTHER_HELD_STATE => {
                self.state = State::ReportedHold;
                return Some(Event::HoldAB);
            }
            _ => {}
        };

        None
    }

    fn handle_a(
        &mut self,
        event: SingleButtonEvent,
        other_button_is_pressed: bool,
    ) -> Option<Event> {
        self.handle::<ButtonADescriptor>(event, other_button_is_pressed)
    }

    fn handle_b(
        &mut self,
        event: SingleButtonEvent,
        other_button_is_pressed: bool,
    ) -> Option<Event> {
        self.handle::<ButtonBDescriptor>(event, other_button_is_pressed)
    }
}

/// Wrapper for two [`PollButton`]s generating click and hold events.
///
/// The buttons don't have to be the micro:bit's built-in buttons, though the
/// generated [`Event`]s include 'A' and 'B' in their names.
pub struct Monitor<A: PollButton, B: PollButton, H: HoldDescriptor> {
    button_a: A,
    button_b: B,
    hold_annotator_a: HoldAnnotator<H>,
    hold_annotator_b: HoldAnnotator<H>,
    state: MonitorState,
}

impl<A: PollButton, B: PollButton, H: HoldDescriptor> Monitor<A, B, H> {
    /// Takes ownership of two [`PollButton`]s and returns a `Monitor`.
    pub fn new(button_a: A, button_b: B) -> Monitor<A, B, H> {
        Monitor {
            button_a,
            button_b,
            hold_annotator_a: HoldAnnotator::new(),
            hold_annotator_b: HoldAnnotator::new(),
            state: MonitorState::new(),
        }
    }

    /// Gives the underlying [`PollButton`] instances back.
    pub fn free(self) -> (A, B) {
        (self.button_a, self.button_b)
    }

    fn poll_a(&mut self) -> Option<Event> {
        self.hold_annotator_a.annotate(self.button_a.poll_transition())
            .and_then(|event| {
            self.state.handle_a(event, self.button_b.is_pressed())
        })
    }

    fn poll_b(&mut self) -> Option<Event> {
        self.hold_annotator_b.annotate(self.button_b.poll_transition())
            .and_then(|event| {
            self.state.handle_b(event, self.button_a.is_pressed())
        })
    }

    /// Polls both buttons and filters for events.
    ///
    /// If one button has been down for longer than the hold threshold and the
    /// other button hasn't been pressed, returns `Some(HoldA)` or
    /// `Some(HoldB)`.
    ///
    /// If both buttons have been held down for longer than the hold
    /// threshold, returns `Some(HoldAB)`.
    ///
    /// Otherwise, if both buttons have been pressed, returns `Some(ClickAB)`
    /// when the second one is released.
    ///
    /// Otherwise, returns `Some(ClickA)` if the first button was released or
    /// `Some(ClickB)` if the second button was released.
    ///
    /// Otherwise returns `None`.
    ///
    /// Once a hold event has been reported, doesn't report any further events
    /// until after both buttons have been released.
    ///
    /// The hold threshold is determined by the monitor's [`HoldDescriptor`].
    pub fn poll(&mut self) -> Option<Event> {
        let event_a = self.poll_a();
        let event_b = self.poll_b();
        // MonitorState promises this
        assert!(event_a.is_none() || event_b.is_none());
        event_a.or(event_b)
    }

}

