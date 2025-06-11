// === MacAlgorithm Trait ===

use std::marker::PhantomData;


use hmac::{Hmac, Mac};

use proposed_traits::mac::{MacOp, Error, MacInit, MacAlgorithm, ErrorType, ErrorKind};
use sha2::Sha256;

pub struct HmacSha256;

impl MacAlgorithm for HmacSha256 {
    const OUTPUT_BITS: usize = 256;
    type MacOutput = [u8; 32];
    type Key = [u8; 32];
}

// === Custom Error Type ===

#[derive(Debug)]
pub enum MacError {
    Init,
    Update,
    Finalize,
}

impl Error for MacError {
    fn kind(&self) -> ErrorKind {
        match self {
            MacError::Init => ErrorKind::InitializationError,
            MacError::Update => ErrorKind::UpdateError,
            MacError::Finalize => ErrorKind::FinalizationError,
        }
    }
}

// === MacInit Implementation ===

pub struct HmacEngine;

impl ErrorType for HmacEngine {
    type Error = MacError;
}

impl MacInit<HmacSha256> for HmacEngine {
    type OpContext<'a> = HmacContext<'a> where Self: 'a;

    fn init<'a>(
        &'a mut self,
        _algo: HmacSha256,
        key: &<HmacSha256 as MacAlgorithm>::Key,
    ) -> Result<Self::OpContext<'a>, Self::Error> {
        let mac = Hmac::<Sha256>::new_from_slice(key).map_err(|_| MacError::Init)?;
        Ok(HmacContext { mac, _marker: PhantomData })
    }
}

// === MacOp Implementation ===

pub struct HmacContext<'a> {
    _marker: PhantomData<&'a ()>,
    mac: Hmac<Sha256>,
}

impl<'a> ErrorType for HmacContext<'a> {
    type Error = MacError;
}

impl<'a> MacOp for HmacContext<'a> {
    type Output = [u8; 32];

    fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
        Mac::update(&mut self.mac, input);
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        let result = self.mac.finalize().into_bytes();
        Ok(result.into())
    }
}
