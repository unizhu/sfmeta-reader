use serde::{Deserialize, Serialize};

use super::super::is_false;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub object_permissions: Vec<ObjectPermission>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub field_permissions: Vec<FieldPermission>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub application_visibilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionSet {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub object_permissions: Vec<ObjectPermission>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub field_permissions: Vec<FieldPermission>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectPermission {
    pub object: String,
    #[serde(skip_serializing_if = "is_false", default)]
    pub allow_create: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub allow_read: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub allow_edit: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub allow_delete: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldPermission {
    pub field: String,
    #[serde(skip_serializing_if = "is_false", default)]
    pub readable: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub editable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharingRule {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
}
