#[macro_export]
macro_rules! pin_set {
    ($pin:expr, $state:expr, $cs:expr) => {
        if let Some(ref mut output) = $pin.borrow($cs).borrow_mut().deref_mut() {
            output.set_state($state.into()).unwrap();
        }
    };
}

#[macro_export]
macro_rules! pin_toggle {
    ($pin:expr, $cs:expr) => {
        if let Some(ref mut output) = $pin.borrow($cs).borrow_mut().deref_mut() {
            output.toggle().unwrap();
        }
    };
}

use crate::bsp_button_manage;

pub fn bsp_hw_manage() {
    bsp_button_manage();
}
