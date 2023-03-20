use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;
use tonic::{Code, Status};

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            //CustomError::Unauthorized(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => {
                write!(f, "Database Error: {}", cause)
            }
        }
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<db::TokioPostgresError> for CustomError {
    fn from(err: db::TokioPostgresError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

impl From<db::PoolError> for CustomError {
    fn from(err: db::PoolError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

impl From<serde_json::error::Error> for CustomError {
    fn from(err: serde_json::error::Error) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

// For gRPC we raise a custom error and it gets converted to a gRPC status code.
impl From<CustomError> for Status {
    fn from(error: CustomError) -> Status {
        match error {
            CustomError::Database(cause) => Status::new(Code::Internal, cause),
            CustomError::FaultySetup(cause) => Status::new(Code::Internal, cause),
        }
    }
}
