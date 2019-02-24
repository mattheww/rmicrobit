//! Implementation of [`DisplayTimer`] for the nrf51 `TIMER1`.
//!
//! [`DisplayTimer`]: crate::display_timer::DisplayTimer

use microbit::hal::nrf51;

use crate::display_timer::DisplayTimer;

// Check whether the event for a CC register has been generated, then clear
// the event register.
fn check_cc(timer: &mut nrf51::TIMER1, index: usize) -> bool {
    let event_reg = &timer.events_compare[index];
    let fired = event_reg.read().bits() != 0;
    if fired {event_reg.write(|w| unsafe {w.bits(0)} )}
    return fired;
}

/// Implementation of [`DisplayTimer`] for the nrf51 `TIMER1`.
///
/// The timer is set to 16-bit mode, using a 62.5kHz clock (16 µs ticks).
///
/// Uses CC0 for the primary cycle and CC1 for the secondary alarm. Uses the
/// CC0_CLEAR shortcut to implement the primary cycle.
///
/// The `initialise_cycle` implementation assumes the timer is in the state it
/// would have after system reset.
impl DisplayTimer for nrf51::TIMER1 {

    fn initialise_cycle(&mut self, ticks: u16) {
        self.prescaler.write(|w| unsafe { w.bits(8) });
        self.cc[0].write(|w| unsafe { w.bits(ticks as u32) });
        self.bitmode.write(|w| w.bitmode()._32bit());
        self.shorts.write(|w| w.compare0_clear().enabled());
        self.intenset.write(|w| w.compare0().set());
        self.tasks_start.write(|w| unsafe { w.bits(1) });
    }

    fn enable_secondary(&mut self) {
        self.intenset.write(|w| w.compare1().set());
    }

    fn disable_secondary(&mut self) {
        self.intenclr.write(|w| w.compare1().clear());
    }

    fn program_secondary(&mut self, ticks: u16) {
        self.cc[1].write(|w| unsafe { w.bits(ticks as u32) });
    }

    fn check_primary(&mut self) -> bool {
        return check_cc(self, 0);
    }

    fn check_secondary(&mut self) -> bool {
        return check_cc(self, 1);
    }

}

