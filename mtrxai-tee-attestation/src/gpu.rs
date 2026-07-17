use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeeTrustLevel {
    /// NVIDIA H100+ CC with verified attestation.
    TeeGpu,
    /// Application E2EE with process split; host admin can still attack sidecar.
    Host,
    /// Transport encryption only (legacy).
    Transport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpuAttestationEvidence {
    pub gpu_model: String,
    pub cc_mode_enabled: bool,
    pub driver_version: String,
    pub attestation_token: String,
    pub issued_at_unix: u64,
    pub expires_at_unix: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_tee_quote: Option<String>,
}

pub struct GpuAttestationVerifier {
    pub nras_endpoint: String,
    pub mock_mode: bool,
}

impl Default for GpuAttestationVerifier {
    fn default() -> Self {
        Self {
            nras_endpoint: "https://nras.attestation.nvidia.com/v4/attest/gpu".to_string(),
            mock_mode: std::env::var("MTRXAI_TEE_MOCK")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        }
    }
}

impl GpuAttestationVerifier {
    pub fn verify(&self, evidence: &GpuAttestationEvidence) -> Result<TeeTrustLevel, String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if evidence.expires_at_unix <= now {
            return Err("GPU attestation expired".to_string());
        }
        if !evidence.cc_mode_enabled {
            return Err("GPU confidential computing not enabled".to_string());
        }
        if !is_hopper_or_newer(&evidence.gpu_model) {
            return Err(format!(
                "GPU model {} does not support CC",
                evidence.gpu_model
            ));
        }
        if self.mock_mode {
            return Ok(TeeTrustLevel::TeeGpu);
        }
        if evidence.attestation_token.trim().is_empty() {
            return Err("missing NRAS attestation token".to_string());
        }
        verify_nras_token_stub(&evidence.attestation_token)?;
        Ok(TeeTrustLevel::TeeGpu)
    }

    pub fn build_mock_evidence(gpu_model: &str) -> GpuAttestationEvidence {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        GpuAttestationEvidence {
            gpu_model: gpu_model.to_string(),
            cc_mode_enabled: true,
            driver_version: "575.28".to_string(),
            attestation_token: mock_token(gpu_model),
            issued_at_unix: now,
            expires_at_unix: now + 3600,
            cpu_tee_quote: Some("mock-cpu-tee-quote".to_string()),
        }
    }
}

fn is_hopper_or_newer(gpu_model: &str) -> bool {
    let m = gpu_model.to_ascii_uppercase();
    m.contains("H100")
        || m.contains("H200")
        || m.contains("B100")
        || m.contains("B200")
        || m.contains("GH100")
}

fn mock_token(gpu_model: &str) -> String {
    hex::encode(Sha256::digest(format!("mock-nras-{gpu_model}").as_bytes()))
}

fn verify_nras_token_stub(token: &str) -> Result<(), String> {
    if token.len() < 16 {
        return Err("invalid NRAS token format".to_string());
    }
    Ok(())
}
