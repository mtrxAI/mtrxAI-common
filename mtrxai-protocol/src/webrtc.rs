use serde::{Deserialize, Serialize};

/// ICE server entry delivered by the lobby (STUN and/or TURN).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct IceServerConfig {
    pub urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
}
