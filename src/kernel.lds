/*
 * Set the architecture to mips.
 */
OUTPUT_ARCH(mips)

/*
 * Set the ENTRY point of the program to _start.
 */
ENTRY(_start)

SECTIONS {

	. = 0x80000000;
	.tlb_miss_entry : {
		*(.text.tlb_miss_entry)
	}

	. = 0x80000180;
	.exc_gen_entry : {
		*(.text.exc_gen_entry)
	}

	/* fill in the correct address of the key sections: text, data, bss. */

	. = 0x80020000;

	start = . ;

	.text : { *(.text.*) }

	.data : { *(.data.*) }
	bss_start = .;

	.bss : { *(.bss.*) }
	bss_end = .;
	. = 0x80400000;
	end = . ;
}
