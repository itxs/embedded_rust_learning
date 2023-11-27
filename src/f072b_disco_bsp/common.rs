#[macro_export]
macro_rules! pin_set {
    ($pin:expr, $state:expr, $cs:expr) => {
        if let Some(ref mut output) = $pin.borrow($cs).borrow_mut().deref_mut() {
            output.set_state($state.into()).unwrap();
        }
    };
}
