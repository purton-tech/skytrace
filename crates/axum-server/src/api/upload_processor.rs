use crate::errors::CustomError;
use db::{queries, DataSharingPolicy, Pool};
use grpc_api::trace::UploadDataRequest;
use prost::Message;

/***
 * Find all conjunctions where the probability of collision is high
 * and we have at least one space object registered.
 *
 * If they don't already have a negotiations entry in the
 * DB then create one.
 */
pub async fn upload_data(pool: Pool, data_upload: &UploadDataRequest) -> Result<(), CustomError> {
    let mut client = pool.get().await?;

    if let Some(msg) = &data_upload.msg {
        if let Some(grpc_api::trace::data_message::Data::Cdm(cdm)) = &msg.data {
            if let Some(header) = &cdm.header {
                let cdm_json = serde_json::to_value(cdm).unwrap();

                let byte_data = cdm.encode_to_vec();

                let transaction = client.transaction().await?;

                transaction
                    .query(
                        &format!("SET LOCAL row_level_security.user_id = {}", 1),
                        &[],
                    )
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
                            &1,
                            &byte_data.as_ref(),
                            &&cdm_json,
                            &"Empty".as_bytes(),
                            &DataSharingPolicy::Public,
                        )
                        .await?;
                }

                transaction.commit().await?;
            }
        }
    }

    Ok(())
}
