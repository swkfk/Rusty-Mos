//! Define all the status bits for the cp0 reg.

pub const STATUS_CU3: u32 = 0x80000000;
pub const STATUS_CU2: u32 = 0x40000000;
pub const STATUS_CU1: u32 = 0x20000000;
pub const STATUS_CU0: u32 = 0x10000000;
pub const STATUS_BEV: u32 = 0x00400000;
pub const STATUS_IM0: u32 = 0x0100;
pub const STATUS_IM1: u32 = 0x0200;
pub const STATUS_IM2: u32 = 0x0400;
pub const STATUS_IM3: u32 = 0x0800;
pub const STATUS_IM4: u32 = 0x1000;
pub const STATUS_IM5: u32 = 0x2000;
pub const STATUS_IM6: u32 = 0x4000;
pub const STATUS_IM7: u32 = 0x8000;
pub const STATUS_UM: u32 = 0x0010;
pub const STATUS_R0: u32 = 0x0008;
pub const STATUS_ERL: u32 = 0x0004;
pub const STATUS_EXL: u32 = 0x0002;
pub const STATUS_IE: u32 = 0x0001;
