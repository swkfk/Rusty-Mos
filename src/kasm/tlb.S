LEAF tlb_out
.set noreorder
	mfc0    $t0, CP0_ENTRYHI
	mtc0    $a0, CP0_ENTRYHI

	# Step 1: Use 'tlbp' to probe TLB entry
	nop
	tlbp
	nop

	# Step 2: Fetch the probe result from CP0.Index
	mfc0    $t1, CP0_INDEX

.set reorder
	bltz    $t1, NO_SUCH_ENTRY
.set noreorder
	mtc0    $zero, CP0_ENTRYHI
	mtc0    $zero, CP0_ENTRYLO0
	mtc0    $zero, CP0_ENTRYLO1

	# Step 3: Use 'tlbwi' to write CP0.EntryHi/Lo into TLB at CP0.Index
	nop
	tlbwi
	nop

.set reorder

NO_SUCH_ENTRY:
	mtc0    $t0, CP0_ENTRYHI
	j       $ra
END tlb_out
