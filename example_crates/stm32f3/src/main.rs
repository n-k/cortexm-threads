#![no_std]
#![no_main]

extern crate panic_halt;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry};
use cortex_m_semihosting::{hprintln};

use f3::{
    hal::{
		i2c::I2c,
		prelude::*, 
		stm32f30x
	},
    led::Leds,
	Lsm303dlhc,
};

use cortexm_threads::{init, create_thread_with_config, sleep};

static mut LEDS: Option<Leds> = None;
static mut SENSOR: Option<Lsm303dlhc> = None;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
	let dp = stm32f30x::Peripherals::take().unwrap();
	
	let mut rcc = dp.RCC.constrain();
	let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));
	unsafe {
		LEDS = Some(leds);
	}
	
	let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

	let mut flash = dp.FLASH.constrain();
	let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
	unsafe {
		SENSOR = Some(Lsm303dlhc::new(i2c).unwrap());
	}
    
	let mut syst = cp.SYST;
    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // tick every 12.5ms
    syst.set_reload(100_000);
    syst.enable_counter();
    syst.enable_interrupt();

	let mut stack1 = [0xDEADBEEF; 1024];
    let mut stack2 = [0xDEADBEEF; 1024];
    let _ = create_thread_with_config(&mut stack1, user_task_1, 0xff, true);
    let _ = create_thread_with_config(&mut stack2, user_task_2, 0x00, false);
    init();
}

pub fn user_task_1() -> ! {
	loop {
		if unsafe { LEDS.is_some() } {
			let leds = unsafe { LEDS.as_mut().unwrap() };
			for curr in 0..8 {
				let next = (curr + 1) % 8;
	
				leds[next].on();
				sleep(4);
				leds[curr].off();
				sleep(4);
			}
		} 
	}
}

pub fn user_task_2() -> ! {
	loop {
		if unsafe { SENSOR.is_some() } {
			let sensor = unsafe { SENSOR.as_mut().unwrap() };
			let x = sensor.mag();
			let _ = hprintln!("{:?}", x);
			sleep(50);
		}
	}
}
