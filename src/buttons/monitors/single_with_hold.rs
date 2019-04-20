//! High-level driver for a single button, with 'hold' support.

use crate::buttons::core::PollButton;
use crate::buttons::monitors::holding::{self, HoldAnnotator, HoldDescriptor};

/// An event from this module's [`Monitor`].
#[derive(Debug)]
pub enum Event {
    Click,
    Hold,
}

/// Wrapper for a single [`PollButton`] generating click and hold events.
pub struct Monitor<T: PollButton, H: HoldDescriptor> {
    button: T,
    hold_annotator: HoldAnnotator<H>,
}

impl<T: PollButton, H: HoldDescriptor> Monitor<T, H> {

    /// Takes ownership of a [`PollButton`] and returns a `Monitor`.
    pub fn new(button: T) -> Monitor<T, H> {
        Monitor {
            button,
            hold_annotator: HoldAnnotator::new(),
        }
    }

    /// Gives the underlying [`PollButton`] instance back.
    pub fn free(self) -> T {
        self.button
    }

    /// Polls the button and filters for events.
    ///
    /// Returns `Some(Hold)` if the button has been down for longer than the
    /// hold threshold.
    ///
    /// Returns `Some(Click)` if the button was released (unless the monitor
    /// has already reported a 'hold' for this press).
    ///
    /// Otherwise returns `None`.
    ///
    /// The hold threshold is determined by the monitor's [`HoldDescriptor`].
    pub fn poll(&mut self) -> Option<Event> {
        match self.hold_annotator.annotate(self.button.poll_transition()) {
            Some(holding::Event::Press) => None,
            Some(holding::Event::Release) => Some(Event::Click),
            Some(holding::Event::Hold) => Some(Event::Hold),
            None => None,
        }
    }

}

