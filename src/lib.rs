#![no_std]

//! A library for controlling the [micro:bit](https://microbit.org/) 5×5 LED
//! display.
//!
//! # Scope
//!
//! The crate includes:
//! - support for driving the LED display from a timer interrupt;
//! - ten levels of brightness for each LED;
//! - simple 5×5 greyscale and black-and-white image types;
//! - a copy of the 'pendolino' font from the [micro:bit runtime][dal];
//! - support for scrolling text.
//!
//! The crate doesn't define interrupt handlers directly; instead it provides
//! a function to be called from a timer interrupt, and a trait describing the
//! interface it needs to program the timer.
//!
//! # Demo
//!
//! `examples/demo` demonstrates all the features of this crate, using
//! the [cortex-m-rtfm] framework.
//!
//! # Coordinate system
//!
//! LEDs are identified using (x,y) coordinates as follows:
//!
//! ```
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
//! The [`render::Render`] trait defines the interface that an image-like type
//! needs to provide in order to be displayed.
//!
//! It contains a single function:
//! [`brightness_at(x, y)`][render::Render::brightness_at], returning a
//! brightness level.
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
//! The [`display_timer::DisplayTimer`] trait defines the interface that it
//! needs.
//!
//! At present this crate provides only one [`DisplayTimer`] implementation,
//! which uses the nrf51's `TIMER1`.
//!
//! The system is designed to use a 6ms period to light each of the three
//! internal rows, so that the entire display is updated every 18ms. But the
//! [`DisplayTimer`] trait doesn't strictly require this.
//!
//! When rendering greyscale images, the `Display` requests extra interrupts
//! within each 6ms period. It only requests interrupts for the greyscale
//! levels which are actually required for what's currently being displayed.
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
//! When your program starts, call [`display::initialise_control()`] (passing
//! it the gpio peripheral) and [`display::initialise_timer()`] (passing it
//! the timer), and create a [`Display`] struct (a `Display<MicrobitFrame>`).
//!
//! In an interrupt handler for the timer you used for `initialise_timer()`,
//! call [`Display::handle_event()`], passing it the timer and the gpio
//! peripheral.
//!
//! To change what's displayed, call [`Display::set_frame()`] with a
//! [`MicrobitFrame`] instance. You can do that at at any time, so long as
//! you're not interrupting, or interruptable by, `handle_event()`.
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
//! [`Display::handle_event()`]: display::Display::handle_event
//! [`Display::set_frame()`]: display::Display::set_frame
//! [`DisplayTimer`]: display_timer::DisplayTimer
//! [`Display`]: display::Display
//! [`Frame`]: display::Frame
//! [`GreyscaleImage`]: image::GreyscaleImage
//! [`MicrobitFrame`]: microbit_matrix::MicrobitFrame
//! [`Render`]: render::Render
//! [`Scrollable`]: scrolling::Scrollable
//! [`ScrollingImages`]: scrolling::ScrollingImages
//! [`ScrollingBufferedText`]: scrolling_text::ScrollingBufferedText
//! [`ScrollingStaticText`]: scrolling_text::ScrollingStaticText
//!

pub mod display;
pub mod display_control;
pub mod display_timer;
pub mod font;
pub mod image;
pub mod render;
pub mod scrolling;
pub mod scrolling_text;

pub mod microbit_control;
pub mod microbit_matrix;
pub mod microbit_timer;
