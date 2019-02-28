#![no_std]
#![no_main]

extern crate panic_halt;

use core::ptr;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};

use cortexm_threads::{tick, init, create_thread};

#[entry]
unsafe fn main() -> ! {
    let _ = hprintln!("Hello, world!").unwrap();

    // set pendsv as low priority
    ptr::write_volatile(0xE000ED20 as *mut u32, 0xFF << 16);

    let mut stack1 = [0xDEADBEEF; 256];
    let mut stack2 = [0xDEADBEEF; 256];
    create_thread(&mut stack1, user_task_1);
    create_thread(&mut stack2, user_task_2);
    init();

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // tick every 250ms
    syst.set_reload(2_000_000);
    syst.enable_counter();
    syst.enable_interrupt();
    loop {}
}

pub fn user_task_1() -> ! {
    loop {
        let _ = hprintln!("in user task 1 !!");
        for _i in 1..5000 {
            cortex_m::asm::nop();
        }
    }
}

pub fn user_task_2() -> ! {
    loop {
        let _ = hprintln!("in user task 2 !!");
        for _i in 1..5000 {
            cortex_m::asm::nop();
        }
    }
}

#[exception]
unsafe fn SysTick() {
    // let _ = hprintln!(".").unwrap();
    tick();
}