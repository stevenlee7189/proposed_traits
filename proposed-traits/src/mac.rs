use crate::serde::Serde;

/// Error kind.
///
/// This represents a common set of digest operation errors. Implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The input data length is not valid for the hash function.
    InvalidInputLength,

    /// The specified hash algorithm is not supported by the hardware or software implementation.
    UnsupportedAlgorithm,

    /// Failed to allocate memory for the hash computation.
    MemoryAllocationFailure,

    /// Failed to initialize the hash computation context.
    InitializationError,

    /// Error occurred while updating the hash computation with new data.
    UpdateError,

    /// Error occurred while finalizing the hash computation.
    FinalizationError,

    /// The hardware accelerator is busy and cannot process the hash computation.
    HardwareAcceleratorBusy,

    /// General hardware failure during hash computation.
    HardwareFailure,

    /// The specified output size is not valid for the hash function.
    InvalidOutputSize,

    /// Insufficient permissions to access the hardware or perform the hash computation.
    PermissionDenied,

    /// The hash computation context has not been initialized.
    NotInitialized,
}

pub trait Error: core::fmt::Debug {
    /// Convert error to a generic error kind
    ///
    /// By using this method, errors freely defined by Algo implementations
    /// can be converted to a set of generic errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    /// Convert error to a generic Mac error kind.
    ///
    /// By using this method, Mac errors freely defined by Algo implementations
    /// can be converted to a set of generic I2C errors upon which generic
    /// code can act.    
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

pub trait ErrorType {
    /// Error type.
    type Error: Error;
}

/// Message Authentication algorithm
pub trait Mac: ErrorType {
    type InitParams<'a> : where Self: 'a;

    type Key<'a>: Serde where Self: 'a;
    /// Init instance of the crypto function with the given context.
    ///
    /// # Parameters
    ///
    /// - `init_params`: The context or configuration parameters for the crypto function.
    ///
    /// # Returns
    ///
    /// A new instance of the hash function.    
    fn init(init_params: Self::InitParams<'_>) -> Result<(), Self::Error>;

    /// Sets the key for the HMAC algorithm.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to be used for HMAC.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. On success, returns `Ok(())`. On failure, returns an error of type `Self::Error`.
    fn set_key(&mut self, key: Self::Key<'_>) -> Result<(), Self::Error>;

    /// Update state using provided input data.
    ///
    /// # Parameters
    ///
    /// - `input`: The input data to be hashed. This can be any type that implements `AsRef<[u8]>`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. On success, returns `Ok(())`. On failure, returns a `CryptoError`.    
    fn update(&mut self, input: &[u8]) -> Result<(), Self::Error>;

    /// Reset instance to its initial state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. On success, returns `Ok(())`. On failure, returns a `CryptoError`.    
    fn reset(&mut self) -> Result<(), Self::Error>;

    /// Finalize the computation and produce the output.
    ///
    /// # Parameters
    ///
    /// - `out`: A mutable slice to store the hash output. The length of the slice must be at least `MAX_OUTPUT_SIZE`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. On success, returns `Ok(())`. On failure, returns a `CryptoError`.    
    fn finalize(&mut self, out: &mut [u8]) -> Result<(), Self::Error>;

    /// Verifies if the given MAC tag matches the expected result.
    ///
    /// # Parameters
    ///
    /// - `tag`: The MAC tag to be verified.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. On success, returns `Ok(())`. On failure, returns an error of type `Self::Error`.    
    fn verify(&mut self, tag: &[u8]) -> Result<(), Self::Error>;
}
