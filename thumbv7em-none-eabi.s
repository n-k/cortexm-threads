.thumb
.syntax unified

.global __CORTEXM_THREADS_GLOBAL_PTR

.global __CORTEXM_THREADS_cpsid
.thumb_func
__CORTEXM_THREADS_cpsid:
	cpsid	i
	bx		lr

.global __CORTEXM_THREADS_cpsie
.thumb_func
__CORTEXM_THREADS_cpsie:
	cpsie	i
	bx		lr

.global PendSV
.thumb_func
PendSV:
	cpsid	i
	ldr		r1,			=__CORTEXM_THREADS_GLOBAL_PTR /* r1 = &&OS_PTR */
	ldr		r1,			[r1, 0x0] /* r1 = &OS_PTR */
	ldr		r1,			[r1, 0x0] /* r1 = OS_PTR.curr ( &current_thread ) */
	cmp		r1,			0x0
	beq		__CORTEXM_THREADS_PENDSV_RESTORE
	mrs		r0,			psp
	stmdb	r0!,		{r4-r11}
	str		r0,			[r1, 0x0] /* current_thread.sp = sp */
	__CORTEXM_THREADS_PENDSV_RESTORE:
	ldr		r1,			=__CORTEXM_THREADS_GLOBAL_PTR	/* r1 = &&OS_PTR */
	ldr		r1,			[r1, 0x0]	/* r1 = &OS_PTR */
	ldr 	r2,			[r1, 0x4]	/* r2 = OS_PTR.next */
	ldr		r3,			[r2, 0x0]	/* r3 = OS_PTR.next.sp */
	ldr		r1,			=__CORTEXM_THREADS_GLOBAL_PTR	/* r1 = &&OS_PTR */
	ldr		r1,			[r1, 0x0]	/* r1 = &OS_PTR */
	ldr		r2,			[r1, 0x4]	/* r2 = &OS.next */
	str		r2,			[r1, 0x0]	/* set OS.curr = os.next */
	ldmia	r3!,		{r4-r11}
	msr 	psp,		r3
	ldr 	r0,			=0xFFFFFFFD
	cpsie	i
	bx 		r0
