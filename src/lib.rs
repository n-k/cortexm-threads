#![allow(non_snake_case)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
use cortex_m_semihosting::{debug, hprintln};

#[no_mangle]
static mut __OS_PTR: u32 = 0;

static mut __OS: OS = OS {
    curr: 0,
        next: 0,
        _idx: 0,
        threads: [
            OSThread {sp: 0},
            OSThread {sp: 0}
        ],
};

#[repr(C)]
pub struct OS {
    curr: u32,
    next: u32,
    _idx: usize,
    threads: [OSThread; 2],
}

#[repr(C)]
pub struct OSThread {
    pub sp: u32,
}

pub unsafe extern "C" fn init(threads: [OSThread; 2]) {
    let os: OS = OS {
        curr: 0,
        next: 0,
        _idx: 0,
        threads: threads,
    };
    __OS = os;
    __OS_PTR = core::intrinsics::transmute(&__OS);
    let _ = hprintln!("OS @ : 0x{:x} {}", __OS_PTR, __OS_PTR);
}

#[no_mangle]
pub unsafe extern "C" fn tick() {
    if __OS_PTR != 0 {
        let os: &mut OS = &mut *(__OS_PTR as *mut OS);
        let mut pend_sv = false;
        if os.curr == os.next {
            let _ = hprintln!("curr == next");
            // schedule a thread to be run
            os.next = core::intrinsics::transmute(&os.threads[os._idx]);
            os._idx = os._idx + 1;
            if os._idx > os.threads.len() - 1 {
                os._idx = 0;
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
