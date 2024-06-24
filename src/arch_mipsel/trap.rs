//! Handle exceptions(traps) and handler definitions.

use crate::println;

/// Things need to be stored in the trapframe.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TrapFrame {
    /// All the regular registers.
    pub regs: [u32; 32],
    /// The Status register in CP0.
    pub cp0_status: u32,
    /// The HI register.
    pub hi: u32,
    /// The LO register.
    pub lo: u32,
    /// The BadAddr register in CP0.
    pub cp0_badvaddr: u32,
    /// The Cause register in CP0.
    pub cp0_cause: u32,
    /// The EPC register in CP0.
    pub cp0_epc: u32,
}

impl TrapFrame {
    /// The default value of the registers.
    pub const fn const_construct() -> Self {
        Self {
            regs: [0; 32],
            cp0_status: 0,
            hi: 0,
            lo: 0,
            cp0_badvaddr: 0,
            cp0_cause: 0,
            cp0_epc: 0,
        }
    }
}

extern "C" {
    /// Handle unknown exception code. Invoke a kernel panic.
    pub fn handle_reserved(trap_frame: *const TrapFrame);
    /// Skip the current instruction.
    pub fn handle_skip(trap_frame: *const TrapFrame);
    /// Handle the clock-interrupt.
    pub fn handle_int(trap_frame: *const TrapFrame);
    /// Handle the TLBL or TLBS exception.
    pub fn handle_tlb(trap_frame: *const TrapFrame);
    /// Handle the TLB Mod exception.
    pub fn handle_mod(trap_frame: *const TrapFrame);
    /// Handle the syscall.
    pub fn handle_sys(trap_frame: *const TrapFrame);
}

/// Exception handlers table. This will be exported as `exception_handlers`.
#[export_name = "exception_handlers"]
pub static EXCEPTION_HANDLERS: [unsafe extern "C" fn(*const TrapFrame); 32] = [
    /*  0 */ handle_int,
    /*  1 */ handle_mod, // TLB Mod
    /*  2 */ handle_tlb, // TLB L
    /*  3 */ handle_tlb, // TLB S
    /*  4 */ handle_skip, // AdEL
    /*  5 */ handle_skip, // AdES
    /*  6 */ handle_reserved,
    /*  7 */ handle_reserved,
    /*  8 */ handle_sys,
    /*  9 */ handle_reserved,
    /* 10 */ handle_skip, // RI
    /* 11 */ handle_reserved,
    /* 12 */ handle_skip, // Ov
    /* 13 */ handle_reserved,
    /* 14 */ handle_reserved,
    /* 15 */ handle_reserved,
    /* 16 */ handle_reserved,
    /* 17 */ handle_reserved,
    /* 18 */ handle_reserved,
    /* 19 */ handle_reserved,
    /* 20 */ handle_reserved,
    /* 21 */ handle_reserved,
    /* 22 */ handle_reserved,
    /* 23 */ handle_reserved,
    /* 24 */ handle_reserved,
    /* 25 */ handle_reserved,
    /* 26 */ handle_reserved,
    /* 27 */ handle_reserved,
    /* 28 */ handle_reserved,
    /* 29 */ handle_reserved,
    /* 30 */ handle_reserved,
    /* 31 */ handle_reserved,
];

/// Invoke a panic.
///
/// # Safety
///
/// The `trap_frame` *shall* be a valid address.
#[no_mangle]
pub unsafe fn do_reserved(trap_frame: *const TrapFrame) {
    panic!("Unknown ExcCode {:2}", (*trap_frame).cp0_cause >> 2 & 0x1f);
}

/// Skip the current instruction.
///
/// # Safety
///
/// The `trap_frame` *shall* be a valid address.
#[no_mangle]
pub unsafe fn do_skip(trap_frame: *mut TrapFrame) {
    println!(
        "\x1b[31mExcCode {:2} detected. Skipped!\x1b[0m",
        (*trap_frame).cp0_cause >> 2 & 0x1f
    );
    (*trap_frame).cp0_epc += 4;
}
