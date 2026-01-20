use crate::domain::schema::model::{FieldType, PrimitiveType, Schema};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Payload is not valid JSON")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Field {0} is required but missing")]
    MissingField(String),
    #[error("Field {0} has invalid type")]
    InvalidType(String),
    #[error("Field {0} value {1} is less than min {2}")]
    MinValue(String, f64, f64),
    #[error("Field {0} value {1} is greater than max {2}")]
    MaxValue(String, f64, f64),
    #[error("Field {0} length {1} is less than min {2}")]
    MinLength(String, usize, i32),
    #[error("Field {0} length {1} is greater than max {2}")]
    MaxLength(String, usize, i32),
    #[error("Field {0} does not match regex {1}")]
    Regex(String, String),
}

pub fn validate_event_payload(payload: &[u8], schema: &Schema) -> Result<(), Vec<ValidationError>> {
    let json_val: Value = match serde_json::from_slice(payload) {
        Ok(v) => v,
        Err(e) => return Err(vec![ValidationError::InvalidJson(e)]),
    };

    let mut errors = Vec::new();

    // 1. Iterate Schema Fields
    for (field_name, field_def) in &schema.fields {
        let val = json_val.get(field_name);

        // Required Check
        if val.is_none() || val.unwrap().is_null() {
             if let Some(constraints) = &field_def.constraints {
                 if constraints.required {
                     errors.push(ValidationError::MissingField(field_name.clone()));
                 }
             }
             continue; // If missing and not required, skip further checks
        }
        
        let val = val.unwrap();

        // Type Check
        match &field_def.field_type {
            FieldType::Primitive(p) => {
                match p {
                    PrimitiveType::String => if !val.is_string() { errors.push(ValidationError::InvalidType(field_name.clone())); },
                    PrimitiveType::Number => if !val.is_number() { errors.push(ValidationError::InvalidType(field_name.clone())); },
                    PrimitiveType::Boolean => if !val.is_boolean() { errors.push(ValidationError::InvalidType(field_name.clone())); },
                }
            },
            // Simplified check for complex types for MVP
            _ => {},
        }

        // Constraints Check
        if let Some(constraints) = &field_def.constraints {
            // Min/Max Value
            if let Some(n) = val.as_f64() {
                if let Some(min) = constraints.min_value {
                    if n < min {
                        errors.push(ValidationError::MinValue(field_name.clone(), n, min));
                    }
                }
                if let Some(max) = constraints.max_value {
                    if n > max {
                        errors.push(ValidationError::MaxValue(field_name.clone(), n, max));
                    }
                }
            }

            // Length constraints
            if let Some(s) = val.as_str() {
                if let Some(min) = constraints.min_length {
                    if (s.len() as i32) < min {
                        errors.push(ValidationError::MinLength(field_name.clone(), s.len(), min));
                    }
                }
                if let Some(max) = constraints.max_length {
                    if (s.len() as i32) > max {
                        errors.push(ValidationError::MaxLength(field_name.clone(), s.len(), max));
                    }
                }
                // Regex... (Requires regex crate, skipping for MVP/avoid dep if not present. Cargo.toml no regex)
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
