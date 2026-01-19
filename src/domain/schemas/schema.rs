use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub fields: HashMap<String, Field>,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Number,
    String,
    Enum(Vec<String>),
    SubSchema(Box<Schema>),
    Array(Box<FieldType>),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub field_type: FieldType,
    pub nullable: bool,
    pub overrides_on_null: bool,
}
