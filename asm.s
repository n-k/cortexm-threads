.thumb
.syntax unified

.global __CORTEXM_THREADS_GLOBAL_PTR

.global __CORTEXM_THREADS_cpsid
.thumb_func
__CORTEXM_THREADS_cpsid:
	cpsid	i
	bx lr

.global __CORTEXM_THREADS_cpsie
.thumb_func
__CORTEXM_THREADS_cpsie:
	cpsie	i
	bx lr

.global PendSV
.thumb_func
PendSV:
	cpsid	i
	ldr		r1, =__CORTEXM_THREADS_GLOBAL_PTR /* r1 = &&OS_PTR */
	ldr		r1,	[r1, 0x0] /* r1 = &OS_PTR */
	ldr		r1,	[r1, 0x0] /* r1 = OS_PTR.curr ( &current_thread ) */
	cmp		r1,	0x0
	beq		__CORTEXM_THREADS_PENDSV_RESTORE
	push	{r4-r7}
	mov		r4,	r8
	mov		r5, r9
	mov		r6, r10
	mov		r7, r11
	push	{r4-r7}
	mov		r2, sp
	str		r2, [r1, 0x0] /* current_thread.sp = sp */
	__CORTEXM_THREADS_PENDSV_RESTORE:
	ldr		r1, =__CORTEXM_THREADS_GLOBAL_PTR	/* r1 = &&OS_PTR */
	ldr		r1,	[r1, 0x0]	/* r1 = &OS_PTR */
	ldr 	r2, [r1, 0x4]	/* r2 = OS_PTR.next */
	ldr		r2, [r2, 0x0]	/* r2 = OS_PTR.next.sp */
	mov 	sp, r2			/* sp = OS_PTR.next.sp */
	ldr		r1, =__CORTEXM_THREADS_GLOBAL_PTR	/* r1 = &&OS_PTR */
	ldr		r1,	[r1, 0x0]	/* r1 = &OS_PTR */
	ldr		r2,	[r1, 0x4]	/* r1 = &OS.curr */
	str		r2,	[r1, 0x0]	/* set OS.curr = os.next */
	/* pop		{r4-r11}		 pop regs */
	pop		{r4-r7}
	mov		r4,	r8
	mov		r5, r9
	mov		r6, r10
	mov		r7, r11
	pop		{r4-r7}
	cpsie	i
	bx lr
