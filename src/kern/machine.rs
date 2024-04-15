// TODO: include instead of writing it directly
use crate::println;

const KSEG1: u32 = 0xA0000000;
const MALTA_PCIIO_BASE: u32 = 0x18000000;
const MALTA_SERIAL_BASE: u32 = MALTA_PCIIO_BASE + 0x3f8;
const MALTA_SERIAL_DATA: u32 = MALTA_SERIAL_BASE;
const MALTA_SERIAL_LSR: u32 = MALTA_SERIAL_BASE + 0x5;
const MALTA_SERIAL_THR_EMPTY: u8 = 0x20;
const MALTA_FPGA_BASE: u32 = 0x1f000000;

pub fn print_charc(ch: u8) {
    if ch == b'\n' {
        print_charc(b'\r');
    }
    let lsr_ptr = KSEG1 + MALTA_SERIAL_LSR;
    let data_ptr = KSEG1 + MALTA_SERIAL_DATA;
    unsafe {
        while (core::ptr::read_volatile(lsr_ptr as *const u8) & MALTA_SERIAL_THR_EMPTY) == 0 {}
    }
    unsafe { core::ptr::write_volatile(data_ptr as *mut u8, ch) }
}

pub fn halt() -> ! {
    unsafe { core::ptr::write_volatile((KSEG1 + MALTA_FPGA_BASE + 0x500) as *mut u8, 0x42) };
    println!("> machine.rs: halt is not supported in this machine!\n");
    unreachable!();
}
