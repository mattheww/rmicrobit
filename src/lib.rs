#![cfg_attr(not(test), no_std)]

//! A library for working with the [micro:bit](https://microbit.org/).
//!
//! # Features
//!
//! This crate currently provides:
//! - Support for the 5×5 LED display (see [`display`])
//! - A library for working with 5×5 images (see [`graphics`])
//! - Support for the hardware buttons (see [`buttons`])
//!
//! # Demo
//!
//! `examples/demo` demonstrates all the features of this crate, using
//! the [cortex-m-rtfm] framework.
//!
//! [cortex-m-rtfm]: https://japaric.github.io/cortex-m-rtfm/book/en/

pub mod buttons;
pub mod display;
pub mod gpio;
pub mod graphics;
pub mod prelude;
