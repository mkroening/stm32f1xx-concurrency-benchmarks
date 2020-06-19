#![no_main]
#![no_std]

use async_embedded::task;
use cortex_m_rt::entry;
use panic_semihosting as _; // panic handler
use stm32f1xx_concurrency_benchmarks::{r#yield::Yield, static_pins::StaticPA};
use stm32f1xx_hal::{pac::Peripherals, prelude::*};
use typenum::U5;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    task::spawn(async {
        loop {
            StaticPA::<U5>::set_low();
            Yield::default().await;
        }
    });

    task::block_on(async {
        loop {
            StaticPA::<U5>::set_high();
            Yield::default().await;
        }
    })
}
