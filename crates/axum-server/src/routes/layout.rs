use crate::errors::CustomError;
use axum::http::Response;
use db::queries::users::User;
use hyper::{Body, StatusCode};

pub fn name(user: &User) -> String {
    if let (Some(first_name), Some(last_name)) = (user.first_name.as_ref(), user.last_name.as_ref())
    {
        return format!("{} {}", first_name, last_name);
    }

    user.email.clone()
}

pub fn redirect_and_snackbar(
    url: &str,
    message: &'static str,
) -> Result<Response<Body>, CustomError> {
    let builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("location", url)
        .header("set-cookie", format!("flash_aargh={}; Max-Age=6", message))
        .body(Body::empty());
    let response =
        builder.map_err(|_| CustomError::FaultySetup("Could not build redirect".to_string()))?;
    Ok(response)
}
