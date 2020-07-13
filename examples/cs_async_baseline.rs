#![no_main]
#![no_std]

use core::{future::Future, task::Context};
use cortex_m_rt::entry;
use futures::{pin_mut, task};
use panic_halt as _; // panic handler
use stm32f1xx_concurrency_benchmarks::{r#yield::Yield, static_pins::StaticPA};
use stm32f1xx_hal::{pac::Peripherals, prelude::*};
use typenum::U5;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    let set_low = async {
        loop {
            StaticPA::<U5>::set_low();
            Yield::default().await;
        }
    };
    pin_mut!(set_low);

    let set_high = async {
        loop {
            StaticPA::<U5>::set_high();
            Yield::default().await;
        }
    };
    pin_mut!(set_high);

    let waker = task::noop_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        let _ = set_low.as_mut().poll(&mut cx);
        let _ = set_high.as_mut().poll(&mut cx);
    }
}
