//! Definitions about the memory / page and conventions. Include some helper functions to do the conversion.
//!
//! The virtual address (**32-bit**) structure is as follows:
//! ```text
//! | 31                 22 | 21             12 | 11           0 |
//! | Page Directory Offset | Page Table Offset | In-Page Offset |
//! ```
//!
//! The page table entry (PTE, **32-bit**) structure is as follows:
//! ```text
//! | 31                 12 | 11               6 | 5                0 |
//! | Physical Frame number | Flags for hardware | Flags for software |
//! ```
//!

/// Shifted the *In-Page Offset* out to get the **Page Table Offset**
pub const PGSHIFT: u8 = 12;
/// Shifted the *Page Table Offset* and *In-Page Offset* out to get the **Page Directory Offset**
pub const PDSHIFT: u8 = 22;

/// Page table / directory entry flag shift
const PTE_HARDFLAG_SHIFT: u8 = 6;

/// Valid bit. If 0 any address matching this entry will cause a tlb miss exception (TLBL/TLBS).
pub const PTE_V: u32 = 0x0002 << PTE_HARDFLAG_SHIFT;
/// Cache Coherency Attributes bit.
pub const PTE_C_CACHEABLE: u32 = 0x0018 << PTE_HARDFLAG_SHIFT;

/// Get the physical address of the virtual address in **kernel segment**
///
/// Someway the opposite of [KADDR](crate::KADDR)
/// # Panic
/// Assertion will fail if the virtual address is not in the **kernel segment**.
#[macro_export]
macro_rules! PADDR {
    ($x: expr) => {{
        assert!($x >= 0x80000000);
        $x - 0x80000000
    }};
}

/// Get the virtual address (in **kernel segment**) from the physical address
///
/// Someway the opposite of [PADDR](crate::PADDR)
/// # Panic
/// **NOT IMPLEMENTED**
/// Assertion will fail if the virtual address is not in the **kernel segment**.
#[macro_export]
macro_rules! KADDR {
    ($x: expr) => {{
        // assert!(($x >> 12) < npage);
        $x + 0x80000000
    }};
}

/// Get the address(or the frame number etc.) from the page table entry (PTE)
#[macro_export]
macro_rules! PTE_ADDR {
    ($pte: expr) => {{
        $pte & !0xFFF
    }};
}

/// Get the **Page Directory Offset** from the virtual address
///
/// See also: [PTX](crate::PTX)
#[macro_export]
macro_rules! PDX {
    ($va: expr) => {{
        ($va >> $crate::kdef::mmu::PDSHIFT) & 0x03FF
    }};
}

/// Get the **Page Table Offset** from the virtual address
///
/// See also: [PDX](crate::PDX)
#[macro_export]
macro_rules! PTX {
    ($va: expr) => {{
        ($va >> $crate::kdef::mmu::PGSHIFT) & 0x03FF
    }};
}

/// Get the Page Number from the physical address
#[macro_export]
macro_rules! PPN {
    ($pa: expr) => {
        $pa >> $crate::kdef::mmu::PGSHIFT
    };
}
