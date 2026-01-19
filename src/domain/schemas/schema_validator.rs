use crate::domain::schemas::{
    schema::{Field, FieldType, Schema},
    schema_visitor::SchemaVisitor,
};

pub struct SchemaValidator;

impl SchemaVisitor for SchemaValidator {
    fn visit_schema(&mut self, schema: &Schema) -> Result<(), String> {
        if schema.name.is_empty() {
            return Err("Schema name cannot be empty".to_string());
        }
        Ok(())
    }

    fn visit_field(&mut self, name: &str, field: &Field) -> Result<(), String> {
        if name.is_empty() {
            return Err("Field name cannot be empty".to_string());
        }
        match &field.field_type {
            FieldType::Enum(variants) if variants.is_empty() => {
                return Err(format!(
                    "Enum field '{}' must have at least one variant",
                    name
                ));
            }
            _ => {}
        }
        Ok(())
    }
}
