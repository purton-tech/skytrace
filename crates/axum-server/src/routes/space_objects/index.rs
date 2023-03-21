use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::space_objects;
use db::Pool;

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let space_objects = space_objects::space_objects()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    Ok(Html(ui_components::space_objects::index::space_objects(
        space_objects,
        team_id,
    )))
}
