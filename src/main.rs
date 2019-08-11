#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f4::stm32f407;

use stm32f407g_disc1::shared_resources as shared_resources;
use stm32f407g_disc1::leds as led_driver;
use stm32f407g_disc1::buttons as button_driver;

use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::rcc::RccExt;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
fn main() -> !
{
    hprintln!("App started").unwrap();


    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let stm32f407_peripherals = stm32f407::Peripherals::take().unwrap();
    let gpioa = stm32f407_peripherals.GPIOA.split();
    let gpiod = stm32f407_peripherals.GPIOD.split();
    let mut nvic = cortex_peripherals.NVIC;
    let rcc = stm32f407_peripherals.RCC.constrain();

    shared_resources::init(stm32f407_peripherals.EXTI, stm32f407_peripherals.SYSCFG);
    button_driver::init(gpioa);
    let mut leds = led_driver::init(gpiod);
    let clocks = rcc.cfgr.freeze();
    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);

    nvic.enable(stm32f407::Interrupt::EXTI0);

    loop
    {
        leds.red.set_high();
        delay.delay_ms(500_u16);
        leds.red.set_low();
    }
}