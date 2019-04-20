//! High-level driver for a single button.

use crate::buttons::core::{PollButton, TransitionEvent};

/// An event from one of this module's monitors.
#[derive(Debug)]
pub enum Event {
    Click,
}

/// Wrapper for a single [`PollButton`] generating click events on release.
pub struct LazyMonitor<T: PollButton> {
    button: T,
}

impl<T: PollButton> LazyMonitor<T> {

    /// Takes ownership of a [`PollButton`] and returns a `LazyMonitor`.
    pub fn new(button: T) -> LazyMonitor<T> {
        LazyMonitor {button}
    }

    /// Gives the underlying [`PollButton`] instance back.
    pub fn free(self) -> T {
        self.button
    }

    /// Polls the button and filters for events.
    ///
    /// Returns `Some(Click)` if the button was released, otherwise `None`.
    pub fn poll(&mut self) -> Option<Event> {
        match self.button.poll_event() {
            Some(TransitionEvent::Press) => None,
            Some(TransitionEvent::Release) => Some(Event::Click),
            None => None,
        }
    }

}


/// Wrapper for a single [`PollButton`] generating click events on press.
pub struct EagerMonitor<T: PollButton> {
    button: T,
}

impl<T: PollButton> EagerMonitor<T> {

    /// Takes ownership of a [`PollButton`] and returns an `EagerMonitor`.
    pub fn new(button: T) -> EagerMonitor<T> {
        EagerMonitor {button}
    }

    /// Gives the underlying [`PollButton`] instance back.
    pub fn free(self) -> T {
        self.button
    }

    /// Polls the button and filters for events.
    ///
    /// Returns `Some(Click)` if the button was pressed, otherwise `None`.
    pub fn poll(&mut self) -> Option<Event> {
        match self.button.poll_event() {
            Some(TransitionEvent::Press) => Some(Event::Click),
            Some(TransitionEvent::Release) => None,
            None => None,
        }
    }

}

