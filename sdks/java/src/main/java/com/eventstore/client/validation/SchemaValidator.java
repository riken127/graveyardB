package com.eventstore.client.validation;

import com.eventstore.client.model.Field;
import com.eventstore.client.model.FieldConstraints;
import com.eventstore.client.model.FieldType; // Import FieldType needed
import com.eventstore.client.model.Schema;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.regex.Pattern;

public class SchemaValidator {

    private static final ObjectMapper objectMapper = new ObjectMapper();

    /**
     * Validates a JSON payload against a Schema.
     *
     * @param payload JSON bytes.
     * @param schema  The Proto Schema.
     * @return List of error messages. Empty if valid.
     */
    public static List<String> validate(byte[] payload, Schema schema) {
        List<String> errors = new ArrayList<>();
        JsonNode root;
        try {
            root = objectMapper.readTree(payload);
        } catch (IOException e) {
            errors.add("Invalid JSON payload: " + e.getMessage());
            return errors;
        }

        // Iterate defined fields in schema
        for (Map.Entry<String, Field> entry : schema.getFieldsMap().entrySet()) {
            String fieldName = entry.getKey();
            Field fieldDef = entry.getValue();
            JsonNode node = root.get(fieldName);

            // 1. Required / Nullable Check
            // Note: Schema doesn't explicitly have "required". It has "nullable".
            // If constraints.required is set, or if nullable is false?
            // GraveyardField annotation has nullable() default true.
            // FieldConstraints has required(). 
            // We should check FieldConstraints.required first.
            boolean required = false;
            if (fieldDef.hasConstraints()) {
                required = fieldDef.getConstraints().getRequired();
            }
            // Logic: if required is true, it MUST be present and not null.
            // If required is false, check nullable? If nullable is false, it MUST be not null IF present?
            // Let's stick to 'required' constraint as primary enforcement for presence.
            
            if (node == null || node.isNull()) {
                if (required) {
                    errors.add(String.format("Field '%s' is required but missing or null", fieldName));
                }
                continue; // Skip further checks if null behavior satisfied
            }

            // 2. Type Check
            validateType(fieldName, node, fieldDef, errors);

            // 3. Constraints Check
            if (fieldDef.hasConstraints()) {
                validateConstraints(fieldName, node, fieldDef.getConstraints(), errors);
            }
        }
        return errors;
    }

    private static void validateType(String fieldName, JsonNode node, Field fieldDef, List<String> errors) {
        com.eventstore.client.model.FieldType ft = fieldDef.getFieldType();
        switch (ft.getKindCase()) {
            case PRIMITIVE:
                switch (ft.getPrimitive()) {
                    case STRING:
                        if (!node.isTextual()) errors.add(String.format("Field '%s' must be a STRING", fieldName));
                        break;
                    case NUMBER:
                        if (!node.isNumber()) errors.add(String.format("Field '%s' must be a NUMBER", fieldName));
                        break;
                    case BOOLEAN:
                        if (!node.isBoolean()) errors.add(String.format("Field '%s' must be a BOOLEAN", fieldName));
                        break;
                    default:
                        break;
                }
                break;
            case ENUM_DEF:
                String val = node.asText();
                List<String> variants = ft.getEnumDef().getVariantsList();
                if (!variants.contains(val)) {
                    errors.add(String.format("Field '%s' value '%s' is not a valid enum variant %s", fieldName, val, variants));
                }
                break;
            case ARRAY_DEF:
                if (!node.isArray()) {
                    errors.add(String.format("Field '%s' must be an ARRAY", fieldName));
                } else {
                    // Check element types? skipping for MVP brevity but should traverse
                }
                break;
            case SUB_SCHEMA:
                 if (!node.isObject()) {
                     errors.add(String.format("Field '%s' must be an OBJECT (Nested Schema)", fieldName));
                 } else {
                     // Recurse?
                     // Need to convert node to bytes? No, we need a recursive validator that takes JsonNode.
                     // For MVP, we skip deep recursion unless we refactor validate() to take JsonNode.
                 }
                break;
            default:
                break;
        }
    }

    private static void validateConstraints(String fieldName, JsonNode node, FieldConstraints constraints, List<String> errors) {
        if (node.isNumber()) {
            double v = node.asDouble();
            if (constraints.hasMinValue() && v < constraints.getMinValue()) {
                errors.add(String.format("Field '%s' value %f is less than min %f", fieldName, v, constraints.getMinValue()));
            }
            if (constraints.hasMaxValue() && v > constraints.getMaxValue()) {
                errors.add(String.format("Field '%s' value %f is greater than max %f", fieldName, v, constraints.getMaxValue()));
            }
        }
        if (node.isTextual()) {
            String s = node.asText();
            if (constraints.hasMinLength() && s.length() < constraints.getMinLength()) {
                errors.add(String.format("Field '%s' length %d is less than min %d", fieldName, s.length(), constraints.getMinLength()));
            }
            if (constraints.hasMaxLength() && s.length() > constraints.getMaxLength()) {
                errors.add(String.format("Field '%s' length %d is greater than max %d", fieldName, s.length(), constraints.getMaxLength()));
            }
            if (constraints.hasRegex()) {
                String regex = constraints.getRegex();
                if (!regex.isEmpty() && !Pattern.matches(regex, s)) {
                     errors.add(String.format("Field '%s' value '%s' does not match regex '%s'", fieldName, s, regex));
                }
            }
        }
    }
}
