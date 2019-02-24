//! The interface that [`Display`] needs to work with a timer.
//!
//! [`Display`]: crate::display::Display


use microbit::hal::nrf51;

/// The interface that [`Display`] needs to work with a timer.
///
/// The timer should count _ticks_ of 16µs.
///
/// It should reset itself after the number of ticks passed to
/// `initialise_cycle()` have elapsed (the _primary cycle_), and signal an
/// interrupt.
///
/// It should also provide a _secondary alarm_ which can be programmed to
/// signal an interrupt at a specified point during the primary cycle.
///
/// If you use a tick period other than 16µs, the display as a whole will run
/// correspondingly faster or slower.
///
/// If you provide a 'no-op' implementation of the secondary-alarm features,
/// the effect will be a display which treats brightness level 9 as on and all
/// other levels as off.
///
/// [`Display`]: crate::display::Display

pub trait DisplayTimer {

    /// Initialises the timer.
    ///
    /// This is intended to be called once, before using the display.
    ///
    /// The `ticks` parameter is the number of ticks in the primary cycle.
    ///
    /// Leaves the secondary alarm disabled.
    fn initialise_cycle(&mut self, ticks: u16);

    /// Enables the secondary alarm.
    ///
    /// After this is called, an interrupt should be generated each time the
    /// tick last passed to [`program_secondary()`]is reached.
    ///
    /// [`program_secondary()`]: DisplayTimer::program_secondary
    fn enable_secondary(&mut self);

    /// Disables the secondary alarm.
    ///
    /// After this is called, no more interrupts should be generated from the
    /// secondary alarm until it is enabled again.
    fn disable_secondary(&mut self);

    /// Specifies the tick to use for the secondary alarm.
    ///
    /// Note that `ticks` represents the time after the start of the primary
    /// cycle (not the time after the previous secondary signal).
    fn program_secondary(&mut self, ticks: u16);

    /// Checks whether a new primary cycle has begun since the last call to
    /// this method.
    fn check_primary(&mut self) -> bool;

    /// Checks whether the secondary alarm has signalled an interrupt since
    /// the last call to this method.
    fn check_secondary(&mut self) -> bool;

}

// Check whether the event for a CC register has been generated, then clear
// the event register.
fn check_cc(timer: &mut nrf51::TIMER1, index: usize) -> bool {
    let event_reg = &timer.events_compare[index];
    let fired = event_reg.read().bits() != 0;
    if fired {event_reg.write(|w| unsafe {w.bits(0)} )}
    fired
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
        check_cc(self, 0)
    }

    fn check_secondary(&mut self) -> bool {
        check_cc(self, 1)
    }

}

