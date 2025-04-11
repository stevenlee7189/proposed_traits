use core::fmt::Debug;

pub trait Error: core::fmt::Debug {
    /// Convert error to a generic error kind
    ///
    /// By using this method, errors freely defined by HAL implementations
    /// can be converted to a set of generic errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

pub trait ErrorType {
    /// Error type.
    type Error: Error;
}

/// Error kind.
///
/// This represents a common set of digest operation errors. Implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    Busy,
    InvalidSignature,
    KeyGenError,
    SigningError,
    Other,
}

pub trait HashMarker {
    fn size() -> usize;
}

pub trait EcdsaCurve {
    fn id() -> u32;
}

pub trait EcdsaTypes {
    type PrivateKey;
    type PublicKey;
    type Signature;
    type Curve: EcdsaCurve;
}


/// Trait for ECDSA key generation.
///
/// This trait defines the methods required for generating ECDSA key pairs.
pub trait EcdsaKeyGen: ErrorType + EcdsaTypes {

    /// Generates an ECDSA key pair.
    ///
    /// # Parameters
    /// - `curve`: The elliptic curve to use for key generation.
    ///
    /// # Returns
    /// A result containing the generated private and public keys, or an error.    
    fn generate_key_pair(
        curve: &Self::Curve,
    ) -> Result<(Self::PrivateKey, Self::PublicKey), Self::Error>;
}

/// Trait for ECDSA signing.
///
/// This trait defines the methods required for signing messages using ECDSA.
pub trait EcdsaSign: ErrorType {
    type PrivateKey;
    type Curve: EcdsaCurve;
    type Signature;

    /// Signs a message hash using the private key and elliptic curve.
    ///
    /// # Parameters
    /// - `curve`: The elliptic curve to use for signing.
    /// - `private_key`: The private key to use for signing.
    /// - `message_hash`: The hash of the message to sign.
    ///
    /// # Returns
    /// A result containing the generated signature, or an error.    
    fn sign<H: HashMarker>(
        curve: &Self::Curve,
        private_key: &Self::PrivateKey,
        message_hash: impl AsRef<[u8]>,
    ) -> Result<Self::Signature, Self::Error>;
}

/// Trait for ECDSA verification.
///
/// This trait defines the methods required for verifying ECDSA signatures.
pub trait EcdsaVerify: ErrorType {
    type PublicKey;
    type Curve: EcdsaCurve;
    type Signature;

    /// Verifies an ECDSA signature.
    ///
    /// # Parameters
    /// - `curve`: The elliptic curve to use for verification.
    /// - `public_key`: The public key to use for verification.
    /// - `message_hash`: The hash of the message to verify.
    /// - `signature`: The signature to verify.
    ///
    /// # Returns
    /// A result indicating whether the signature is valid, or an error.    
    fn verify<H: HashMarker>(
        curve: &Self::Curve,
        public_key: &Self::PublicKey,
        message_hash: impl AsRef<[u8]>,
        signature: &Self::Signature,
    ) -> Result<(), Self::Error>;
}

