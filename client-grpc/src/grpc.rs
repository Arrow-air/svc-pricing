/// Are you Ready?
///
/// No arguments
#[derive(Copy, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyRequest {}
/// I'm Ready
#[derive(Copy, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    /// Indicate if the service is ready to accept requests.
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
/// Get the price for a type of service.
///
/// Two required fields:
/// - `service_type`: the type of service. 0 = cargo, 1 = rideshare, 2 =
///    charter
/// - `distance`: the distance of the trip in km
#[derive(Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricingRequest {
    /// service type
    /// 0 = cargo
    /// 1 = rideshare
    /// 2 = charter
    #[prost(enumeration = "pricing_request::ServiceType", tag = "1")]
    pub service_type: i32,
    /// distance in kilometers
    ///
    /// weight in kg - Not in use for now
    ///
    /// conversations are ongoing to determine how weight
    /// impacts pricing
    ///
    /// required float weight_kg = 3;
    #[prost(float, tag = "2")]
    pub distance_km: f32,
}
/// Nested message and enum types in `PricingRequest`.
pub mod pricing_request {
    /// Service type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ServiceType {
        /// Cargo service that can transport goods.
        Cargo = 0,
        /// Rideshare service that can transport passengers.
        Rideshare = 1,
        /// Charter service that can be reserved for a specific trip.
        Charter = 2,
    }
    impl ServiceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServiceType::Cargo => "CARGO",
                ServiceType::Rideshare => "RIDESHARE",
                ServiceType::Charter => "CHARTER",
            }
        }
    }
}
/// Price for a service
#[derive(Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricingResponse {
    /// price in dollars
    #[prost(float, tag = "1")]
    pub price: f32,
}
/// Generated client implementations.
pub mod pricing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Pricing for different services: cargo, rideshare, and charter
    #[derive(Debug, Clone)]
    pub struct PricingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PricingClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PricingClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PricingClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PricingClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_pricing(
            &mut self,
            request: impl tonic::IntoRequest<super::PricingRequest>,
        ) -> Result<tonic::Response<super::PricingResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grpc.Pricing/GetPricing");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn is_ready(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadyRequest>,
        ) -> Result<tonic::Response<super::ReadyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grpc.Pricing/IsReady");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
