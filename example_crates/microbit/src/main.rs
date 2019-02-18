#![no_main]
#![no_std]

use panic_semihosting;
use cortex_m_semihosting::hprintln;
use microbit::hal::nrf51::{interrupt};

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::{entry, exception};

use cortexm_threads::{tick, init, create_thread, PendSV};

#[entry]
fn main() -> ! {
    let _ = hprintln!("started!");
    if let (Some(p), Some(mut cp)) = (microbit::Peripherals::take(), Peripherals::take()) {
        // TODO: what do these next 3 lines do
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });
        /* Setup rtc1 */
        cp.NVIC.enable(microbit::Interrupt::RTC0);
        p.RTC0.prescaler.write(|w| unsafe { w.bits(4095) });
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });
    }
    let mut stack1 = [0xDEADBEEF; 256];
    let mut stack2 = [0xDEADBEEF; 256];
    unsafe {
        create_thread(&mut stack1, UserTask1);
        create_thread(&mut stack2, UserTask2);
        init();
    }
    loop {
        continue;
    }
}

#[no_mangle]
pub fn UserTask1() -> ! {
    loop {
        let _ = hprintln!("in user task 1 !!");
        for _i in 1..50 {
            cortex_m::asm::nop();
        }
    }
}

#[no_mangle]
pub fn UserTask2() -> ! {
    loop {
        let _ = hprintln!("in user task 2 !!");
        for _i in 1..50 {
            cortex_m::asm::nop();
        }
    }
}

#[interrupt]
fn RTC0() {
    let _ = hprintln!("RTC0!");
    unsafe {
        tick();
    }
}
