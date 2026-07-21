use crate::peer::GpuHostStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProtocolMessage {
    Registered {
        name: String,
        #[serde(rename = "cluster_id", alias = "room_id")]
        cluster_id: Uuid,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "cluster_name",
            alias = "room_name"
        )]
        cluster_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        required_attestation_flags: Option<u64>,
    },
    UpdatePeerInfo {
        lat: f64,
        lon: f64,
        asn: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        country: Option<String>,
    },
    UpdateModels {
        models: Vec<Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gpu_host: Option<GpuHostStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        accepting_jobs: Option<bool>,
    },
    AvailableModels { models: Vec<Value> },
    GetPeersForModel { req_id: String, model: String },
    PeersForModel {
        req_id: String,
        model: String,
        peers: Vec<String>,
    },
    RequestPeerConnect {
        req_id: String,
        model: String,
    },
    ConnectOffer {
        req_id: String,
        model: String,
        requested_by: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requested_by_attestation_flags: Option<u64>,
    },
    RespondConnectOffer {
        req_id: String,
        accept: bool,
    },
    ConnectUpdate {
        req_id: String,
        model: String,
        status: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        provider_peer: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        provider_attestation_flags: Option<u64>,
    },
    Route {
        to: String,
        from: String,
        payload: Value,
    },
    ReportTokenUsage {
        req_id: String,
        role: String,
        peer_id: String,
        remote_peer_id: String,
        model: String,
        path: String,
        prompt_tokens: u32,
        completion_tokens: u32,
        total_tokens: u32,
        bytes_sent: u64,
        bytes_received: u64,
        duration_ms: u64,
    },
    RequestModelStart {
        req_id: String,
        model: String,
        #[serde(rename = "cluster_id", alias = "room_id")]
        cluster_id: Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gpu_host: Option<GpuHostStatus>,
    },
    ModelStartUpdate {
        req_id: String,
        model: String,
        status: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        progress_pct: Option<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        provider_peer: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        peers: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
    ModelStartOffer {
        req_id: String,
        model: String,
        requested_by: String,
        estimated_vram_mb: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        disk_size_mb: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        run_on_requester: Option<bool>,
    },
    RespondModelStart {
        req_id: String,
        accept: bool,
    },
    ReportModelStartProgress {
        req_id: String,
        status: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        progress_pct: Option<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
    ReportPeer {
        target_peer_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
    PeerReportAck {
        target_peer_id: String,
        report_count: u32,
        banned: bool,
    },
    PeerBanned {
        peer_id: String,
        report_count: u32,
    },
}
