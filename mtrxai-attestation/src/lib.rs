pub mod peer_auth;

/// Official allowlisted mtrxAI client build attestation.
pub const ATTESTATION_MTRXAI_BUILD: u64 = 1 << 0;
/// Reserved for future LLM server attestation.
pub const ATTESTATION_LLM_SERVER: u64 = 1 << 1;
/// Reserved for GPU TEE attestation.
pub const ATTESTATION_TEE: u64 = 1 << 2;

/// Returns true when `peer_flags` contains every bit set in `required_flags`.
pub fn has_required_flags(peer_flags: u64, required_flags: u64) -> bool {
    if required_flags == 0 {
        return true;
    }
    (peer_flags & required_flags) == required_flags
}

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttestationChallenge {
    pub challenge_id: Uuid,
    pub nonce: String,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttestationClaims {
    pub challenge_id: Uuid,
    pub nonce: String,
    pub binary_sha256: String,
    pub build_id: Uuid,
    pub version: String,
    pub git_sha: String,
    pub platform: String,
    pub issued_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttestationProof {
    pub challenge_id: Uuid,
    pub nonce: String,
    pub binary_sha256: String,
    pub build_id: Uuid,
    pub version: String,
    pub git_sha: String,
    pub platform: String,
    pub issued_at: i64,
    pub signature: String,
}

impl AttestationProof {
    pub fn claims(&self) -> AttestationClaims {
        AttestationClaims {
            challenge_id: self.challenge_id,
            nonce: self.nonce.clone(),
            binary_sha256: self.binary_sha256.clone(),
            build_id: self.build_id,
            version: self.version.clone(),
            git_sha: self.git_sha.clone(),
            platform: self.platform.clone(),
            issued_at: self.issued_at,
        }
    }
}

#[derive(Debug, Error)]
pub enum AttestationError {
    #[error("invalid public key: {0}")]
    InvalidPublicKey(String),
    #[error("invalid signing key: {0}")]
    InvalidSigningKey(String),
    #[error("invalid signature: {0}")]
    InvalidSignature(String),
    #[error("signature verification failed")]
    VerificationFailed,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn canonical_claims_bytes(claims: &AttestationClaims) -> Vec<u8> {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        claims.challenge_id,
        claims.nonce,
        claims.binary_sha256,
        claims.build_id,
        claims.version,
        claims.git_sha,
        claims.platform,
        claims.issued_at,
    )
    .into_bytes()
}

pub fn hash_bytes(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    hex::encode(digest)
}

pub fn hash_file(path: &std::path::Path) -> Result<String, AttestationError> {
    let bytes = std::fs::read(path)?;
    Ok(hash_bytes(&bytes))
}

pub fn signing_key_from_seed_hex(seed_hex: &str) -> Result<SigningKey, AttestationError> {
    let bytes = hex::decode(seed_hex.trim())
        .map_err(|e| AttestationError::InvalidSigningKey(e.to_string()))?;
    let seed: [u8; 32] = bytes
        .try_into()
        .map_err(|_| AttestationError::InvalidSigningKey("expected 32-byte seed".into()))?;
    Ok(SigningKey::from_bytes(&seed))
}

pub fn verifying_key_from_hex(public_key_hex: &str) -> Result<VerifyingKey, AttestationError> {
    let bytes = hex::decode(public_key_hex.trim())
        .map_err(|e| AttestationError::InvalidPublicKey(e.to_string()))?;
    let key_bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| AttestationError::InvalidPublicKey("expected 32-byte key".into()))?;
    VerifyingKey::from_bytes(&key_bytes)
        .map_err(|e| AttestationError::InvalidPublicKey(e.to_string()))
}

pub fn sign_claims(
    claims: &AttestationClaims,
    signing_key: &SigningKey,
) -> Result<String, AttestationError> {
    let sig = signing_key.sign(&canonical_claims_bytes(claims));
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        sig.to_bytes(),
    ))
}

pub fn verify_claims_signature(
    claims: &AttestationClaims,
    signature_b64: &str,
    public_key_hex: &str,
) -> Result<(), AttestationError> {
    let verifying_key = verifying_key_from_hex(public_key_hex)?;
    let sig_bytes = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        signature_b64.trim(),
    )
    .map_err(|e| AttestationError::InvalidSignature(e.to_string()))?;
    let sig_array: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| AttestationError::InvalidSignature("expected 64-byte signature".into()))?;
    let signature = Signature::from_bytes(&sig_array);
    verifying_key
        .verify(&canonical_claims_bytes(claims), &signature)
        .map_err(|_| AttestationError::VerificationFailed)
}

pub fn generate_keypair_hex() -> (String, String) {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    (
        hex::encode(signing_key.to_bytes()),
        hex::encode(verifying_key.to_bytes()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_roundtrip() {
        let (seed_hex, pub_hex) = generate_keypair_hex();
        let signing_key = signing_key_from_seed_hex(&seed_hex).unwrap();
        let claims = AttestationClaims {
            challenge_id: Uuid::new_v4(),
            nonce: base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                b"nonce-bytes-123456",
            ),
            binary_sha256: "abc123".into(),
            build_id: Uuid::new_v4(),
            version: "0.1.0".into(),
            git_sha: "deadbeef".into(),
            platform: "linux/x86_64".into(),
            issued_at: 1_700_000_000,
        };
        let signature = sign_claims(&claims, &signing_key).unwrap();
        verify_claims_signature(&claims, &signature, &pub_hex).unwrap();
    }

    #[test]
    fn has_required_flags_checks_bitmask() {
        assert!(has_required_flags(
            ATTESTATION_MTRXAI_BUILD,
            ATTESTATION_MTRXAI_BUILD
        ));
        assert!(!has_required_flags(0, ATTESTATION_MTRXAI_BUILD));
        assert!(has_required_flags(
            ATTESTATION_MTRXAI_BUILD | ATTESTATION_TEE,
            ATTESTATION_MTRXAI_BUILD
        ));
    }
}
