use embedded_hal::i2c::SevenBitAddress;
use embedded_hal::i2c::I2c;


use embedded_hal::i2c::ErrorType as I2CErrorType;

use proposed_traits::i3c_master::I3c;
use proposed_traits::i3c_master::I3cSpeed;
use proposed_traits::i3c_master::ErrorType as I3CErrorType;

// Dummy error type for demonstration
#[derive(Debug)]
pub struct DummyI3cError;


// Implement embedded_hal::i2c::Error for DummyI3cError
impl embedded_hal::i2c::Error for DummyI3cError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

impl proposed_traits::i3c_master::Error for DummyI3cError {
    fn kind(&self) -> proposed_traits::i3c_master::ErrorKind {
        proposed_traits::i3c_master::ErrorKind::Other
    }
}

impl core::fmt::Display for DummyI3cError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Dummy I3C error")
    }
}


// Dummy implementation
pub struct DummyI3cController;

impl I3CErrorType for DummyI3cController {
    type Error = DummyI3cError;
}

impl I2CErrorType for DummyI3cController {
    type Error = DummyI3cError;
}


impl I2c for DummyI3cController {
    fn read(&mut self, _address: SevenBitAddress, _buffer: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(&mut self, _address: SevenBitAddress, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    
    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}


impl I3c for DummyI3cController {
    fn assign_dynamic_address(&mut self, static_address: SevenBitAddress) -> Result<SevenBitAddress, Self::Error> {
        // Just return a fixed dynamic address for testing
        Ok(static_address + 1)
    }

    fn acknowledge_ibi(&mut self, _address: SevenBitAddress) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_hot_join(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_bus_speed(&mut self, _speed: I3cSpeed) -> Result<(), Self::Error> {
        Ok(())
    }

    fn request_mastership(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
