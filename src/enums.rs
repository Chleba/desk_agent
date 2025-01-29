#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OllamaTagsResult {
    pub models: Vec<OllamaModel>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OllamaModelDetail {
    pub parent_model: String,
    pub format: String,
    pub family: String,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OllamaModel {
    pub name: String,
    pub model: String,
    pub size: u64,
    pub details: OllamaModelDetail,
}

#[derive(Clone, Debug)]
pub enum BroadcastMsg {
    SetOllamaURL(String),
    SetOllamaModels(Vec<OllamaModel>),
    
    GetOllamaURL,
    OllamaURL(String),

    GetOllamaModels,
    OllamaModels(Vec<OllamaModel>)
}
