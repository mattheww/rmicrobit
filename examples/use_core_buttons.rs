#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use microbit::hal::nrf51;
use microbit_blinkenlights::prelude::*;
use microbit_blinkenlights::{self, Display, DisplayPort, MicrobitDisplayTimer, MicrobitFrame, Render};
use microbit_blinkenlights::font;
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


#[app(device = microbit::hal::nrf51)]
const APP: () = {

    static mut DISPLAY_PORT: DisplayPort = ();
    static mut DISPLAY_TIMER: MicrobitDisplayTimer<nrf51::TIMER1> = ();
    static mut DISPLAY: Display<MicrobitFrame> = ();
    static mut BUTTON_A: ButtonA = ();
    static mut BUTTON_B: ButtonB = ();
    static mut DEMO: DemoState = ();

    #[init]
    fn init() -> init::LateResources {
        let _core: rtfm::Peripherals = core;
        let p: nrf51::Peripherals = device;

        let PinsByKind {display_pins, button_pins, ..} = p.GPIO.split_by_kind();
        let mut display_port = DisplayPort::new(display_pins);
        let (button_a, button_b) = buttons::from_pins(button_pins);
        let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
        microbit_blinkenlights::initialise_display(&mut timer, &mut display_port);
        let demo = DemoState{letter: b'-'};

        init::LateResources {
            DISPLAY_PORT : display_port,
            DISPLAY_TIMER : timer,
            DISPLAY : {
                let mut frame = MicrobitFrame::const_default();
                frame.set(&demo.current_graphic());
                let mut display = Display::new();
                display.set_frame(&frame);
                display
            },
            BUTTON_A : button_a,
            BUTTON_B : button_b,
            DEMO: demo
        }
    }

    #[interrupt(priority = 2,
                spawn = [handle_buttons],
                resources = [DISPLAY_TIMER, DISPLAY_PORT, DISPLAY])]
    fn TIMER1() {
        let display_event = microbit_blinkenlights::handle_display_event(
            &mut resources.DISPLAY,
            resources.DISPLAY_TIMER,
            resources.DISPLAY_PORT,
        );
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

