// extern crate alloc;
// use alloc::string::ToString;

use crate::kern::machine::print_charc;
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

pub fn _write_str(s: &str) {
    for c in s.chars() {
        print_charc(c as u8);
    }
}

pub fn _write_integar(mut s: i32, index: u8) {
    if s == 0 {
        print_charc(b'0');
        return;
    }
    if s < 0 {
        print_charc(b'-');
        s = -s;
    }
    if s >= index as i32 {
        _write_integar(s / index as i32, index);
    }
    let tail = s % index as i32;
    print_charc(tail as u8 + (if tail <= 9 { b'0' } else { b'A' - 10 }));
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

#[macro_export]
macro_rules! printnumln {
    ($fmt:expr $(, $arg:expr)*) => {
        printnumln!($fmt $(, $arg)*; 10);
    };
    ($fmt:expr $(, $arg:expr)* ; $index: expr) => {
        $crate::print!($fmt);
        $(
            $crate::print!(" ");
            match $index {
                2 => $crate::print!("0b"),
                8 => $crate::print!("0o"),
                10 => (),
                16 => $crate::print!("0X"),
                _ => {
                    $crate::print!("(Base: ");
                    $crate::klib::print::_write_integar($index, 10);
                    $crate::print!(")");
                },
            };
            $crate::klib::print::_write_integar($arg, $index);
        )*
        $crate::print!("\n");
    };
}
