//! The display driver and Frame trait.

use crate::display_control::DisplayControl;
use crate::display_timer::DisplayTimer;
use crate::render::{Render, BRIGHTNESSES, MAX_BRIGHTNESS};


/// A set of matrix column indices.
///
/// Supports maximum index 15.
#[derive(Copy, Clone)]
struct ColumnSet (u16);

impl ColumnSet {

    /// Returns a new empty set.
    const fn empty() -> ColumnSet {
        ColumnSet(0)
    }

    /// Adds column index 'col' to the set.
    fn set(&mut self, col: usize) {
        self.0 |= 1<<col;
    }

    /// Returns the set as a bitmap in a u32 (LSB is index 0).
    fn as_u32(&self) -> u32 {
        self.0 as u32
    }

    /// Says whether the set is empty.
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}


/// A 'compiled' representation of the part of an image displayed on a single
/// matrix row.
///
/// RowPlans are created and contained by [`Frame`]s.
// This is effectively a map brightness -> set of columns
#[derive(Copy, Clone)]
pub struct RowPlan (
    [ColumnSet; BRIGHTNESSES],
);

impl RowPlan {

    /// Returns a new RowPlan with all LEDs brightness 0.
    pub const fn default() -> RowPlan {
        RowPlan([ColumnSet::empty(); BRIGHTNESSES])
    }

    /// Resets all LEDs to brightness 0.
    fn clear(&mut self) {
        self.0 = RowPlan::default().0;
    }

    /// Says which LEDs have the specified brightness.
    fn lit_cols(&self, brightness: u8) -> ColumnSet {
        self.0[brightness as usize]
    }

    /// Sets a single LED to the specified brightness.
    fn light_col(&mut self, brightness: u8, col: usize) {
        self.0[brightness as usize].set(col);
    }

}


/// Description of a device's LED layout.
///
/// This describes the correspondence between the visible layout of LEDs and
/// the pins controlling them.
pub trait Matrix {
    /// The number of pins connected to LED columns.
    ///
    /// At present this can be at most 16.
    const MATRIX_COLS: usize;

    /// The number of pins connected to LED rows.
    ///
    /// This should normally be a small number (eg 3).
    const MATRIX_ROWS: usize;

    // Note that nothing uses IMAGE_COLS and IMAGE_ROWS directly; having these
    // constants allows us to document them.

    /// The number of visible LED columns.
    const IMAGE_COLS: usize;

    /// The number of visible LED rows.
    const IMAGE_ROWS: usize;

    /// Returns the image coordinates (x, y) to use for the LED at (col, row).
    ///
    /// Returns None if (col, row) doesn't control an LED.
    ///
    /// Otherwise the return value is in (0..IMAGE_COLS, 0..IMAGE_ROWS), with
    /// (0, 0) representing the top left.
    ///
    /// # Panics
    ///
    /// Panics if the provided col and row are out of range 0..MATRIX_COLS and
    /// 0..MATRIX_ROWS.

    fn image_coordinates(col: usize, row: usize) -> Option<(usize, usize)>;
}


/// A 'Compiled' representation of an image to be displayed.
///
/// `Frame`s are populated from images implementing [`Render`], then passed on
/// to [`Display::set_frame()`].
///
/// Implementations of `Frame` specify the [`Matrix`] used to convert between
/// image and matrix coordinates, and act like an array of [`RowPlan`]s.
pub trait Frame: Copy + Default {

    /// The Matrix used to convert between image and matrix coordinates.
    type Mtx: Matrix;

    /// The number of pins connected to LED columns.
    const COLS: usize = Self::Mtx::MATRIX_COLS;

    /// The number of pins connected to LED rows.
    const ROWS: usize = Self::Mtx::MATRIX_ROWS;

    /// Returns a reference to the RowPlan for a row of LEDs.
    ///
    /// # Panics
    ///
    /// Panics if `row` is not in the range 0..ROWS
    fn row_plan(&self, row: usize) -> &RowPlan;

    /// Returns a mutable reference to the RowPlan for a row of LEDs.
    ///
    /// # Panics
    ///
    /// Panics if `row` is not in the range 0..ROWS
    fn row_plan_mut(&mut self, row: usize) -> &mut RowPlan;


