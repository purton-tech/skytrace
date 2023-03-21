use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
use rustls::client::{ServerCertVerified, ServerCertVerifier};
use rustls::ServerName;
pub use tokio_postgres::Error as TokioPostgresError;

pub use queries::aarrr::Metrics;
pub use queries::api_keys::ApiKey;
pub use queries::conjunctions::Conjunction;
pub use queries::invitations::Invitation;
pub use queries::negotiations::{Negotiation, TimeLine};
pub use queries::orbit_data::OrbitData;
pub use queries::organisations::GetTeams as Team;
pub use queries::organisations::GetUsers as Member;
pub use queries::organisations::Organisation;
pub use queries::space_objects::RegisteredObject;
pub use queries::tracking_data::TrackingData;
pub use queries::users::User;
pub use types::public::Confidentiality;
pub use types::public::NegotiationAction;
pub use types::public::NegotiationStatus;
pub use types::public::Role;
pub use types::public::ValidationStatus;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();

    let manager = if config.get_ssl_mode() != tokio_postgres::config::SslMode::Disable {
        let tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(DummyTlsVerifier))
            .with_no_client_auth();

        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
        deadpool_postgres::Manager::new(config, tls)
    } else {
        deadpool_postgres::Manager::new(config, tokio_postgres::NoTls)
    };

    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

struct DummyTlsVerifier;

impl ServerCertVerifier for DummyTlsVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: SystemTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
