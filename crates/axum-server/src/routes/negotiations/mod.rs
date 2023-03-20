mod change_status;
mod count;
mod index;
mod negotiation;
mod send_message;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/team/:team_id/negotiations";
pub static COUNT: &str = "/app/team/:team_id/negotiations_count";
pub static NEGOTIATION: &str = "/app/team/:team_id/negotiation/:idor_negotiation_id";
pub static SEND_MESSAGE: &str = "/app/team/:team_id/send_message/:idor_negotiation_id";
pub static CHANGE_STATUS: &str = "/app/team/:team_id/status/:idor_negotiation_id";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(COUNT, get(count::count))
        .route(NEGOTIATION, get(negotiation::index))
        .route(SEND_MESSAGE, post(send_message::send_message))
        .route(CHANGE_STATUS, post(change_status::change_status))
}

pub fn negotiation_route(organisation_id: i32, negotiation_id: i32) -> String {
    format!(
        "/app/team/{}/negotiation/{}",
        organisation_id, negotiation_id
    )
}
