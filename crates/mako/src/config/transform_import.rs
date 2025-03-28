use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TransformImportStyle {
    Built(String),
    Source(bool),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransformImportConfig {
    pub library_name: String,
    pub library_directory: Option<String>,
    pub style: Option<TransformImportStyle>,
    #[serde(default = "default_as_true")]
    pub camel2_dash_component_name: bool,
}

fn default_as_true() -> bool {
    true
}
