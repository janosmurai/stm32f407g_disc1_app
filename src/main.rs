#![no_main]
#![no_std]

extern crate panic_semihosting;

use stm32f4xx_hal::gpio::gpiod::PD12;
use stm32f4xx_hal::gpio::gpiod::PD13;
use stm32f4xx_hal::gpio::gpiod::PD14;
use stm32f4xx_hal::gpio::gpiod::PD15;
use stm32f4xx_hal::gpio::PushPull;
use stm32f4xx_hal::gpio::Output;
use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};


struct LEDs
{
    green: PD12<Output<PushPull>>,
    orange: PD13<Output<PushPull>>,
    red: PD14<Output<PushPull>>,
    blue: PD15<Output<PushPull>>,
}

#[entry]
fn main() -> !
{
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let stm32f407_peripherals = stm32::Peripherals::take().unwrap();
    let rcc = stm32f407_peripherals.RCC.constrain();
    let gpiod = stm32f407_peripherals.GPIOD.split();
    let clocks = rcc.cfgr.freeze();
    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);
    
    let mut leds = LEDs
    {
        green: gpiod.pd12.into_push_pull_output(),
        orange: gpiod.pd13.into_push_pull_output(),
        red: gpiod.pd14.into_push_pull_output(),
        blue: gpiod.pd15.into_push_pull_output(),
    };

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
