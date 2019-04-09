#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit::hal::nrf51;
use microbit_blinkenlights::prelude::*;
use microbit_blinkenlights::{self, Display, DisplayPort, MicrobitDisplayTimer, MicrobitFrame};
use microbit_blinkenlights::gpio::PinsByKind;
use microbit::hal::lo_res_timer::{LoResTimer, FREQ_16HZ};

mod animation;
mod buttons;
mod demo;


#[app(device = microbit::hal::nrf51)]
const APP: () = {

    static mut DISPLAY_PORT: DisplayPort = ();
    static mut GPIOTE: nrf51::GPIOTE = ();
    static mut DISPLAY_TIMER: MicrobitDisplayTimer<nrf51::TIMER1> = ();

    static mut TIMER2: nrf51::TIMER2 = ();
    static mut ANIM_TIMER: LoResTimer<nrf51::RTC0> = ();
    static mut DISPLAY: Display<MicrobitFrame> = ();
    static mut DEMO: demo::Demo = ();

    #[init]
    fn init() -> init::LateResources {
        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;

        // nrf51 peripherals
        let mut p: nrf51::Peripherals = device;

        buttons::initialise_pins(&mut p);

        let PinsByKind {display_pins, ..} = p.GPIO.split_by_kind();
        let mut display_port = DisplayPort::new(display_pins);

        let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
        microbit_blinkenlights::initialise_display(&mut timer, &mut display_port);

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
            DISPLAY_PORT : display_port,
            GPIOTE : p.GPIOTE,
            DISPLAY_TIMER : timer,
            TIMER2 : p.TIMER2,
            ANIM_TIMER : rtc0,
            DEMO : demo::Demo::new(),
            DISPLAY : {
                let mut frame = MicrobitFrame::const_default();
                frame.set(demo::initial_frame());
                let mut display = Display::new();
                display.set_frame(&frame);
                display
            },
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

    #[interrupt(priority = 1,
                resources = [GPIOTE, TIMER2, DISPLAY, DEMO])]
    fn GPIOTE() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();

        let a_pressed = buttons::a_pressed(
            &mut resources.GPIOTE, &mut resources.TIMER2);
        let b_pressed = buttons::b_pressed(
            &mut resources.GPIOTE, &mut resources.TIMER2);

        if a_pressed {
            resources.DEMO.next_state();
        } else if b_pressed {
            resources.DEMO.next_state_or_modify_current_state();
        }
        if a_pressed || b_pressed {
            if resources.DEMO.is_static() {
                FRAME.set(resources.DEMO.current_image());
                resources.DISPLAY.lock(|display| {
                    display.set_frame(FRAME);
                });
            }
        }
    }

    #[interrupt(priority = 1, resources = [GPIOTE, TIMER2])]
    fn TIMER2() {
        buttons::handle_debounce_timer(
            &mut resources.GPIOTE, &mut resources.TIMER2);
    }

};

