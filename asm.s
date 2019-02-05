.thumb
.syntax unified

.global activate
.thumb_func
activate:
	/* save kernel state */
	mrs ip, psr
	push {r4, r5, r6, r7, r8, r9, r10, r11, ip, lr}
	
	/* switch to process stack */
	msr psp, r0
	mov r0, 3
	msr control, r0

	/* load user state */
	pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}

	/* jump to user task */
	bx lr

.global HardFaultTrampoline
.thumb_func
HardFaultTrampoline:
    mrs r0, MSP
    b HardFault
