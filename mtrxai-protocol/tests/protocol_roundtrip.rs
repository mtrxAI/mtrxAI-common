use mtrxai_protocol::{IceServerConfig, ProtocolMessage};
use uuid::Uuid;

#[test]
fn registered_cluster_id_uuid_roundtrip() {
    let cluster_id = Uuid::parse_str("00000000-0000-0000-0000-0000000000aa").unwrap();
    let msg = ProtocolMessage::Registered {
        name: "peer-1".into(),
        cluster_id,
        cluster_name: Some("lab".into()),
        required_attestation_flags: Some(1),
        ice_servers: None,
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("cluster_id"));
    let back: ProtocolMessage = serde_json::from_str(&json).unwrap();
    match back {
        ProtocolMessage::Registered {
            cluster_id: id,
            name,
            ..
        } => {
            assert_eq!(id, cluster_id);
            assert_eq!(name, "peer-1");
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn registered_ice_servers_roundtrip() {
    let cluster_id = Uuid::parse_str("00000000-0000-0000-0000-0000000000aa").unwrap();
    let ice = vec![
        IceServerConfig {
            urls: vec!["stun:stun.l.google.com:19302".into()],
            username: None,
            credential: None,
        },
        IceServerConfig {
            urls: vec!["turn:turn.example.com:3478?transport=udp".into()],
            username: Some("12345:peer-1".into()),
            credential: Some("secret".into()),
        },
    ];
    let msg = ProtocolMessage::Registered {
        name: "peer-1".into(),
        cluster_id,
        cluster_name: None,
        required_attestation_flags: None,
        ice_servers: Some(ice.clone()),
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("ice_servers"));
    assert!(json.contains("turn:turn.example.com"));
    let back: ProtocolMessage = serde_json::from_str(&json).unwrap();
    match back {
        ProtocolMessage::Registered { ice_servers, .. } => {
            assert_eq!(ice_servers, Some(ice));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn room_id_alias_deserializes() {
    let raw =
        r#"{"type":"registered","name":"p","room_id":"00000000-0000-0000-0000-0000000000bb"}"#;
    let msg: ProtocolMessage = serde_json::from_str(raw).unwrap();
    match msg {
        ProtocolMessage::Registered { cluster_id, .. } => {
            assert_eq!(
                cluster_id,
                Uuid::parse_str("00000000-0000-0000-0000-0000000000bb").unwrap()
            );
        }
        other => panic!("unexpected: {other:?}"),
    }
}
