use crate::errors::CustomError;
use db::{queries, Confidentiality, Transaction};
use grpc_api::trace::data_message::Data;
use grpc_api::trace::UploadDataRequest;
use prost::Message;

/***
 * Find all conjunctions where the probability of collision is high
 * and we have at least one space object registered.
 *
 * If they don't already have a negotiations entry in the
 * DB then create one.
 */
pub async fn upload_data(
    transaction: Transaction<'_>,
    data_upload: &UploadDataRequest,
    user_id: i32,
) -> Result<(), CustomError> {
    if let Some(msg) = &data_upload.msg {
        match &msg.data {
            Some(Data::Cdm(cdm)) => {
                if let Some(header) = &cdm.header {
                    let cdm_json = serde_json::to_value(cdm).unwrap();

                    let byte_data = cdm.encode_to_vec();

                    transaction
                        .query(
                            &format!("SET LOCAL row_level_security.user_id = {}", user_id),
                            &[],
                        )
                        .await?;

                    let upload_org = queries::organisations::get_upload_org()
                        .bind(&transaction)
                        .one()
                        .await?;

                    // Do we already have a conjunction like this?
                    let count = queries::conjunctions::count_conjunction_by_message_id()
                        .bind(&transaction, &header.message_id.as_ref())
                        .one()
                        .await?;

                    if count == 0 {
                        queries::conjunctions::add_conjunction()
                            .bind(
                                &transaction,
                                &upload_org,
                                &byte_data.as_ref(),
                                &&cdm_json,
                                &"Empty".as_bytes(),
                                &Confidentiality::Public,
                            )
                            .await?;
                    }

                    transaction.commit().await?;
                }
            }
            Some(Data::Oem(eom)) => {
                let cdm_json = serde_json::to_value(eom).unwrap();

                let byte_data = eom.encode_to_vec();

                transaction
                    .query(
                        &format!("SET LOCAL row_level_security.user_id = {}", user_id),
                        &[],
                    )
                    .await?;

                let upload_org = queries::organisations::get_upload_org()
                    .bind(&transaction)
                    .one()
                    .await?;

                queries::orbit_data::add_orbit_data()
                    .bind(
                        &transaction,
                        &upload_org,
                        &byte_data.as_ref(),
                        &&cdm_json,
                        &"Empty".as_bytes(),
                        &Confidentiality::Public,
                    )
                    .await?;

                transaction.commit().await?;
            }
            Some(Data::Opm(_opm)) => {
                // TODO
                tracing::error!("Not implemented");
            }
            None => {
                tracing::error!("No data in upload");
            }
        }
    }

    Ok(())
}
