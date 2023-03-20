use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::{negotiations, users};
use db::types::public::NegotiationAction;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Message {
    pub message: String,
}

pub async fn send_message(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path((team_id, idor_negotiation_id)): Path<(i32, i32)>,
    Form(message): Form<Message>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    if message.validate().is_ok() {
        let negotiation = negotiations::negotiation()
            .bind(&transaction, &idor_negotiation_id)
            .one()
            .await?;

        let user = users::user()
            .bind(&transaction, &current_user.user_id)
            .one()
            .await?;

        negotiations::new_time_line()
            .bind(
                &transaction,
                &negotiation.id,
                &user.id,
                &format!(
                    "Messsage from {} on behalf of Team XXXX",
                    crate::routes::layout::name(&user)
                )
                .as_ref(),
                &message.message.as_ref(),
                &NegotiationAction::MessageSent,
                &negotiation.status,
            )
            .await?;
    }

    transaction.commit().await?;

    crate::routes::layout::redirect_and_snackbar(
        &super::negotiation_route(team_id, idor_negotiation_id),
        "Message Sent",
    )
}
