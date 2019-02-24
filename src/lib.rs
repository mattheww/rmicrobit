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
//! `TIMER1` to provide that interrupt.
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
//! The `Display` expects to control a single timer.
//!
//! At present the only timer suported by this crate is the micro:bit's
//! `TIMER1`.
//!
//! This uses a 6ms period to light each of the three internal LED rows, so
//! that the entire display is updated every 18ms.
//!
//! When rendering greyscale images, the `Display` requests extra interrupts
//! within each 6ms period. It only requests interrupts for the greyscale
//! levels which are actually required for what's currently being displayed.
//!
//! # The `MicrobitGpio` and `MicrobitTimer1` wrappers
//!
//! The [`MicrobitGpio`] and [`MicrobitTimer1`] wrappers are tuple-like
//! structs holding a reference to a `nrf51::GPIO` or `nrf51::TIMER1`
//! respectively. They provide the interface between the `Display` and the
//! micro:bit's GPIO and timer peripherals.
//!
//! They are typically created with code something like this:
//!
//! ```ignore
//! let mut p: nrf51::Peripherals = …;
//! &mut MicrobitGpio(&mut p.GPIO)
//! &mut MicrobitTimer1(&mut p.TIMER1)
//! ```
//!
//! If it makes resource management easier, you can remake the wrappers from
//! the underlying peripherals each time they need to be used.
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
//! When your program starts, call [`initialise_control()`] (passing it a
//! [`MicrobitGpio`]) and [`initialise_timer()`] (passing it a
//! [`MicrobitTimer1`]), and create a [`Display`] struct (a
//! `Display<MicrobitFrame>`).
//!
//! In an interrupt handler for the timer you used for `initialise_timer()`,
//! call [`Display::handle_event()`], passing it a `MicrobitTimer1` and a
//! `MicrobitGpio`.
//!
//! To change what's displayed, call [`Display::set_frame()`] with a
//! [`MicrobitFrame`] instance. You can do that at any time, so long as you're
//! not interrupting, or interruptable by, `handle_event()`.
//!
//! Once you've called `set_frame()`, you are free to reuse the `Frame`
//! instance.
//!
//!
//! [cortex-m-rtfm]: https://japaric.github.io/cortex-m-rtfm/book/en/
//! [dal]: https://lancaster-university.github.io/microbit-docs/
//! [micropython]: https://microbit-micropython.readthedocs.io/
//!
//! [`Animate`]: scrolling::Animate
//! [`BitImage`]: image::BitImage
//! [`DisplayTimer`]: tiny_led_matrix::DisplayTimer
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
    initialise_control,
    initialise_timer,
};

mod microbit_control;
mod microbit_matrix;
mod microbit_timer;

pub mod font;
pub mod image;
pub mod scrolling;
pub mod scrolling_text;

pub use microbit_control::MicrobitGpio;
pub use microbit_matrix::MicrobitFrame;
pub use microbit_timer::MicrobitTimer1;

