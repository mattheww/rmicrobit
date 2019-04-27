#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use rmicrobit::nrf51;
use rmicrobit::nrf51_hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
use rmicrobit::prelude::*;
use rmicrobit::display::{DisplayPort, MicrobitDisplay, MicrobitFrame};
use rmicrobit::gpio::PinsByKind;
use rmicrobit::graphics::scrolling_text::ScrollingStaticText;

const MESSAGE: &[u8] = b"Hello, world!";

#[app(device = rmicrobit::nrf51)]
const APP: () = {

    static mut DISPLAY: MicrobitDisplay<nrf51::TIMER1> = ();
    static mut ANIM_TIMER: LoResTimer<nrf51::RTC0> = ();
    static mut SCROLLER: ScrollingStaticText = ();

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
        let display_port = DisplayPort::new(display_pins);
        let display = MicrobitDisplay::new(display_port, p.TIMER1);

        let mut scroller = ScrollingStaticText::default();
        scroller.set_message(MESSAGE);

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

