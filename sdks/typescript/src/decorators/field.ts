import 'reflect-metadata';

export const FIELD_METADATA_KEY = Symbol("graveyard:field");

export interface GraveyardFieldOptions {
    nullable?: boolean;
    overridesOnNull?: boolean;

    // Constraints
    min?: number;
    max?: number;
    minLength?: number;
    maxLength?: number;
    regex?: string;
}

export function GraveyardField(options: GraveyardFieldOptions = {}) {
    return function (target: Object, propertyKey: string) {
        // Collect existing fields or create new map
        const fields = Reflect.getMetadata(FIELD_METADATA_KEY, target.constructor) || {};
        fields[propertyKey] = options;
        Reflect.defineMetadata(FIELD_METADATA_KEY, fields, target.constructor);
    };
}
