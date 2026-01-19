use crate::domain::schemas::schema::{Field, FieldType, Schema};

pub trait SchemaVisitor {
    fn visit_schema(&mut self, schema: &Schema) -> Result<(), String>;
    fn visit_field(&mut self, name: &str, field: &Field) -> Result<(), String>;
}

impl Schema {
    pub fn accept<V: SchemaVisitor>(&self, visitor: &mut V) -> Result<(), String> {
        visitor.visit_schema(self)?;
        for (name, field) in &self.fields {
            visitor.visit_field(name, field)?;
            field.accept(visitor)?;
        }
        Ok(())
    }
}

impl Field {
    pub fn accept<V: SchemaVisitor>(&self, visitor: &mut V) -> Result<(), String> {
        match &self.field_type {
            FieldType::SubSchema(schema) => schema.accept(visitor)?,
            FieldType::Array(inner) => {
                let dummy_field = Field {
                    field_type: *inner.clone(),
                    nullable: false,
                    overrides_on_null: false,
                };
                dummy_field.accept(visitor)?
            }
            _ => {}
        }
        Ok(())
    }
}
