# trap.S
# Assembly-level trap handler.
# Inspired by Stephen Marz (sos) and Robert Morris (xv6)
# Convert to a trampoline page in the future

.section .text
.global asm_trap_vector
asm_trap_vector:
    # We get here when the CPU is interrupted
	# for any reason.
	call kinit 
    mret
