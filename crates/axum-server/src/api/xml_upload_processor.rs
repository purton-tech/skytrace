use crate::errors::CustomError;
use db::Transaction;
use grpc_api::trace::UploadXmlDataRequest;

pub async fn upload_xml_data(
    transaction: Transaction<'_>,
    data_upload: &UploadXmlDataRequest,
    user_id: i32,
) -> Result<(), CustomError> {
    // Check to see which type of xml we have
    let data_upload = if data_upload.msg.contains("</oem>") {
        tracing::info!("Received OEM data upload request");
        Some(super::oem::convert_to_proto(&data_upload.msg).await?)
    } else {
        None
    };

    tracing::debug!("Data upload: {:?}", data_upload);

    if let Some(data_upload) = data_upload {
        let result = super::upload_processor::upload_data(transaction, &data_upload, user_id).await;
        if let Err(e) = result {
            tracing::error!("Error uploading data: {}", e);
        }
    }

    Ok(())
}
