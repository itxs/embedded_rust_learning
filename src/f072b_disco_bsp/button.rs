use core::{cell::RefCell, ops::DerefMut};
use cortex_m::interrupt::{free, Mutex};
use stm32f0xx_hal::{
    gpio::{gpioa::*, Floating, Input},
    pac::{interrupt, EXTI, GPIOA, NVIC, TIM6},
};

static BUTTON_CALLBACK: Mutex<RefCell<Option<fn()>>> = Mutex::new(RefCell::new(None));
static BUTTON_STATE: Mutex<RefCell<Option<bool>>> = Mutex::new(RefCell::new(None));
static BUTTON_EXEC: Mutex<RefCell<Option<bool>>> = Mutex::new(RefCell::new(None));

pub fn bsp_init_button(pin: PA0<Input<Floating>>, exti: &EXTI, tim: &TIM6) {
    free(|cs| {
        pin.into_pull_down_input(cs);
    });
    // Timer init
    tim.arr.write(|w| w.arr().bits(10));
    tim.psc.write(|w| w.psc().bits(1000 * 48));
    tim.dier.write(|w| w.uie().set_bit());

    exti.rtsr.write(|w| w.tr0().set_bit());
    exti.ftsr.write(|w| w.tr0().set_bit());
    exti.imr.write(|w| w.mr0().set_bit());
    unsafe {
        NVIC::unmask(interrupt::EXTI0_1);
        NVIC::unmask(interrupt::TIM6_DAC);
    }
}

pub fn bsp_button_assign_action(f: fn()) {
    free(|cs| {
        BUTTON_CALLBACK.borrow(cs).borrow_mut().replace(f);
    })
}

pub fn bsp_button_manage() {
    free(|cs| {
        let mut need_execute = BUTTON_EXEC.borrow(cs).borrow_mut();
        if let Some(true) = *need_execute {
            if let Some(f) = *BUTTON_CALLBACK.borrow(cs).borrow() {
                f();
            }
            *need_execute = Some(false);
        }
    })
}

#[cortex_m_rt::interrupt]
unsafe fn EXTI0_1() {
    let exti = unsafe { &(*EXTI::ptr()) };

    if exti.pr.read().pif0().bit_is_set() {
        exti.imr.modify(|_, w| w.mr0().clear_bit());
        let tim = unsafe { &(*TIM6::ptr()) };

        free(|cs| {
            let gpioa = unsafe { &(*GPIOA::ptr()) };
            BUTTON_STATE
                .borrow(cs)
                .borrow_mut()
                .deref_mut()
                .replace(gpioa.idr.read().idr0().bit());
        });
        tim.cr1.write(|w| w.cen().set_bit());
        exti.pr.write(|w| w.pif0().set_bit());
    }
}

#[cortex_m_rt::interrupt]
unsafe fn TIM6_DAC() {
    let tim = unsafe { &(*TIM6::ptr()) };
    if tim.sr.read().uif().bit_is_set() {
        free(|cs| {
            let gpioa = unsafe { &(*GPIOA::ptr()) };
            let state: bool = gpioa.idr.read().idr0().bit();
            let prev_state = BUTTON_STATE.borrow(cs).borrow_mut().deref_mut().unwrap();
            BUTTON_STATE
                .borrow(cs)
                .borrow_mut()
                .deref_mut()
                .replace(state);
            if (state == prev_state) && state {
                BUTTON_EXEC.borrow(cs).borrow_mut().replace(true);
            }
        });
        tim.cr1.modify(|_, w| w.cen().clear_bit());
        let exti = unsafe { &(*EXTI::ptr()) };
        exti.imr.write(|w| w.mr0().set_bit());
        tim.sr.modify(|_, w| w.uif().clear_bit());
    }
}
