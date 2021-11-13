#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use rmicrobit::nrf51;
use rmicrobit::nrf51_hal::lo_res_timer::{LoResTimer, FREQ_8HZ};
use rmicrobit::prelude::*;
use rmicrobit::display::{DisplayPort, MicrobitDisplay, MicrobitFrame};
use rmicrobit::gpio::PinsByKind;
use rmicrobit::graphics::image::GreyscaleImage;
use rmicrobit::graphics::scrolling::ScrollingImages;

const BLANK: GreyscaleImage = GreyscaleImage::blank();
const HEART: GreyscaleImage = GreyscaleImage::new(&[
    [0, 9, 0, 9, 0],
    [9, 0, 9, 0, 9],
    [9, 0, 0, 0, 9],
    [0, 9, 0, 9, 0],
    [0, 0, 9, 0, 0],
]);
const GREY_HEART: GreyscaleImage = GreyscaleImage::new(&[
    [0, 9, 0, 9, 0],
    [9, 5, 9, 5, 9],
    [9, 5, 5, 5, 9],
    [0, 9, 5, 9, 0],
    [0, 0, 9, 0, 0],
]);
const IMAGES: &[&GreyscaleImage] =
    &[&HEART, &BLANK, &GREY_HEART, &BLANK, &HEART];

#[app(device = rmicrobit::nrf51, peripherals = true)]
const APP: () = {

    struct Resources {
        display: MicrobitDisplay<nrf51::TIMER1>,
        anim_timer: LoResTimer<nrf51::RTC0>,
        scroller: ScrollingImages<&'static GreyscaleImage>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let p: nrf51::Peripherals = cx.device;

        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        let mut rtc0 = LoResTimer::new(p.RTC0);
        // 8Hz; 125ms period
        rtc0.set_frequency(FREQ_8HZ);
        rtc0.enable_tick_event();
        rtc0.enable_tick_interrupt();
        rtc0.start();

        let PinsByKind {display_pins, ..} = p.GPIO.split_by_kind();
        let display_port = DisplayPort::new(display_pins);
        let display = MicrobitDisplay::new(display_port, p.TIMER1);

        let mut scroller = ScrollingImages::default();
        scroller.set_images(IMAGES);

        init::LateResources {
            display : display,
            anim_timer : rtc0,
            scroller: scroller,
        }
    }

    #[task(binds = TIMER1, priority = 2,
           resources = [display])]
    fn timer1(cx: timer1::Context) {
        cx.resources.display.handle_event();
    }

    #[task(binds = RTC0, priority = 1,
           resources = [anim_timer, display, scroller])]
    fn rtc0(mut cx: rtc0::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        cx.resources.anim_timer.clear_tick_event();
        if !cx.resources.scroller.is_finished() {
            cx.resources.scroller.tick();
            FRAME.set(cx.resources.scroller);
            cx.resources.display.lock(|display| {
                display.set_frame(FRAME);
            });
        }
    }

};