    /// Stores a new image into the frame.
    ///
    /// Example:
    ///
    /// ```
    /// frame.set(GreyscaleImage::blank());
    /// ```
    fn set<T>(&mut self, image: &T) where T: Render + ?Sized {
        for row in 0..Self::ROWS {
            let plan = self.row_plan_mut(row);
            plan.clear();
            for col in 0..Self::COLS {
                if let Some((x, y)) = Self::Mtx::image_coordinates(col, row) {
                    let brightness = image.brightness_at(x, y);
                    plan.light_col(brightness, col);
                }
            }
        }
    }
}


// With a 16µs period, 375 ticks is 6ms
const CYCLE_TICKS: u16 = 375;

const GREYSCALE_TIMINGS: [u16; BRIGHTNESSES-2] = [
//   Delay,   Bright, Ticks, Duration, Relative power
//   375,  //   0,      0,      0µs,    ---
     373,  //   1,      2,     32µs,    inf
     371,  //   2,      4,     64µs,   200%
     367,  //   3,      8,    128µs,   200%
     360,  //   4,     15,    240µs,   187%
     347,  //   5,     28,    448µs,   187%
     322,  //   6,     53,    848µs,   189%
     273,  //   7,    102,   1632µs,   192%
     176,  //   8,    199,   3184µs,   195%
//     0,  //   9,    375,   6000µs,   188%
];

/// Starts the timer you plan to use with a [`Display`].
///
/// Call this once before using a [`Display`].
///
/// This calls the timer's
/// [`initialise_cycle()`][DisplayTimer::initialise_cycle] implementation.
pub fn initialise_timer(timer: &mut impl DisplayTimer) {
    timer.initialise_cycle(CYCLE_TICKS);
}


/// Initialises the display hardware you plan to use with a [`Display`].
///
/// Call this once before using a [`Display`].
///
/// This calls the [`DisplayControl`]'s
/// [`initialise_for_display()`][DisplayControl::initialise_for_display]
/// implementation.
pub fn initialise_control(control: &mut impl DisplayControl) {
    control.initialise_for_display();
}


/// Manages a small LED display.
///
/// There should normally be a single `Display` instance for a single piece of
/// display hardware.
///
/// Display is generic over a [`Frame`] type, which holds image data suitable
/// for display on a particular piece of hardware.
///
/// Call [`initialise_control()`] and [`initialise_timer()`] before using a
/// `Display`.
///
/// # Example
///
/// Using `cortex-m-rtfm` v0.4.1:
/// ```
/// #[app(device = microbit::hal::nrf51)]
/// const APP: () = {
///     static mut GPIO: nrf51::GPIO = ();
///     static mut TIMER1: nrf51::TIMER1 = ();
///     static mut DISPLAY: Display<MicrobitFrame> = ();
///
///     #[init]
///     fn init() -> init::LateResources {
///         let mut p: nrf51::Peripherals = device;
///         display::initialise_control(&mut p.GPIO);
///         display::initialise_timer(&mut p.TIMER1);
///         init::LateResources {
///             GPIO : p.GPIO,
///             TIMER1 : p.TIMER1,
///             DISPLAY : Display::new(),
///         }
///     }
/// }
/// ```

pub struct Display<F: Frame> {
    // index (0..F::ROWS) of the row being displayed
    row_strobe      : usize,
    // brightness level (0..=MAX_BRIGHTNESS) to process next
    next_brightness : u8,
    frame           : F,
    current_plan    : RowPlan
}

impl<F: Frame> Display<F> {

    /// Creates a Display instance, initially holding a blank image.
    pub fn new() -> Display<F> {
        return Display {
            row_strobe: 0,
            next_brightness: 0,
            frame: F::default(),
            current_plan: RowPlan::default(),
        };
    }

    /// Accepts a new image to be displayed.
    ///
    /// The code that calls this method must not be interrupting, or
    /// interruptable by, [`handle_event()`][Display::handle_event].
    ///
    /// After calling this, it's safe to modify the frame again (its data is
    /// copied into the `Display`).
    ///
    /// # Example
    ///
    /// In the style of `cortex-m-rtfm` v0.4:
    ///
    /// ```
    /// #[interrupt(priority = 1, resources = [RTC0, DISPLAY])]
    /// fn RTC0() {
    ///     static mut frame: MicrobitFrame = MicrobitFrame::const_default();
    ///     let event_reg = &resources.RTC0.events_tick;
    ///     event_reg.write(|w| unsafe {w.bits(0)} );
    ///     frame.set(GreyscaleImage::blank());
    ///     resources.DISPLAY.lock(|display| {
    ///         display.set_frame(frame);
    ///     });
    /// }
    /// ```
    pub fn set_frame(&mut self, frame: &F) {
        self.frame = *frame;
    }

