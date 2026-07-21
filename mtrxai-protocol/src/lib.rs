//! Shared lobby wire protocol types and ranking helpers.

mod peer;
mod protocol;
mod ranking;
mod presence;

pub use peer::{GpuDeviceInfo, GpuHostStatus, PeerInfo};
pub use presence::{PresencePeer, PresencePeersResponse};
pub use protocol::ProtocolMessage;
pub use ranking::{asn_key, haversine_km, peer_load_score};
