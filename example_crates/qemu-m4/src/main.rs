#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};
use cortexm_threads::{tick, init, create_thread};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(80_000);
    syst.enable_counter();
    syst.enable_interrupt();

	let mut stack1 = [0xDEADBEEF; 512];
    let mut stack2 = [0xDEADBEEF; 512];
    create_thread(&mut stack1, user_task_1);
    create_thread(&mut stack2, user_task_2);
    init();
    loop {}
}

pub fn user_task_1() -> ! {
    loop {
        let _ = hprintln!("in user task 1 !!");
        for _i in 1..500000 {
            cortex_m::asm::nop();
        }
    }
}

pub fn user_task_2() -> ! {
    loop {
        let _ = hprintln!("in user task 2 !!");
        for _i in 1..600000 {
            cortex_m::asm::nop();
        }
    }
}

#[exception]
fn SysTick() {
    tick();
}
