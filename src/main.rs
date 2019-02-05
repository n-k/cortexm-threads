#![allow(non_snake_case)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_semihosting::{debug, hprintln};

// extern defs, from link.x or asm.s
extern "C" {
    fn _estack();
    fn activate(stack: &u32) -> &'static u32;
    fn init_activate_env(stack: u32) -> &'static u32;
    fn systick_handler();
    fn svc_handler();
    fn syscall();
}

const HANDLER_MSP: u32 = 0xFFFFFFF1;
const THREAD_MSP: u32 = 0xFFFFFFF9;
const THREAD_PSP: u32 = 0xFFFFFFFD;

#[no_mangle]
pub unsafe extern "C" fn Reset() {
    main();
}

fn main() -> ! {
    let _ = hprintln!("entered _main");

    let init_area: [u32; 32] = [0; 32];
    unsafe {
        let _init_addr: u32 = core::intrinsics::transmute(&init_area[31]);
        init_activate_env(_init_addr + 4);
    }

    let mut usertask_stack: [u32; 256] = [0; 256];
    let mut _addr = create_task(&mut usertask_stack, UserTask1);

    let _ = hprintln!("back in main");
    for i in 1..5 {
        unsafe {
            _addr = activate(_addr);
        }
        let _ = hprintln!("back in main");
    }
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

fn create_task(stack: &mut [u32; 256], _f: unsafe extern "C" fn() -> !) -> &'static u32 {
    for idx in 0..256 {
        stack[idx] = idx as u32;
    }
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
    loop {
        let _ = hprintln!("entered user task!!");
        syscall();
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
    Vector { handler: DefaultExceptionHandler }, // nmi
    Vector { handler: HardFault },
    Vector { handler: DefaultExceptionHandler }, // mem manage
    Vector { handler: DefaultExceptionHandler }, // bus fault
    Vector { handler: DefaultExceptionHandler }, // usage fault
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: svc_handler }, // SVC
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: DefaultExceptionHandler }, // pendsv
    Vector { handler: systick_handler }, // systick
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
