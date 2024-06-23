.section .text.tlb_miss_entry
tlb_miss_entry:
	j       exc_gen_entry

.section .text.exc_gen_entry
exc_gen_entry:
	SAVE_ALL tag_exc_gen_entry_no_sp
	mfc0    $t0, CP0_STATUS
	and     $t0, $t0, ~(STATUS_UM | STATUS_EXL | STATUS_IE)
	mtc0    $t0, CP0_STATUS
	mfc0    $t0, CP0_CAUSE
	andi    $t0, 0x7c
	lw      $t0, exception_handlers($t0)
	jr      $t0
