pub use typenum::consts;

use core::marker::PhantomData;
use stm32f1xx_hal::pac;
use typenum::Unsigned;

pub struct StaticPA<PIN> {
    phantom_data: PhantomData<PIN>,
}

impl<PIN> StaticPA<PIN>
where
    PIN: Unsigned,
{
    fn write_bits_to_register(bits: u32) {
        // Safety: Atomic write to a stateless register
        unsafe { (*pac::GPIOA::ptr()).bsrr.write(|w| w.bits(bits)) }
    }

    pub fn set_high() {
        Self::write_bits_to_register(1 << (16 + PIN::I32));
    }

    pub fn set_low() {
        Self::write_bits_to_register(1 << PIN::I32);
    }
}
