#![no_main]
#![no_std]

use helloworld1::{self as _, hw_init}; // global logger + panicking-behavior + memory layout
use heapless::Vec;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    hw_init();

    let mut vector : Vec<u8, 10> = Vec::from_slice(&[1,2,3]).unwrap();
    vector.push(4).unwrap();
    defmt::println!("{:?}", vector.as_slice());

    helloworld1::exit()
}
