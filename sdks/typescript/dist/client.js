"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.EventStoreClient = void 0;
const grpc = __importStar(require("@grpc/grpc-js"));
const eventstore_1 = require("./proto/eventstore");
const config_1 = require("./config");
const generator_1 = require("./schema/generator");
class EventStoreClient {
    constructor(config = {}) {
        this.config = { ...config_1.defaultConfig, ...config };
        const address = `${this.config.host}:${this.config.port}`;
        const credentials = this.config.useTls
            ? grpc.credentials.createSsl()
            : grpc.credentials.createInsecure();
        this.client = new eventstore_1.EventStoreClient(address, credentials);
    }
    getDeadline() {
        return new Date(Date.now() + this.config.timeoutMs);
    }
    async appendEvent(streamId, events, expectedVersion = -1) {
        const req = {
            streamId,
            events,
            expectedVersion
        };
        return new Promise((resolve, reject) => {
            const metadata = new grpc.Metadata();
            this.client.appendEvent(req, metadata, { deadline: this.getDeadline() }, (err, response) => {
                if (err)
                    return reject(err);
                if (!response)
                    return reject(new Error('No response received'));
                resolve(response.success);
            });
        });
    }
    async getEvents(streamId) {
        const req = { streamId };
        const events = [];
        return new Promise((resolve, reject) => {
            const stream = this.client.getEvents(req, { deadline: this.getDeadline() });
            stream.on('data', (event) => {
                events.push(event);
            });
            stream.on('end', () => {
                resolve(events);
            });
            stream.on('error', (err) => {
                reject(err);
            });
        });
    }
    async upsertSchema(entityClass) {
        const schema = generator_1.SchemaGenerator.generate(entityClass);
        const req = { schema };
        return new Promise((resolve, reject) => {
            const metadata = new grpc.Metadata();
            this.client.upsertSchema(req, metadata, { deadline: this.getDeadline() }, (err, response) => {
                if (err)
                    return reject(err);
                if (!response)
                    return reject(new Error('No response received'));
                resolve(response);
            });
        });
    }
    close() {
        this.client.close();
    }
}
exports.EventStoreClient = EventStoreClient;
