#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit::hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
use microbit::hal::nrf51;
use microbit_blinkenlights::prelude::*;
use microbit_blinkenlights::{self, Display, DisplayPort, MicrobitDisplayTimer, MicrobitFrame};
use microbit_blinkenlights::gpio::PinsByKind;
use microbit_blinkenlights::image::GreyscaleImage;

fn heart_image(inner_brightness: u8) -> GreyscaleImage {
    let b = inner_brightness;
    GreyscaleImage::new(&[
        [0, 7, 0, 7, 0],
        [7, b, 7, b, 7],
        [7, b, b, b, 7],
        [0, 7, b, 7, 0],
        [0, 0, 7, 0, 0],
    ])
}

#[app(device = microbit::hal::nrf51)]
const APP: () = {

    static mut DISPLAY_PORT: DisplayPort = ();
    static mut DISPLAY_TIMER: MicrobitDisplayTimer<nrf51::TIMER1> = ();
    static mut ANIM_TIMER: LoResTimer<nrf51::RTC0> = ();
    static mut DISPLAY: Display<MicrobitFrame> = ();

    #[init]
    fn init() -> init::LateResources {
        let p: nrf51::Peripherals = device;

        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        let mut rtc0 = LoResTimer::new(p.RTC0);
        // 16Hz; 62.5ms period
        rtc0.set_frequency(FREQ_16HZ);
        rtc0.enable_tick_event();
        rtc0.enable_tick_interrupt();
        rtc0.start();

        let PinsByKind {display_pins, ..} = p.GPIO.split_by_kind();
        let mut display_port = DisplayPort::new(display_pins);

        let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
        microbit_blinkenlights::initialise_display(&mut timer, &mut display_port);

        init::LateResources {
            DISPLAY_PORT : display_port,
            DISPLAY_TIMER : timer,
            ANIM_TIMER : rtc0,
            DISPLAY : Display::new(),
        }
    }

    #[interrupt(priority = 2,
                resources = [DISPLAY_TIMER, DISPLAY_PORT, DISPLAY])]
    fn TIMER1() {
        microbit_blinkenlights::handle_display_event(
            &mut resources.DISPLAY,
            resources.DISPLAY_TIMER,
            resources.DISPLAY_PORT,
        );
    }

    #[interrupt(priority = 1,
                resources = [ANIM_TIMER, DISPLAY])]
    fn RTC0() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        static mut STEP: u8 = 0;

        &resources.ANIM_TIMER.clear_tick_event();

        let inner_brightness = match *STEP {
            0..=8 => 9-*STEP,
            9..=12 => 0,
            _ => unreachable!()
        };

        FRAME.set(&mut heart_image(inner_brightness));
        resources.DISPLAY.lock(|display| {
            display.set_frame(FRAME);
        });

        *STEP += 1;
        if *STEP == 13 {*STEP = 0};
    }

};

