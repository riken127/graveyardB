pub mod config;
pub mod infra;
pub mod domain;
pub mod api {
    tonic::include_proto!("eventstore");
}

#[derive(Clone)]
struct GraveyardB;
