use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
    routing::post,
    Router,
};
use db::{queries, Confidentiality, Pool};
use prost::Message; // So we can use decode on a proto message
use serde::{Deserialize, Serialize};

use grpc_api::ccsds::schema::{CdmType, OemType, OpmType};
use ui_components::routes::data_upload::UPLOAD;

pub fn routes() -> Router {
    Router::new().route(UPLOAD, post(upload))
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UploadData {
    pub payloadtype: String,
    pub payload: String,
    pub signature: String,
    pub public: bool,
}

pub async fn upload(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(upload_data): Form<UploadData>,
) -> Result<impl IntoResponse, CustomError> {
    let byte_data = base64::decode(&upload_data.payload);
    let sig_data = base64::decode(&upload_data.signature);

    if let (Ok(byte_data), Ok(sig_data)) = (byte_data, sig_data) {
        let bytes = bytes::Bytes::from(byte_data.clone());

        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;
        super::rls::set_row_level_security_user(&transaction, &current_user).await?;

        match upload_data.payloadtype.as_ref() {
            "CDM" => {
                let cdm = CdmType::decode(bytes);
                if let Ok(cdm) = cdm {
                    let cdm_json = serde_json::to_value(&cdm)?;

                    queries::conjunctions::add_conjunction()
                        .bind(
                            &transaction,
                            &team_id,
                            &byte_data.as_ref(),
                            &&cdm_json,
                            &sig_data.as_ref(),
                            &Confidentiality::Public,
                        )
                        .await?;
                }
            }
            "OEM" => {
                let oem = OemType::decode(bytes);
                if let Ok(oem) = oem {
                    let oem_json = serde_json::to_value(&oem)?;

                    queries::orbit_data::add_orbit_data()
                        .bind(
                            &transaction,
                            &team_id,
                            &byte_data.as_ref(),
                            &&oem_json,
                            &sig_data.as_ref(),
                            &Confidentiality::Public,
                        )
                        .await?;
                }
            }
            "OPM" => {
                let opm = OpmType::decode(bytes);
                if let Ok(opm) = opm {
                    let opm_json = serde_json::to_value(&opm)?;

                    queries::orbit_data::add_orbit_data()
                        .bind(
                            &transaction,
                            &team_id,
                            &byte_data.as_ref(),
                            &&opm_json,
                            &sig_data.as_ref(),
                            &Confidentiality::Public,
                        )
                        .await?;
                }
            }
            _ => (),
        }

        transaction.commit().await?;
    }

    crate::routes::layout::redirect_and_snackbar(
        &crate::routes::space_objects::index_route(team_id),
        "Data Uploaded",
    )
}
