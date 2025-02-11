//! Things related to the architecture.

pub mod machine;

#[cfg(feature = "mipsel")]
mod mipsel;
#[cfg(feature = "riscv32")]
mod riscv32;

#[cfg(feature = "mipsel")]
pub use mipsel::asm;
#[cfg(feature = "mipsel")]
pub use mipsel::cp0_reg as reg_prefabs;

#[cfg(feature = "riscv32")]
pub use riscv32::asm;
#[cfg(feature = "riscv32")]
pub use riscv32::csr_reg as reg_prefabs;
