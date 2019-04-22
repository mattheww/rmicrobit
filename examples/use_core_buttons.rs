#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit_blinkenlights::nrf51;
use microbit_blinkenlights::prelude::*;
use microbit_blinkenlights::display::{
    DisplayPort, MicrobitDisplay, MicrobitFrame, Render};
use microbit_blinkenlights::graphics::font;
use microbit_blinkenlights::gpio::PinsByKind;
use microbit_blinkenlights::buttons;
use microbit_blinkenlights::buttons::core::TransitionEvent;
use microbit_blinkenlights::buttons::builtin::{ButtonA, ButtonB};

pub struct DemoState {
    letter: u8,
}

impl DemoState {
    fn current_graphic(&self) -> impl Render{
        font::character(self.letter)
    }

    fn handle_a(&mut self, event:TransitionEvent) {
        self.letter = match event {
            TransitionEvent::Press => b'A',
            TransitionEvent::Release => b'a',
        }
    }

    fn handle_b(&mut self, event:TransitionEvent) {
        self.letter = match event {
            TransitionEvent::Press => b'B',
            TransitionEvent::Release => b'b',
        }
    }

}


#[app(device = microbit_blinkenlights::nrf51)]
const APP: () = {

    static mut BUTTON_A: ButtonA = ();
    static mut BUTTON_B: ButtonB = ();
    static mut DISPLAY: MicrobitDisplay<nrf51::TIMER1> = ();
    static mut DEMO: DemoState = ();

    #[init]
    fn init() -> init::LateResources {
        let _core: rtfm::Peripherals = core;
        let p: nrf51::Peripherals = device;

        let PinsByKind {display_pins, button_pins, ..} = p.GPIO.split_by_kind();
        let display_port = DisplayPort::new(display_pins);
        let (button_a, button_b) = buttons::from_pins(button_pins);
        let mut display = MicrobitDisplay::new(display_port, p.TIMER1);
        let demo = DemoState{letter: b'-'};

        let mut frame = MicrobitFrame::const_default();
        frame.set(&demo.current_graphic());
        display.set_frame(&frame);

        init::LateResources {
            DISPLAY : display,
            BUTTON_A : button_a,
            BUTTON_B : button_b,
            DEMO: demo
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
           resources = [DISPLAY, BUTTON_A, BUTTON_B, DEMO])]
    fn handle_buttons() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        let button_a = resources.BUTTON_A;
        let button_b = resources.BUTTON_B;
        let mut invalidated = false;
        if let Some(event) = button_a.poll_event() {
            resources.DEMO.handle_a(event);
            invalidated = true;
        }
        if let Some(event) = button_b.poll_event() {
            resources.DEMO.handle_b(event);
            invalidated = true;
        }
        if invalidated {
            FRAME.set(&resources.DEMO.current_graphic());
            resources.DISPLAY.lock(|display| {
                display.set_frame(FRAME);
            });
        }
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn SWI0();
        fn SWI1();
        fn SWI2();
        fn SWI3();
    }

};

