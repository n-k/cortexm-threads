#![no_std]

use core::ptr;

#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;

static mut __CORTEXM_THREADS_GLOBAL: OS = OS {
    curr: 0,
        next: 0,
        idx: 0,
        threads: [
            ThreadControlBlock {sp: 0},
            ThreadControlBlock {sp: 0}
        ],
};

#[repr(C)]
struct OS {
    curr: u32,
    next: u32,
    idx: usize,
    threads: [ThreadControlBlock; 2],
}

#[repr(C)]
pub struct ThreadControlBlock {
    pub sp: u32,
}

pub unsafe extern "C" fn init(threads: [ThreadControlBlock; 2]) {
    let os: OS = OS {
        curr: 0,
        next: 0,
        idx: 0,
        threads: threads,
    };
    __CORTEXM_THREADS_GLOBAL = os;
    __CORTEXM_THREADS_GLOBAL_PTR = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
}

#[no_mangle]
pub unsafe extern "C" fn tick() {
    if __CORTEXM_THREADS_GLOBAL_PTR != 0 {
        let os: &mut OS = &mut *(__CORTEXM_THREADS_GLOBAL_PTR as *mut OS);
        if os.curr == os.next {
            // schedule a thread to be run
            os.next = core::intrinsics::transmute(&os.threads[os.idx]);
            os.idx = os.idx + 1;
            if os.idx > os.threads.len() - 1 {
                os.idx = 0;
            }
        }
        if os.curr != os.next {
            let pend = ptr::read_volatile(0xE000ED04 as *const u32);
            ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
        }
    }
}

extern "C" {
    pub fn PendSVHandler();
}
