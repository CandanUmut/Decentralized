use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use ipfs::PeerId;
use sha2::{Digest, Sha256};
use bs58; // Add Base58 encoding library
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::signature::{Signer, Verifier};
use rsa::pkcs1v15::SigningKey;
use rsa::signature::SignatureEncoding;
use rsa::pkcs1v15::VerifyingKey;
use rsa::pkcs1v15::Signature;
// Signs a message using the private key and PKCS#1 v1.5 padding

/// Signs a message using PKCS#1 v1.5 padding and SHA-256
pub fn sign_message(private_key: &RsaPrivateKey, message: &[u8]) -> Vec<u8> {
    // Create a signing key with the private RSA key
    let signing_key = SigningKey::<Sha256>::new(private_key.clone());

    // Sign the message
    signing_key.sign(message).to_vec()
}

/// Verifies a signature using PKCS#1 v1.5 padding and SHA-256
pub fn verify_signature(public_key: &RsaPublicKey, message: &[u8], signature: &[u8]) -> bool {
    // Create a verifying key with the public RSA key
    let verifying_key = VerifyingKey::<Sha256>::new(public_key.clone());

    // Attempt to parse the signature into the expected type
    match Signature::try_from(signature) {
        Ok(parsed_signature) => {
            // Verify the signature
            verifying_key.verify(message, &parsed_signature).is_ok()
        }
        Err(_) => false, // Return false if the signature can't be parsed
    }
}

pub fn derive_peer_id(public_key: &str) -> PeerId {
    let hash = Sha256::digest(public_key.as_bytes());
    PeerId::from_bytes(&hash).expect("Invalid PeerID")
}

pub fn derive_wallet_address(public_key: &str) -> String {
    format!("0x{}", hex::encode(Sha256::digest(public_key.as_bytes())))
}



