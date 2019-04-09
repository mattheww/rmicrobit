//! Low-level control of the micro:bit's 5×5 LED display.
//!
//! Provides an implementation of [`DisplayControl`].
//!
//! [`DisplayControl`]: tiny_led_matrix::DisplayControl

use microbit::hal::nrf51;
use tiny_led_matrix::DisplayControl;
use crate::gpio::DisplayPins;
use pin_constants::*;


/// Constants identifying GPIO pins used in the LED matrix.
///
/// This module is intended to be suitable for glob-importing:
/// ```
/// pub use microbit_blinkenlights::pin_constants::*
/// ```
pub mod pin_constants {
    const fn bit_range(lo: usize, count: usize) -> u32 {
        ((1<<count) - 1) << lo
    }

    /// The number of column pins (9).
    pub const MATRIX_COLS : usize = 9;

    /// Number in the GPIO port of the first column pin
    pub const FIRST_COL_PIN : usize = 4;
    /// Number in the GPIO port of the last column pin
    pub const LAST_COL_PIN : usize = FIRST_COL_PIN + MATRIX_COLS - 1;
    /// u32 bitmask representing the GPIO port numbers of the column pins
    pub const COL_PINS_MASK : u32 = bit_range(FIRST_COL_PIN, MATRIX_COLS);

    /// The number of row pins (3).
    pub const MATRIX_ROWS : usize = 3;
    /// Number in the GPIO port of the first row pin
    pub const FIRST_ROW_PIN : usize = 13;
    /// Number in the GPIO port of the last row pin
    pub const LAST_ROW_PIN : usize = FIRST_ROW_PIN + MATRIX_ROWS - 1;
    /// u32 bitmask representing the GPIO port numbers of the row pins
    pub const ROW_PINS_MASK : u32 = bit_range(FIRST_ROW_PIN, MATRIX_ROWS);
}


/// Write access to the GPIO pins connected to the 5×5 LED display.
///
/// `DisplayPort` permits writing to the display's GPIO pins and ignores
/// requests to write to any other part of the GPIO port.
///
/// `DisplayPort` implements the [`DisplayControl`] trait, so it can be used
/// with a [`Display`].
///
/// To light an LED, set its row pin and clear its column pin.
///
/// Use the [`pin_constants`] to find the GPIO pin numbers for each row
/// and column.
///
/// See the [DAL documentation] for how these rows and columns correspond to
/// the physical LED layout.
///
/// # Example
///
/// ```
/// use microbit_blinkenlights::prelude::*;
/// use microbit_blinkenlights::gpio::PinsByKind;
/// use microbit_blinkenlights::pin_constants::*;
/// let p: nrf51::Peripherals = _;
/// let PinsByKind {display_pins, ..} = p.GPIO.split_by_kind();
/// let mut display_port = DisplayPort::new(display_pins);
/// // Row whose third column is the top-right led
/// const UPPER_RIGHT_ROW : usize = FIRST_ROW_PIN;
/// // Row whose third column is the bottom-left led
/// const LOWER_LEFT_ROW : usize = FIRST_ROW_PIN+2;
///
/// // Set all cols except the third high
/// display_port.set(COL_PINS_MASK ^ 1<<(FIRST_COL_PIN+2));
///
/// // Light the top-right LED
/// display_port.set(1<<UPPER_RIGHT_ROW);
/// // (sleep a short time here)
/// // Clear the top-right LED
/// display_port.clear(1<<UPPER_RIGHT_ROW);
/// // (sleep a short time here)
///
/// // Light the bottom-left LED
/// display_port.set(1<<LOWER_LEFT_ROW);
/// // (sleep a short time here)
/// // Clear the bottom-left LED
/// display_port.clear(1<<LOWER_LEFT_ROW);
/// // (sleep a short time here)
/// ```
///
/// [`Display`]: tiny_led_matrix::Display
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
/// [DAL documentation]: https://lancaster-university.github.io/microbit-docs/ubit/display/
pub struct DisplayPort(DisplayPins);

impl DisplayPort {

    /// Takes ownership of the display's GPIO pins and returns a `DisplayPort`.
    ///
    /// Sets the pins to output mode.
    // Note we never call any methods on the nrf51-hal Pins held in
    // DisplayPins; we just use them as a token proving that nothing else is
    // talking to this part of the GPIO space.
    pub fn new(pins: DisplayPins) -> DisplayPort {
        let mut port = DisplayPort(pins);
        port.reset();
        port
    }

    /// Gives the underlying `DisplayPins` instance back.
    pub fn free(self) -> DisplayPins {
        self.0
    }

    /// Set all the pins to output mode.
    fn reset(&mut self) {
        // NOTE(unsafe) writes restricted to pins we own.
        unsafe {
            let gpio = &*nrf51::GPIO::ptr();
            for ii in FIRST_COL_PIN ..= LAST_COL_PIN {
                gpio.pin_cnf[ii].write(|w| w.dir().output());
            }
            for ii in FIRST_ROW_PIN ..= LAST_ROW_PIN {
                gpio.pin_cnf[ii].write(|w| w.dir().output());
            }
        }
    }

    /// Sets the specified pins high, leaving the others unchanged.
    ///
    /// The u32 `pins` parameter is a bitmask representing the set of pins to
    /// affect: a 1 in bit position *n* says to set GPIO pin *n*.
    ///
    /// Bits in `pins` not representing row or column pins are ignored.
    pub fn set(&mut self, pins: u32) {
        let to_set = pins & (ROW_PINS_MASK | COL_PINS_MASK);
        // NOTE(unsafe) writes restricted to affecting pins we own.
        unsafe {
            let gpio = &*nrf51::GPIO::ptr();
            gpio.outset.write(|w| { w.bits(to_set) });
        }
    }

    /// Sets the specified pins low, leaving the others unchanged.
    ///
    /// The u32 `pins` parameter is a bitmask representing the set of pins to
    /// affect: a 1 in bit position *n* says to clear GPIO pin *n*.
    ///
    /// Bits in `pins` not representing row or column pins are ignored.
    pub fn clear(&mut self, pins: u32) {
        let to_clear = pins & (ROW_PINS_MASK | COL_PINS_MASK);
        // NOTE(unsafe) writes restricted to affecting pins we own.
        unsafe {
            let gpio = &*nrf51::GPIO::ptr();
            gpio.outclr.write(|w| { w.bits(to_clear) });
        }
    }

}


/// Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
///
/// This controls the micro:bit's 5×5 LED display.
///
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
impl DisplayControl for DisplayPort {

    fn initialise_for_display(&mut self) {
        // Start with all cols high, leaving all rows low.
        self.set(COL_PINS_MASK);
    }

    fn display_row_leds(&mut self, row: usize, cols: u32) {
        // In `cols`, the least-significant bit represents column 0, and so on.
        // To light an LED, we set the row bit and clear the col bit.
        let rows_to_set = 1<<(FIRST_ROW_PIN+row);
        let rows_to_clear = ROW_PINS_MASK ^ rows_to_set;
        let cols_to_clear = cols << FIRST_COL_PIN;
        let cols_to_set = COL_PINS_MASK ^ cols_to_clear;

        self.set(rows_to_set | cols_to_set);
        self.clear(rows_to_clear | cols_to_clear);
    }

    fn light_current_row_leds(&mut self, cols: u32) {
        // In `cols`, the least-significant bit represents column 0, and so on.
        self.clear(cols << FIRST_COL_PIN)
    }

}

