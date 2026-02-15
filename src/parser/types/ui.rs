use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layout {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sections: Vec<LayoutSection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutSection {
    pub label: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tab {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}
