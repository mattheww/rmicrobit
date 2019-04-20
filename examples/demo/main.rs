#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit::hal::nrf51;
use microbit_blinkenlights::prelude::*;
use microbit_blinkenlights::display::{DisplayPort, MicrobitDisplay, MicrobitFrame};
use microbit_blinkenlights::gpio::PinsByKind;
use microbit::hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
use microbit_blinkenlights::buttons;
use microbit_blinkenlights::buttons::dual_with_hold::ABMonitor;

mod animation;
mod demo;


#[app(device = microbit::hal::nrf51)]
const APP: () = {

    static mut ANIM_TIMER: LoResTimer<nrf51::RTC0> = ();
    static mut DISPLAY: MicrobitDisplay<nrf51::TIMER1> = ();
    static mut BUTTON_MONITOR: ABMonitor = ();
    static mut DEMO: demo::Demo = ();

    #[init]
    fn init() -> init::LateResources {
        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;

        // nrf51 peripherals
        let p: nrf51::Peripherals = device;

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
            DISPLAY : display,
            ANIM_TIMER : rtc0,
            DEMO : demo::Demo::new(),
            BUTTON_MONITOR : button_monitor,
        }
    }

    #[interrupt(priority = 2,
                spawn = [handle_buttons],
                resources = [DISPLAY])]
    fn TIMER1() {
        let display_event = resources.DISPLAY.handle_event();
        if display_event.is_new_row() {
            spawn.handle_buttons().ok();
        }
    }

    #[task(priority = 1,
           resources = [DISPLAY, BUTTON_MONITOR, DEMO])]
    fn handle_buttons() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        if let Some(event) = resources.BUTTON_MONITOR.poll() {
            resources.DEMO.handle_button_event(event);
            if resources.DEMO.is_static() {
                FRAME.set(resources.DEMO.current_image());
                resources.DISPLAY.lock(|display| {
                    display.set_frame(FRAME);
                });
            }
        }
    }

    #[interrupt(priority = 1,
                resources = [ANIM_TIMER, DISPLAY, DEMO])]
    fn RTC0() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();

        &resources.ANIM_TIMER.clear_tick_event();
        if resources.DEMO.is_animating() {
            FRAME.set(&resources.DEMO.next_animation_frame());
        } else if resources.DEMO.is_scrolling() {
            FRAME.set(resources.DEMO.next_scrolling_frame());
        } else {
            return
        }
        resources.DISPLAY.lock(|display| {
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

