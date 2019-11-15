#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use rmicrobit::nrf51;
use rmicrobit::nrf51_hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
use rmicrobit::prelude::*;
use rmicrobit::display::{DisplayPort, MicrobitDisplay, MicrobitFrame};
use rmicrobit::gpio::PinsByKind;
use rmicrobit::graphics::image::GreyscaleImage;

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

#[app(device = rmicrobit::nrf51, peripherals = true)]
const APP: () = {

    struct Resources {
        display: MicrobitDisplay<nrf51::TIMER1>,
        anim_timer: LoResTimer<nrf51::RTC0>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let p: nrf51::Peripherals = cx.device;

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
        let display_port = DisplayPort::new(display_pins);
        let display = MicrobitDisplay::new(display_port, p.TIMER1);

        init::LateResources {
            display : display,
            anim_timer : rtc0,
        }
    }

    #[task(binds = TIMER1, priority = 2,
           resources = [display])]
    fn timer1(cx: timer1::Context) {
        cx.resources.display.handle_event();
    }

    #[task(binds = RTC0, priority = 1,
           resources = [anim_timer, display])]
    fn rtc0(mut cx: rtc0::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        static mut STEP: u8 = 0;

        &cx.resources.anim_timer.clear_tick_event();

        let inner_brightness = match *STEP {
            0..=8 => 9-*STEP,
            9..=12 => 0,
            _ => unreachable!()
        };

        FRAME.set(&mut heart_image(inner_brightness));
        cx.resources.display.lock(|display| {
            display.set_frame(FRAME);
        });

        *STEP += 1;
        if *STEP == 13 {*STEP = 0};
    }

};

