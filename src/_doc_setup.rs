//! How to use rmicrobit.
//!
//! # How to use rmicrobit
//!
//! ## Setting up a new project
//!
//! You will typically need:
//!  - a linker that can work with ARM binaries
//!  - a debugger that can work with ARM binaries
//!  - an OCD server
//!
//! The following instructions are written for Debian 10 ('Buster'). For other
//! platforms, see [The Embedded Rust Book].
//!
//! Install the following packages:
//!  - `binutils-arm-none-eabi`
//!  - `gdb-multiarch`
//!  - `openocd`
//!
//! Create a new project using the `rmicrobit-quickstart` template:
//! ```text
//! cargo install cargo-generate
//! ```
//! then
//! ```text
//! cargo generate --git https://mjw.woodcraft.me.uk/rmicrobit-quickstart/git/
//! ```
//! or
//! ```text
//! cargo generate --git https://github.com/mattheww/rmicrobit-quickstart
//! ```
//!
//! The template provides the following files:
//! -  `Cargo.toml`
//! - `.cargo/config`
//! - `microbit.gdb`
//! - `src/main.rs`
//!
//! The example `main.rs` uses `cortex-m-rtfm` and `cortex-m-semihosting`, but
//! projects using rmicrobit don't have to do so; you can remove them from
//! `Cargo.toml` if you don't need them.
//!
//! ### Running the new project
//!
//! Connect your micro:bit to your development machine by USB.
//!
//! In a separate shell session:
//! ```text
//! openocd -f interface/cmsis-dap.cfg -f target/nrf51.cfg
//! ```
//!
//! In the new project directory:
//! ```text
//! cargo run
//! ```
//!
//! This should launch `gdb`, which should tell the OCD server to flash the
//! program onto the micro:bit and run it. Use `Ctrl-C` followed by `Ctrl-D`
//! to exit.
//!
//! To change which version of `gdb` it runs, edit `.cargo/config`. To change
//! the instructions that `gdb` follows, edit `microbit.gdb`.
//!
//! [The Embedded Rust Book]: https://rust-embedded.github.io/book/intro/install.html
