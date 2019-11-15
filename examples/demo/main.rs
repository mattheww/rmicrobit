#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use rmicrobit::nrf51;
use rmicrobit::prelude::*;
use rmicrobit::display::{DisplayPort, MicrobitDisplay, MicrobitFrame};
use rmicrobit::gpio::PinsByKind;
use rmicrobit::nrf51_hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
use rmicrobit::buttons;
use rmicrobit::buttons::dual_with_hold::ABMonitor;

mod animation;
mod demo;


#[app(device = rmicrobit::nrf51, peripherals = true)]
const APP: () = {

    struct Resources {
        anim_timer: LoResTimer<nrf51::RTC0>,
        display: MicrobitDisplay<nrf51::TIMER1>,
        button_monitor: ABMonitor,
        demo: demo::Demo,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // nrf51 peripherals
        let p: nrf51::Peripherals = cx.device;

        let PinsByKind {display_pins, button_pins, ..} = p.GPIO.split_by_kind();
        let display_port = DisplayPort::new(display_pins);
        let (button_a, button_b) = buttons::from_pins(button_pins);
        let mut display = MicrobitDisplay::new(display_port, p.TIMER1);
        let button_monitor = ABMonitor::new(button_a, button_b);

        let mut frame = MicrobitFrame::const_default();
        frame.set(demo::initial_frame());
        display.set_frame(&frame);

        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        let mut rtc0 = LoResTimer::new(p.RTC0);
        // 62.5ms period
        rtc0.set_frequency(FREQ_16HZ);
        rtc0.enable_tick_event();
        rtc0.enable_tick_interrupt();
        rtc0.start();

        init::LateResources {
            display : display,
            anim_timer : rtc0,
            demo : demo::Demo::new(),
            button_monitor : button_monitor,
        }
    }

    #[task(binds = TIMER1, priority = 2,
           spawn = [handle_buttons],
           resources = [display])]
    fn timer1(cx: timer1::Context) {
        let display_event = cx.resources.display.handle_event();
        if display_event.is_new_row() {
            cx.spawn.handle_buttons().ok();
        }
    }

    #[task(priority = 1,
           resources = [display, button_monitor, demo])]
    fn handle_buttons(mut cx: handle_buttons::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        if let Some(event) = cx.resources.button_monitor.poll() {
            cx.resources.demo.handle_button_event(event);
            if cx.resources.demo.is_static() {
                FRAME.set(cx.resources.demo.current_image());
                cx.resources.display.lock(|display| {
                    display.set_frame(FRAME);
                });
            }
        }
    }

    #[task(binds = RTC0, priority = 1,
           resources = [anim_timer, display, demo])]
    fn rtc0(mut cx: rtc0::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();

        &cx.resources.anim_timer.clear_tick_event();
        if cx.resources.demo.is_animating() {
            FRAME.set(&cx.resources.demo.next_animation_frame());
        } else if cx.resources.demo.is_scrolling() {
            FRAME.set(cx.resources.demo.next_scrolling_frame());
        } else {
            return
        }
        cx.resources.display.lock(|display| {
            display.set_frame(FRAME);
        });
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn SWI0();
        fn SWI1();
        fn SWI2();
        fn SWI3();
    }

};

