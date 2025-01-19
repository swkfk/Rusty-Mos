//! Apply print-related functions.

use crate::arch::machine::print_charc;

use core::fmt::{self, Write};

/// An empty struct to hold the `fmt::Write` trait.
struct Stdout;

impl fmt::Write for Stdout {
    /// Put strings onto the screen. See Also: [_write_str].
    fn write_str(&mut self, s: &str) -> fmt::Result {
        _write_str(s);
        Ok(())
    }
}

/// Formatted print wrapper. See Also: [Stdout::write_fmt].
pub fn _print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}

/// Put the strings directly onto the screen. See Also: [print_charc].
fn _write_str(s: &str) {
    for c in s.chars() {
        print_charc(c as u8);
    }
}

/// Formatted Print: with *no* *new-line* at the end.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::library::print::_print(format_args!($($arg)*));
    }};
}

/// Formatted Print: with a *new-line* at the end.
#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\n")
    };
}

/// Debug Formatted Print: with a *new-line* at the end. Will perform nothing
/// in *release* profile.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debugln {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {
        $crate::print!("\x1b[35m");
        $crate::print!($($arg)*);
        $crate::print!("\x1b[0m");
        $crate::print!("\n")
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debugln {
    () => {};
    ($($arg:tt)*) => {};
}

/// Debug Formatted Print: with *no* *new-line* at the end. Will perform nothing
/// in *release* profile.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::print!("\x1b[35m");
        $crate::print!($($arg)*);
        $crate::print!("\x1b[0m");
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
