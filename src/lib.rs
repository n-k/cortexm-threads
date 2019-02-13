#![no_std]

use core::ptr;

#[repr(C)]
struct OS {
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
    let os = &mut __CORTEXM_THREADS_GLOBAL;
    os.threads[os.add_idx] = tcb;
    os.add_idx = os.add_idx + 1;
}

#[no_mangle]
pub unsafe extern "C" fn tick() {
    if __CORTEXM_THREADS_GLOBAL_PTR != 0 {
        let os: &mut OS = &mut *(__CORTEXM_THREADS_GLOBAL_PTR as *mut OS);
        if os.inited && os.add_idx > 0 {
            if os.curr == os.next {
                // schedule a thread to be run
                os.next = core::intrinsics::transmute(&os.threads[os.idx]);
                os.idx = os.idx + 1;
                if os.idx >= os.add_idx {
                    os.idx = 0;
                }
            }
            if os.curr != os.next {
                let pend = ptr::read_volatile(0xE000ED04 as *const u32);
                ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
            }
        }
    }
}

extern "C" {
    pub fn PendSVHandler();
}


// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: OS = OS {
    curr: 0,
    next: 0,
    inited: false,
    idx: 0,
    add_idx: 0,
    threads: [
        ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0},
    ],
};
