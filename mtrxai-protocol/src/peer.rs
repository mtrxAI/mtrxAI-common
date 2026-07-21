use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeerInfo {
    pub lat: f64,
    pub lon: f64,
    pub asn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpuDeviceInfo {
    pub index: u8,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pci_bus_id: Option<String>,
    pub utilization_pct: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_utilization_pct: Option<u8>,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature_c: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_draw_w: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_limit_w: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fan_speed_pct: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpuHostStatus {
    pub available: bool,
    pub utilization_pct: u8,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_free_mb: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cuda_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_count: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature_c: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_utilization_pct: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_draw_w: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampled_at_unix: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<GpuDeviceInfo>>,
}
