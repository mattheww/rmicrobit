//! Example of working with the simplest single-button monitors.

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
use rmicrobit::buttons::single_lazy::{
    ButtonAMonitor, ButtonBMonitor, ButtonEvent};
// Alternatively, use this to receive events on press rather than release:
// use rmicrobit::buttons::single_eager::{
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


#[app(device = rmicrobit::nrf51, peripherals = true)]
const APP: () = {

    struct Resources {
        monitor_a: ButtonAMonitor,
        monitor_b: ButtonBMonitor,
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
        let monitor_a = ButtonAMonitor::new(button_a);
        let monitor_b = ButtonBMonitor::new(button_b);
        let demo = DemoState{letter: b'-'};

        let mut frame = MicrobitFrame::const_default();
        frame.set(&demo.current_graphic());
        display.set_frame(&frame);

        init::LateResources {
            display : display,
            monitor_a : monitor_a,
            monitor_b : monitor_b,
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
           resources = [display, monitor_a, monitor_b, demo])]
    fn handle_buttons(mut cx: handle_buttons::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        let monitor_a = cx.resources.monitor_a;
        let monitor_b = cx.resources.monitor_b;
        let mut invalidated = false;
        // Note the only possible event with these monitors is Click.
        if let Some(ButtonEvent::Click) = monitor_a.poll() {
            cx.resources.demo.handle_click_a();
            invalidated = true;
        }
        if let Some(ButtonEvent::Click) = monitor_b.poll() {
            cx.resources.demo.handle_click_b();
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

