use crate::api as proto;
use crate::domain::schema::model::{
    EnumType, Field, FieldConstraints, FieldType, PrimitiveType, Schema,
};
use std::collections::HashMap;

// Proto -> Domain

impl From<proto::Schema> for Schema {
    fn from(proto_schema: proto::Schema) -> Self {
        let mut fields = HashMap::new();
        for (name, field) in proto_schema.fields {
            fields.insert(name, field.into());
        }
        Schema {
            name: proto_schema.name,
            fields,
        }
    }
}

impl From<proto::Field> for Field {
    fn from(proto_field: proto::Field) -> Self {
        Field {
            field_type: proto_field
                .field_type
                .map_or(FieldType::Primitive(PrimitiveType::String), |ft| ft.into()),
            nullable: proto_field.nullable,
            overrides_on_null: proto_field.overrides_on_null,
            constraints: proto_field.constraints.map(|c| c.into()),
        }
    }
}

impl From<proto::FieldConstraints> for FieldConstraints {
    fn from(proto_c: proto::FieldConstraints) -> Self {
        FieldConstraints {
            required: proto_c.required,
            min_value: proto_c.min_value,
            max_value: proto_c.max_value,
            min_length: proto_c.min_length,
            max_length: proto_c.max_length,
            regex: proto_c.regex,
        }
    }
}

impl From<proto::FieldType> for FieldType {
    fn from(proto_ft: proto::FieldType) -> Self {
        if let Some(kind) = proto_ft.kind {
            match kind {
                proto::field_type::Kind::Primitive(p) => {
                    use std::convert::TryFrom;
                    let prim = match proto::field_type::Primitive::try_from(p)
                        .unwrap_or(proto::field_type::Primitive::String)
                    {
                        proto::field_type::Primitive::Number => PrimitiveType::Number,
                        proto::field_type::Primitive::String => PrimitiveType::String,
                        proto::field_type::Primitive::Boolean => PrimitiveType::Boolean,
                    };
                    FieldType::Primitive(prim)
                }
                proto::field_type::Kind::EnumDef(e) => FieldType::Enum(EnumType {
                    variants: e.variants,
                }),
                proto::field_type::Kind::SubSchema(s) => FieldType::SubSchema(Box::new(s.into())),
                proto::field_type::Kind::ArrayDef(a) => {
                    let inner_type = a
                        .element_type
                        .map(|et| *Box::new((*et).into()))
                        .unwrap_or(FieldType::Primitive(PrimitiveType::String));
                    FieldType::Array(Box::new(inner_type))
                }
            }
        } else {
            FieldType::Primitive(PrimitiveType::String)
        }
    }
}

// Domain -> Proto

impl From<Schema> for proto::Schema {
    fn from(domain_schema: Schema) -> Self {
        let mut fields = HashMap::new();
        for (name, field) in domain_schema.fields {
            fields.insert(name, field.into());
        }
        proto::Schema {
            name: domain_schema.name,
            fields,
        }
    }
}

impl From<Field> for proto::Field {
    fn from(domain_field: Field) -> Self {
        proto::Field {
            field_type: Some(domain_field.field_type.into()),
            nullable: domain_field.nullable,
            overrides_on_null: domain_field.overrides_on_null,
            constraints: domain_field.constraints.map(|c| c.into()),
        }
    }
}

impl From<FieldConstraints> for proto::FieldConstraints {
    fn from(domain_c: FieldConstraints) -> Self {
        proto::FieldConstraints {
            required: domain_c.required,
            min_value: domain_c.min_value,
            max_value: domain_c.max_value,
            min_length: domain_c.min_length,
            max_length: domain_c.max_length,
            regex: domain_c.regex,
        }
    }
}

impl From<FieldType> for proto::FieldType {
    fn from(domain_ft: FieldType) -> Self {
        match domain_ft {
            FieldType::Primitive(p) => {
                let proto_p = match p {
                    PrimitiveType::Number => proto::field_type::Primitive::Number,
                    PrimitiveType::String => proto::field_type::Primitive::String,
                    PrimitiveType::Boolean => proto::field_type::Primitive::Boolean,
                };
                proto::FieldType {
                    kind: Some(proto::field_type::Kind::Primitive(proto_p.into())),
                }
            }
            FieldType::Enum(e) => proto::FieldType {
                kind: Some(proto::field_type::Kind::EnumDef(proto::field_type::Enum {
                    variants: e.variants,
                })),
            },
            FieldType::SubSchema(s) => proto::FieldType {
                kind: Some(proto::field_type::Kind::SubSchema((*s).into())),
            },
            FieldType::Array(t) => proto::FieldType {
                kind: Some(proto::field_type::Kind::ArrayDef(Box::new(
                    proto::field_type::Array {
                        element_type: Some(Box::new((*t).into())),
                    },
                ))),
            },
        }
    }
}
