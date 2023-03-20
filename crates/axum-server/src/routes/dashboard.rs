use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::Path;
use axum::Extension;
use axum::{response::Html, routing::get, Router};
use db::queries::aarrr;
use db::Pool;

pub static INDEX: &str = "/app/team/:team_id/dashboard";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index))
}

async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let metrics = aarrr::pirate_metrics().bind(&transaction).one().await?;

    Ok(Html(ui_components::dashboard::index(team_id, metrics)))
}
