use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::{negotiations, users};
use db::types::public::{NegotiationAction, NegotiationStatus};
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Action {
    pub action: String,
}

pub async fn change_status(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path((team_id, idor_negotiation_id)): Path<(i32, i32)>,
    Form(action): Form<Action>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    if action.validate().is_ok() {
        let negotiation = negotiations::negotiation()
            .bind(&transaction, &idor_negotiation_id)
            .one()
            .await?;

        let user = users::user()
            .bind(&transaction, &current_user.user_id)
            .one()
            .await?;

        let action = action_from_string(&action.action);

        let new_status = status_from_action(action, negotiation.status);

        negotiations::new_time_line()
            .bind(
                &transaction,
                &negotiation.id,
                &user.id,
                &format!(
                    "Status Change from {} on behalf of Team XXXX",
                    crate::routes::layout::name(&user)
                )
                .as_ref(),
                &"Fill in the details",
                &action,
                &new_status,
            )
            .await?;
    }

    transaction.commit().await?;

    crate::routes::layout::redirect_and_snackbar(
        &super::negotiation_route(team_id, idor_negotiation_id),
        "Message Sent",
    )
}

fn status_from_action(
    action: NegotiationAction,
    current_status: NegotiationStatus,
) -> NegotiationStatus {
    match action {
        NegotiationAction::NegotiationTriggered => current_status,
        NegotiationAction::TriggerDeltaNegotiation => NegotiationStatus::ManoeuvreNegotiate,
        NegotiationAction::ManoeuvreNotRequired => NegotiationStatus::ManoeuvreNotRequired,
        NegotiationAction::ManoeuvreAssigned => NegotiationStatus::ManoeuvreRequired,
        NegotiationAction::ManoeuvrePlanned => NegotiationStatus::ManoeuvrePlanned,
        NegotiationAction::ManoeuvreScreened => NegotiationStatus::ManoeuvreScreened,
        NegotiationAction::ManoeuvreChange => NegotiationStatus::ManoeuvreRequired,
        NegotiationAction::ManoeuvreSigned => NegotiationStatus::ManoeuvreSigned,
        NegotiationAction::ManoeuvreUplinked => NegotiationStatus::ManoeuvreUplinked,
        NegotiationAction::ManoeuvreFailed => NegotiationStatus::ManoeuvreFailed,
        NegotiationAction::MessageSent => current_status,
        NegotiationAction::NewCCSDSData => current_status,
    }
}

fn action_from_string(action: &str) -> NegotiationAction {
    match action {
        "NegotiationTriggered" => NegotiationAction::NegotiationTriggered,
        "TriggerDeltaNegotiation" => NegotiationAction::TriggerDeltaNegotiation,
        "ManoeuvreNotRequired" => NegotiationAction::ManoeuvreNotRequired,
        "ManoeuvreAssigned" => NegotiationAction::ManoeuvreAssigned,
        "ManoeuvrePlanned" => NegotiationAction::ManoeuvrePlanned,
        "ManoeuvreScreened" => NegotiationAction::ManoeuvreScreened,
        "ManoeuvreChange" => NegotiationAction::ManoeuvreChange,
        "ManoeuvreSigned" => NegotiationAction::ManoeuvreSigned,
        "ManoeuvreUplinked" => NegotiationAction::ManoeuvreUplinked,
        "ManoeuvreFailed" => NegotiationAction::ManoeuvreFailed,
        "MessageSent" => NegotiationAction::MessageSent,
        "NewCCSDSData" => NegotiationAction::NewCCSDSData,
        _ => panic!("Missing action"),
    }
}
