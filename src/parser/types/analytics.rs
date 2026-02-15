use serde::{Deserialize, Serialize};

use super::super::is_false;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub columns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dashboard {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub components: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LightningComponent {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_exposed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailTemplate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub available: bool,
}
