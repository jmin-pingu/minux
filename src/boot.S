# boot.S
# Inspired by Stephen Marz (sos) and Robert Morris (xv6)

.option norvc
.section .data
.section .text.init
.global _start
_start:
	# Any hardware threads (hart) that are not bootstrapping
	# need to wait for an IPI
	csrr	t0, mhartid
	bnez	t0, 3f
	# SATP should be zero, but let's make sure
	csrw	satp, zero
.option push
.option norelax
	la		gp, _global_pointer
.option pop
	# The BSS section is expected to be zero
	la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:
	# Control registers, set the stack, mstatus, mepc,
	# and mtvec to return to the main function.
	# li		t5, 0xffff;
	# csrw	medeleg, t5
	# csrw	mideleg, t5
	la		sp, _stack
	# We use mret here so that the mstatus register
	# is properly updated.
	li		t0, (0b11 << 11) | (1 << 7) | (1 << 3) # get mstatus register
	# enable M timer interrupts
	# li 		t1, (1 << 7) 
	# add     t0, t0, t1
	# # enable machine mode interrupts
	# li 		t2, (1 << 7) 
	# add     t0, t0, t2
	csrw	mstatus, t0

	# Machine's exception program counter (MEPC) is set to `kinit`.
	la		t1, kmain
	csrw	mepc, t1
	# Machine's trap vector base address is set to `asm_trap_vector`.
	la		t2, asm_trap_vector
	csrw	mtvec, t2
	# Set the return address to get us into supervisor mode
	la		ra, spin
	# We use mret here so that the mstatus register is properly updated.
	mret
3:

	# Parked harts go here. We need to set these
	# to only awaken if it receives a software interrupt,
	# which we're going to call the SIPI (Software Intra-Processor Interrupt).
	# We only use these to run user-space programs, although this may
	# change.

spin:
        j spin
4:
	wfi
	j		4b

