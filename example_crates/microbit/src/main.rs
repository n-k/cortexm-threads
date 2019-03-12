#![no_main]
#![no_std]

extern crate panic_semihosting;
use cortex_m_semihosting::hprintln;
use cortex_m_rt::{entry};
use cortexm_threads::{init, create_thread};

extern "C" {
	fn SysTick();
}

#[entry]
fn main() -> ! {
    let _ = hprintln!("started!");
    if let Some(p) = microbit::Peripherals::take() {
        // TODO: what do these next 3 lines do
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });
    }
    let mut stack1 = [0xDEADBEEF; 256];
    let mut stack2 = [0xDEADBEEF; 256];
    let _ = create_thread(&mut stack1, user_task1);
    let _ = create_thread(&mut stack2, user_task2);
    init();
}

#[no_mangle]
pub fn user_task1() -> ! {
    loop {
        let _ = hprintln!("in user task 1 !!");
        for _i in 1..50000 {
            cortex_m::asm::nop();
        }
        unsafe {SysTick();}
    }
}

#[no_mangle]
pub fn user_task2() -> ! {
    loop {
        let _ = hprintln!("in user task 2 !!");
        for _i in 1..50000 {
            cortex_m::asm::nop();
        }
        unsafe {SysTick();}
    }
}
