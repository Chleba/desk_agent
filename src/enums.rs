use crate::ollama_state::OllamaState;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OllamaTagsResult {
    pub models: Vec<OllamaModel>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct OllamaModelDetail {
    parent_model: String,
    format: String,
    family: String,
    parameter_size: String,
    quantization_level: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OllamaModel {
    name: String,
    model: String,
    size: u64,
    details: OllamaModelDetail,
}

#[derive(Clone, Debug)]
pub enum BroadcastMsg {
    OllamaMsg(OllamaState),

    SetOllamaURL(String),
}
