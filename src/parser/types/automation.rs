use serde::{Deserialize, Serialize};

use super::super::is_false;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flow {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_type: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApexClass {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApexTrigger {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub active: bool,
}
