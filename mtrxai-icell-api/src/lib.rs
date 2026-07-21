//! Inference-cell admin API contract (`/mtrxai/v1/*`).

use serde::{Deserialize, Serialize};

pub const ADMIN_API_PREFIX: &str = "/mtrxai/v1";
pub const INFO_PATH: &str = "/mtrxai/v1/info";
pub const MODELS_PULL_PATH: &str = "/mtrxai/v1/models/pull";
pub const MODELS_LOAD_PATH: &str = "/mtrxai/v1/models/load";
pub const MODELS_UNLOAD_PATH: &str = "/mtrxai/v1/models/unload";
pub const MODELS_DELETE_PATH: &str = "/mtrxai/v1/models/delete";

pub mod paths {
    pub fn pull_job(job_id: &str) -> String {
        format!("{}/{}", super::MODELS_PULL_PATH, job_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EngineKind {
    Ollama,
    Llamacpp,
}

impl EngineKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::Llamacpp => "llamacpp",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "ollama" => Some(Self::Ollama),
            "llamacpp" | "llama.cpp" | "llama_cpp" => Some(Self::Llamacpp),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellInfo {
    pub engine: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_api: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_api: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PullRequest {
    pub repo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PullAccepted {
    pub job_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Importing,
    Ok,
    Failed,
}

impl JobStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Importing => "importing",
            Self::Ok => "ok",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PullJobRecord {
    pub id: String,
    pub status: JobStatus,
    pub repo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub started_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelActionRequest {
    pub model_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_parse() {
        assert_eq!(EngineKind::parse("Ollama"), Some(EngineKind::Ollama));
        assert_eq!(EngineKind::parse("llama.cpp"), Some(EngineKind::Llamacpp));
    }
}
