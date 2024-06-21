#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

pub mod kdef;
pub mod kern;
pub mod klib;
pub mod ktests;
pub mod memory;
pub mod utils;

use memory::buddy_allocator::BuddyAllocator;

#[global_allocator]
static BUDDY_ALLOCATOR: BuddyAllocator<4> = BuddyAllocator::<4>::new();
