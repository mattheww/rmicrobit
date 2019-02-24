//! The display driver and Frame type.

use microbit::hal::nrf51;

use crate::display_timer::DisplayTimer;
use crate::render::{Render, BRIGHTNESSES, MAX_BRIGHTNESS};

const fn bit_range(lo: usize, count: usize) -> u32 {
    ((1<<count) - 1) << lo
}

const COLS : usize = 9;
const FIRST_COL_PIN : usize = 4;
const LAST_COL_PIN : usize = FIRST_COL_PIN + COLS - 1;
const COL_BITS : u32 = bit_range(FIRST_COL_PIN, COLS);

const ROWS : usize = 3;
const FIRST_ROW_PIN : usize = 13;
const LAST_ROW_PIN : usize = FIRST_ROW_PIN + ROWS - 1;
const ROW_BITS : u32 = bit_range(FIRST_ROW_PIN, ROWS);

/// Gives the LED (x, y) coordinates for a given pin row and column.
/// The origin is in the top-left.
const LED_LAYOUT: [[Option<(usize, usize)>; 3]; 9] = [
    [Some((0, 0)), Some((4, 2)), Some((2, 4))],
    [Some((2, 0)), Some((0, 2)), Some((4, 4))],
    [Some((4, 0)), Some((2, 2)), Some((0, 4))],
    [Some((4, 3)), Some((1, 0)), Some((0, 1))],
    [Some((3, 3)), Some((3, 0)), Some((1, 1))],
    [Some((2, 3)), Some((3, 4)), Some((2, 1))],
    [Some((1, 3)), Some((1, 4)), Some((3, 1))],
    [Some((0, 3)), None,         Some((4, 1))],
    [Some((1, 2)), None,         Some((3, 2))],
];


#[derive(Copy, Clone)]
struct ColumnSet (u16);

impl ColumnSet {

    const fn empty() -> ColumnSet {
        ColumnSet(0)
    }

    fn set(&mut self, col: usize) {
        self.0 |= 1<<col;
    }

    fn as_pins(&self) -> u32 {
        (self.0 as u32) << FIRST_COL_PIN
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}


/// Effectively a map brightness -> set of columns
#[derive(Copy, Clone)]
struct GreyscalePlan (
    [ColumnSet; BRIGHTNESSES],
);

impl GreyscalePlan {

    const fn default() -> GreyscalePlan {
        GreyscalePlan([ColumnSet::empty(); BRIGHTNESSES])
    }

    fn from_image_row<T>(row: usize, image: &T) -> GreyscalePlan
            where T: Render + ?Sized {
        let mut plan = GreyscalePlan::default();
        for col in 0..COLS {
            if let Some((x, y)) = LED_LAYOUT[col][row] {
                let brightness = image.brightness_at(x, y) as usize;
                plan.0[brightness].set(col);
            }
        }
        plan
    }

    fn lit_cols(&self, brightness: usize) -> ColumnSet {
        self.0[brightness]
    }

}

/// 'Compiled' representation of a 5×5 image to be displayed.
///
/// `Frame`s are populated from images implementing [`Render`], then passed on
/// to [`Display::set_frame()`].
#[derive(Copy, Clone)]
pub struct Frame (
    [GreyscalePlan; ROWS],
);

impl Frame {

    /// Return a new frame, initially blank.
    pub const fn default() -> Frame {
        Frame([GreyscalePlan::default(); ROWS])
    }

    fn get_plan(&self, row: usize) -> &GreyscalePlan {
        &self.0[row]
    }

