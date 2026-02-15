pub mod format;
pub mod parse;
pub mod types;

use serde::{Deserialize, Serialize};

pub use format::{to_compact_format, to_toon_format};
pub use parse::parse_salesforce_xml;
pub use types::*;

/// Root metadata container aggregating all parsed Salesforce metadata types.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SalesforceMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // Object metadata
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub fields: Vec<Field>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub validation_rules: Vec<ValidationRule>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub workflows: Vec<Workflow>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub record_types: Vec<RecordType>,

    // Security metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set: Option<PermissionSet>,

    // Automation metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
}

/// Helper function for serde to skip false booleans.
pub(crate) fn is_false(value: &bool) -> bool {
    !*value
}
