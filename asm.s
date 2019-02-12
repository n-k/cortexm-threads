.thumb
.syntax unified

.global __OS_PTR


.global PendSVHandler
.thumb_func
PendSVHandler:
	cpsid	i
	ldr		r1, =__OS_PTR /* r1 = &&OS_PTR */
	ldr		r1,	[r1, 0x0] /* r1 = &OS_PTR */
	ldr		r1,	[r1, 0x0] /* r1 = OS_PTR.curr ( &current_thread ) */
	cbz		r1,	__PENDSV_RESTORE
	push	{r4-r11}
	str		sp, [r1, 0x0] /* current_thread.sp = sp */
	__PENDSV_RESTORE:
	ldr		r1, =__OS_PTR /* r1 = &&OS_PTR */
	ldr		r1,	[r1, 0x0] /* r1 = &OS_PTR */
	ldr 	r2, [r1, 0x4] /* r2 = OS_PTR.next */
	ldr 	sp, [r2, 0x0] /* sp = OS_PTR.next.sp */
	ldr		r1, =__OS_PTR /* r1 = &&OS_PTR */
	ldr		r1,	[r1, 0x0] /* r1 = &OS_PTR */
	ldr		r2,	[r1, 0x4] /* r1 = &OS.curr */
	str		r2,	[r1, 0x0] /* set OS.curr = os.next */
	pop		{r4-r11} /* popped regs */
	cpsie	i
	bx lr