    /// Updates the display for the start of a new primary cycle.
    ///
    /// Leaves the timer's secondary alarm enabled iff there are any
    /// intermediate brightnesses in the current image.
    fn render_row(&mut self,
                  control: &mut impl DisplayControl,
                  timer: &mut impl DisplayTimer) {
        assert! (self.row_strobe < F::ROWS);
        self.row_strobe += 1;
        if self.row_strobe == F::ROWS {self.row_strobe = 0};

        let plan = self.frame.row_plan(self.row_strobe);

        let lit_cols = plan.lit_cols(MAX_BRIGHTNESS);
        control.display_row_leds(self.row_strobe, lit_cols.as_u32());

        // We copy this so that we'll continue using it for the rest of this
        // 'tick' even if set_frame() is called part way through
        self.current_plan = *plan;
        self.next_brightness = MAX_BRIGHTNESS;
        self.program_next_brightness(timer);
        if self.next_brightness != 0 {
            timer.enable_secondary();
        }
    }

    /// Updates the display to represent an intermediate brightness.
    ///
    /// This is called after an interrupt from the secondary alarm.
    fn render_subrow(&mut self,
                     control: &mut impl DisplayControl,
                     timer: &mut impl DisplayTimer) {
        // When this method is called, next_brightness is an intermediate
        // brightness in the range 1..8 (the one that it's time to display).

        let additional_cols = self.current_plan.lit_cols(self.next_brightness);
        control.light_current_row_leds(additional_cols.as_u32());

        self.program_next_brightness(timer);
    }

    /// Updates next_brightness to the next (dimmer) brightness that needs
    /// displaying, and program the timer's secondary alarm correspondingly.
    ///
    /// If no further brightness needs displaying for this row, this means
    /// disabling the secondary alarm.
    fn program_next_brightness(&mut self, timer: &mut impl DisplayTimer) {
        loop {
            self.next_brightness -= 1;
            if self.next_brightness == 0 {
                timer.disable_secondary();
                break;
            }
            if !self.current_plan.lit_cols(self.next_brightness).is_empty() {
                timer.program_secondary(
                    GREYSCALE_TIMINGS[(self.next_brightness-1) as usize]
                );
                break;
            }
        }
    }

    /// Updates the LEDs and timer state during a timer interrupt.
    ///
    /// You should call this each time the timer's interrupt is signalled.
    ///
    /// The `timer` parameter must be the same each time you call this method,
    /// and the same as originally passed to [`initialise_timer()`].
    ///
    /// The `control` parameter must be the same each time you call this method,
    /// and the same as originally passed to [`initialise_control()`].
    ///
    /// As well as updating the LED state by calling [`DisplayControl`]
    /// methods on `control`, this method may update the timer state by
    /// calling the timer's
    /// [`program_secondary()`][DisplayTimer::program_secondary],
    /// [`enable_secondary()`][DisplayTimer::enable_secondary], and/or
    /// [`disable_secondary()`][DisplayTimer::disable_secondary] methods.
    ///
    /// This method uses the timer's
    /// [`check_primary()`][DisplayTimer::check_primary] and
    /// [`check_secondary()`][DisplayTimer::check_secondary] methods. Note
    /// that the implementation of these methods for `TIMER1` takes care of
    /// clearing the timer's event registers.
    ///
    /// # Example
    ///
    /// In the style of `cortex-m-rtfm` v0.4:
    ///
    /// ````
    /// #[interrupt(priority = 2, resources = [TIMER1, GPIO, DISPLAY])]
    /// fn TIMER1() {
    ///    resources.DISPLAY.handle_event(resources.TIMER1, resources.GPIO);
    /// }
    /// ````
    pub fn handle_event(&mut self,
                        timer: &mut impl DisplayTimer,
                        control: &mut impl DisplayControl) {
        let row_timer_fired = timer.check_primary();
        let brightness_timer_fired = timer.check_secondary();
        if row_timer_fired {
            self.render_row(control, timer);
        } else if brightness_timer_fired {
            self.render_subrow(control, timer);
        }

    }

}

