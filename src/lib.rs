//! The *MOS* operation system written in Rust. The origin OS is written in C
//! and is for *BUAA OS Course*.
//!
//! This version of MOS is *independently* completed by
//! [kai_Ker](mailto:kai_Ker@buaa.edu.cn).
//!
//! The C-Version of MOS is developed and continuously improved by the OS
//! course team. I express my sincere gratitude to the teachers and *S.A.R.T*s
//! who have continuously contributed to the MOS and to out course.
//!
//! Beyond the origin functions, this version os MOS also has a buddy system
//! allocator and a multi-pages shared memory pool.
//!
//! The MOS itself has the design concept of *microkernel*. The *fork* and the
//! *file system* are done mainly in the user space. The MOS uses a page-memory
//! management to organize the virtual memories.

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

/// Global Buddy Allocator. Can only alloc up to 32 KB at once.
#[global_allocator]
static BUDDY_ALLOCATOR: BuddyAllocator<4> = BuddyAllocator::<4>::new();
