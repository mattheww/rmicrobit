//! Support for scrolling ascii text horizontally.

use tiny_led_matrix::Render;

use crate::font;
use crate::image::BitImage;
use crate::scrolling::{Animate, ScrollingState, Scrollable};

/// A [`Scrollable`] displaying a static ascii byte-string slice.
#[derive(Default)]
pub struct ScrollingStaticText {
    message: &'static [u8],
    state: ScrollingState,
}

impl ScrollingStaticText {

    /// Specifies the ascii byte-string slice to be displayed.
    ///
    /// This also resets the animation to the beginning.
    pub fn set_message(&mut self, message: &'static [u8]) {
        self.message = message;
        self.reset();
    }

}

impl Scrollable for ScrollingStaticText {

    type Subimage = BitImage;

    fn length(&self) -> usize {
        self.message.len()
    }

    fn state(&self) -> &ScrollingState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ScrollingState {
        &mut self.state
    }

    fn subimage(&self, index: usize) -> &BitImage {
        font::character(self.message[index])
    }

}


impl Render for ScrollingStaticText {

    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.current_brightness_at(x, y)
    }

}


/// A [`Scrollable`] displaying an ascii byte-string of up to 128 bytes.
pub struct ScrollingBufferedText {
    length: usize,
    message: [u8; 128],
    state: ScrollingState,
}

impl ScrollingBufferedText {

    /// Specifies the ascii byte-string to be displayed.
    ///
    /// Makes a copy of the byte-string.
    ///
    /// This also resets the animation to the beginning.
    ///
    /// # Panics
    ///
    /// Panics if `message` is more than 128 bytes long.
    pub fn set_message(&mut self, message: &[u8]) {
        assert!(message.len() <= 128, "message too long");
        self.length = message.len();
        self.message[..self.length].copy_from_slice(message);
        self.reset();
    }


}

impl Default for ScrollingBufferedText {

    fn default() -> ScrollingBufferedText {
        ScrollingBufferedText {
            length: 0,
            message: [0; 128],
            state: Default::default(),
        }
    }

}

impl Scrollable for ScrollingBufferedText {

    type Subimage = BitImage;

    fn length(&self) -> usize {
        self.length
    }

    fn state(&self) -> &ScrollingState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ScrollingState {
        &mut self.state
    }

    fn subimage(&self, index: usize) -> &BitImage {
        font::character(self.message[index])
    }

}

impl Render for ScrollingBufferedText {

    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.current_brightness_at(x, y)
    }

}

