#![no_main]
#![no_std]

use async_embedded::task;
use async_stm32f1xx::exti::AsyncPin;
use cortex_m_rt::entry;
use panic_halt as _; // panic handler
use stm32f1xx_hal::{
    gpio::{Edge, ExtiPin, State},
    pac::Peripherals,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let trigger_pin = gpioa.pa5.into_floating_input(&mut gpioa.crl);
    let mut trigger_pin = AsyncPin::new(trigger_pin, &mut afio, &dp.EXTI);
    trigger_pin
        .as_mut()
        .trigger_on_edge(&dp.EXTI, Edge::RISING_FALLING);

    let mut reaction_pin = gpioa
        .pa6
        .into_push_pull_output_with_state(&mut gpioa.crl, State::High);

    task::block_on(async move {
        loop {
            reaction_pin.toggle().unwrap();
            trigger_pin.trigger().await;
        }
    })
}
