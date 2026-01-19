use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    pub name: String,
    pub fields: HashMap<String, Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    pub field_type: FieldType,
    pub nullable: bool,
    pub overrides_on_null: bool,
    pub constraints: Option<FieldConstraints>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldConstraints {
    pub required: bool,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub regex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    Primitive(PrimitiveType),
    Enum(EnumType),
    Array(Box<FieldType>),  // Recursive for array of types
    SubSchema(Box<Schema>), // Nested schema
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrimitiveType {
    Number,
    String,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumType {
    pub variants: Vec<String>,
}
