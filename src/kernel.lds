/*
 * Set the architecture to mips.
 */
OUTPUT_ARCH(mips)

/*
 * Set the ENTRY point of the program to _start.
 */
ENTRY(_start)

PHDRS {
	exceptions PT_LOAD ;
	text PT_LOAD ;
	data PT_LOAD ;
}

SECTIONS {

	. = 0x80020000;

	start = . ;

	.text : { *(.text.*) } :text

	. = ALIGN(0x8000);

	exc_handler = . ;

	.tlb_miss_entry : {
		KEEP(*(.tlb_miss_entry))
	} :exceptions

	. = exc_handler + 0x0180;

	.exc_gen_entry : {
		KEEP(*(.exc_gen_entry))
	} :exceptions

	.rodata : { *(.rodata.*) }

	.data : { *(.data.*) } :data
	bss_start = .;

	.bss : { *(.bss.*) }
	bss_end = .;
	. = 0x80400000;
	end = . ;

	/DISCARD/ : {
		*(.comment)
		*(.eh_frame)
		*(.eh_frame_hdr)
	}
}
