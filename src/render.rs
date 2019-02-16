//! The interface between images and the display.

/// The number of brightness levels for greyscale images.
pub const BRIGHTNESSES : usize = 10;

/// The maximum brightness level for greyscale images (ie, 9; the minimum is 0).
pub const MAX_BRIGHTNESS : usize = BRIGHTNESSES-1;


/// A trait providing the information that Display needs to render an image.
pub trait Render {

    /// Returns the brightness value for a single LED.
    ///
    /// The x and y coordinates must be in 0..5.
    ///
    /// 0, 0 is the top left.
    ///
    /// The result must be in the range 0..=`MAX_BRIGHTNESS`
    fn brightness_at(&self, x: usize, y: usize) -> u8;

}

