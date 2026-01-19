"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SchemaGenerator = void 0;
require("reflect-metadata");
const eventstore_1 = require("../proto/eventstore");
const entity_1 = require("../decorators/entity");
const field_1 = require("../decorators/field");
class SchemaGenerator {
    static generate(target) {
        const entityMeta = Reflect.getMetadata(entity_1.ENTITY_METADATA_KEY, target);
        if (!entityMeta) {
            throw new Error(`Class ${target.name} is not annotated with @GraveyardEntity`);
        }
        const fieldsMeta = Reflect.getMetadata(field_1.FIELD_METADATA_KEY, target) || {};
        const schemaFields = {};
        // In TS, we don't have easy reflection for types at runtime without emitDecoratorMetadata
        // and even then it's limited (Object, String, Number).
        // We will infer basics from 'design:type' metadata if available, or rely on manual spec/convention?
        // With 'emitDecoratorMetadata: true', we get basic types.
        for (const [propKey, options] of Object.entries(fieldsMeta)) {
            const opts = options;
            const designType = Reflect.getMetadata("design:type", target.prototype, propKey);
            schemaFields[propKey] = {
                fieldType: SchemaGenerator.determineFieldType(designType),
                nullable: opts.nullable ?? true,
                overridesOnNull: opts.overridesOnNull ?? false,
                constraints: SchemaGenerator.buildConstraints(opts)
            };
        }
        return {
            name: entityMeta.name,
            fields: schemaFields
        };
    }
    static determineFieldType(type) {
        const fieldType = {};
        if (type === String) {
            fieldType.primitive = eventstore_1.FieldType_Primitive.STRING;
        }
        else if (type === Number) {
            fieldType.primitive = eventstore_1.FieldType_Primitive.NUMBER;
        }
        else if (type === Boolean) {
            fieldType.primitive = eventstore_1.FieldType_Primitive.BOOLEAN;
        }
        else if (type === Array) {
            // Arrays are hard to infer element type from without manual specification in decorator
            // For MVP, defaulting to array of Strings or we need explicit type in decorator
            fieldType.arrayDef = {
                elementType: { primitive: eventstore_1.FieldType_Primitive.STRING }
            };
        }
        else {
            // Default to STRING if unknown
            fieldType.primitive = eventstore_1.FieldType_Primitive.STRING;
        }
        // TODO: Handle nested schemas, Enums
        return fieldType;
    }
    static buildConstraints(opts) {
        let hasConstraints = false;
        const c = {
            required: false, // logic for required vs nullable
        };
        if (opts.min !== undefined) {
            c.minValue = opts.min;
            hasConstraints = true;
        }
        if (opts.max !== undefined) {
            c.maxValue = opts.max;
            hasConstraints = true;
        }
        if (opts.minLength !== undefined) {
            c.minLength = opts.minLength;
            hasConstraints = true;
        }
        if (opts.maxLength !== undefined) {
            c.maxLength = opts.maxLength;
            hasConstraints = true;
        }
        if (opts.regex !== undefined) {
            c.regex = opts.regex;
            hasConstraints = true;
        }
        return hasConstraints ? c : undefined;
    }
}
exports.SchemaGenerator = SchemaGenerator;
