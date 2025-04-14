use tock::gpio::Gpio;
use embedded_hal::digital::{InputPin, OutputPin};

pub struct TockGpioAdapter {
    gpio: Gpio,
}

impl TockGpioAdapter {
    pub fn new(gpio: Gpio) -> Self {
        TockGpioAdapter { gpio }
    }
}

impl InputPin for TockGpioAdapter {
    type Error = ();

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.gpio.read() == 1)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.gpio.read() == 0)
    }
}

impl OutputPin for TockGpioAdapter {
    type Error = ();

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.gpio.set()
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.gpio.clear()
    }
}

// Example usage
fn main() {
    let gpio = Gpio::new(); // Assume Gpio::new() initializes a GPIO pin in Tock
    let tock_gpio_adapter = TockGpioAdapter::new(gpio);
    let mut driver = MyGpioDriver::new(tock_gpio_adapter);

    driver.set_high();
    println!("GPIO is high: {}", driver.is_high());
    driver.set_low();
    println!("GPIO is low: {}", driver.is_low());
}