    /// Store a new image into the frame.
    ///
    /// Example:
    ///
    /// ```
    /// frame.set(GreyscaleImage::blank());
    /// ```
    pub fn set<T>(&mut self, image: &T) where T: Render + ?Sized {
        for row in 0..ROWS {
            self.0[row] = GreyscalePlan::from_image_row(row, image);
        }
    }

}




// With a 16µs period, 375 ticks is 6ms
const CYCLE_TICKS: u16 = 375;

const GREYSCALE_TIMINGS : [u16; BRIGHTNESSES-2] = [
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

/// Start the timer you plan to use with a [`Display`].
///
/// Call this once before using a [`Display`].
///
/// This calls the timer's
/// [`initialise_cycle()`][DisplayTimer::initialise_cycle] implementation.
pub fn initialise_timer(timer: &mut impl DisplayTimer) {
    timer.initialise_cycle(CYCLE_TICKS);
}


/// Program the micro:bit LED GPIO pins for use with Display.
///
/// Call this once before using a Display.
pub fn initialise_pins(p: &mut nrf51::Peripherals) {
    for ii in FIRST_COL_PIN ..= LAST_COL_PIN {
        p.GPIO.pin_cnf[ii].write(|w| w.dir().output());
    }
    for ii in FIRST_ROW_PIN ..= LAST_ROW_PIN {
        p.GPIO.pin_cnf[ii].write(|w| w.dir().output());
    }

    // Set all cols high.
    p.GPIO.outset.write(|w| unsafe { w.bits(
        (FIRST_COL_PIN ..= LAST_COL_PIN).map(|pin| 1<<pin).sum()
    )});
}

/// Light LEDs in a single matrix row.
///
/// In the specified row, lights exactly the LEDs listed in 'cols'.
/// Turns off all LEDs in the other internal rows.
fn display_row_leds(gpio: &mut nrf51::GPIO, row: usize, cols: ColumnSet) {
    // To light an LED, we set the row bit and clear the col bit.
    let rows_to_set = 1<<(FIRST_ROW_PIN+row);
    let rows_to_clear = ROW_BITS ^ rows_to_set;
    let cols_to_clear = cols.as_pins();
    let cols_to_set = COL_BITS ^ cols_to_clear;

    gpio.outset.write(|w| unsafe { w.bits(rows_to_set | cols_to_set) });
    gpio.outclr.write(|w| unsafe { w.bits(rows_to_clear | cols_to_clear) });
}

/// Light additional LEDs in the current matrix row.
///
/// Affects the row most recently passed to display_row_leds().
/// Lights the LEDs listed in 'cols', in addition to any already lit.
fn light_current_row_leds(gpio: &mut nrf51::GPIO, cols: ColumnSet) {
    gpio.outclr.write(|w| unsafe {
        w.bits(cols.as_pins())
    });
}


/// Manages the micro:bit LED display.
///
/// There should normally be a single `Display` instance in a program.
///
/// Call [`initialise_pins()`] and [`initialise_timer()`] before using a
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
///     static mut DISPLAY: Display = ();
///
///     #[init]
///     fn init() -> init::LateResources {
///         let mut p: nrf51::Peripherals = device;
///         display::initialise_pins(&mut p);
///         display::initialise_timer(&mut p.TIMER1);
///         init::LateResources {
///             GPIO : p.GPIO,
///             TIMER1 : p.TIMER1,
///             DISPLAY : Display::new(),
///         }
///     }
/// }
/// ```

pub struct Display {
    // index (0..=2) of the row being displayed
    row_strobe      : usize,
    // brightness level (0..=8) to process next
    next_brightness : usize,
    frame           : Frame,
    current_plan    : GreyscalePlan,
}

impl Display {

    /// Creates a Display instance, initially holding a blank image.
    pub fn new() -> Display {
        Display {
            row_strobe: 0,
            next_brightness: 0,
            frame: Frame::default(),
            current_plan: GreyscalePlan::default(),
        }
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
    ///     static mut frame: Frame = Frame::default();
    ///     let event_reg = &resources.RTC0.events_tick;
    ///     event_reg.write(|w| unsafe {w.bits(0)} );
    ///     frame.set(GreyscaleImage::blank());
    ///     resources.DISPLAY.lock(|display| {
    ///         display.set_frame(frame);
    ///     });
    /// }
    /// ```
    pub fn set_frame(&mut self, frame: &Frame) {
        self.frame = *frame;
    }

    /// Updates the display for the start of a new primary cycle.
    ///
    /// Leaves the timer's secondary alarm enabled iff there are any
    /// intermediate brightnesses in the current image.
    fn render_row(&mut self,
                  gpio: &mut nrf51::GPIO,
                  timer: &mut impl DisplayTimer) {
        assert! (self.row_strobe < ROWS);
        self.row_strobe += 1;
        if self.row_strobe == ROWS {self.row_strobe = 0};

        let plan = self.frame.get_plan(self.row_strobe);

        let lit_cols = plan.lit_cols(MAX_BRIGHTNESS);
        display_row_leds(gpio, self.row_strobe, lit_cols);

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
                     gpio: &mut nrf51::GPIO,
                     timer: &mut impl DisplayTimer) {
        // When this method is called, next_brightness is an intermediate
        // brightness in the range 1..8 (the one that it's time to display).

        let additional_cols = self.current_plan.lit_cols(self.next_brightness);
        light_current_row_leds(gpio, additional_cols);

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
                    GREYSCALE_TIMINGS[self.next_brightness-1]
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
    /// As well as updating the LED state, this method may call the timer's
    /// [`program_secondary()`][DisplayTimer::program_secondary],
    /// [`enable_secondary()`][DisplayTimer::enable_secondary], and/or
    /// [`disable_secondary()`][DisplayTimer::disable_secondary] methods to
    /// update the timer state.
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
    ///    resources.DISPLAY.handle_event(resources.TIMER1, &mut resources.GPIO);
    /// }
    /// ````
    pub fn handle_event(&mut self,
                        timer: &mut impl DisplayTimer,
                        gpio: &mut nrf51::GPIO) {
        let row_timer_fired = timer.check_primary();
        let brightness_timer_fired = timer.check_secondary();
        if row_timer_fired {
            self.render_row(gpio, timer);
        } else if brightness_timer_fired {
            self.render_subrow(gpio, timer);
        }

    }

}

