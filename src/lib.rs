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

// Re-export the versions of some libraries we're using
pub use nrf51_hal;
pub use nrf51_hal::nrf51 as nrf51;
pub use nrf51_hal::hal as embedded_hal;

