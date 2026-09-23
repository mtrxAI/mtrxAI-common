//! Shared lobby wire protocol types and ranking helpers.

mod peer;
mod presence;
mod protocol;
mod ranking;
mod webrtc;

pub use peer::{GpuDeviceInfo, GpuHostStatus, PeerInfo};
pub use presence::{PresencePeer, PresencePeersResponse};
pub use protocol::ProtocolMessage;
pub use ranking::{asn_key, haversine_km, peer_load_score};
pub use webrtc::IceServerConfig;
