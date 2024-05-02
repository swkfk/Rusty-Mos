use core::{cmp::min, ptr::null};

use crate::{
    kdef::{
        elf::{Elf32Phdr, ElfMapperFn, PF_W},
        env::EnvNode,
        error::KError,
        mmu::{PAGE_SIZE, PTE_D, PTE_V},
    },
    ROUNDDOWN,
};

/// # Safety
///
pub unsafe fn elf_load_seg(
    ph: *const Elf32Phdr,
    bin: *const u8,
    map_page: ElfMapperFn,
    data: *const EnvNode,
) -> Result<(), KError> {
    let va = (*ph).vaddr;
    let bin_size = (*ph).filesz;
    let seg_size = (*ph).memsz;

    let mut perm = PTE_V;
    if (*ph).flags & PF_W > 0 {
        perm |= PTE_D;
    }

    let offset = va - ROUNDDOWN!(va; PAGE_SIZE as u32);
    if offset != 0 {
        map_page(
            data,
            va as usize,
            offset as isize,
            perm,
            bin,
            min(bin_size, PAGE_SIZE as u32 - offset) as usize,
        )?;
    }

    let mut i = if offset != 0 {
        min(bin_size, PAGE_SIZE as u32 - offset) as usize
    } else {
        0
    };

    while i < bin_size as usize {
        map_page(
            data,
            va as usize + i,
            0,
            perm,
            bin.add(i),
            min(bin_size as usize - i, PAGE_SIZE),
        )?;
        i += PAGE_SIZE;
    }

    while i < seg_size as usize {
        map_page(
            data,
            va as usize + i,
            0,
            perm,
            null(),
            min(seg_size as usize - i, PAGE_SIZE),
        )?;
        i += PAGE_SIZE;
    }

    Ok(())
}
