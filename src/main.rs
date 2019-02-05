#![allow(non_snake_case)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
use cortex_m_semihosting::{debug, hprintln};

// extern defs, from link.x or asm.s
extern "C" {
    fn _estack();
    fn activate(stack: &u32) -> &'static u32;
    fn init_activate_env(stack: u32) -> &'static u32;
    fn systick_handler();
    fn svc_handler();
    fn syscall();
    // markers for regions
    static mut _sbss: u8;
    static mut _ebss: u8;
    static mut _sdata: u8;
    static mut _edata: u8;
    static _sidata: u8;
}

const THREAD_PSP: u32 = 0xFFFFFFFD;

#[no_mangle]
pub unsafe extern "C" fn Reset() {
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    let init_area: [u32; 32] = [0; 32];
    let _init_addr: u32 = core::intrinsics::transmute(&init_area[31]);
    init_activate_env(_init_addr + 4);
    let _ = hprintln!("inited threads");

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
    syst.set_reload(12_000_000);
    syst.enable_counter();
    syst.enable_interrupt();

    let _ = hprintln!("finished init");
    main();
}

unsafe fn main() -> ! {
    let _ = hprintln!("entered _main");

    let mut usertask_stack1: [u32; 256] = [0; 256];
    let mut usertask_stack2: [u32; 256] = [0; 256];
    let mut stack_pointers: [&u32; 2] = [
        create_task(&mut usertask_stack1, UserTask1),
        create_task(&mut usertask_stack2, UserTask2),
    ];

    let _ = hprintln!("starting round-robin scheduler");
    let mut idx: usize = 0;
    for _i in 1..5 {
        stack_pointers[idx] = activate(stack_pointers[idx]);
        let _ = hprintln!("back in main");
        idx = if idx == 1 { 0 } else { idx + 1 };
    }
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

fn create_task(stack: &mut [u32; 256], _f: unsafe extern "C" fn() -> !) -> &'static u32 {
    stack[(256 - 17) + 8] = THREAD_PSP;
    stack[(256 - 17) + 16] = 0x01000000;
    unsafe {
        stack[(256 - 17) + 15] = core::intrinsics::transmute(_f as *const fn());;
        activate(&stack[(256 - 17)])
    }
}

#[link_section = ".usertask.task1"]
#[no_mangle]
pub unsafe extern "C" fn UserTask1() -> ! {
    syscall();
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
    syscall();
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
        handler: svc_handler,
    }, // SVC
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: DefaultExceptionHandler,
    }, // pendsv
    Vector {
        handler: systick_handler,
    }, // systick
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    let _ = hprintln!("Default handler!");
    loop {}
}

#[no_mangle]
pub extern "C" fn HardFault() {
    let _ = hprintln!("!!!Hard fault!!!");
    loop {}
}
