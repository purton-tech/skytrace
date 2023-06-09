use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries;
use db::Pool;

pub async fn index(
    current_user: Authentication,
    Path(organisation_id): Path<i32>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let teams = queries::organisations::get_teams()
        .bind(&transaction, &current_user.user_id)
        .all()
        .await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    Ok(Html(ui_components::team_members::team_popup::team_popup(
        teams, team,
    )))
}
