use crate::kern::machine::print_charc;

use core::fmt::{self, Write};

struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        _write_str(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}

pub fn _write_str(s: &str) {
    for c in s.chars() {
        print_charc(c as u8);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::klib::print::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\n")
    };
}

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
