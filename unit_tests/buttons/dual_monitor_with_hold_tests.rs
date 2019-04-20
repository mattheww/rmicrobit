#![allow(non_snake_case)]

use super::*;
use crate::buttons::core::Transition;
use crate::buttons::monitors::holding::{HoldAnnotator, DefaultHoldDescriptor};

struct Monitor {
    button_a_is_pressed: bool,
    button_b_is_pressed: bool,
    hold_annotator_a: HoldAnnotator<DefaultHoldDescriptor>,
    hold_annotator_b: HoldAnnotator<DefaultHoldDescriptor>,
    state: MonitorState,
}

impl Monitor {
    fn new() -> Monitor {
        Monitor {
            button_a_is_pressed: false,
            button_b_is_pressed: false,
            hold_annotator_a: HoldAnnotator::new(),
            hold_annotator_b: HoldAnnotator::new(),
            state: MonitorState::new(),
        }
    }

    fn _do_a(&mut self, transition: Transition) -> Option<Event> {
        match self.hold_annotator_a.annotate(transition) {
            Some(event) => {
                println!("A: {:?}", event);
                self.state.handle_a(event, self.button_b_is_pressed)
            }
            None => None
        }
    }

    fn _do_b(&mut self, transition: Transition) -> Option<Event> {
        match self.hold_annotator_b.annotate(transition) {
            Some(event) => {
                println!("B: {:?}", event);
                self.state.handle_b(event, self.button_a_is_pressed)
            }
            None => None
        }
    }

    fn press_a(&mut self) -> Option<Event> {
        assert!(!self.button_a_is_pressed);
        self.button_a_is_pressed = true;
        self._do_a(Transition {was_pressed: false, is_pressed: true})
    }

    fn release_a(&mut self) -> Option<Event> {
        assert!(self.button_a_is_pressed);
        self.button_a_is_pressed = false;
        self._do_a(Transition {was_pressed: true, is_pressed: false})
    }

    fn press_b(&mut self) -> Option<Event> {
        assert!(!self.button_b_is_pressed);
        self.button_b_is_pressed = true;
        self._do_b(Transition {was_pressed: false, is_pressed: true})
    }

    fn release_b(&mut self) -> Option<Event> {
        assert!(self.button_b_is_pressed);
        self.button_b_is_pressed = false;
        self._do_b(Transition {was_pressed: true, is_pressed: false})
    }

    fn tick_a(&mut self) -> Option<Event> {
        self._do_a(match self.button_a_is_pressed {
            true => Transition {was_pressed: true, is_pressed: true},
            false => Transition {was_pressed: false, is_pressed: false},
        })
    }

    fn tick_b(&mut self) -> Option<Event> {
        self._do_b(match self.button_b_is_pressed {
            true => Transition {was_pressed: true, is_pressed: true},
            false => Transition {was_pressed: false, is_pressed: false},
        })
    }

    fn ticks(&mut self, ticks: usize) {
        for tick in 0..ticks {
            assert_eq!(self.tick_a(), None, "ticking A after {}", tick);
            assert_eq!(self.tick_b(), None, "ticking B after {}", tick);
        }
    }
}


#[test]
fn simple_a_click() {
    let mut m = Monitor::new();
    // Simple A click
    assert_eq!(m.press_a(), None);
    assert_eq!(m.tick_b(), None);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), None);
    assert_eq!(m.release_a(), Some(Event::ClickA));
    m.ticks(300);
}

#[test]
fn simple_b_click() {
    let mut m = Monitor::new();
    // Simple B click
    assert_eq!(m.press_b(), None);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), None);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.release_b(), Some(Event::ClickB));
    m.ticks(300);
}

#[test]
fn simple_a_hold() {
    let mut m = Monitor::new();
    // Simple A hold
    assert_eq!(m.press_a(), None);
    m.ticks(249);
    assert_eq!(m.tick_a(), Some(Event::HoldA));
    m.ticks(1000);
    assert_eq!(m.release_a(), None);
    m.ticks(300);
}

#[test]
fn simple_b_hold() {
    let mut m = Monitor::new();
    // Simple B hold
    assert_eq!(m.press_b(), None);
    m.ticks(249);
    assert_eq!(m.tick_b(), Some(Event::HoldB));
    m.ticks(1000);
    assert_eq!(m.release_b(), None);
    m.ticks(300);
}

