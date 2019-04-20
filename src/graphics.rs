//! Graphics for rendering on the 5×5 LED display.
//!
//! # Features
//!
//! This module provides:
//! - simple 5×5 greyscale and black-and-white image types;
//! - a copy of the 'pendolino' font from the [micro:bit runtime][dal];
//! - support for scrolling text.
//!
//! # The `Render` trait
//!
//! The graphics types in this module implement the [`display::Render`] trait,
//! which defines the interface that the display code needs.
//!
//! It supports ten levels of brightness; see [Greyscale model][greyscale].
//!
//! # Simple images
//!
//! The [`graphics::image`] module provides two static image types
//! implementing `Render`:
//!
//! - [`GreyscaleImage`], allowing all 10 levels (using one byte for each LED)
//! - [`BitImage`], allowing only 'on' and 'off' (using five bytes)
//!
//! # Fonts
//!
//! The [`graphics::font`] module provides 5×5 representations of the ascii
//! printable characters as [`BitImage`]s.
//!
//! These are taken from the "pendolino" font supplied with the
//! [micro:bit runtime][dal].
//!
//! # Scrolling images and text
//!
//! The [`graphics::scrolling`] module supports horizontal scrolling for a
//! sequence of images via a [`ScrollingImages`] type which implements
//! `Render` and an [`Animate`] interface.
//!
//! The [`graphics::scrolling_text`] module supports scrolling messages,
//! providing [`ScrollingStaticText`] and [`ScrollingBufferedText`] types.
//!
//! [dal]: https://lancaster-university.github.io/microbit-docs/
//! [greyscale]: crate::display
//! [`Animate`]: graphics::scrolling::Animate
//! [`BitImage`]: graphics::image::BitImage
//! [`GreyscaleImage`]: graphics::image::GreyscaleImage
//! [`display::Render`]: crate::display::Render
//! [`Scrollable`]: graphics::scrolling::Scrollable
//! [`ScrollingImages`]: graphics::scrolling::ScrollingImages
//! [`ScrollingBufferedText`]: graphics::scrolling_text::ScrollingBufferedText
//! [`ScrollingStaticText`]: graphics::scrolling_text::ScrollingStaticText

pub mod font;
pub mod image;
pub mod scrolling;
pub mod scrolling_text;
