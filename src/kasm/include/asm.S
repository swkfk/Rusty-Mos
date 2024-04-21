.include "src/kasm/include/cp0reg.S"

.macro LEAF symbol
	.globl \symbol;
	.align 2;
	.type \symbol, @function;
	.ent \symbol;
	\symbol:
	.frame $sp, 0, $ra
.endmacro

.macro NESTED symbol, framesize, rpc
	.globl \symbol;
	.align 2;
	.type \symbol, @function;
	.ent \symbol;
	\symbol:
	.frame $sp, \framesize, \rpc
.endmacro

.macro END function
	.end \function;
	.size \function, .- \function
.endmacro

.macro EXPORT symbol
	.globl \symbol;
	\symbol:
.endmacro

.macro FEXPORT symbol
	.globl \symbol;
	.type \symbol, @function;
	\symbol:
.endmacro

# .set KSTACKTOP, 0x80400000