#[test]
fn click_Abab() {
    let mut m = Monitor::new();
    // AB click (ABab)
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn click_ABba() {
    let mut m = Monitor::new();
    // AB click (ABba)
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn click_BAba() {
    let mut m = Monitor::new();
    // AB click (BAba)
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn click_BAab() {
    let mut m = Monitor::new();
    // AB click (BAab)
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn click_ABbBba() {
    let mut m = Monitor::new();
    // AB click (ABbBba)
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn click_ABaAba() {
    let mut m = Monitor::new();
    // AB click (ABaAba)
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn hold_AB_ab() {
    let mut m = Monitor::new();
    // AB hold (AB_ab)
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(249);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), Some(Event::HoldAB));
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(300);
}

#[test]
fn hold_AB_ba() {
    let mut m = Monitor::new();
    // AB hold (AB_ba)
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(249);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), Some(Event::HoldAB));
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(300);
}

#[test]
fn hold_A_B_ab() {
    let mut m = Monitor::new();
    // A hold despite long B press later (A_B_ab)
    assert_eq!(m.press_a(), None);
    m.ticks(249);
    assert_eq!(m.tick_a(), Some(Event::HoldA));
    assert_eq!(m.tick_b(), None);
    assert_eq!(m.press_b(), None);
    m.ticks(300);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(300);
}

#[test]
fn nohold_AB_ba() {
    let mut m = Monitor::new();
    // AB_ba, with hold timeout passing for A only
    // Not obvious what the user wants here; choosing ClickAB.
    assert_eq!(m.press_a(), None);
    m.ticks(200);
    assert_eq!(m.press_b(), None);
    // A's timeout passes here
    m.ticks(100);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn nohold_ABb_a() {
    let mut m = Monitor::new();
    // ABb_a
    // Not obvious what the user wants here; choosing ClickAB.
    assert_eq!(m.press_a(), None);
    m.ticks(10);
    assert_eq!(m.press_b(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    // A's timeout passes here
    m.ticks(300);
    assert_eq!(m.release_a(), Some(Event::ClickAB));
    m.ticks(300);
}

#[test]
fn hold_AxB_a_b() {
    let mut m = Monitor::new();
    // AB Hold: A-B_a_b
    assert_eq!(m.press_a(), None);
    m.ticks(100);
    assert_eq!(m.press_b(), None);
    // A's timeout passes here
    m.ticks(200);
    assert_eq!(m.release_a(), None);
    m.ticks(49);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), Some(Event::HoldAB));
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(300);
}

#[test]
fn hold_AxB_a_A_ab() {
    let mut m = Monitor::new();
    // AB Hold: A-B_a_A_ab
    assert_eq!(m.press_a(), None);
    m.ticks(100);
    assert_eq!(m.press_b(), None);
    // A's timeout passes here
    m.ticks(200);
    assert_eq!(m.release_a(), None);
    m.ticks(49);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), Some(Event::HoldAB));
    m.ticks(10);
    assert_eq!(m.press_a(), None);
    // A's timeout passes again here
    m.ticks(300);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(300);
}

#[test]
fn hold_AxB_aA_ab() {
    let mut m = Monitor::new();
    // AB Hold: A-B_aA_ab
    assert_eq!(m.press_a(), None);
    m.ticks(100);
    assert_eq!(m.press_b(), None);
    // A's timeout passes here
    m.ticks(200);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.press_a(), None);
    m.ticks(39);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), Some(Event::HoldAB));
    // A's timeout passes again here
    m.ticks(300);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.release_b(), None);
    m.ticks(300);
}

#[test]
fn hold_AxB_aA_ba() {
    let mut m = Monitor::new();
    // AB Hold: A-B_aA_ba
    assert_eq!(m.press_a(), None);
    m.ticks(100);
    assert_eq!(m.press_b(), None);
    // Both timeouts pass here
    m.ticks(249);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), Some(Event::HoldAB));
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(10);
    assert_eq!(m.press_a(), None);
    // A's timeout passes again here
    m.ticks(300);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(300);
}

#[test]
fn hold_AxB_bB_ba() {
    let mut m = Monitor::new();
    // "Try to get HoldAB but slip off B and try again"
    // HoldAB: A-B_bB_ba
    assert_eq!(m.press_a(), None);
    m.ticks(100);
    assert_eq!(m.press_b(), None);
    // A's timeout passes here
    m.ticks(200);
    assert_eq!(m.release_b(), None);
    m.ticks(50);
    assert_eq!(m.press_b(), None);
    m.ticks(249);
    assert_eq!(m.tick_a(), None);
    assert_eq!(m.tick_b(), Some(Event::HoldAB));
    m.ticks(50);
    assert_eq!(m.release_b(), None);
    m.ticks(10);
    assert_eq!(m.release_a(), None);
    m.ticks(300);
}

