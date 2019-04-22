//! Example of working directly with DisplayPort

#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit_blinkenlights::nrf51_hal::prelude::*;
use microbit_blinkenlights::nrf51_hal::hi_res_timer::TimerFrequency;
use microbit_blinkenlights::nrf51_hal::delay::DelayTimer;
use microbit_blinkenlights::prelude::*;
use microbit_blinkenlights::gpio::PinsByKind;
use microbit_blinkenlights::display::DisplayPort;
use microbit_blinkenlights::display::pin_constants::{col_pin_number, row_pin_number, COL_PINS_MASK};

#[app(device = microbit_blinkenlights::nrf51)]
const APP: () = {

    #[init]
    fn init() {
        let p = device;
        let PinsByKind {display_pins, ..} = p.GPIO.split_by_kind();
        let mut display_port = DisplayPort::new(display_pins);
        // Row whose third column is the bottom-left led
        const LOWER_LEFT_ROW : u32 = row_pin_number(2);
        // Row whose third column is the central led
        const MID_ROW : u32 = row_pin_number(1);
        // Row whose third column is the top-right led
        const UPPER_RIGHT_ROW : u32 = row_pin_number(0);

        // 32bits @ 1MHz = ~72 minutes
        let mut delay_timer = DelayTimer::new(p.TIMER0, TimerFrequency::Freq1MHz);
        const LONG_MS: u16 = 800;
        const SHORT_MS: u16 = 400;

        // Set all cols except the third high
        display_port.set(COL_PINS_MASK ^ 1<<col_pin_number(2));

        // Light the bottom-left LED
        display_port.set(1<<LOWER_LEFT_ROW);
        delay_timer.delay_ms(LONG_MS);
        // Clear the bottom-left LED
        display_port.clear(1<<LOWER_LEFT_ROW);
        delay_timer.delay_ms(SHORT_MS);

        // Light the central LED
        display_port.set(1<<MID_ROW);
        delay_timer.delay_ms(LONG_MS);
        // Clear the central LED
        display_port.clear(1<<MID_ROW);
        delay_timer.delay_ms(SHORT_MS);

        // Light the top-right LED
        display_port.set(1<<UPPER_RIGHT_ROW);
        delay_timer.delay_ms(LONG_MS);
        // Clear the top-right LED
        display_port.clear(1<<UPPER_RIGHT_ROW);
        delay_timer.delay_ms(SHORT_MS);
    }

};
