#![no_main]
#![no_std]

use helloworld1::{self as _, *}; // global logger + panicking-behavior + memory layout
use heapless::Vec;
use f072b_disco_bsp::{leds::*, button::*, common::bsp_hw_manage};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    hw_init();

    let mut vector : Vec<u8, 10> = Vec::from_slice(&[1,2,3]).unwrap();
    vector.push(255).unwrap();
    defmt::println!("{:?}", vector.as_slice());
    bsp_set_led(Led::RED, true);
    bsp_set_led(Led::BLUE, true);
    bsp_set_led(Led::ORANGE, true);
    bsp_set_led(Led::GREEN, true);
    bsp_button_assign_action(|| bsp_toggle_led(Led::BLUE));
    loop {
        bsp_hw_manage()
    }
}
