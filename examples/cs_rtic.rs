#![no_main]
#![no_std]

use panic_halt as _; // panic handler
use rtic::app;
use stm32f1xx_concurrency_benchmarks::static_pins::{consts::U5, StaticPA};
use stm32f1xx_hal::prelude::*;

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    #[init(spawn = [set_high])]
    fn init(cx: init::Context) {
        let dp = cx.device;
        let mut rcc = dp.RCC.constrain();
        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

        gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

        cx.spawn.set_high().unwrap()
    }

    #[task(spawn = [set_high])]
    fn set_low(cx: set_low::Context) {
        StaticPA::<U5>::set_low();
        cx.spawn.set_high().unwrap()
    }

    #[task(spawn = [set_low])]
    fn set_high(cx: set_high::Context) {
        StaticPA::<U5>::set_high();
        cx.spawn.set_low().unwrap()
    }

    extern "C" {
        fn EXTI9_5();
        fn EXTI15_10();
    }
};
