//! Support for hardware buttons.
//!
//! # Scope
//!
//! This module provides:
//! - a lower-level interface for reading a button or switch's state
//! - optional 'debouncing'
//! - a higher-level interface for retrieving button events, including:
//!   - simple 'click' events
//!   - support for treating two buttons as a single device, with a 'clicked
//!     both' event
//!   - support for detecting 'hold' (long press) events
//! - convenience APIs for using these features with the built-in buttons
//!
//! # Polling model
//!
//! This module's client is responsible for calling polling functions at
//! regular intervals (nothing in this module itself uses interrupts or
//! timers).
//!
//! The intended polling interval is 6ms.
//!
//! In practice it appears no debouncing is needed for the micro:bit's
//! built-in buttons given this polling interval, so by default no debouncing
//! is applied.
//!
//! With a 6ms polling interval, the 'with hold' drivers report a hold event
//! after a press 1.5s long.
//!
//! # Usage
//!
//! The simplest way to access the higher-level features is via one of the
//! high-level driver modules:
//! - [`single_eager`]
//! - [`single_lazy`]
//! - [`single_with_hold`]
//! - [`dual`]
//! - [`dual_with_hold`]
//!
//! Each of these modules defines a similar interface, including a
//! `ButtonEvent` enum and either a `Monitor` type for each button or a
//! `Monitor` type for both buttons treated as a single device.
//!
//! Each `Monitor` type defines a `new()` method which expects a [`ButtonA`],
//! a [`ButtonB`], or one of each.
//!
//! Use the [`from_pins()`] function to retrieve [`ButtonA`] and [`ButtonB`].
//!
//! Each `Monitor` type defines a `poll()` method which returns an
//! `Option(ButtonEvent)`; this method should be called at regular intervals
//! (in practice every 6ms).
//!
//! ## Lower-level access
//!
//! See the [`buttons::core`] module if none of the event types above are
//! suitable for your purposes, or to use an external button.
//!
//! See the [`buttons::debouncing`] module if you need to control debouncing
//! behaviour.
//!
//! # Examples
//!
//! ```ignore
//! use microbit_blinkenlights::prelude::*;
//! use microbit_blinkenlights::gpio::PinsByKind;
//! use microbit_blinkenlights::buttons;
//! use microbit_blinkenlights::buttons::dual::{ABMonitor, ButtonEvent};
//! let p: nrf51::Peripherals = _;
//! let PinsByKind {button_pins, ..} = p.GPIO.split_by_kind();
//! let (button_a, button_b) = buttons::from_pins(button_pins);
//! let monitor = ABMonitor::new(button_a, button_b);
//! loop {
//!     // every 6ms
//!     match monitor.poll() {
//!         Some(ButtonEvent::ClickA) => { ... }
//!         Some(ButtonEvent::ClickB) => { ... }
//!         Some(ButtonEvent::ClickAB) => { ... }
//!         None => {}
//!     }
//! }
//! ```
//!
//! See `examples/use_single_button_monitor.rs` and
//! `examples/use_dual_button_monitor.rs` for complete examples.
//!
//! [`single_eager`]: crate::buttons::single_eager
//! [`single_lazy`]: crate::buttons::single_lazy
//! [`single_with_hold`]: crate::buttons::single_with_hold
//! [`dual`]: crate::buttons::dual
//! [`dual_with_hold`]: crate::buttons::dual_with_hold
//! [`from_pins()`]: crate::buttons::from_pins
//! [`ButtonA`]: crate::buttons::builtin::ButtonA
//! [`ButtonB`]: crate::buttons::builtin::ButtonB

pub use crate::buttons::builtin::from_pins;

pub mod builtin;
pub mod core;
pub mod debouncing;

/// Implementations of the high-level button drivers.
pub mod monitors {
    pub mod holding;
    pub mod single;
    pub mod dual;
    pub mod single_with_hold;
    pub mod dual_with_hold;
}


/// High-level driver for a single button, with events on press.
pub mod single_eager {
    pub use crate::buttons::monitors::single::Event as ButtonEvent;
    pub use crate::buttons::builtin::{
        EagerButtonAMonitor as ButtonAMonitor,
        EagerButtonBMonitor as ButtonBMonitor,
    };
}

/// High-level driver for a single button, with events on release.
pub mod single_lazy {
    pub use crate::buttons::monitors::single::Event as ButtonEvent;
    pub use crate::buttons::builtin::{
        LazyButtonAMonitor as ButtonAMonitor,
        LazyButtonBMonitor as ButtonBMonitor,
    };
}

/// High-level driver for two buttons together.
pub mod dual {
    pub use crate::buttons::monitors::dual::Event as ButtonEvent;
    pub use crate::buttons::builtin::{
        ABMonitor,
    };
}

/// High-level driver for a single button, with 'hold' support.
pub mod single_with_hold {
    pub use crate::buttons::monitors::single_with_hold::Event as ButtonEvent;
    pub use crate::buttons::builtin::{
        ButtonAMonitorWithHold as ButtonAMonitor,
        ButtonBMonitorWithHold as ButtonBMonitor,
    };
}


/// High-level driver for two buttons together, with 'hold' support.
pub mod dual_with_hold {
    pub use crate::buttons::monitors::dual_with_hold::Event as ButtonEvent;
    pub use crate::buttons::builtin::{
        ABMonitorWithHold as ABMonitor,
    };
}

