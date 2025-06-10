use p256::{
    ecdsa::{SigningKey, VerifyingKey, Signature},
};
use proposed_traits::digest::DigestAlgorithm;
use proposed_traits::ecdsa::{EcdsaSign, EcdsaVerify, ErrorType};
use p256::ecdsa::signature::hazmat::PrehashVerifier;
 use p256::ecdsa::signature::hazmat::PrehashSigner;

/// Digest algorithm for SHA-256
pub struct Sha2_256;

impl DigestAlgorithm for Sha2_256 {
    const OUTPUT_BITS: usize = 256;
    type DigestOutput = [u8; 32];
}

/// ECDSA signer using P-256 and SHA-256
pub struct P256Signer {
    key: SigningKey,
}

impl ErrorType for P256Signer {
    type Error = core::convert::Infallible;
}

impl EcdsaSign<Sha2_256> for P256Signer {
    type PrivateKey<'a> = SigningKey;
    type Signature = Signature;

    fn sign<R: rand_core::RngCore + rand_core::CryptoRng>(
        &mut self,
        private_key: &Self::PrivateKey<'_>,
        digest: [u8; 32],
        _rng: R,
    ) -> Result<Self::Signature, Self::Error> {
        Ok(private_key.sign_prehash(&digest).unwrap())
    }
}

/// ECDSA verifier using P-256 and SHA-256
pub struct P256Verifier {
    key: VerifyingKey,
}

impl ErrorType for P256Verifier {
    type Error = core::convert::Infallible;
}

impl EcdsaVerify<Sha2_256> for P256Verifier {
    type PublicKey = VerifyingKey;
    type Signature = Signature;

    fn verify(
        &mut self,
        public_key: &Self::PublicKey,
        digest: [u8; 32],
        signature: &Self::Signature,
    ) -> Result<(), Self::Error> {
        public_key.verify_prehash(&digest, signature).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::{
        ecdsa::{SigningKey, VerifyingKey},
        elliptic_curve::rand_core::OsRng,
    };
    use sha2::{Sha256, Digest};

    #[test]
    fn test_ecdsa_sign_and_verify_sha256() {
        // Generate key pair
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        // Message to hash and sign
        let message = b"test message";
        let digest = Sha256::digest(message);

        // Sign the digest
        let mut signer = P256Signer { key: signing_key.clone() };
        let signature = signer
            .sign(&signing_key, digest.into(), OsRng)
            .expect("Signing failed");

        // Verify the signature
        let mut verifier = P256Verifier { key: verifying_key.clone() };
        verifier
            .verify(&verifying_key, digest.into(), &signature)
            .expect("Verification failed");
    }
}
