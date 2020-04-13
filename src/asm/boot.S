.option norvc # Disable generation of compressed instructions
.section .data

.section .text.init
.global _start
_start:
	csrr t0, mhartid # control status register read
	bnez t0, 3f # branch if not zero
	csrw satp, zero # control status register write
.option push
.option norelax
	la gp, _global_pointer # load address
.option pop
	la a0, _bss_start
	la a1, _bss_end
	bgeu a0, a1, 2f # if a0 >= a1, jump
1:
	sd zero, (a0) # store double
	addi a0, a0, 8
	bltu a0, a1, 1b # branch less than unsigned
2:
	la sp, _stack
	# 0b11 << 11 previous protection mode is 3 (MPP=3)
	# 1 << 7 previous interrupt-enable bit is 1
	# 1 << 3 interrupt-enable bit is 1
	li t0, (0b11 << 11) | (1 << 7) | (1 << 3)
	csrw mstatus, t0
	# Exception program counter to kmain
	la t1, kmain
	csrw mepc, t1
	# Trap vector base address to asm_trap_vector
	la t2, asm_trap_vector
	csrw mtvec, t2
	# 1 << 3 machine-mode software interrupt-enable bit is 1
	# 1 << 7 time interrupt-enable bit is 1
	# 1 << 11 external interrupt-enable bit is 1
	li t3, (1 << 3) | (1 << 7) | (1 << 11)
	csrw mie, t3
	# Set return addres to wait for interrupts
	la ra, 4f
	mret
3:
	wfi
	j 3b
4:
	wfi
	j 4b
