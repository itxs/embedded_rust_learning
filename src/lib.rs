#![no_main]
#![no_std]

pub mod f072b_disco_bsp;

use cortex_m_semihosting::debug;
use defmt_rtt as _;
use f072b_disco_bsp::{button::*, leds::*};
use panic_probe as _;
use stm32f0xx_hal::{self as _, gpio::*, pac, prelude::*, rcc::RccExt};

pub fn hw_init() {
    let mut p = pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
    let mut exti = p.EXTI;
    let pa = p.GPIOA.split(&mut rcc);
    let pc = p.GPIOC.split(&mut rcc);
    bsp_init_button(pa.pa0, &mut exti);
    bsp_init_leds(pc.pc6, pc.pc7, pc.pc8, pc.pc9);
    bsp_button_assign_action(|| bsp_set_led(Led::BLUE, true));
    // Store the pin in the global static
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
