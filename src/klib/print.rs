// extern crate alloc;
// use alloc::string::ToString;

use crate::kern::machine::_write_str;
use crate::println;

use core::fmt;

pub fn _print(args: fmt::Arguments) {
    if let Some(s) = args.as_str() {
        _write_str(s)
    } else {
        // _write_str(&args.to_string());
        println!("> print.rs: Oops! Format with args are not supported yet! Since there is no allocator\n");
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
