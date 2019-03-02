#![no_std]

use core::ptr;

// functions defined in assembly
//
extern "C" {
    fn __CORTEXM_THREADS_cpsid();
    fn __CORTEXM_THREADS_cpsie();
    fn __CORTEXM_THREADS_wfe();
}

///
/// Context switching and threads' state
#[repr(C)]
struct ThreadsState {
    // offset of curr, next, and inited fields are used by asm code, don't change their position
    curr: usize,
    next: usize,
    inited: bool,
    // following fields are only used internally
    idx: usize,
    add_idx: usize,
    threads: [ThreadControlBlock; 32],
}

///
/// Thread status
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum ThreadStatus {
    Idle,
    Sleeping,
}

///
/// A single thread's state
#[repr(C)]
#[derive(Clone, Copy)]
struct ThreadControlBlock {
    /// current stack pointer of this thread
    sp: u32,
    status: ThreadStatus,
    sleep_ticks: u32,
}

// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: ThreadsState = ThreadsState {
    curr: 0,
    next: 0,
    inited: false,
    idx: 0,
    add_idx: 1,
    threads: [ThreadControlBlock{sp: 0, status: ThreadStatus::Idle, sleep_ticks: 0}; 32],
};
// end GLOBALS

pub extern "C" fn init() -> ! {
    unsafe {
        __CORTEXM_THREADS_cpsid();
        __CORTEXM_THREADS_GLOBAL_PTR = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
        __CORTEXM_THREADS_cpsie();
        let mut idle_stack = [0xDEADBEEF; 64];
        let tcb = create_tcb(
            &mut idle_stack,
            || {
                loop {
                    __CORTEXM_THREADS_wfe();
                }        
            });
        insert_tcb(0, tcb);
        __CORTEXM_THREADS_GLOBAL.inited = true;
        loop {
            __CORTEXM_THREADS_wfe();
        }
    }
}

pub extern "C" fn create_thread(stack: &mut [u32], handler: fn() -> !) {
    unsafe {
        __CORTEXM_THREADS_cpsid();
    }
    unsafe {
        let tcb = create_tcb(stack, handler);
        let handler = &mut __CORTEXM_THREADS_GLOBAL;
        insert_tcb(handler.add_idx, tcb);
        handler.add_idx = handler.add_idx + 1;
    }
    unsafe {
        __CORTEXM_THREADS_cpsie();
    }
}

#[no_mangle]
pub extern "C" fn tick() {
    unsafe {
        __CORTEXM_THREADS_cpsid();
    }
    let handler = unsafe {&mut __CORTEXM_THREADS_GLOBAL};
    if handler.inited {
        if handler.curr == handler.next {
            // schedule a thread to be run
            handler.idx = get_next_thread_idx();
            unsafe {
                handler.next = core::intrinsics::transmute(&handler.threads[handler.idx]);
            }
        }
        if handler.curr != handler.next {
            unsafe {
                let pend = ptr::read_volatile(0xE000ED04 as *const u32);
                ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
            }
        }
    }
    unsafe {
        __CORTEXM_THREADS_cpsie();
    }
}

#[no_mangle]
pub extern "C" fn sleep(ticks: u32) {
    let handler = unsafe {&mut __CORTEXM_THREADS_GLOBAL};
    if handler.idx > 0 {
        handler.threads[handler.idx].status = ThreadStatus::Sleeping;
        handler.threads[handler.idx].sleep_ticks = ticks;
        tick();
    }
}

fn get_next_thread_idx() -> usize {
    let handler = unsafe {&mut __CORTEXM_THREADS_GLOBAL};
    if handler.add_idx > 1 {
        // threads were added
        // update sleeping threads
        for i in 1..handler.add_idx {
            if handler.threads[i].status == ThreadStatus::Sleeping {
                if handler.threads[i].sleep_ticks > 0 {
                    handler.threads[i].sleep_ticks = handler.threads[i].sleep_ticks - 1;
                } else {
                    handler.threads[i].status = ThreadStatus::Idle;
                }
            }
        }
        let start_idx = handler.idx + 1;
        if start_idx >= handler.add_idx {
            let start_idx = 1;
            for i in start_idx..handler.add_idx {
                if handler.threads[i].status != ThreadStatus::Sleeping {
                    return i;
                }
            }
            return 0;
        } else {
            for i in start_idx..handler.add_idx {
                if handler.threads[i].status != ThreadStatus::Sleeping {
                    return i;
                }
            }
            for i in 1..start_idx {
                if handler.threads[i].status != ThreadStatus::Sleeping {
                    return i;
                }
            }
            return 0;  
        }
    } else {
        return 0;
    }
}

fn create_tcb(stack: &mut [u32], handler: fn() -> !) -> ThreadControlBlock {
    let idx = stack.len() - 1;
    stack[idx] = 1 << 24; // xPSR
    stack[idx - 1] = unsafe { core::intrinsics::transmute(handler as *const fn()) }; // PC
    stack[idx - 2] = 0xFFFFFFFD; // LR
    stack[idx - 3] = 0xCCCCCCCC; // R12
    stack[idx - 4] = 0x33333333; // R3
    stack[idx - 5] = 0x22222222; // R2
    stack[idx - 6] = 0x11111111; // R1
    stack[idx - 7] = 0x00000000; // R0
    // aditional regs
    stack[idx - 08] = 0x77777777; // R7
    stack[idx - 09] = 0x66666666; // R6
    stack[idx - 10] = 0x55555555; // R5
    stack[idx - 11] = 0x44444444; // R4
    stack[idx - 12] = 0xBBBBBBBB; // R11
    stack[idx - 13] = 0xAAAAAAAA; // R10
    stack[idx - 14] = 0x99999999; // R9
    stack[idx - 15] = 0x88888888; // R8
    unsafe {
        let tcb = ThreadControlBlock {
            sp: core::intrinsics::transmute(&stack[stack.len() - 16]),
            status: ThreadStatus::Idle,
            sleep_ticks: 0,
        };
        tcb
    }
}

fn insert_tcb(idx: usize, tcb: ThreadControlBlock) {
    unsafe {
        let handler = &mut __CORTEXM_THREADS_GLOBAL;
        handler.threads[idx] = tcb;
    }
}
