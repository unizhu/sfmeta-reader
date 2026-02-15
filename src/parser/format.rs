use super::SalesforceMetadata;

/// Convert to TOON format (Token-Oriented Object Notation).
///
/// Produces a human-readable, LLM-friendly representation
/// of the parsed Salesforce metadata.
pub fn to_toon_format(metadata: &SalesforceMetadata) -> String {
    let mut output = String::new();

    if let Some(ref name) = metadata.full_name {
        output.push_str(&format!("object: {}\n", name));
    }

    if let Some(ref obj_type) = metadata.object_type {
        output.push_str(&format!("type: {}\n", obj_type));
    }

    if let Some(ref label) = metadata.label {
        output.push_str(&format!("label: {}\n", label));
    }

    if !metadata.fields.is_empty() {
        output.push_str("\nfields:\n  name type required\n");
        for field in &metadata.fields {
            output.push_str(&format!(
                "  {} {} {}\n",
                field.name,
                field.field_type.as_deref().unwrap_or("-"),
                if field.required { "Y" } else { "N" }
            ));
        }
    }

    if !metadata.validation_rules.is_empty() {
        output.push_str("\nvalidations:\n  name active\n");
        for rule in &metadata.validation_rules {
            output.push_str(&format!(
                "  {} {}\n",
                rule.name,
                if rule.active { "Y" } else { "N" }
            ));
        }
    }

    if !metadata.workflows.is_empty() {
        output.push_str("\nworkflows:\n  name active\n");
        for workflow in &metadata.workflows {
            output.push_str(&format!(
                "  {} {}\n",
                workflow.name,
                if workflow.active { "Y" } else { "N" }
            ));
        }
    }

    if !metadata.record_types.is_empty() {
        output.push_str("\nrecord_types:\n  name active\n");
        for rt in &metadata.record_types {
            output.push_str(&format!(
                "  {} {}\n",
                rt.name,
                if rt.active { "Y" } else { "N" }
            ));
        }
    }

    output
}

/// Convert to compact single-line format.
///
/// Produces a pipe-separated, minimal representation
/// suitable for embedding in constrained contexts.
pub fn to_compact_format(metadata: &SalesforceMetadata) -> String {
    let mut parts = Vec::new();

    if let Some(ref name) = metadata.full_name {
        parts.push(format!("obj:{}", name));
    }

    if let Some(ref obj_type) = metadata.object_type {
        parts.push(format!("type:{}", obj_type));
    }

    for field in &metadata.fields {
        let req = if field.required { "!" } else { "" };
        parts.push(format!(
            "{}{}:{}",
            field.name,
            req,
            field.field_type.as_deref().unwrap_or("?")
        ));
    }

    parts.join(" | ")
}
