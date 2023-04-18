mod api;
mod authentication;
mod config;
mod email;
mod errors;
mod routes;

use axum::body::Body;
use axum::extract::Extension;
use axum::{routing::get, Router};
use db::create_pool;
use grpc_api::trace::trace_server::TraceServer;
use http::{header::CONTENT_TYPE, Request};
use std::net::SocketAddr;
use tonic::transport::Server;
use tower::{make::Shared, steer::Steer, BoxError, ServiceExt};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config = config::Config::new();

    let pool = create_pool(&config.database_url);

    // All the routes for our server side rendered pages
    let app = Router::new()
        .merge(routes::dashboard::routes())
        .merge(routes::team::routes())
        .merge(routes::data_upload::routes())
        .merge(routes::space_objects::routes())
        .merge(routes::conjunctions::routes())
        .merge(routes::orbit_data::routes())
        .merge(routes::api_keys::routes())
        .merge(routes::registration_handler::routes())
        .merge(routes::negotiations::routes())
        .merge(routes::profile::routes())
        .merge(routes::search::routes())
        .route("/static/*path", get(routes::static_files::static_path))
        .layer(Extension(config))
        .layer(Extension(pool.clone()))
        .map_err(BoxError::from)
        .boxed_clone();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_api::trace::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // Handle gRPC API requests
    let grpc = Server::builder()
        .accept_http1(true)
        // Notice the `enable` method. This gives us gRPC-Web support.
        .add_service(tonic_web::enable(TraceServer::new(
            api::trace_grpc_service::TraceService { pool },
        )))
        .add_service(reflection_service)
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .boxed_clone();

    // Create a service that can respond to Web and gRPC
    let http_grpc = Steer::new(vec![app, grpc], |req: &Request<Body>, _svcs: &[_]| {
        if req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes()) != Some(b"application/grpc") {
            0
        } else {
            1
        }
    });

    // Listen to incoming gRPC and HTTP requests.
    let addr = SocketAddr::from(([0, 0, 0, 0], 7403));
    tracing::info!("listening on {}", addr);
    let server = axum::Server::bind(&addr).serve(Shared::new(http_grpc));

    if let Err(e) = server.await {
        tracing::error!("server error: {}", e);
    }
}
