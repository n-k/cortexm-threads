#![no_main]
#![no_std]

use panic_semihosting;
use cortex_m_semihosting::hprintln;
use microbit::hal::nrf51::{interrupt};

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::{entry, exception};

use core::ptr;
use cortexm_threads::{tick, init, create_thread, PendSV, __CORTEXM_THREADS_cpsid, __CORTEXM_THREADS_cpsie};

#[entry]
fn main() -> ! {
    let _ = hprintln!("started!");
    let mut stack1 = [0xDEADBEEF; 256];
    let mut stack2 = [0xDEADBEEF; 256];
    unsafe {
        create_thread(&mut stack1, UserTask1);
        create_thread(&mut stack2, UserTask2);
        init();
    }
    if let (Some(p), Some(mut cp)) = (microbit::Peripherals::take(), Peripherals::take()) {
        // TODO: what do these next 3 lines do
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });
        unsafe {
            // set pendsv as low priority
            ptr::write_volatile(0xE000ED20 as *mut u32, 0xFF << 16);
            cp.NVIC.set_priority(microbit::Interrupt::RTC0, 0x00);
        }
        /* Setup rtc1 */
        // cp.NVIC.enable(microbit::Interrupt::RTC0);
        // p.RTC0.prescaler.write(|w| unsafe { w.bits(10000) });
        // p.RTC0.evtenset.write(|w| w.tick().set_bit());
        // p.RTC0.intenset.write(|w| w.tick().set_bit());
        // p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });
    }
    loop {
        // for _i in 1..50000 {
        //     cortex_m::asm::nop();
        // }
        unsafe {
            tick();
            // let pend = ptr::read_volatile(0xE000ED04 as *const u32);
            // ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
        }
        let _ = hprintln!("DEBUG: IN MAIN !!");
    }
}

#[no_mangle]
pub fn UserTask1() -> ! {
    loop {
        let _ = hprintln!("in user task 1 !!");
        for _i in 1..50000 {
            cortex_m::asm::nop();
        }
        unsafe {
            tick();
            // let pend = ptr::read_volatile(0xE000ED04 as *const u32);
            // ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
        }
    }
}

#[no_mangle]
pub fn UserTask2() -> ! {
    loop {
        let _ = hprintln!("in user task 2 !!");
        for _i in 1..50000 {
            cortex_m::asm::nop();
        }
        unsafe {
            tick();
            // let pend = ptr::read_volatile(0xE000ED04 as *const u32);
            // ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
        }
    }
}

static mut TICK_COUNT: u32 = 0;

#[no_mangle]
pub extern "C" fn RTC0() {
    unsafe {
        __CORTEXM_THREADS_cpsid();
        if TICK_COUNT == 0 {
            // let _ = hprintln!("DEBUG: Pend SV!");
            // tick();
        }
        TICK_COUNT = TICK_COUNT + 1;
        if TICK_COUNT == 10 {
            TICK_COUNT = 0;
        }
        __CORTEXM_THREADS_cpsie();
    }
}
