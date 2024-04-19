pub const PGSHIFT: u8 = 12;
pub const PDSHIFT: u8 = 22;
const PTE_HARDFLAG_SHIFT: u8 = 6;
pub const PTE_V: u32 = 0x0002 << PTE_HARDFLAG_SHIFT;
pub const PTE_C_CACHEABLE: u32 = 0x0018 << PTE_HARDFLAG_SHIFT;

#[macro_export]
macro_rules! PADDR {
    ($x: expr) => {{
        assert!($x >= 0x80000000);
        $x - 0x80000000
    }};
}

#[macro_export]
macro_rules! KADDR {
    ($x: expr) => {{
        // assert!(($x >> 12) < npage);
        $x + 0x80000000
    }};
}

#[macro_export]
macro_rules! PTE_ADDR {
    ($pte: expr) => {{
        $pte & !0xFFF
    }};
}

#[macro_export]
macro_rules! PDX {
    ($va: expr) => {{
        ($va >> $crate::kdef::mmu::PDSHIFT) & 0x03FF
    }};
}

#[macro_export]
macro_rules! PTX {
    ($va: expr) => {{
        ($va >> $crate::kdef::mmu::PGSHIFT) & 0x03FF
    }};
}

#[macro_export]
macro_rules! PPN {
    ($pa: expr) => {
        $pa >> $crate::kdef::mmu::PGSHIFT
    };
}
