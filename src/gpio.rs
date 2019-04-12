//! Support for the GPIO peripheral.
//!
//! The types in this module provide structured access to the micro:bit's GPIO
//! pins, organised in functional groups.
//!
//! This system supports working with different devices without having to
//! manage a shared reference to the single GPIO peripheral.
//!
//! The structs don't hold any data at runtime; they exist to manage ownership
//! of the pins.
//!
//! Use `GPIO.split_by_kind()` to retrieve one instance of each Xxx`Pins` type.
//!
//! # Example
//!
//! ```
//! use microbit_blinkenlights::prelude::*;
//! use microbit_blinkenlights::gpio::PinsByKind;
//! let p: nrf51::Peripherals = _;
//! let PinsByKind {display_pins, button_pins, ..} = p.GPIO.split_by_kind();
//! ```

use microbit::hal::nrf51::GPIO;
use microbit::hal::gpio::{Input, Floating, GpioExt};


/// The GPIO pins connected to the micro:bit's LED display.
///
/// See also [`pin_constants`] for dealing with these pin numbers.
///
/// [`pin_constants`]: crate::display_port::pin_constants
///
/// The pins for columns 1,2,3,7,8,9 are also presented on the edge connector.
pub struct DisplayPins {
    /// The GPIO pin connected to LED matrix column 1
    ///
    /// Also connected to edge connector strip 3
    pub pin4: microbit::hal::gpio::gpio::PIN4<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 2
    ///
    /// Also connected to edge connector strip 4
    pub pin5: microbit::hal::gpio::gpio::PIN5<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 3
    ///
    /// Also connected to edge connector strip 10
    pub pin6: microbit::hal::gpio::gpio::PIN6<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 4
    pub pin7: microbit::hal::gpio::gpio::PIN7<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 5
    pub pin8: microbit::hal::gpio::gpio::PIN8<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 6
    pub pin9: microbit::hal::gpio::gpio::PIN9<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 7
    ///
    /// Also connected to edge connector strip 9
    pub pin10: microbit::hal::gpio::gpio::PIN10<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 8
    ///
    /// Also connected to edge connector strip 7
    pub pin11: microbit::hal::gpio::gpio::PIN11<Input<Floating>>,
    /// The GPIO pin connected to LED matrix column 9
    ///
    /// Also connected to edge connector strip 6
    pub pin12: microbit::hal::gpio::gpio::PIN12<Input<Floating>>,
    /// The GPIO pin connected to LED matrix row 1
    pub pin13: microbit::hal::gpio::gpio::PIN13<Input<Floating>>,
    /// The GPIO pin connected to LED matrix row 2
    pub pin14: microbit::hal::gpio::gpio::PIN14<Input<Floating>>,
    /// The GPIO pin connected to LED matrix row 3
    pub pin15: microbit::hal::gpio::gpio::PIN15<Input<Floating>>,
}

/// The GPIO pins connected to the micro:bit's user buttons.
pub struct ButtonPins {
    /// The GPIO pin connected to Button A.
    pub pin17: microbit::hal::gpio::gpio::PIN17<Input<Floating>>,
    /// The GPIO pin connected to Button B.
    pub pin26: microbit::hal::gpio::gpio::PIN26<Input<Floating>>,
}

/// The GPIO pins connected to the micro:bit's USB serial port.
///
/// These pins are directly connected to the on-board Kinetis interface MCU,
/// which then makes the serial connection available over USB.
pub struct SerialPins {
    /// The 'tx' GPIO pin (micro:bit to USB)
    pub pin24: microbit::hal::gpio::gpio::PIN24<Input<Floating>>,
    /// The 'rx' GPIO pin (USB to micro:bit)
    pub pin25: microbit::hal::gpio::gpio::PIN25<Input<Floating>>,
}

/// The GPIO pins used for the micro:bit's I2C interface.
///
/// These pins are connected to the internal I2C devices (the accelerometer
/// and magnetometer), and are also presented on the edge connector.
pub struct I2cPins {
    /// The I2C SCL (clock) GPIO pin.
    ///
    /// Also connected to edge connector strip 19.
    pub pin0: microbit::hal::gpio::gpio::PIN0<Input<Floating>>,
    /// The I2C SDA (data) GPIO pin.
    ///
    /// Also connected to edge connector strip 20.
    pub pin30: microbit::hal::gpio::gpio::PIN30<Input<Floating>>,
}

