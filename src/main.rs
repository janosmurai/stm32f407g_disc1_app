#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};
use stm32f407g_disc1::leds as led_driver;

#[entry]
fn main() -> !
{
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let stm32f407_peripherals = stm32::Peripherals::take().unwrap();
    let rcc = stm32f407_peripherals.RCC.constrain();
    let gpiod = stm32f407_peripherals.GPIOD.split();
    let clocks = rcc.cfgr.freeze();
    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);
    
    let mut leds = led_driver::init(gpiod);

    leds.orange.set_high();
    delay.delay_ms(200_u16);
    loop
    {
        leds.red.set_high();
        delay.delay_ms(100_u16);

        leds.orange.set_low();
        delay.delay_ms(100_u16);
        leds.blue.set_high();
        delay.delay_ms(100_u16);

        leds.red.set_low();
        delay.delay_ms(100_u16);
        leds.green.set_high();
        delay.delay_ms(100_u16);

        leds.blue.set_low();
        delay.delay_ms(100_u16);
        leds.orange.set_high();
        delay.delay_ms(100_u16);

        leds.green.set_low();
    }
}
