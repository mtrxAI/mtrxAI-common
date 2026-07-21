//! Shared lobby wire protocol types and ranking helpers.

mod peer;
mod presence;
mod protocol;
mod ranking;

pub use peer::{GpuDeviceInfo, GpuHostStatus, PeerInfo};
pub use presence::{PresencePeer, PresencePeersResponse};
pub use protocol::ProtocolMessage;
pub use ranking::{asn_key, haversine_km, peer_load_score};
