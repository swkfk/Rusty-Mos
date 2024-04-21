.text
.global _start;
_start:
.set at
.set reorder
	# clear .bss segment
	la      $v0, bss_start
	la      $v1, bss_end
clear_bss_loop:
	beq     $v0, $v1, clear_bss_done
	sb      $zero, 0($v0)
	addiu   $v0, $v0, 1
	j       clear_bss_loop

clear_bss_done:
	# disable interrupts
	mtc0    $zero, CP0_STATUS

	# set up the kernel stack
	la		$sp, KSTACKTOP
	# jump to rust_mips_main */
	j		rust_mips_init
