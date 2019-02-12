#![allow(non_snake_case)]
#![no_std]
#![no_main]

#![feature(asm)]

use core::panic::PanicInfo;
use core::ptr;
use cortex_m_semihosting::{debug, hprintln};

/* RTOS */

#[no_mangle]
static mut __OS_PTR: u32 = 0;

#[repr(C)]
pub struct OS {
    curr: u32,
    next: u32,
    _idx: usize,
    threads: [OSThread; 2],
}

#[repr(C)]
pub struct OSThread {
    sp: u32,
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

#[no_mangle]
pub unsafe extern "C" fn SystickHandler() {
    let _ = hprintln!("Systick handler!");
    tick();
}

extern "C" {
    fn PendSVHandler();
}

/* END RTOS */

// extern defs, from link.x or asm.s
extern "C" {
    fn _estack();
    // markers for regions
    static mut _sbss: u8;
    static mut _ebss: u8;
    static mut _sdata: u8;
    static mut _edata: u8;
    static _sidata: u8;
}

#[no_mangle]
pub unsafe extern "C" fn Reset() {
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
    syst.set_reload(12_000_000);
    syst.enable_counter();
    syst.enable_interrupt();
    // set pendsv as low priority
    ptr::write_volatile(0xE000ED20 as *mut u32, 0xFF << 16);

    let _ = hprintln!("finished init");
    main();
}

unsafe fn main() -> ! {
    let _ = hprintln!("entered _main");
    let mut stack1: [u32; 256] = [0xDEADBEEF; 256];
    stack1[255] = 1 << 24;
    stack1[254] = core::intrinsics::transmute(UserTask1 as *const fn());
    stack1[253] = 0x0000000E;
    stack1[252] = 0x0000000C;
    stack1[251] = 0x00000003;
    stack1[250] = 0x00000002;
    stack1[249] = 0x00000001;
    stack1[248] = 0x00000000;
    // aditional regs
    stack1[247] = 0x0000000B;
    stack1[246] = 0x0000000A;
    stack1[245] = 0x00000009;
    stack1[244] = 0x00000008;
    stack1[243] = 0x00000007;
    stack1[242] = 0x00000006;
    stack1[241] = 0x00000005;
    stack1[240] = 0x00000004;
    let mut stack2: [u32; 256] = [0xDEADBEEF; 256];
    stack2[255] = 1 << 24;
    stack2[254] = core::intrinsics::transmute(UserTask2 as *const fn());
    stack2[253] = 0x0000000E;
    stack2[252] = 0x0000000C;
    stack2[251] = 0x00000003;
    stack2[250] = 0x00000002;
    stack2[249] = 0x00000001;
    stack2[248] = 0x00000000;
    // additional regs
    // aditional regs
    stack2[247] = 0x0000000B;
    stack2[246] = 0x0000000A;
    stack2[245] = 0x00000009;
    stack2[244] = 0x00000008;
    stack2[243] = 0x00000007;
    stack2[242] = 0x00000006;
    stack2[241] = 0x00000005;
    stack2[240] = 0x00000004;

    let sp1_addr: u32 = core::intrinsics::transmute(&stack1[240]);
    let _ = hprintln!("sp1: 0x{:x}", sp1_addr);

    let _t1_addr: u32 = core::intrinsics::transmute(&stack1);
    let _t2_addr: u32 = core::intrinsics::transmute(&stack2);
    let _ = hprintln!("threads: 0x{:x} 0x{:x}", _t1_addr, _t2_addr);
    let _ = hprintln!("threads: {} {}", _t1_addr, _t2_addr);

    let os: OS = OS {
        curr: 0,
        next: 0,
        _idx: 0,
        threads: [
            OSThread {
                sp: core::intrinsics::transmute(&stack1[240]),
            },
            OSThread {
                sp: core::intrinsics::transmute(&stack2[240]),
            },
        ],
    };
    __OS_PTR = core::intrinsics::transmute(&os);
    let _ = hprintln!("OS @ : 0x{:x} 0x{:?}", __OS_PTR, __OS_PTR);
    // debug::exit(debug::EXIT_SUCCESS);
    loop {
        for _i in 1..500000 {
            cortex_m::asm::nop();
        }
        let _ = hprintln!("in main");
    }
}

#[link_section = ".usertask.task1"]
#[no_mangle]
pub unsafe extern "C" fn UserTask1() -> ! {
    loop {
        let _ = hprintln!("in user task 1 !!");
        for _i in 1..500000 {
            cortex_m::asm::nop();
        }
    }
}

#[link_section = ".usertask.task2"]
#[no_mangle]
pub unsafe extern "C" fn UserTask2() -> ! {
    loop {
        let _ = hprintln!("in user task 2 !!");
        for _i in 1..500000 {
            cortex_m::asm::nop();
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    let _ = hprintln!("panic!");
    loop {}
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[link_section = ".isr_vector"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 16] = [
    Vector { handler: _estack },
    Vector { handler: Reset },
    Vector {
        handler: DefaultExceptionHandler,
    }, // nmi
    Vector { handler: HardFault },
    Vector {
        handler: DefaultExceptionHandler,
    }, // mem manage
    Vector {
        handler: DefaultExceptionHandler,
    }, // bus fault
    Vector {
        handler: DefaultExceptionHandler,
    }, // usage fault
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: DefaultExceptionHandler,
    }, // SVC
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: PendSVHandler,
    }, // pendsv
    Vector {
        handler: SystickHandler,
    }, // systick
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    let _ = hprintln!("Default handler!");
}

#[no_mangle]
pub extern "C" fn HardFault() {
    let _ = hprintln!("!!!Hard fault!!!");
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
