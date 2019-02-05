#![feature(asm, const_fn, naked_functions, core_intrinsics)]
#![allow(non_snake_case)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_semihosting::{debug, hprintln};

// extern defs
extern "C" {
    fn _estack();
    fn activate(stack: &u32);
}

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() {
    _main();
}

fn _main() -> ! {
    let _ = hprintln!("entered _main");
    let mut usertask_stack: [u32; 256] = [0; 256];
    unsafe {
        for idx in 0..256 {
            usertask_stack[idx] = idx as u32;
        }
        let _user_task_addr_u32: u32 = core::intrinsics::transmute(&UserTask1);
        usertask_stack[248] = 17;//_user_task_addr_u32
        activate(&usertask_stack[240]);
    }

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[link_section = ".usertask"]
#[no_mangle]
pub unsafe extern "C" fn UserTask1() -> ! {
    let _ = hprintln!("entered user task");
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
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
pub static EXCEPTIONS: [Vector; 4] = [
    Vector { handler: _estack },
    Vector { handler: Reset },
    Vector { handler: NMI },
    Vector { handler: HardFault },
];

#[no_mangle]
pub extern "C" fn NMI() {
    let _ = hprintln!("!!!NMI!!!");
    loop {}
}

#[no_mangle]
pub extern "C" fn HardFault() {
    let _ = hprintln!("!!!Hard fault!!!");
    loop {}
}
