#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

pub mod arch_mipsel;
pub mod consts;
pub mod library;
pub mod memory;
pub mod process;
pub mod utils;

use memory::buddy_allocator::BuddyAllocator;

#[global_allocator]
static BUDDY_ALLOCATOR: BuddyAllocator<4> = BuddyAllocator::<4>::new();
