"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FIELD_METADATA_KEY = void 0;
exports.GraveyardField = GraveyardField;
require("reflect-metadata");
exports.FIELD_METADATA_KEY = Symbol("graveyard:field");
function GraveyardField(options = {}) {
    return function (target, propertyKey) {
        // Collect existing fields or create new map
        const fields = Reflect.getMetadata(exports.FIELD_METADATA_KEY, target.constructor) || {};
        fields[propertyKey] = options;
        Reflect.defineMetadata(exports.FIELD_METADATA_KEY, fields, target.constructor);
    };
}
