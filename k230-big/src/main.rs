
use k230_big::io::gpio::{Gpio, Pin, GpioMode, GpioState};
fn main() { 
    let pin9 = Gpio::new(Pin::Pin9, GpioMode::Output);
    pin9.set(GpioState::HIGH);

    println!("pin 9 set");
    
}
