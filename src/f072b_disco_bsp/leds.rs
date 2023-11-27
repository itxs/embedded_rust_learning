use crate::pin_set;
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use stm32f0xx_hal::{
    gpio::{gpioc::*, *},
    prelude::*,
};

#[derive(PartialEq)]
pub enum Led {
    RED,
    GREEN,
    BLUE,
    ORANGE,
}

static RED_LED_PIN: Mutex<RefCell<Option<PC6<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static GREEN_LED_PIN: Mutex<RefCell<Option<PC9<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static ORANGE_LED_PIN: Mutex<RefCell<Option<PC8<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static BLUE_LED_PIN: Mutex<RefCell<Option<PC7<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));

pub fn bsp_init_leds(
    red_led_pin: PC6<Input<Floating>>,
    blue_led_pin: PC7<Input<Floating>>,
    orange_led_pin: PC8<Input<Floating>>,
    green_led_pin: PC9<Input<Floating>>,
) {
    free(|cs| {
        RED_LED_PIN.borrow(cs).replace(Some(red_led_pin.into_push_pull_output(cs)));
        BLUE_LED_PIN.borrow(cs).replace(Some(blue_led_pin.into_push_pull_output(cs)));
        ORANGE_LED_PIN.borrow(cs).replace(Some(orange_led_pin.into_push_pull_output(cs)));
        GREEN_LED_PIN.borrow(cs).replace(Some(green_led_pin.into_push_pull_output(cs)));
    });
}

pub fn bsp_set_led(led: Led, state: bool) {
    free(|cs| match led {
        Led::RED => pin_set!(RED_LED_PIN, state, cs),
        Led::BLUE => pin_set!(BLUE_LED_PIN, state, cs),
        Led::ORANGE => pin_set!(ORANGE_LED_PIN, state, cs),
        Led::GREEN => pin_set!(GREEN_LED_PIN, state, cs),
    });
}
