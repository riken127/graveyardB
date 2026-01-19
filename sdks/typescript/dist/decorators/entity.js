"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ENTITY_METADATA_KEY = void 0;
exports.GraveyardEntity = GraveyardEntity;
require("reflect-metadata");
exports.ENTITY_METADATA_KEY = Symbol("graveyard:entity");
function GraveyardEntity(options) {
    return function (constructor) {
        const name = typeof options === 'string' ? options : options.name;
        Reflect.defineMetadata(exports.ENTITY_METADATA_KEY, { name }, constructor);
    };
}
