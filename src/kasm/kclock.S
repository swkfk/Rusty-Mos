.set TIMER_INTERVAL, 500000

.macro RESET_KCLOCK
	li      $t0, TIMER_INTERVAL
	mtc0    $zero, CP0_COUNT
	mtc0    $t0, CP0_COMPARE
.endmacro
