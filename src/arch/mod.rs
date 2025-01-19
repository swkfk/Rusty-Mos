//! Things related to the architecture. **Mipsel** here is.

pub mod machine;

mod mipsel;

#[cfg(feature = "mipsel")]
pub use mipsel::cp0reg;
