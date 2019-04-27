//! Example of working with the 'with hold' dual-button monitor.

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
use rmicrobit::buttons::dual_with_hold::{
    ABMonitor, ButtonEvent};

pub struct DemoState {
    letter: u8,
}

impl DemoState {
    fn current_graphic(&self) -> impl Render{
        font::character(self.letter)
    }

    fn set_letter(&mut self, letter: u8) {
        if self.letter == letter {
            self.letter.make_ascii_lowercase();
        } else {
            self.letter = letter;
        }
    }

    fn handle(&mut self, event: ButtonEvent) {
        match event {
            ButtonEvent::ClickA => {
                self.set_letter(b'A');
            },
            ButtonEvent::ClickB => {
                self.set_letter(b'B');
            },
            ButtonEvent::ClickAB => {
                self.set_letter(b'X');
            },
            ButtonEvent::HoldA => {
                self.set_letter(b'H');
            },
            ButtonEvent::HoldB => {
                self.set_letter(b'I');
            },
            ButtonEvent::HoldAB => {
                self.set_letter(b'Y');
            },
        }
     }
}


#[app(device = rmicrobit::nrf51)]
const APP: () = {

    static mut MONITOR: ABMonitor = ();
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
        let monitor = ABMonitor::new(button_a, button_b);
        let demo = DemoState{letter: b'-'};

        let mut frame = MicrobitFrame::const_default();
        frame.set(&demo.current_graphic());
        display.set_frame(&frame);

        init::LateResources {
            DISPLAY : display,
            MONITOR : monitor,
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
           resources = [DISPLAY, MONITOR, DEMO])]
    fn handle_buttons() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        if let Some(event) = resources.MONITOR.poll() {
            resources.DEMO.handle(event);
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

