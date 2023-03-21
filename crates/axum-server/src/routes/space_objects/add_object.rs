use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::space_objects;
use db::types::public::{AvoidanceStrategy, IdentifierType, ManoeuvreStrategy};
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewSpaceObject {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The COSPAR ID is mandatory"))]
    pub cospar_id: String,
    #[validate(length(min = 1, message = "The NORAD ID is mandatory"))]
    pub norad_id: String,
    pub manoeuvrability: bool,
    pub implementation_latency: i32,
    pub public: bool,
    pub avoidance_strategy: String,
    pub manoeuvering_strategy: String,
    pub remaining_fuel: String,
    pub type_of_rso: String,
}

pub async fn add_space_object(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
    Form(new_space_object): Form<NewSpaceObject>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let man_strat = match new_space_object.manoeuvering_strategy.as_str() {
        "Impulsive" => ManoeuvreStrategy::Impulsive,
        "ContinuousElectrical" => ManoeuvreStrategy::ContinuousElectrical,
        _ => ManoeuvreStrategy::ContinuousChemical,
    };

    let avoid_strat = match new_space_object.avoidance_strategy.as_str() {
        "Thrusting" => AvoidanceStrategy::Thrusting,
        _ => AvoidanceStrategy::InTrack,
    };

    if new_space_object.validate().is_ok() {
        let object_id = space_objects::add_object()
            .bind(
                &transaction,
                &team_id,
                &new_space_object.name.as_ref(),
                &new_space_object.manoeuvrability,
                &new_space_object.implementation_latency,
                &avoid_strat,
                &man_strat,
                &new_space_object.remaining_fuel.parse::<f64>().ok(),
            )
            .one()
            .await?;

        space_objects::add_designator()
            .bind(
                &transaction,
                &object_id,
                &new_space_object.cospar_id.as_ref(),
                &IdentifierType::Cospar,
            )
            .await?;

        space_objects::add_designator()
            .bind(
                &transaction,
                &object_id,
                &new_space_object.norad_id.as_ref(),
                &IdentifierType::Satcat,
            )
            .await?;
    }

    transaction.commit().await?;

    crate::routes::layout::redirect_and_snackbar(&super::index_route(team_id), "Object Created")
}
