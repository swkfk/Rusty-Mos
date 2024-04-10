/*
 * Set the architecture to mips.
 */
OUTPUT_ARCH(mips)

/*
 * Set the ENTRY point of the program to _start.
 */
ENTRY(_start)

SECTIONS {

	/* fill in the correct address of the key sections: text, data, bss. */

	. = 0x80020000;

	start = . ;

	.text : { *(.text.*) }

	.data : { *(.data.*) }
	bss_start = .;

	.bss : { *(.bss) }
	bss_end = .;
	. = 0x80400000;
	end = . ;
}
