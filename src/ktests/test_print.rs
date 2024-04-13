use crate::{debugln, print, println, printnumln};

pub fn test_println() {
    debugln!("> test_print.rs: test the println! macro and print! macro");
    println!("Simple string format: {}", "Compiler will done it!");
    println!("Simple integer format: {}", 123);
    print!("Print with no new-line...");
    println!("A new line!");
    debugln!("> test_print.rs: done");
}

pub fn test_printnum() {
    debugln!("> test_print.rs: test the printnumln! macro");
    printnumln!("With no integer");
    printnumln!("With an integer:", 1024);
    printnumln!("With an integer:", 1029; 16);
    printnumln!("With an integer:", 1029; 8);
    printnumln!("With an integer:", 1029; 2);
    printnumln!("With an integer:", 1029; 7);
    printnumln!("With a zero:", 0);
    printnumln!("With a negetive:", -123);
    printnumln!("With a lot of numbers:", 123, 321, -666; 16);
    debugln!("> test_print.rs: done");
}
