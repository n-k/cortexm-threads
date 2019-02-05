#![feature(asm, const_fn, naked_functions, core_intrinsics)]
#![allow(non_snake_case)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;

use cortex_m_semihosting::{debug, hprintln};

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    _main();
}

fn _main() -> ! {
    extern "C" {
        fn activate(stack: &u32);
    }
    let _ = hprintln!("entered _main");

    let mut usertask_stack: [u32; 256] = [0; 256];
    unsafe {
        for idx in 0..256 {
            usertask_stack[idx] = idx as u32;
        }
        let _user_task_addr: usize = core::intrinsics::transmute(&UserTask1);
        let _user_task_addr_u32: u32 = core::intrinsics::transmute(&UserTask1);
        usertask_stack[248] = core::intrinsics::transmute(&UserTask1);
        activate(&usertask_stack[240]);
    }

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn UserTask1() -> ! {
    let _ = hprintln!("entered user task");
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    let _ = hprintln!("panic!");
    loop {}
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFaultTrampoline();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector {
        handler: HardFaultTrampoline,
    },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    let _ = hprintln!("Default handler!");
    loop {}
}

#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    let _ = hprintln!("!!!Hard fault!!!, _ef: {:?}", _ef);

    loop {}
}
