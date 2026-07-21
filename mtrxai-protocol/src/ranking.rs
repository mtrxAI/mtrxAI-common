use crate::peer::GpuHostStatus;
use serde_json::Value;

pub fn asn_key(asn: &str) -> String {
    asn.split_whitespace()
        .next()
        .unwrap_or(asn)
        .to_ascii_uppercase()
}

pub fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    EARTH_RADIUS_KM * c
}

pub fn peer_load_score(model_entry: &Value, gpu_host: &Option<GpuHostStatus>) -> f64 {
    let mut score = 0.0;
    if let Some(status) = model_entry.get("_status") {
        let loaded = status
            .get("loaded")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if loaded {
            let cpu = status.get("cpu_pct").and_then(|v| v.as_u64()).unwrap_or(0) as f64;
            let gpu = status.get("gpu_pct").and_then(|v| v.as_u64()).unwrap_or(0) as f64;
            score += cpu.max(gpu);
        } else {
            score += 25.0;
        }
    }
    if let Some(gpu) = gpu_host {
        if gpu.available {
            score += f64::from(gpu.utilization_pct);
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn asn_key_takes_first_token() {
        assert_eq!(asn_key("AS123 Example ISP"), "AS123");
    }

    #[test]
    fn haversine_same_point_is_zero() {
        assert!((haversine_km(45.0, 9.0, 45.0, 9.0)).abs() < 1e-6);
    }

    #[test]
    fn load_score_unloaded_baseline() {
        let entry = json!({ "_status": { "loaded": false } });
        assert!((peer_load_score(&entry, &None) - 25.0).abs() < 1e-6);
    }
}
