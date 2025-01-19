//! Machine related implementations.

use crate::println;

/// The KSEG1 to visit the devices.
const KSEG1: u32 = 0xA0000000;

/// QEMU MMIO address definitions for PCIIO (Used for text I/O).
const MALTA_PCIIO_BASE: u32 = 0x18000000;
/// QEMU MMIO address definitions for FPGA (Used for [halt]).
const MALTA_FPGA_BASE: u32 = 0x1f000000;

/// 16550 Serial UART device definitions (Base).
const MALTA_SERIAL_BASE: u32 = MALTA_PCIIO_BASE + 0x3f8;
/// 16550 Serial UART device definitions (Data: Offset: 0x0).
const MALTA_SERIAL_DATA: u32 = MALTA_SERIAL_BASE;
/// 16550 Serial UART device definitions (LSR: Offset: 0x5).
const MALTA_SERIAL_LSR: u32 = MALTA_SERIAL_BASE + 0x5;
/// Serial data is ready.
///
const MALTA_SERIAL_DATA_READY: u8 = 0x1;
/// Serial the is empty.
const MALTA_SERIAL_THR_EMPTY: u8 = 0x20;

/// Put the character (whose size is 1 byte) into teh serial. Busy-wait if
/// the serial thr is not empty. The `'\r'` will be added before a `'\n'`.
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

/// Get a character (whose size is 1 byte) from the serial. Zero will be
/// returned if the data is not ready for reading.
pub fn scan_charc() -> u8 {
    let lsr_ptr = KSEG1 + MALTA_SERIAL_LSR;
    let data_ptr = KSEG1 + MALTA_SERIAL_DATA;
    unsafe {
        if (core::ptr::read_volatile(lsr_ptr as *const u8) & MALTA_SERIAL_DATA_READY) != 0 {
            core::ptr::read_volatile(data_ptr as *const u8)
        } else {
            0
        }
    }
}

/// Actually trigger the reboot of the board. But in QEMU, we choose to
/// 'no-reboot'. So the simulation will exit if [halt] is called.
pub fn halt() -> ! {
    unsafe { core::ptr::write_volatile((KSEG1 + MALTA_FPGA_BASE + 0x500) as *mut u8, 0x42) };
    println!("> machine.rs: halt is not supported in this machine!\n");
    unreachable!();
}
