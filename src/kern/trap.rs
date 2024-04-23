#[derive(Clone, Copy, Debug, Default)]
pub struct TrapFrame {
    pub regs: [u32; 32],
    pub cp0_status: u32,
    pub hi: u32,
    pub lo: u32,
    pub cp0_badvaddr: u32,
    pub cp0_cause: u32,
    pub cp0_epc: u32,
}

impl TrapFrame {
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
