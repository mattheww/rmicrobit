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
//! # Re-exports
//!
//! The following dependencies are re-exported under
//! `rmicrobit::`, so that crates using this library can be sure
//! to be using consistent versions:
//! - `nrf51` (register-level access to the SoC peripherals)
//! - `nrf51_hal` (higher-level access to the SoC peripherals)
//! - `embedded_hal` (traits used by some `nrf51_hal` interfaces)
//!
//! In particular, if you use [cortex-m-rtfm], use
//! `rmicrobit::nrf51` as the `device` parameter to `#[app]`.
//!
//! # Getting started
//!
//! See [How to use rmicrobit](_doc_setup).
//!
//! # Examples
//!
//! There are a number of example programs in the `examples` directory. You
//! can run an example as follows:
//!
//! ```sh
//! cargo run --example scroll_text -- -x microbit.gdb
//! ```
//!
//! `examples/demo` demonstrates all the features of this crate, using
//! the [cortex-m-rtfm] framework.
//!
//! # Tests
//!
//! There are a few tests which can be run on the host machine. Run them as
//! follows (from a checked-out working copy of `rmicrobit`):
//! ```text
//! cargo test --lib --target x86_64-unknown-linux-gnu
//! ```
//! (or substitute your development machine's native target)
//!
//! [cortex-m-rtfm]: https://rtfm.rs/

pub mod buttons;
pub mod display;
pub mod gpio;
pub mod graphics;
pub mod prelude;

// Re-export the versions of some libraries we're using
pub use embedded_hal;
pub use nrf51_hal;
pub use nrf51_hal::nrf51;

pub mod _doc_setup;
