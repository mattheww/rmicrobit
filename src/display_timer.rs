//! The interface that [`Display`] needs to work with a timer.
//!
//! [`Display`]: crate::display::Display


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

