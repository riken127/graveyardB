#[cfg(test)]
mod tests {
    use crate::domain::schema::model::{Field, FieldConstraints, FieldType, PrimitiveType, Schema};
    use crate::domain::schema::validation::{validate_event_payload, ValidationError};
    use std::collections::HashMap;

    #[test]
    fn test_validation_success() {
        let mut fields = HashMap::new();
        fields.insert(
            "name".to_string(),
            Field {
                field_type: FieldType::Primitive(PrimitiveType::String),
                nullable: false,
                overrides_on_null: false,
                constraints: Some(FieldConstraints {
                    required: true,
                    min_length: Some(1),
                    ..Default::default()
                }),
            },
        );

        let schema = Schema {
            name: "User".to_string(),
            fields,
        };

        let json = serde_json::json!({
            "name": "Alice"
        });
        let payload = serde_json::to_vec(&json).unwrap();

        assert!(validate_event_payload(&payload, &schema).is_ok());
    }

    #[test]
    fn test_validation_failure() {
        let mut fields = HashMap::new();
        fields.insert(
            "age".to_string(),
            Field {
                field_type: FieldType::Primitive(PrimitiveType::Number),
                nullable: false,
                overrides_on_null: false,
                constraints: Some(FieldConstraints {
                    required: true,
                    min_value: Some(18.0),
                    ..Default::default()
                }),
            },
        );

        let schema = Schema {
            name: "User".to_string(),
            fields,
        };

        let json = serde_json::json!({
            "age": 10
        });
        let payload = serde_json::to_vec(&json).unwrap();

        let res = validate_event_payload(&payload, &schema);
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);
        match &errs[0] {
            ValidationError::MinValue(_, val, min) => {
                assert_eq!(*val, 10.0);
                assert_eq!(*min, 18.0);
            }
            _ => panic!("Wrong error"),
        }
    }
}
