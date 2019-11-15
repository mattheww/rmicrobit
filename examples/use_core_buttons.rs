#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use rmicrobit::nrf51;
use rmicrobit::prelude::*;
use rmicrobit::display::{
    DisplayPort, MicrobitDisplay, MicrobitFrame, Render};
use rmicrobit::graphics::font;
use rmicrobit::gpio::PinsByKind;
use rmicrobit::buttons;
use rmicrobit::buttons::core::TransitionEvent;
use rmicrobit::buttons::builtin::{ButtonA, ButtonB};

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


#[app(device = rmicrobit::nrf51, peripherals = true)]
const APP: () = {

    struct Resources {
        button_a: ButtonA,
        button_b: ButtonB,
        display: MicrobitDisplay<nrf51::TIMER1>,
        demo: DemoState,
    }


    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let p: nrf51::Peripherals = cx.device;

        let PinsByKind {display_pins, button_pins, ..} = p.GPIO.split_by_kind();
        let display_port = DisplayPort::new(display_pins);
        let (button_a, button_b) = buttons::from_pins(button_pins);
        let mut display = MicrobitDisplay::new(display_port, p.TIMER1);
        let demo = DemoState{letter: b'-'};

        let mut frame = MicrobitFrame::const_default();
        frame.set(&demo.current_graphic());
        display.set_frame(&frame);

        init::LateResources {
            display : display,
            button_a : button_a,
            button_b : button_b,
            demo: demo
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
           resources = [display, button_a, button_b, demo])]
    fn handle_buttons(mut cx: handle_buttons::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        let button_a = cx.resources.button_a;
        let button_b = cx.resources.button_b;
        let mut invalidated = false;
        if let Some(event) = button_a.poll_event() {
            cx.resources.demo.handle_a(event);
            invalidated = true;
        }
        if let Some(event) = button_b.poll_event() {
            cx.resources.demo.handle_b(event);
            invalidated = true;
        }
        if invalidated {
            FRAME.set(&cx.resources.demo.current_graphic());
            cx.resources.display.lock(|display| {
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

