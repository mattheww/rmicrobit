#![no_std]

//! A library for controlling the [micro:bit](https://microbit.org/) 5×5 LED
//! display.
//!
//! # Scope
//!
//! Together with `tiny-led-matrix`, this crate provides:
//! - support for driving the LED display from a timer interrupt;
//! - ten levels of brightness for each LED;
//! - simple 5×5 greyscale and black-and-white image types;
//! - a copy of the 'pendolino' font from the [micro:bit runtime][dal];
//! - support for scrolling text.
//!
//! The crate doesn't define interrupt handlers directly; instead it provides
//! a function to be called from a timer interrupt. It knows how to program
//! one of the micro:bit's timers to provide that interrupt.
//!
//! # Demo
//!
//! `examples/demo` demonstrates all the features of this crate, using
//! the [cortex-m-rtfm] framework.
//!
//! # Coordinate system
//!
//! The LEDs are identified using (x,y) coordinates as follows:
//!
//! ```text
//! (0,0) ... (4,0)
//!  ...  ...  ...
//! (4,0) ... (4,4)
//! ```
//!
//! # Greyscale model
//!
//! LED brightness levels are described using a scale from 0 (off) to 9
//! (brightest) inclusive.
//!
//! These are converted to time slices using the same timings as used by the
//! [micro:bit MicroPython port][micropython] (this is different to the 0 to
//! 255 scale used by the [micro:bit runtime][dal]).
//!
//! The time slice for each level above 1 is approximately 1.9× the slice for
//! the previous level.
//!
//! An LED with brightness 9 is lit for one third of the time (because
//! internally there are three 'rows' of LEDs which have to be addressed one
//! at a time).
//!
//! # Images and Render
//!
//! The [`Render`] trait defines the interface that an image-like type needs
//! to provide in order to be displayed.
//!
//! It contains a single function: [`brightness_at(x,
//! y)`][Render::brightness_at], returning a brightness level.
//!
//! The [`image`] module provides two static image types implementing `Render`:
//! - [`GreyscaleImage`], allowing all 9 levels (using one byte for each LED)
//! - [`BitImage`], allowing only 'on' and 'off' (using five bytes)
//!
//! # Display
//!
//! A [`Display`] instance controls the LEDs and programs a timer. There
//! should normally be a single `Display` instance in the program.
//!
//! # Frames
//!
//! Types implementing [`Render`] aren't used directly with the [`Display`];
//! instead they're used to update a [`MicrobitFrame`] instance which is in
//! turn passed to the `Display`.
//!
//! A `MicrobitFrame` instance is a 'compiled' representation of a 5×5
//! greyscale image, in a form that's more directly usable by the display
//! code.
//!
//! This is exposed in the public API so that you can construct the
//! `MicrobitFrame` representation in code running at a low priority. Then
//! only [`Display::set_frame()`] has to be called in code that can't be
//! interrupted by the display timer.
//!
//! # Timer integration
//!
//! The `Display` expects to control a single timer. It can use the
//! micro:bit's `TIMER0`, `TIMER1`, or `TIMER2`.
//!
//! This uses a 6ms period to light each of the three internal LED rows, so
//! that the entire display is updated every 18ms.
//!
//! When rendering greyscale images, the `Display` requests extra interrupts
//! within each 6ms period. It only requests interrupts for the greyscale
//! levels which are actually required for what's currently being displayed.
//!
//! ## Technical details
//!
//! The timer is set to 16-bit mode, using a 62.5kHz clock (16 µs ticks). It
//! resets every 375 ticks.
//!
//! # Fonts
//!
//! The [`font`] module provides 5×5 representations of the ascii printable
//! characters as [`BitImage`]s.
//!
//! These are taken from the "pendolino" font supplied with the [micro:bit
//! runtime][dal].
//!
//! # Scrolling images and text
//!
//! The [`scrolling`] module supports horizontal scrolling for a sequence of
//! images via a [`ScrollingImages`] type which implements [`Render`] and an
//! [`Animate`] interface.
//!
//! The [`scrolling_text`] module supports scrolling messages, providing
//! [`ScrollingStaticText`] and [`ScrollingBufferedText`] types.
//!
//!
//! # Usage
//!
//! `use microbit_blinkenlights::prelude::*` to make trait methods available.
//!
//! Choose a timer to drive the display from (`TIMER0`, `TIMER1`, or
//! `TIMER2`).
//!
//! When your program starts:
//! * use [`GPIO.split_by_kind()`] to get a [`DisplayPins`] struct, and
//! create a [`DisplayPort`] by passing that to [`DisplayPort::new()`]
//! * create a [`MicrobitDisplayTimer`] struct, passing the timer you chose to
//! [`MicrobitDisplayTimer::new()`]
//! * call [`initialise_display()`], passing it the `MicrobitDisplayTimer` and
//! the `DisplayPort`.
//! * create a [`Display`] struct (a `Display<MicrobitFrame>`).
//!
//! In an interrupt handler for the same timer, call
//! [`handle_display_event()`].
//!
//! To change what's displayed: create a [`MicrobitFrame`] instance, use
//! [`.set()`](`Frame::set()`) to put an image (something implementing
//! [`Render`]) in it, then call [`Display::set_frame()`].
//!
//! You can call `set_frame()` at any time, so long as you're not
//! interrupting, or interruptable by, `handle_display_event()`.
//!
//! Once you've called `set_frame()`, you are free to reuse the
//! `MicrobitFrame`.
//!
//! See [`doc_example`] for a complete working example.
//!
//! [cortex-m-rtfm]: https://japaric.github.io/cortex-m-rtfm/book/en/
//! [dal]: https://lancaster-university.github.io/microbit-docs/
//! [micropython]: https://microbit-micropython.readthedocs.io/
//!
//! [`Animate`]: scrolling::Animate
//! [`BitImage`]: image::BitImage
//! [`DisplayTimer`]: tiny_led_matrix::DisplayTimer
//! [`DisplayPins`]: gpio::DisplayPins
//! [`GPIO.split_by_kind()`]: gpio::MicrobitGpioExt::split_by_kind
//! [`GreyscaleImage`]: image::GreyscaleImage
//! [`Scrollable`]: scrolling::Scrollable
//! [`ScrollingImages`]: scrolling::ScrollingImages
//! [`ScrollingBufferedText`]: scrolling_text::ScrollingBufferedText
//! [`ScrollingStaticText`]: scrolling_text::ScrollingStaticText
//!

