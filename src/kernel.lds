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

	. = 0x80000000;
	.tlb_miss_entry : {
		KEEP(*(.text.tlb_miss_entry))
	} :exceptions

	. = 0x80000180;
	.exc_gen_entry : {
		KEEP(*(.text.exc_gen_entry))
	} :exceptions

	/* fill in the correct address of the key sections: text, data, bss. */

	. = 0x80020000;

	start = . ;

	.text : { *(.text.*) } :text

	.rodata : { *(.rodata.*) }

	.data : { *(.data.*) } :data
	bss_start = .;

	.bss : { *(.bss.*) }
	bss_end = .;
	. = 0x80400000;
	end = . ;
}
