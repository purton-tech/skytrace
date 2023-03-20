mod index;

use axum::{routing::get, Router};
use ui_components::routes::conjunctions::INDEX;

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index::index))
}
