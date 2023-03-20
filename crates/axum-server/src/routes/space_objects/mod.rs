mod add_object;
mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/team/:team_id/space_objects";
pub static NEWOBJECT: &str = "/app/team/:team_id/add_space_object";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEWOBJECT, post(add_object::add_space_object))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/space_objects", organisation_id)
}
