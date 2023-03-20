use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::{routing::get, Router};
use db::Pool;

pub static SEARCH: &str = "/app/team/:team_id/search/:query";

pub fn routes() -> Router {
    Router::new().route(SEARCH, get(search))
}

pub async fn search(
    Path((_team_id, _query)): Path<(i32, String)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<String, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let rows = transaction
        .query("SELECT coalesce(json -> 'object1_metadata' ->> 'object_name', '') as first_object FROM conjunctions LIMIT 5", &[])
        .await?;

    let mut s = "".to_string();
    for row in rows {
        s += "<tr>";
        for i in 0..row.len() {
            let value: &str = row.get(i);
            s += &format!("<td>{}</td>", value);
        }
        s += "</tr>";
    }

    Ok(s)
}
