//! Algorithms for debouncing buttons.
//!
//! These algorithms assume the button is polled at a regular interval.
//!
//! With a 6ms poll interval, the micro:bit's built-in buttons seem to perform
//! accurately with no additional debouncing.

/// A debouncing algorithm and an associated state.
pub trait Debounce: Default {

    /// Accepts new polled data and returns the 'debounced' state.
    ///
    /// `pressed_state` `true` indicates that the button is closed.
    fn debounce(&mut self, pressed_state: bool) -> bool;
}

/// A 'debouncer' which returns the most recent polled state unchanged.
pub struct TrivialDebouncer ();

impl Default for TrivialDebouncer {
    fn default() -> TrivialDebouncer {
        TrivialDebouncer()
    }
}


impl Debounce for TrivialDebouncer {

    fn debounce(&mut self, pressed_state: bool) -> bool {
        pressed_state
    }

}

/// A debouncer based on net open/closed counts, with saturation.
///
/// This is intended to be the same as the algorithm used in the [micro:bit
/// runtime][dal] (as of version 2.1), when used with a 6ms polling interval.
///
/// The documentation suggests that this algorithm is suitable for use with
/// touch-sensing input as well as the buttons.
///
/// [dal]: https://lancaster-university.github.io/microbit-docs/
pub struct CountingDebouncer {
    pressed_state: bool,
    count: u8,
}

const SIGMA_MIN : u8 = 0;
const SIGMA_MAX : u8 = 12;
const SIGMA_LOW_THRESHOLD : u8 = 2;
const SIGMA_HIGH_THRESHOLD : u8 = 8;

impl Default for CountingDebouncer {
    fn default() -> CountingDebouncer {
        CountingDebouncer{pressed_state: false, count: SIGMA_MIN}
    }
}

impl Debounce for CountingDebouncer {

    fn debounce(&mut self, pressed_state: bool) -> bool {
        if pressed_state {
            if self.count != SIGMA_MAX {
                self.count += 1;
            }
            if self.count > SIGMA_HIGH_THRESHOLD {
                self.pressed_state = true;
            }
        } else {
            if self.count != SIGMA_MIN {
                self.count -= 1;
            }
            if self.count < SIGMA_LOW_THRESHOLD {
                self.pressed_state = false;
            }
        };
        self.pressed_state
    }

}

