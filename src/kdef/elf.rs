//! Define the Elf32 struct and the basic contants for use.

use core::{mem::size_of, ptr::null};

use super::{env::EnvNode, error::KError};

/// Half a word (16 bits).
type Elf32Half = u16;
/// A word (32 bits).
type Elf32Word = u32;
/// The offset type (32 bits).
type Elf32Off = u32;
/// The address type (32 bits).
type Elf32Addr = u32;

/// The type used in the walker of the header.
pub type ElfMapperFn =
    fn(*const EnvNode, usize, isize, u32, *const u8, usize) -> Result<(), KError>;

/// The magic number size for elf file.
const EI_NIDNET: usize = 16;

/// The ELF32 file header structure. The members are defined in the same order
/// as in the actual elf file.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Elf32Ehdr {
    /// Magic number and other information.
    ident: [u8; EI_NIDNET],
    /// The object file type.
    ftype: Elf32Half,
    /// The architecture.
    _machine: Elf32Half,
    /// The object file version.
    _version: Elf32Word,
    /// The **virtual** address for the entry point.
    pub entry: Elf32Addr,
    /// The offset of the program header table from the file's start.
    phoff: Elf32Off,
    /// The offset of the section header table from the file's start.
    _shoff: Elf32Off,
    /// The processor-specific flag.
    _flags: Elf32Word,
    /// The elf header size (in bytes).
    _ehsize: Elf32Half,
    /// The program header table entry size.
    phentsize: Elf32Half,
    /// The program header table entry count.
    phnum: Elf32Half,
    /// The section header table entry size.
    _shentsize: Elf32Half,
    /// The section header table entry count.
    _shnum: Elf32Half,
    /// The section header string table index.
    _shstrndx: Elf32Half,
}

impl Elf32Ehdr {
    /// Build the Elf32Ehdr object from the binary.
    ///
    /// # Return
    ///
    /// The raw-pointer of the binary start if the size and the magic number
    /// is valid. Otherwise, a `null` point will be returned.
    pub fn from(binary: *const u8, size: usize) -> *const Self {
        let ehdr = unsafe { *(binary as *const Self) };
        if size >= size_of::<Elf32Ehdr>()
            && ehdr.ftype == 2
            && ehdr.ident[0] == 0x7f
            && ehdr.ident[1] == b'E'
            && ehdr.ident[2] == b'L'
            && ehdr.ident[3] == b'F'
        {
            binary as *const Self
        } else {
            null()
        }
    }

    /// Walk all the program header entry, use the function `apply` passed.
    pub fn foreach(&self, apply: impl Fn(Elf32Off)) {
        let mut ph_off = self.phoff;
        for _ in 0..self.phnum {
            apply(ph_off);
            ph_off += self.phentsize as Elf32Off;
        }
    }
}

/// The program segment header structure. The members are defined in the same
/// order as in the actual elf file.
#[repr(C)]
pub struct Elf32Phdr {
    /// The segment type.
    pub stype: Elf32Word,
    /// The segment offset.
    pub offset: Elf32Off,
    /// The **virtual** address of the segment.
    pub vaddr: Elf32Addr,
    /// The **physical** address of the segment.
    _paddr: Elf32Addr,
    /// The segment size in **file**.
    pub filesz: Elf32Word,
    /// The segment size in **memory**.
    pub memsz: Elf32Word,
    /// The segment flag.
    pub flags: Elf32Word,
    /// The segment alignment.
    _align: Elf32Word,
}

/// Mark the segment as executable.
pub const PF_X: u32 = 1 << 0;

/// Mark the segment as writable.
pub const PF_W: u32 = 1 << 1;

/// Mark the segment as readable.
pub const PF_R: u32 = 1 << 2;

/// Mark the segment as loadble and load-needed.
pub const PT_LOAD: u32 = 1;
