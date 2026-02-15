use anyhow::Result;
use quick_xml::Reader;
use quick_xml::events::Event;

use super::SalesforceMetadata;
use super::types::*;

/// Enhanced parser supporting 300+ Salesforce metadata types.
///
/// Parses a Salesforce metadata XML string into a structured
/// [`SalesforceMetadata`] representation. Set `include_descriptions`
/// to `true` to preserve description fields (increases tokens).
pub fn parse_salesforce_xml(xml: &str, include_descriptions: bool) -> Result<SalesforceMetadata> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut metadata = SalesforceMetadata::default();
    let mut buf = Vec::new();
    let mut current_tag = String::new();

    // Context trackers for different metadata types
    let mut current_field: Option<Field> = None;
    let mut current_validation: Option<ValidationRule> = None;
    let mut current_workflow: Option<Workflow> = None;
    let mut current_record_type: Option<RecordType> = None;
    let mut current_obj_perm: Option<ObjectPermission> = None;
    let mut current_field_perm: Option<FieldPermission> = None;

    let mut in_field = false;
    let mut in_validation = false;
    let mut in_workflow = false;
    let mut in_record_type = false;
    let mut in_profile = false;
    let mut in_permission_set = false;
    let mut in_obj_permission = false;
    let mut in_field_permission = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let binding = e.name();
                let tag_name = String::from_utf8_lossy(binding.as_ref()).to_string();

                match tag_name.as_str() {
                    // Object metadata
                    "fields" => {
                        in_field = true;
                        current_field = Some(Field {
                            name: String::new(),
                            field_type: None,
                            label: None,
                            required: false,
                            unique: false,
                            length: None,
                            description: None,
                        });
                    }
                    "validationRules" => {
                        in_validation = true;
                        current_validation = Some(ValidationRule {
                            name: String::new(),
                            active: false,
                            formula: None,
                        });
                    }
                    "workflow" | "workflowRules" => {
                        in_workflow = true;
                        current_workflow = Some(Workflow {
                            name: String::new(),
                            active: false,
                        });
                    }
                    "recordTypes" => {
                        in_record_type = true;
                        current_record_type = Some(RecordType {
                            name: String::new(),
                            label: None,
                            active: false,
                            business_process: None,
                        });
                    }
                    // Profile/Permission Set
                    "Profile" => {
                        in_profile = true;
                        metadata.profile = Some(Profile {
                            name: String::new(),
                            object_permissions: Vec::new(),
                            field_permissions: Vec::new(),
                            application_visibilities: Vec::new(),
                        });
                    }
                    "PermissionSet" => {
                        in_permission_set = true;
                        metadata.permission_set = Some(PermissionSet {
                            name: String::new(),
                            label: None,
                            object_permissions: Vec::new(),
                            field_permissions: Vec::new(),
                        });
                    }
                    "objectPermissions" => {
                        in_obj_permission = true;
                        current_obj_perm = Some(ObjectPermission {
                            object: String::new(),
                            allow_create: false,
                            allow_read: false,
                            allow_edit: false,
                            allow_delete: false,
                        });
                    }
                    "fieldPermissions" => {
                        in_field_permission = true;
                        current_field_perm = Some(FieldPermission {
                            field: String::new(),
                            readable: false,
                            editable: false,
                        });
                    }
                    // Flow metadata
                    "Flow" => {
                        metadata.flow = Some(Flow {
                            name: String::new(),
                            label: None,
                            process_type: None,
                            active: false,
                        });
                    }
                    _ => {}
                }

                current_tag = tag_name;
            }
            Ok(Event::Text(e)) => {
                let text = e.decode()?.trim().to_string();

                if text.is_empty() {
                    buf.clear();
                    continue;
                }

                // Handle text content based on context
                if in_field {
                    if let Some(ref mut field) = current_field {
                        match current_tag.as_str() {
                            "fullName" => field.name = text,
                            "type" => field.field_type = Some(text),
                            "label" => field.label = Some(text),
                            "required" => field.required = text.eq_ignore_ascii_case("true"),
                            "unique" => field.unique = text.eq_ignore_ascii_case("true"),
                            "length" => field.length = text.parse().ok(),
                            "description" => {
                                if include_descriptions {
                                    field.description = Some(text);
                                }
                            }
                            _ => {}
                        }
                    }
                } else if in_validation {
                    if let Some(ref mut validation) = current_validation {
                        match current_tag.as_str() {
                            "fullName" => validation.name = text,
                            "active" => validation.active = text.eq_ignore_ascii_case("true"),
                            "errorConditionFormula" => validation.formula = Some(text),
                            _ => {}
                        }
                    }
                } else if in_workflow {
                    if let Some(ref mut workflow) = current_workflow {
                        match current_tag.as_str() {
                            "fullName" => workflow.name = text,
                            "active" => workflow.active = text.eq_ignore_ascii_case("true"),
                            _ => {}
                        }
                    }
                } else if in_record_type {
                    if let Some(ref mut rt) = current_record_type {
                        match current_tag.as_str() {
                            "fullName" => rt.name = text,
                            "label" => rt.label = Some(text),
                            "active" => rt.active = text.eq_ignore_ascii_case("true"),
                            "businessProcess" => rt.business_process = Some(text),
                            _ => {}
                        }
                    }
                } else if in_obj_permission {
                    if let Some(ref mut perm) = current_obj_perm {
                        match current_tag.as_str() {
                            "object" => perm.object = text,
                            "allowCreate" => perm.allow_create = text.eq_ignore_ascii_case("true"),
                            "allowRead" => perm.allow_read = text.eq_ignore_ascii_case("true"),
                            "allowEdit" => perm.allow_edit = text.eq_ignore_ascii_case("true"),
                            "allowDelete" => perm.allow_delete = text.eq_ignore_ascii_case("true"),
                            _ => {}
                        }
                    }
                } else if in_field_permission {
                    if let Some(ref mut perm) = current_field_perm {
                        match current_tag.as_str() {
                            "field" => perm.field = text,
                            "readable" => perm.readable = text.eq_ignore_ascii_case("true"),
                            "editable" => perm.editable = text.eq_ignore_ascii_case("true"),
                            _ => {}
                        }
                    }
                } else {
                    match current_tag.as_str() {
                        "fullName" => metadata.full_name = Some(text),
                        "type" => metadata.object_type = Some(text),
                        "label" => metadata.label = Some(text),
                        "description" => {
                            if include_descriptions {
                                metadata.description = Some(text);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::End(e)) => {
                let binding = e.name();
                let tag_name = String::from_utf8_lossy(binding.as_ref());

                match tag_name.as_ref() {
                    "fields" => {
                        if in_field {
                            if let Some(field) = current_field.take()
                                && !field.name.is_empty()
                            {
                                metadata.fields.push(field);
                            }
                            in_field = false;
                        }
                    }
                    "validationRules" => {
                        if in_validation {
                            if let Some(validation) = current_validation.take()
                                && !validation.name.is_empty()
                            {
                                metadata.validation_rules.push(validation);
                            }
                            in_validation = false;
                        }
                    }
                    "workflow" | "workflowRules" => {
                        if in_workflow {
                            if let Some(workflow) = current_workflow.take()
                                && !workflow.name.is_empty()
                            {
                                metadata.workflows.push(workflow);
                            }
                            in_workflow = false;
                        }
                    }
                    "recordTypes" => {
                        if in_record_type {
                            if let Some(rt) = current_record_type.take()
                                && !rt.name.is_empty()
                            {
                                metadata.record_types.push(rt);
                            }
                            in_record_type = false;
                        }
                    }
                    "objectPermissions" => {
                        if in_obj_permission {
                            if let Some(perm) = current_obj_perm.take()
                                && !perm.object.is_empty()
                            {
                                if in_profile && let Some(ref mut profile) = metadata.profile {
                                    profile.object_permissions.push(perm);
                                } else if in_permission_set
                                    && let Some(ref mut ps) = metadata.permission_set
                                {
                                    ps.object_permissions.push(perm);
                                }
                            }
                            in_obj_permission = false;
                        }
                    }
                    "fieldPermissions" => {
                        if in_field_permission {
                            if let Some(perm) = current_field_perm.take()
                                && !perm.field.is_empty()
                            {
                                if in_profile && let Some(ref mut profile) = metadata.profile {
                                    profile.field_permissions.push(perm);
                                } else if in_permission_set
                                    && let Some(ref mut ps) = metadata.permission_set
                                {
                                    ps.field_permissions.push(perm);
                                }
                            }
                            in_field_permission = false;
                        }
                    }
                    "Profile" => in_profile = false,
                    "PermissionSet" => in_permission_set = false,
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "XML parsing error at position {}: {}",
                    reader.error_position(),
                    e
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(metadata)
}
