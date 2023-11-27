use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use stm32f0xx_hal::{
    gpio::{gpioa::*, Floating, Input},
    pac::{interrupt, EXTI, self},
};

static BUTTON_CALLBACK: Mutex<RefCell<Option<fn()>>> = Mutex::new(RefCell::new(None));

pub fn bsp_init_button(pin: PA0<Input<Floating>>, exti: &mut EXTI) {
    free(|cs| {
        pin.into_pull_down_input(cs);
    });
    exti.imr.write(|w| w.mr0().set_bit());
    exti.rtsr.write(|w| w.tr0().set_bit());
    unsafe {
        pac::NVIC::unmask(interrupt::EXTI0_1);
    }
}

pub fn bsp_button_assign_action(f: fn()) {
    free(|cs| {
        *BUTTON_CALLBACK.borrow(cs).borrow_mut() = Some(f);
    })
}

fn bsp_button_irq_handler() {
    free(|cs| {
        if let Some(f) = *BUTTON_CALLBACK.borrow(cs).borrow() {
            f();
        }
    })
}

#[cortex_m_rt::interrupt]
unsafe fn EXTI0_1() {
    let exti = unsafe { &(*EXTI::ptr()) };
    // Check if the interrupt line is asserted
    if exti.pr.read().pif0().bit_is_set() {
        // Clear the interrupt flag by writing 1 to the PR0 bit
        exti.pr.write(|w| w.pif0().set_bit());
        bsp_button_irq_handler();
    }
}
