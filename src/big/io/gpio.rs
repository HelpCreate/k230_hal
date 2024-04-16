
pub enum GpioMode {
    Input,
    Output,

}

pub struct Pin {
    pin: u16,
    mode: GpioMode,
}
impl Pin {
    pub fn new(pin: u16, mode: GpioMode) -> Self {
        Self { pin, mode }
    }
    pub fn set(&self, value: bool) {
    }
    pub fn get(&self) -> bool {
        // TODO
        false
    }
}