/// The GPIO pins available on the edge connector and not otherwise connected.
///
/// The edge-connector pins included in [`DisplayPins`] and [`I2cPins`] are
/// excluded from this struct.
pub struct EdgeConnectorPins {
    /// The GPIO pin connected to edge connector ring 2
    pub pin1: microbit::hal::gpio::gpio::PIN1<Input<Floating>>,
    /// The GPIO pin connected to edge connector ring 1
    pub pin2: microbit::hal::gpio::gpio::PIN2<Input<Floating>>,
    /// The GPIO pin connected to edge connector ring 0
    pub pin3: microbit::hal::gpio::gpio::PIN3<Input<Floating>>,
    /// The GPIO pin connected to edge connector strip 16
    pub pin16: microbit::hal::gpio::gpio::PIN16<Input<Floating>>,
    /// The GPIO pin connected to edge connector strip 8
    pub pin18: microbit::hal::gpio::gpio::PIN18<Input<Floating>>,
    /// The GPIO pin connected to edge connector strip 12
    pub pin20: microbit::hal::gpio::gpio::PIN20<Input<Floating>>,
    /// The GPIO pin connected to edge connector strip 15
    ///
    /// Conventionally used for SPI MOSI.
    pub pin21: microbit::hal::gpio::gpio::PIN21<Input<Floating>>,
    /// The GPIO pin connected to edge connector strip 14
    ///
    /// Conventionally used for SPI MISO.
    pub pin22: microbit::hal::gpio::gpio::PIN22<Input<Floating>>,
    /// The GPIO pin connected to edge connector strip 13
    ///
    /// Conventionally used for SPI SCK.
    pub pin23: microbit::hal::gpio::gpio::PIN23<Input<Floating>>,
}

/// The remaining GPIO pins.
///
/// As far as I know none of these pins have any use.
pub struct OtherPins {
    pub pin19: microbit::hal::gpio::gpio::PIN19<Input<Floating>>,
    pub pin27: microbit::hal::gpio::gpio::PIN27<Input<Floating>>,
    pub pin28: microbit::hal::gpio::gpio::PIN28<Input<Floating>>,
    pub pin29: microbit::hal::gpio::gpio::PIN29<Input<Floating>>,
    pub pin31: microbit::hal::gpio::gpio::PIN31<Input<Floating>>,
}

/// The micro:bit's GPIO pins, organised in functional groups.
pub struct PinsByKind {
    pub display_pins: DisplayPins,
    pub button_pins: ButtonPins,
    pub serial_pins: SerialPins,
    pub i2c_pins: I2cPins,
    pub edge_connector_pins: EdgeConnectorPins,
    pub other_pins: OtherPins,
    _reserved: (),
}

/// Extension trait to split the GPIO peripheral into functional groups.
pub trait MicrobitGpioExt {
    /// Splits the GPIO peripheral into groups of pins.
    fn split_by_kind(self) -> PinsByKind;
}

impl MicrobitGpioExt for GPIO {

    fn split_by_kind(self) -> PinsByKind {
        let parts = self.split();
        let display_pins = DisplayPins {
            pin4: parts.pin4,
            pin5: parts.pin5,
            pin6: parts.pin6,
            pin7: parts.pin7,
            pin8: parts.pin8,
            pin9: parts.pin9,
            pin10: parts.pin10,
            pin11: parts.pin11,
            pin12: parts.pin12,
            pin13: parts.pin13,
            pin14: parts.pin14,
            pin15: parts.pin15,
        };
        let button_pins = ButtonPins {
            pin17: parts.pin17,
            pin26: parts.pin26,
        };
        let serial_pins = SerialPins {
            pin24: parts.pin24,
            pin25: parts.pin25,
        };
        let i2c_pins = I2cPins {
            pin0: parts.pin0,
            pin30: parts.pin30,
        };
        let edge_connector_pins = EdgeConnectorPins {
            pin1: parts.pin1,
            pin2: parts.pin2,
            pin3: parts.pin3,
            pin16: parts.pin16,
            pin18: parts.pin18,
            pin20: parts.pin20,
            pin21: parts.pin21,
            pin22: parts.pin22,
            pin23: parts.pin23,
        };
        let other_pins = OtherPins {
            pin19: parts.pin19,
            pin27: parts.pin27,
            pin28: parts.pin28,
            pin29: parts.pin29,
            pin31: parts.pin31,
        };
        PinsByKind{
            display_pins,
            button_pins,
            serial_pins,
            i2c_pins,
            edge_connector_pins,
            other_pins,
            _reserved: (),
        }
    }

}

