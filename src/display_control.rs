//! The interface that [`Display`] needs to control an LED matrix.
//!
//! [`Display`]: crate::display::Display


/// The interface that [`Display`] needs to work with an LED matrix.
///
/// Assumes the matrix is organised by rows and columns, in such a way that
/// LEDs from at most one row are lit at any time.
///
/// [`Display`]: crate::display::Display

pub trait DisplayControl {

    /// Performs any required hardware initialisation.
    ///
    /// This is intended to be called once, before using a display with this
    /// DisplayControl.
    fn initialise_for_display(&mut self);

    /// Lights LEDs in a single matrix row.
    ///
    /// In the specified row, lights exactly the LEDs listed in 'cols'.
    /// Turns off all LEDs in the other matrix rows.
    ///
    /// In 'cols', the least-significant bit represents column 0, and so on.
    fn display_row_leds(&mut self, row: usize, cols: u32);

    /// Lights additional LEDs in the current matrix row.
    ///
    /// Affects the row most recently passed to display_row_leds().
    /// Lights the LEDs listed in 'cols', in addition to any already lit.
    ///
    /// In 'cols', the least-significant bit represents column 0, and so on.
    fn light_current_row_leds(&mut self, cols: u32);

}

