//! Support for the 5×5 LED display.
//!
//! # Scope
//!
//! This module supports driving the LED display from a timer interrupt,
//! providing ten levels of brightness for each LED.
//!
//! Much of the implementation is in the separate `tiny-led-matrix` crate.
//!
//! The driver doesn't define interrupt handlers directly; instead it provides
//! a function to be called from a timer interrupt. It knows how to program
//! one of the micro:bit's timers to provide that interrupt.
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
//! It contains a single function:
//! [`brightness_at(x, y)`][display::Render::brightness_at],
//! returning a brightness level.
//!
//! The [`graphics`] module provides a number of implementations of `Render`.
//!
//! # MicrobitDisplay
//!
//! A [`MicrobitDisplay`] instance controls the LEDs and programs a timer.
//! There should normally be a single `MicrobitDisplay` instance in the
//! program.
//!
//! # Frames
//!
//! Types implementing [`Render`] aren't used directly with the
//! [`MicrobitDisplay`]; instead they're used to update a [`MicrobitFrame`]
//! instance which is in turn passed to the `Display`.
//!
//! A `MicrobitFrame` instance is a 'compiled' representation of a 5×5
//! greyscale image, in a form that's more directly usable by the display
//! code.
//!
//! This is exposed in the public API so that you can construct the
//! `MicrobitFrame` representation in code running at a low priority. Then
//! only [`MicrobitDisplay::set_frame()`][set_frame] has to be called in code
//! that can't be interrupted by the display timer.
//!
//! # Timer integration
//!
//! The `MicrobitDisplay` owns a single timer peripheral. It can use the
//! micro:bit's `TIMER0`, `TIMER1`, or `TIMER2`.
//!
//! It uses a 6ms period to light each of the three internal LED rows, so that
//! the entire display is updated every 18ms.
//!
//! When rendering greyscale images, the `MicrobitDisplay` requests extra
//! interrupts within each 6ms period. It only requests interrupts for the
//! greyscale levels which are actually required for what's currently being
//! displayed.
//!
//! The function called from the timer interrupt returns a value indicating
//! which kind of interrupt has occurred, so that it's possible to rely on the
//! same timer to perform other tasks every 6ms.
//!
//! ## Technical details
//!
//! The timer is set to 16-bit mode, using a 62.5kHz clock (16 µs ticks). It
//! resets every 375 ticks.
//!
//! # Usage
//!
//! `use microbit_blinkenlights::prelude::*` to make trait methods available.
//!
//! Choose a timer to drive the display from (`TIMER0`, `TIMER1`, or
//! `TIMER2`).
//!
//! When your program starts:
//! * use [`GPIO.split_by_kind()`] to get a [`DisplayPins`] struct
//! * create a [`DisplayPort`] by passing that to [`DisplayPort::new()`]
//! * create a [`MicrobitDisplay`] by passing the `DisplayPort` and the
//! timer you chose to [`MicrobitDisplay::new()`]
//!
//! In an interrupt handler for the same timer, call the `MicrobitDisplay`'s
//! [`handle_event()`] method.
//!
//! To change what's displayed: create a [`MicrobitFrame`] instance, use
//! [`.set()`](`display::Frame::set()`) to put an image (something
//! implementing [`Render`]) in it, then call the `MicrobitDisplay`'s
//! [`set_frame()`][set_frame] method.
//!
//! You can call `set_frame()` at any time, so long as you're not
//! interrupting, or interruptable by, `handle_event()`.
//!
//! Once you've called `set_frame()`, you are free to reuse the
//! `MicrobitFrame`.
//!
//! See [`doc_example`] for a complete working example.
//!
//! [dal]: https://lancaster-university.github.io/microbit-docs/
//! [micropython]: https://microbit-micropython.readthedocs.io/
//!
//! [`DisplayPins`]: crate::gpio::DisplayPins
//! [`GPIO.split_by_kind()`]: crate::gpio::MicrobitGpioExt::split_by_kind
//! [`DisplayPort`]: display::DisplayPort
//! [`DisplayPort::new()`]: display::DisplayPort::new
//! [`MicrobitDisplay`]: display::MicrobitDisplay
//! [`MicrobitDisplay::new()`]: display::MicrobitDisplay::new
//! [set_frame]: display::MicrobitDisplay::set_frame
//! [`handle_event()`]: display::MicrobitDisplay::handle_event
//! [`MicrobitFrame`]: display::MicrobitFrame
//! [`Render`]: display::Render
//! [`doc_example`]: display::doc_example
//!

#[doc(no_inline)]
pub use tiny_led_matrix::{
    Render,
    MAX_BRIGHTNESS,
    Frame,
    Event as DisplayEvent,
};

mod display_port;
mod microbit_display;
mod matrix;
mod timer;

pub mod doc_example;

pub use display_port::{pin_constants, DisplayPort};
pub use matrix::MicrobitFrame;
pub use microbit_display::MicrobitDisplay;
