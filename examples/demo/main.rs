#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit::hal::nrf51;

use microbit_blinkenlights::display::{self, Display, Frame};


mod animation;
mod buttons;
mod demo;


#[app(device = microbit::hal::nrf51)]
const APP: () = {

    static mut GPIO: nrf51::GPIO = ();
    static mut GPIOTE: nrf51::GPIOTE = ();
    static mut TIMER1: nrf51::TIMER1 = ();
    static mut TIMER2: nrf51::TIMER2 = ();
    static mut RTC0: nrf51::RTC0 = ();
    static mut DISPLAY: Display = ();
    static mut DEMO: demo::Demo = ();

    #[init]
    fn init() -> init::LateResources {
        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;

        // nrf51 peripherals
        let mut p: nrf51::Peripherals = device;

        buttons::initialise_pins(&mut p);
        display::initialise_pins(&mut p);
        display::initialise_timer(&mut p.TIMER1);

        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        // 16Hz; 62.5ms period
        p.RTC0.prescaler.write(|w| unsafe {w.bits(2047)});
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

        init::LateResources {
            GPIO : p.GPIO,
            GPIOTE : p.GPIOTE,
            TIMER1 : p.TIMER1,
            TIMER2 : p.TIMER2,
            RTC0 : p.RTC0,
            DEMO : demo::Demo::new(),
            DISPLAY : {
                let mut frame = Frame::default();
                frame.set(demo::initial_frame());
                let mut display = Display::new();
                display.set_frame(&frame);
                display
            },
        }
    }

    #[interrupt(priority = 2,
                resources = [TIMER1, GPIO, DISPLAY])]
    fn TIMER1() {
        resources.DISPLAY.handle_event(resources.TIMER1, &mut resources.GPIO);
    }

    #[interrupt(priority = 1,
                resources = [RTC0, DISPLAY, DEMO])]
    fn RTC0() {
        static mut frame: Frame = Frame::default();

        let event_reg = &resources.RTC0.events_tick;
        event_reg.write(|w| unsafe {w.bits(0)} );
        if resources.DEMO.is_animating() {
            frame.set(&resources.DEMO.next_animation_frame());
        } else if resources.DEMO.is_scrolling() {
            frame.set(resources.DEMO.next_scrolling_frame());
        } else {
            return
        }
        resources.DISPLAY.lock(|display| {
            display.set_frame(frame);
        });
    }

    #[interrupt(priority = 1,
                resources = [GPIOTE, TIMER2, DISPLAY, DEMO])]
    fn GPIOTE() {
        static mut frame: Frame = Frame::default();

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
                frame.set(resources.DEMO.current_image());
                resources.DISPLAY.lock(|display| {
                    display.set_frame(frame);
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

