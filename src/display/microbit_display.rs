//! Wrapper for a Display, its GPIO pins, and a timer.

use microbit::hal::hi_res_timer::As16BitTimer;
use tiny_led_matrix::{Display, Event as DisplayEvent};
use crate::display::display_port::DisplayPort;
use crate::display::matrix::MicrobitFrame;
use crate::display::timer::MicrobitDisplayTimer;

/// The micro:bit's display, and one timer to drive it.
pub struct MicrobitDisplay<T: As16BitTimer> {
    timer: MicrobitDisplayTimer<T>,
    port: DisplayPort,
    display: Display<MicrobitFrame>,
}

impl<T: As16BitTimer> MicrobitDisplay<T> {

    /// Takes ownership of the display port and one TIMER, and returns a
    /// `MicrobitDisplay`.
    ///
    /// The `timer` parameter can be any of the three `nrf51::TIMER`*n*
    /// peripherals.
    ///
    /// Initialises the micro:bit hardware to use the display driver.
    ///
    /// The display is initially clear.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use microbit_blinkenlights::prelude::*;
    /// use microbit_blinkenlights::gpio::PinsByKind;
    /// use microbit_blinkenlights::display::{DisplayPort, MicrobitDisplay};
    /// let p: nrf51::Peripherals = _;
    /// let PinsByKind {display_pins, ..} = p.GPIO.split_by_kind();
    /// let display_port = DisplayPort::new(display_pins);
    /// let mut display = MicrobitDisplay::new(display_port, p.TIMER1);
    /// ```
    pub fn new(mut port: DisplayPort, timer: T) -> MicrobitDisplay<T> {
        let mut timer = MicrobitDisplayTimer::new(timer);
        tiny_led_matrix::initialise_control(&mut port);
        tiny_led_matrix::initialise_timer(&mut timer);
        let display = Display::new();
        MicrobitDisplay {timer, port, display}
    }

    /// Gives the underlying devices back.
    ///
    /// Returns the `DisplayPort` and `nrf51::TIMER`*n* instance.
    ///
    /// Stops the TIMER.
    pub fn free(self) -> (DisplayPort, T) {
        (self.port, self.timer.free())
    }

    /// Updates the LEDs and timer state during a timer interrupt.
    ///
    /// Call this in an interrupt handler for the `MicrobitDisplay`'s timer.
    ///
    /// See [`Display::handle_event()`] for details.
    ///
    /// Returns a [`DisplayEvent`] indicating the reason for the interrupt.
    /// You can check this if you wish to perform some other action once every
    /// 6ms.
    ///
    /// # Example
    ///
    /// In the style of `cortex-m-rtfm` v0.4:
    ///
    /// ```ignore
    /// #[interrupt(priority = 2, resources = [DISPLAY])]
    /// fn TIMER1() {
    ///     let display_event = resources.DISPLAY.handle_event();
    ///     if display_event.is_new_row() {
    ///         ...
    ///     }
    /// }
    /// ```
    pub fn handle_event(&mut self) -> DisplayEvent {
        self.display.handle_event(&mut self.timer, &mut self.port)
    }

    /// Accepts a new image to be displayed.
    ///
    /// The code that calls this method must not be interrupting, or
    /// interruptable by, [`handle_event()`].
    ///
    /// After calling this, it's safe to modify the frame again (its data is
    /// copied).
    ///
    /// # Example
    ///
    /// In the style of `cortex-m-rtfm` v0.4:
    ///
    /// ```ignore
    /// #[interrupt(priority = 1, resources = [RTC0, DISPLAY])]
    /// fn RTC0() {
    ///     static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
    ///     &resources.RTC0.clear_tick_event();
    ///     FRAME.set(GreyscaleImage::blank());
    ///     resources.DISPLAY.lock(|display| {
    ///         display.set_frame(FRAME);
    ///     });
    /// }
    /// ```
    ///
    /// [`handle_event()`]: MicrobitDisplay::handle_event
    pub fn set_frame(&mut self, frame: &MicrobitFrame) {
        self.display.set_frame(frame);
    }

}

