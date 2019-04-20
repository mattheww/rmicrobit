//! High-level driver for two buttons together.

use crate::buttons::core::{PollButton, TransitionEvent};

/// An event from this module's [`Monitor`].
#[derive(PartialEq, Eq, Debug)]
pub enum Event {
    ClickA,
    ClickB,
    ClickAB,
}

// The event-generation rules and internal state for Monitor.
//
// Promises that consecutive calls to handle() for different buttons don't
// both return Some.
struct MonitorState {
    seen_both: bool,
}

impl MonitorState {
    fn new() -> MonitorState {
        MonitorState {
            seen_both: false,
        }
    }

    // Handles a single poll result.
    //
    // The event parameter is the event being processed.
    //
    // The click_event parameter is the event to generate for this button, if
    // this is a release in a single-button 'transaction'.
    fn handle(
        &mut self,
        event: TransitionEvent,
        other_button_is_pressed: bool,
        click_event: Event,
    ) -> Option<Event> {
        match event {
            TransitionEvent::Press => {
                self.seen_both = other_button_is_pressed;
            }
            TransitionEvent::Release => {
                if self.seen_both {
                    if !other_button_is_pressed {
                        return Some(Event::ClickAB);
                    }
                } else {
                    return Some(click_event);
                }
            }
        };
        None
    }
}


/// Wrapper for two [`PollButton`]s generating click events on release.
///
/// The buttons don't have to be the micro:bit's built-in buttons, though the
/// generated [`Event`]s include 'A' and 'B' in their names.
pub struct Monitor<A: PollButton, B: PollButton> {
    button_a: A,
    button_b: B,
    state: MonitorState,
}

impl<A: PollButton, B: PollButton> Monitor<A, B> {
    /// Takes ownership of two [`PollButton`]s and returns a `Monitor`.
    pub fn new(button_a: A, button_b: B) -> Monitor<A, B> {
        Monitor {
            button_a,
            button_b,
            state: MonitorState::new(),
        }
    }

    /// Gives the underlying [`PollButton`] instances back.
    pub fn free(self) -> (A, B) {
        (self.button_a, self.button_b)
    }

    fn poll_a(&mut self) -> Option<Event> {
        self.button_a.poll_event()
            .and_then(|event| {
                self.state.handle(
                    event,
                    self.button_b.is_pressed(),
                    Event::ClickA,
                )})
    }

    fn poll_b(&mut self) -> Option<Event> {
        self.button_b.poll_event()
            .and_then(|event| {
                self.state.handle(
                    event,
                    self.button_a.is_pressed(),
                    Event::ClickB,
                )})
    }

    /// Polls both buttons and filters for events.
    ///
    /// If both buttons have been pressed, returns `Some(ClickAB)` when the
    /// second one is released.
    ///
    /// Otherwise, returns `Some(ClickA)` if the first button was released or
    /// `Some(ClickB)` if the second button was released.
    ///
    /// Otherwise returns `None`.
    pub fn poll(&mut self) -> Option<Event> {
        let event_a = self.poll_a();
        let event_b = self.poll_b();
        // MonitorState promises this
        assert!(event_a.is_none() || event_b.is_none());
        event_a.or(event_b)
    }

}

