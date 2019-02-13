#![allow(non_snake_case)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
use cortex_m_semihosting::{debug, hprintln};

use cortexm_threads::{tick, init, create_thread, __CORTEXM_THREADS_PendSVHandler};

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
    let mut stack1 = [0xDEADBEEF; 256];
    let mut stack2 = [0xDEADBEEF; 256];
    create_thread(&mut stack1, UserTask1);
    create_thread(&mut stack2, UserTask2);
    init();
    // unreachable
    loop {
        cortex_m::asm::nop();
    }
}

pub fn UserTask1() -> ! {
    loop {
        let _ = hprintln!("in user task 1 !!");
        for _i in 1..500000 {
            cortex_m::asm::nop();
        }
    }
}

pub fn UserTask2() -> ! {
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
        handler: __CORTEXM_THREADS_PendSVHandler,
    }, // pendsv
    Vector {
        handler: SystickHandler,
    }, // systick
];

#[no_mangle]
pub unsafe extern "C" fn SystickHandler() {
    let _ = hprintln!("Systick Handler!");
    tick();
}

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
