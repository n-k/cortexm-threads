#![no_std]

use core::ptr;

#[repr(C)]
struct ContextSwitchHandler {
    // offset of curr and next fields are used by asm code, don't change their position
    curr: u32,
    next: u32,
    // following fields are only used internally
    inited: bool,
    idx: usize,
    add_idx: usize,
    threads: [ThreadControlBlock; 32],
}

#[repr(C)]
pub struct ThreadControlBlock {
    pub sp: u32,
}

pub unsafe extern "C" fn init() {
    __CORTEXM_THREADS_GLOBAL.inited = true;
    __CORTEXM_THREADS_GLOBAL_PTR = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
}

pub unsafe extern "C" fn create_thread(stack: &mut [u32; 256], handler: fn() -> !) {
    stack[255] = 1 << 24;
    stack[254] = core::intrinsics::transmute(handler as *const fn());
    stack[253] = 0x0000000E;
    stack[252] = 0x0000000C;
    stack[251] = 0x00000003;
    stack[250] = 0x00000002;
    stack[249] = 0x00000001;
    stack[248] = 0x00000000;
    // aditional regs
    stack[247] = 0x0000000B;
    stack[246] = 0x0000000A;
    stack[245] = 0x00000009;
    stack[244] = 0x00000008;
    stack[243] = 0x00000007;
    stack[242] = 0x00000006;
    stack[241] = 0x00000005;
    stack[240] = 0x00000004;
    let tcb = ThreadControlBlock {
        sp: core::intrinsics::transmute(&stack[240]),
    };
    let handler = &mut __CORTEXM_THREADS_GLOBAL;
    handler.threads[handler.add_idx] = tcb;
    handler.add_idx = handler.add_idx + 1;
}

#[no_mangle]
pub unsafe extern "C" fn tick() {
    let handler = &mut __CORTEXM_THREADS_GLOBAL;
    if handler.inited && handler.add_idx > 0 {
        if handler.curr == handler.next {
            // schedule a thread to be run
            handler.next = core::intrinsics::transmute(&handler.threads[handler.idx]);
            handler.idx = handler.idx + 1;
            if handler.idx >= handler.add_idx {
                handler.idx = 0;
            }
        }
        if handler.curr != handler.next {
            let pend = ptr::read_volatile(0xE000ED04 as *const u32);
            ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
        }
    }
}

extern "C" {
    pub fn __CORTEXM_THREADS_PendSVHandler();
}

// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: ContextSwitchHandler = ContextSwitchHandler {
    curr: 0,
    next: 0,
    inited: false,
    idx: 0,
    add_idx: 0,
    threads: [
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
    ],
};
