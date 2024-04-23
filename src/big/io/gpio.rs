
pub enum GpioMode {
    Input,
    Output,

}
pub enum Pin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
}

pub struct Gpio {
    pin: Pin,
    mode: GpioMode,
}
impl Gpio {
    pub fn new(pin: Pin, mode: GpioMode) -> Self {
        Self { pin, mode }
    }
    pub fn set(&self, value: bool) {
    }
    pub fn get(&self) -> bool {
        // TODO
        false
    }
}
