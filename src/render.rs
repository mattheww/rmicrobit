//! The interface between images and the display.

/// The number of brightness levels for greyscale images.
pub const BRIGHTNESSES : usize = 10;

/// The maximum brightness level for greyscale images (ie, 9; the minimum is 0).
pub const MAX_BRIGHTNESS : u8 = (BRIGHTNESSES as u8)-1;


/// A trait providing the information that [`Display`] needs to render an image.
///
/// [`Display`]: crate::display::Display
pub trait Render {

    /// Returns the brightness value for a single LED.
    ///
    /// The ranges for the x and y coordinates are 0..IMAGE_COLS and
    /// 0..IMAGE_ROWS, as defined by the [`Matrix`] for the [`Display`]'s
    /// [`Frame`].
    ///
    /// (0, 0) is the top left.
    ///
    /// The result must be in the range 0..=`MAX_BRIGHTNESS`
    ///
    /// # Panics
    ///
    /// If the provided coordinates are out of range, may panic or return an
    /// arbitrary in-range result.
    ///
    /// [`Display`]: crate::display::Display
    /// [`Matrix`]: crate::display::Matrix
    /// [`Frame`]: crate::display::Frame
    fn brightness_at(&self, x: usize, y: usize) -> u8;

}

