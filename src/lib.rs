//! # graveyar_db
//!
//! `graveyar_db` is a high-performance event store designed for reliability and speed.
//!
//! ## Modules
//!
//! - `config`: Configuration management.
//! - `domain`: Core domain types and event definitions.
//! - `grpc`: gRPC API implementation.
//! - `pipeline`: Event processing pipeline.
//! - `storage`: Pluggable storage engine traits and implementations.

pub mod cluster;
pub mod config;
pub mod domain;
pub mod grpc;
pub mod pipeline;
pub mod storage;

pub mod api {
    tonic::include_proto!("eventstore");
}
