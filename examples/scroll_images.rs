#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit_blinkenlights::nrf51;
use microbit_blinkenlights::nrf51_hal::lo_res_timer::{LoResTimer, FREQ_8HZ};
use microbit_blinkenlights::prelude::*;
use microbit_blinkenlights::display::{DisplayPort, MicrobitDisplay, MicrobitFrame};
use microbit_blinkenlights::gpio::PinsByKind;
use microbit_blinkenlights::graphics::image::GreyscaleImage;
use microbit_blinkenlights::graphics::scrolling::ScrollingImages;

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
const IMAGES: &'static [&'static GreyscaleImage] =
    &[&HEART, &BLANK, &GREY_HEART, &BLANK, &HEART];

#[app(device = microbit_blinkenlights::nrf51)]
const APP: () = {

    static mut DISPLAY: MicrobitDisplay<nrf51::TIMER1> = ();
    static mut ANIM_TIMER: LoResTimer<nrf51::RTC0> = ();
    static mut SCROLLER: ScrollingImages<&'static GreyscaleImage> = ();

    #[init]
    fn init() -> init::LateResources {
        let p: nrf51::Peripherals = device;

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
            DISPLAY : display,
            ANIM_TIMER : rtc0,
            SCROLLER: scroller,
        }
    }

    #[interrupt(priority = 2,
                resources = [DISPLAY])]
    fn TIMER1() {
        resources.DISPLAY.handle_event();
    }

    #[interrupt(priority = 1,
                resources = [ANIM_TIMER, DISPLAY, SCROLLER])]
    fn RTC0() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        &resources.ANIM_TIMER.clear_tick_event();
        if !resources.SCROLLER.is_finished() {
            resources.SCROLLER.tick();
            FRAME.set(resources.SCROLLER);
            resources.DISPLAY.lock(|display| {
                display.set_frame(FRAME);
            });
        }
    }

};