#[doc(no_inline)]
pub use tiny_led_matrix::{
    Render,
    MAX_BRIGHTNESS,
    Display,
    Frame,
};

mod microbit_control;
mod microbit_matrix;
mod microbit_timer;

pub mod font;
pub mod gpio;
pub mod image;
pub mod prelude;
pub mod scrolling;
pub mod scrolling_text;

pub use microbit_control::{pin_constants, DisplayPort};
pub use microbit_matrix::MicrobitFrame;
pub use microbit_timer::MicrobitDisplayTimer;

pub mod doc_example;


use microbit::hal::hi_res_timer::Nrf51Timer;

/// Initialises the micro:bit hardware to use the display driver.
///
/// # Example
///
/// ```ignore
/// let mut p: nrf51::Peripherals = _;
/// let mut timer = microbit_blinkenlights::MicrobitDisplayTimer::new(p.TIMER1);
/// microbit_blinkenlights::initialise_display(&mut timer, &mut p.GPIO);
/// ```
pub fn initialise_display<T: Nrf51Timer>(
    timer: &mut MicrobitDisplayTimer<T>,
    display_port: &mut DisplayPort,
) {
    tiny_led_matrix::initialise_control(display_port);
    tiny_led_matrix::initialise_timer(timer);
}

/// Updates the LEDs and timer state during a timer interrupt.
///
/// The timer parameter must be the same [`MicrobitDisplayTimer`] you used for
/// [`initialise_display()`].
///
/// Call this in an interrupt handler for the timer you're using.
///
/// Takes care of clearing the timer's event registers.
///
/// See [`Display::handle_event()`] for details.
///
/// # Example
///
/// In the style of `cortex-m-rtfm` v0.4:
///
/// ```ignore
/// #[interrupt(priority = 2, resources = [DISPLAY_TIMER, GPIO, DISPLAY])]
/// fn TIMER1() {
///     microbit_blinkenlights::handle_display_event(
///         &mut resources.DISPLAY,
///         resources.DISPLAY_TIMER,
///         resources.GPIO,
///     );
/// }
/// ```
pub fn handle_display_event<T: Nrf51Timer>(
    display: &mut Display<MicrobitFrame>,
    timer: &mut MicrobitDisplayTimer<T>,
    display_port: &mut DisplayPort,
) {
    display.handle_event(timer, display_port);
}
