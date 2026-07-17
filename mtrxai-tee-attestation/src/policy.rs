use crate::gpu::{GpuAttestationEvidence, GpuAttestationVerifier, TeeTrustLevel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeePolicy {
    pub require_tee: bool,
    pub min_gpu_arch: Option<String>,
    pub min_driver_version: Option<String>,
}

impl Default for TeePolicy {
    fn default() -> Self {
        Self {
            require_tee: false,
            min_gpu_arch: Some("H100".to_string()),
            min_driver_version: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeeRankingDecision {
    pub allowed: bool,
    pub trust_level: TeeTrustLevel,
    pub reason: Option<String>,
}

pub fn evaluate_peer_for_policy(
    policy: &TeePolicy,
    evidence: Option<&GpuAttestationEvidence>,
    sidecar_only: bool,
) -> TeeRankingDecision {
    let verifier = GpuAttestationVerifier::default();
    if policy.require_tee {
        let Some(evidence) = evidence else {
            return TeeRankingDecision {
                allowed: false,
                trust_level: TeeTrustLevel::Transport,
                reason: Some("TEE required but no GPU attestation".to_string()),
            };
        };
        match verifier.verify(evidence) {
            Ok(level) => TeeRankingDecision {
                allowed: true,
                trust_level: level,
                reason: None,
            },
            Err(e) => TeeRankingDecision {
                allowed: false,
                trust_level: TeeTrustLevel::Transport,
                reason: Some(e),
            },
        }
    } else if sidecar_only {
        TeeRankingDecision {
            allowed: true,
            trust_level: TeeTrustLevel::Host,
            reason: None,
        }
    } else {
        TeeRankingDecision {
            allowed: true,
            trust_level: TeeTrustLevel::Transport,
            reason: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tee_required_rejects_missing_evidence() {
        let policy = TeePolicy {
            require_tee: true,
            ..Default::default()
        };
        let d = evaluate_peer_for_policy(&policy, None, false);
        assert!(!d.allowed);
    }

    #[test]
    fn mock_tee_accepts_h100() {
        std::env::set_var("MTRXAI_TEE_MOCK", "1");
        let policy = TeePolicy {
            require_tee: true,
            ..Default::default()
        };
        let evidence = GpuAttestationVerifier::build_mock_evidence("NVIDIA H100 80GB");
        let d = evaluate_peer_for_policy(&policy, Some(&evidence), false);
        assert!(d.allowed);
        assert_eq!(d.trust_level, TeeTrustLevel::TeeGpu);
        std::env::remove_var("MTRXAI_TEE_MOCK");
    }
}
