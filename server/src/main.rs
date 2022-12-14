//! Pricing server.
//!
//! This is the entry point for the pricing server that computes the
//! cost of a flight trip.
#[macro_use]
mod loggers;

mod engine;
use crate::engine::get_pricing;
use log::info;
use pricing_grpc::pricing_server::{Pricing, PricingServer};
use tonic::transport::Server;
/// Pricing Client Library: Client Functions, Structs
pub mod pricing_grpc {
    #![allow(unused_qualifications)]
    include!("grpc.rs");
}

/// Struct that implements the Pricing trait.
///
/// This is the main struct that implements the gRPC service.
#[derive(Default, Debug, Clone, Copy)]
pub struct ArrowPricing {}

// implementing the service trait for our struct
#[tonic::async_trait]
impl Pricing for ArrowPricing {
    /// Get pricing for a given query.
    async fn get_pricing(
        &self,
        request: tonic::Request<pricing_grpc::PricingRequest>,
    ) -> Result<tonic::Response<pricing_grpc::PricingResponse>, tonic::Status> {
        grpc_info!("Received request: {:?}", request);
        let query = request.into_inner();
        grpc_debug!("Getting pricing for query: {:?}", query);
        let price = get_pricing(query);
        grpc_debug!("Pricing computed: {}", price);
        let response = pricing_grpc::PricingResponse { price };
        grpc_debug!("Returning response: {:?}", response);
        grpc_info!("Successfully computed pricing; returning response");
        Ok(tonic::Response::new(response))
    }

    /// Return true if this server is ready to serve others.
    async fn is_ready(
        &self,
        _request: tonic::Request<pricing_grpc::ReadyRequest>,
    ) -> Result<tonic::Response<pricing_grpc::ReadyResponse>, tonic::Status> {
        let response = pricing_grpc::ReadyResponse { ready: true };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let log_cfg: &str = "log4rs.yaml";
        if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
            println!("(logger) could not parse {}. {}", log_cfg, e);
            panic!();
        }
    }

    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

    let full_grpc_addr = format!("[::]:{}", grpc_port).parse()?;

    // creating a service
    let pricing = ArrowPricing::default();

    // creating the health check service
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<PricingServer<ArrowPricing>>()
        .await;

    //start server
    info!("Starting gRPC server at: {}", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(PricingServer::new(pricing))
        .serve(full_grpc_addr)
        .await?;
    info!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
