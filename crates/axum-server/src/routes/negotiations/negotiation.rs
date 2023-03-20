use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::negotiations;
use db::Pool;

pub async fn index(
    current_user: Authentication,
    Path((team_id, idor_negotiation_id)): Path<(i32, i32)>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let negotiation = negotiations::negotiation()
        .bind(&transaction, &idor_negotiation_id)
        .one()
        .await?;

    let time_line = negotiations::time_line()
        .bind(&transaction, &idor_negotiation_id)
        .all()
        .await?;

    Ok(Html(ui_components::negotiations::negotiation::negotiation(
        negotiation,
        time_line,
        team_id,
    )))
}
