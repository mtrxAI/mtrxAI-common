use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Default allowed clock skew for peer auth timestamps (±5 minutes).
pub const PEER_AUTH_MAX_SKEW_SECS: i64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeerAuthProof {
    pub peer_id: Uuid,
    pub timestamp: i64,
    pub signature: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum PeerAuthError {
    #[error("invalid public key: {0}")]
    InvalidPublicKey(String),
    #[error("invalid signing key: {0}")]
    InvalidSigningKey(String),
    #[error("invalid signature: {0}")]
    InvalidSignature(String),
    #[error("signature verification failed")]
    VerificationFailed,
    #[error("timestamp expired or in the future")]
    TimestampInvalid,
    #[error("peer_id mismatch")]
    PeerIdMismatch,
}

pub fn canonical_peer_auth_bytes(peer_id: &Uuid, timestamp: i64) -> Vec<u8> {
    format!("{peer_id}|{timestamp}").into_bytes()
}

pub fn signing_key_from_seed_hex(seed_hex: &str) -> Result<SigningKey, PeerAuthError> {
    let bytes = hex::decode(seed_hex.trim())
        .map_err(|e| PeerAuthError::InvalidSigningKey(e.to_string()))?;
    let seed: [u8; 32] = bytes
        .try_into()
        .map_err(|_| PeerAuthError::InvalidSigningKey("expected 32-byte seed".into()))?;
    Ok(SigningKey::from_bytes(&seed))
}

pub fn verifying_key_from_hex(public_key_hex: &str) -> Result<VerifyingKey, PeerAuthError> {
    let bytes = hex::decode(public_key_hex.trim())
        .map_err(|e| PeerAuthError::InvalidPublicKey(e.to_string()))?;
    let key_bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| PeerAuthError::InvalidPublicKey("expected 32-byte key".into()))?;
    VerifyingKey::from_bytes(&key_bytes)
        .map_err(|e| PeerAuthError::InvalidPublicKey(e.to_string()))
}

pub fn public_key_hex_from_signing_key(signing_key: &SigningKey) -> String {
    hex::encode(signing_key.verifying_key().to_bytes())
}

pub fn sign_peer_auth(
    signing_key: &SigningKey,
    peer_id: Uuid,
    timestamp: i64,
) -> PeerAuthProof {
    let message = canonical_peer_auth_bytes(&peer_id, timestamp);
    let sig = signing_key.sign(&message);
    PeerAuthProof {
        peer_id,
        timestamp,
        signature: hex::encode(sig.to_bytes()),
    }
}

pub fn verify_peer_auth_signature(
    proof: &PeerAuthProof,
    public_key_hex: &str,
) -> Result<(), PeerAuthError> {
    let verifying_key = verifying_key_from_hex(public_key_hex)?;
    let sig_bytes = hex::decode(proof.signature.trim())
        .map_err(|e| PeerAuthError::InvalidSignature(e.to_string()))?;
    let sig_array: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| PeerAuthError::InvalidSignature("expected 64-byte signature".into()))?;
    let signature = Signature::from_bytes(&sig_array);
    verifying_key
        .verify(
            &canonical_peer_auth_bytes(&proof.peer_id, proof.timestamp),
            &signature,
        )
        .map_err(|_| PeerAuthError::VerificationFailed)
}

pub fn verify_peer_auth(
    proof: &PeerAuthProof,
    expected_peer_id: &Uuid,
    public_key_hex: &str,
    now_unix: i64,
    max_skew_secs: i64,
) -> Result<(), PeerAuthError> {
    if proof.peer_id != *expected_peer_id {
        return Err(PeerAuthError::PeerIdMismatch);
    }
    let delta = (now_unix - proof.timestamp).abs();
    if delta > max_skew_secs {
        return Err(PeerAuthError::TimestampInvalid);
    }
    verify_peer_auth_signature(proof, public_key_hex)
}

pub fn generate_peer_keypair_hex() -> (String, String) {
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
        let (seed_hex, pub_hex) = generate_peer_keypair_hex();
        let signing_key = signing_key_from_seed_hex(&seed_hex).unwrap();
        let peer_id = Uuid::new_v4();
        let timestamp = 1_700_000_000_i64;
        let proof = sign_peer_auth(&signing_key, peer_id, timestamp);
        verify_peer_auth(&proof, &peer_id, &pub_hex, timestamp, PEER_AUTH_MAX_SKEW_SECS).unwrap();
    }

    #[test]
    fn rejects_stale_timestamp() {
        let (seed_hex, pub_hex) = generate_peer_keypair_hex();
        let signing_key = signing_key_from_seed_hex(&seed_hex).unwrap();
        let peer_id = Uuid::new_v4();
        let timestamp = 1_700_000_000_i64;
        let proof = sign_peer_auth(&signing_key, peer_id, timestamp);
        let err = verify_peer_auth(
            &proof,
            &peer_id,
            &pub_hex,
            timestamp + PEER_AUTH_MAX_SKEW_SECS + 1,
            PEER_AUTH_MAX_SKEW_SECS,
        )
        .unwrap_err();
        assert_eq!(err, PeerAuthError::TimestampInvalid);
    }
}
