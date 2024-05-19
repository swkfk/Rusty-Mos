//! Definitions about the memory / page and conventions. Include some helper
//! functions to do the conversion.
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
/// Shifted the *Page Table Offset* and *In-Page Offset* out to get the
/// **Page Directory Offset**
pub const PDSHIFT: u8 = 22;
/// The maximum count of all the **asid**
pub const NASID: u32 = 256;

/// Page table / directory entry flag shift
const PTE_HARDFLAG_SHIFT: u8 = 6;

/// Global bit. When this bit in a TLB entry is set, that TLB entry will
/// match solely on the VPN field, regardless of whether the TLB entry’s
/// ASID field matches the value in EntryHi.
pub const PTE_G: u32 = 0x0001 << PTE_HARDFLAG_SHIFT;

/// Valid bit. If 0 any address matching this entry will cause a tlb miss
/// exception (TLBL/TLBS).
pub const PTE_V: u32 = 0x0002 << PTE_HARDFLAG_SHIFT;

/// Dirty bit, but really a write-enable bit. 1 to allow writes, 0 and any
/// store using this translation will cause a tlb mod exception (TLB Mod).
pub const PTE_D: u32 = 0x0004 << PTE_HARDFLAG_SHIFT;

/// Cache Coherency Attributes bit. If set, this entry is cache-able.
pub const PTE_C_CACHEABLE: u32 = 0x0018 << PTE_HARDFLAG_SHIFT;

/// The physical page size (in bytes).
pub const PAGE_SIZE: usize = 1 << PGSHIFT;

/// Bytes mapped by a page directory entry.
pub const PDMAP: usize = 1 << PDSHIFT;
/// Bytes mapped by a page table entry (Actually is the PAGE_SIZE).
const PTMAP: usize = PAGE_SIZE;

/// Kernel memory starts from here
pub const KERNBASE: usize = 0x80020000;

/// Kernel stack end at here (the `end` in the linking script).
pub const KSTACKTOP: usize = ULIM + PDMAP;

/// The high-limits of user's memory
pub const ULIM: usize = 0x80000000;

/// User's page tables are stored here (for a [PDMAP](crate::kdef::mmu::PDMAP)
/// size).
pub const UVPT: usize = ULIM - PDMAP;
/// The kernel array `PAGES` will be mapped here.
pub const UPAGES: usize = UVPT - PDMAP;
/// The kernel array `ENVS` will be mapped here.
pub const UENVS: usize = UPAGES - PDMAP;
/// The uer's space higher boundary.
pub const UTOP: usize = UENVS;
/// The exception stack top for the user. See also:
/// [UTOP](crate::kdef::mmu::UTOP)
pub const UXSTACKTOP: usize = UTOP;
/// Normal user stack top.
pub const USTACKTOP: usize = UTOP - 2 * PTMAP;
/// User test segment start.
pub const UTEXT: usize = PDMAP;
/// Reserved for COW (start address).
pub const UCOW: usize = UTEXT - PTMAP;
/// Reserved for temporary usage (start address).
pub const UTEMP: usize = UCOW - PTMAP;
/// KSEG1 Segment
pub const KSEG1: usize = 0xA0000000;

/// Get the physical address of the virtual address in **kernel segment**
///
/// Someway the opposite of [KADDR](crate::KADDR)
///
/// # Panic
/// Assertion will fail if the virtual address is not in the **kernel
/// segment**.
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
///
/// # Panic
/// **NOT IMPLEMENTED**
/// Assertion will fail if the virtual address is not in the **kernel
/// segment**.
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
