#![no_main]
#![no_std]

use cortex_m::asm;
use panic_halt as _; // panic handler
use rtic::app;
use stm32f1xx_hal::{
    gpio::{
        gpioa::{PA5, PA6},
        Edge, ExtiPin, Floating, Input, Output, PushPull, State,
    },
    prelude::*,
};

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        trigger_pin: PA5<Input<Floating>>,
        reaction_pin: PA6<Output<PushPull>>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Extract needed peripherals
        let dp = cx.device;
        let mut rcc = dp.RCC.constrain();
        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
        let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

        let mut trigger_pin = gpioa.pa5.into_floating_input(&mut gpioa.crl);
        trigger_pin.make_interrupt_source(&mut afio);
        trigger_pin.trigger_on_edge(&dp.EXTI, Edge::RISING_FALLING);
        trigger_pin.enable_interrupt(&dp.EXTI);

        let reaction_pin = gpioa
            .pa6
            .into_push_pull_output_with_state(&mut gpioa.crl, State::High);

        init::LateResources {
            reaction_pin,
            trigger_pin,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(binds = EXTI9_5, resources = [reaction_pin, trigger_pin])]
    fn react(cx: react::Context) {
        let react::Resources {
            reaction_pin,
            trigger_pin,
        } = cx.resources;

        reaction_pin.toggle().unwrap();
        trigger_pin.clear_interrupt_pending_bit();
    }
};
