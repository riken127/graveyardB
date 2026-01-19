import 'reflect-metadata';

export const ENTITY_METADATA_KEY = Symbol("graveyard:entity");

export interface EntityOptions {
    name: string;
}

export function GraveyardEntity(options: EntityOptions | string) {
    return function (constructor: Function) {
        const name = typeof options === 'string' ? options : options.name;
        Reflect.defineMetadata(ENTITY_METADATA_KEY, { name }, constructor);
    };
}
