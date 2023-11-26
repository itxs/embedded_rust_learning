#![no_main]
#![no_std]

use helloworld1::{self as _, *}; // global logger + panicking-behavior + memory layout
use heapless::Vec;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    hw_init();

    let mut vector : Vec<u8, 10> = Vec::from_slice(&[1,2,3]).unwrap();
    vector.push(255).unwrap();
    defmt::println!("{:?}", vector.as_slice());
    helloworld1::bsp_set_led(Led::RED, true);
    //helloworld1::bsp_set_led(Led::BLUE, true);
    helloworld1::bsp_set_led(Led::ORANGE, true);
    helloworld1::bsp_set_led(Led::GREEN, true);
    loop {
        
    }
    helloworld1::exit()
}
