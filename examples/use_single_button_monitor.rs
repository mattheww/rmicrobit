//! Example of working with the simplest single-button monitors.

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
use microbit_blinkenlights::buttons::single_lazy::{
    ButtonAMonitor, ButtonBMonitor, ButtonEvent};
// Alternatively, use this to receive events on press rather than release:
// use microbit_blinkenlights::buttons::single_eager::{
//     ButtonAMonitor, ButtonBMonitor, ButtonEvent};


pub struct DemoState {
    letter: u8,
}

impl DemoState {
    fn current_graphic(&self) -> impl Render{
        font::character(self.letter)
    }

    fn handle_click_a(&mut self) {
        self.letter = match self.letter {
            b'A' => b'a',
            _ => b'A',
        };
     }

    fn handle_click_b(&mut self) {
        self.letter = match self.letter {
            b'B' => b'b',
            _ => b'B',
        };
    }

}


#[app(device = microbit_blinkenlights::nrf51)]
const APP: () = {

    static mut MONITOR_A: ButtonAMonitor = ();
    static mut MONITOR_B: ButtonBMonitor = ();
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
        let monitor_a = ButtonAMonitor::new(button_a);
        let monitor_b = ButtonBMonitor::new(button_b);
        let demo = DemoState{letter: b'-'};

        let mut frame = MicrobitFrame::const_default();
        frame.set(&demo.current_graphic());
        display.set_frame(&frame);

        init::LateResources {
            DISPLAY : display,
            MONITOR_A : monitor_a,
            MONITOR_B : monitor_b,
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
           resources = [DISPLAY, MONITOR_A, MONITOR_B, DEMO])]
    fn handle_buttons() {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        let monitor_a = resources.MONITOR_A;
        let monitor_b = resources.MONITOR_B;
        let mut invalidated = false;
        // Note the only possible event with these monitors is Click.
        if let Some(ButtonEvent::Click) = monitor_a.poll() {
            resources.DEMO.handle_click_a();
            invalidated = true;
        }
        if let Some(ButtonEvent::Click) = monitor_b.poll() {
            resources.DEMO.handle_click_b();
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

