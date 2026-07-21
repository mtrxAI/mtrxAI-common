use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PresencePeer {
    pub peer_id: String,
    pub listen_addrs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PresencePeersResponse {
    pub peers: Vec<PresencePeer>,
}
