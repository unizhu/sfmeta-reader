use serde::{Deserialize, Serialize};

use super::super::is_false;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "is_false", default)]
    pub required: bool,

    #[serde(skip_serializing_if = "is_false", default)]
    pub unique: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationRule {
    pub name: String,

    #[serde(skip_serializing_if = "is_false", default)]
    pub active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub name: String,
    #[serde(skip_serializing_if = "is_false", default)]
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordType {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "is_false", default)]
    pub active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_process: Option<String>,
}
