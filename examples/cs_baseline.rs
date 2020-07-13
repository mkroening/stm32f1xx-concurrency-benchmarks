#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _; // panic handler
use stm32f1xx_concurrency_benchmarks::static_pins::{consts::U5, StaticPA};
use stm32f1xx_hal::{pac::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    loop {
        StaticPA::<U5>::set_high();
        StaticPA::<U5>::set_low();
    }
}
