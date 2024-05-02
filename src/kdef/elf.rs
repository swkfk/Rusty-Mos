use core::{mem::size_of, ptr::null};

use super::{env::EnvNode, error::KError};

type Elf32Half = u16;
type Elf32Word = u32;
type Elf32Off = u32;
type Elf32Addr = u32;

pub type ElfMapperFn =
    fn(*const EnvNode, usize, isize, u32, *const u8, usize) -> Result<(), KError>;

const EI_NIDNET: usize = 16;

pub struct Elf32Ehdr {
    ident: [u8; EI_NIDNET],
    ftype: Elf32Half,
    _machine: Elf32Half,
    _version: Elf32Word,
    pub entry: Elf32Addr,
    phoff: Elf32Off,
    _shoff: Elf32Off,
    _flags: Elf32Word,
    _ehsize: Elf32Half,
    phentsize: Elf32Half,
    phnum: Elf32Half,
    _shentsize: Elf32Half,
    _shnum: Elf32Half,
    _shstrndx: Elf32Half,
}

impl Elf32Ehdr {
    pub fn from(binary: *const u8, size: usize) -> *const Self {
        let ehdr = binary as *const Self;
        let ehdr_type = unsafe { (*ehdr).ftype };
        let ehdr_ident = unsafe { (*ehdr).ident };
        if size >= size_of::<Elf32Ehdr>()
            && ehdr_type == 2
            && ehdr_ident[0] == 0x7f
            && ehdr_ident[1] == b'E'
            && ehdr_ident[2] == b'L'
            && ehdr_ident[3] == b'F'
        {
            ehdr
        } else {
            null()
        }
    }

    pub fn foreach(&self, apply: impl Fn(Elf32Off)) {
        let mut ph_off = self.phoff;
        for _ in 0..self.phnum {
            apply(ph_off);
            ph_off += self.phentsize as Elf32Off;
        }
    }
}

pub struct Elf32Phdr {
    pub stype: Elf32Word,
    pub offset: Elf32Off,
    pub vaddr: Elf32Addr,
    _paddr: Elf32Addr,
    pub filesz: Elf32Word,
    pub memsz: Elf32Word,
    pub flags: Elf32Word,
    _align: Elf32Word,
}

pub const PF_X: u32 = 1 << 0;
pub const PF_W: u32 = 1 << 1;
pub const PF_R: u32 = 1 << 2;

pub const PT_LOAD: u32 = 1;
