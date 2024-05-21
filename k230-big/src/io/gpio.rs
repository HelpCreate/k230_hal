use k230_sys_big::gpio::{self, kd_pin_mode, kd_pin_write};


#[derive(Debug, Clone, Copy)]
pub enum GpioMode {
    Input = gpio::_gpio_drive_mode_GPIO_DM_INPUT as isize,
    Output = gpio::_gpio_drive_mode_GPIO_DM_OUTPUT as isize,

}

#[derive(Debug, Clone, Copy)]
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
pub enum GpioState { 
    LOW,
    HIGH,
    
}

pub struct Gpio {
    pin: Pin,
    mode: GpioMode,
}
impl Gpio {
    pub fn new(pin: Pin, mode: GpioMode) -> Self {
        unsafe { kd_pin_mode(pin as i32, mode as i32) }
        Self { pin, mode }
    }
    pub fn set(&self, value: GpioState) { 
        unsafe { kd_pin_write(self.pin as i32, value as i32) }

    
    }
    pub fn get(&self) -> bool {
        // TODO
        false
    }
}
