#![no_main]
#![no_std]

use core::{cell::RefCell, ops::DerefMut};
use cortex_m_semihosting::debug;
use defmt_rtt as _; // global logger
                    // TODO(5) adjust HAL import
                    // use some_hal as _; // memory layout
use cortex_m::interrupt::{free, Mutex};
use panic_probe as _;
use stm32f0xx_hal::{
    self as _,
    gpio::*,
    pac::{self, interrupt, EXTI},
    prelude::*,
    rcc::RccExt,
};

#[derive(PartialEq)]
pub enum Led {
    RED,
    GREEN,
    BLUE,
    ORANGE,
}

static RED_LED_PIN: Mutex<RefCell<Option<gpioc::PC6<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static GREEN_LED_PIN: Mutex<RefCell<Option<gpioc::PC9<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static ORANGE_LED_PIN: Mutex<RefCell<Option<gpioc::PC8<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static BLUE_LED_PIN: Mutex<RefCell<Option<gpioc::PC7<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));

pub fn hw_init() {
    let mut p = pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
    let pc = p.GPIOC.split(&mut rcc);
    let pa = p.GPIOA.split(&mut rcc);
    let exti = p.EXTI;
    exti.imr.modify(|_, w| w.mr0().set_bit());
    exti.rtsr.modify(|_, w| w.tr0().set_bit());
    unsafe {
        pac::NVIC::unmask(interrupt::EXTI0_1);
    }

    // Store the pin in the global static
    free(|cs| {
        pa.pa0.into_pull_down_input(cs);
        let red_led_pin = pc.pc6.into_push_pull_output(cs);
        RED_LED_PIN.borrow(cs).replace(Some(red_led_pin));
        let blue_led_pin = pc.pc7.into_push_pull_output(cs);
        BLUE_LED_PIN.borrow(cs).replace(Some(blue_led_pin));
        let orange_led_pin = pc.pc8.into_push_pull_output(cs);
        ORANGE_LED_PIN.borrow(cs).replace(Some(orange_led_pin));
        let green_led_pin = pc.pc9.into_push_pull_output(cs);
        GREEN_LED_PIN.borrow(cs).replace(Some(green_led_pin));
    });
}

pub fn bsp_set_led(led: Led, state: bool) {
    free(|cs| match led {
        Led::RED => {
            if let Some(ref mut led) = RED_LED_PIN.borrow(cs).borrow_mut().deref_mut() {
                led.set_state(state.into()).unwrap();
            }
        }
        Led::BLUE => {
            if let Some(ref mut led) = BLUE_LED_PIN.borrow(cs).borrow_mut().deref_mut() {
                led.set_state(state.into()).unwrap();
            }
        }
        Led::ORANGE => {
            if let Some(ref mut led) = ORANGE_LED_PIN.borrow(cs).borrow_mut().deref_mut() {
                led.set_state(state.into()).unwrap();
            }
        }
        Led::GREEN => {
            if let Some(ref mut led) = GREEN_LED_PIN.borrow(cs).borrow_mut().deref_mut() {
                led.set_state(state.into()).unwrap();
            }
        }
    });
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

#[cortex_m_rt::interrupt]
unsafe fn EXTI0_1() {
    let exti = unsafe { &(*EXTI::ptr()) };

    // Check if the interrupt line is asserted
    if exti.pr.read().pif0().bit_is_set() {
        // Clear the interrupt flag by writing 1 to the PR0 bit
        exti.pr.write(|w| w.pif0().set_bit());
        
        bsp_set_led(Led::BLUE, true);
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
