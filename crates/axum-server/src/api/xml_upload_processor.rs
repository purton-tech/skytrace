use crate::errors::CustomError;
use db::Pool;
use grpc_api::trace::UploadXmlDataRequest;

pub async fn upload_xml_data(
    pool: Pool,
    data_upload: &UploadXmlDataRequest,
) -> Result<(), CustomError> {
    let mut _client = pool.get().await?;

    // Check to see which type of xml we have
    let data_upload = if data_upload.msg.contains("</oem>") {
        Some(super::oem::convert_to_proto(&data_upload.msg).await?)
    } else {
        None
    };

    if let Some(data_upload) = data_upload {
        super::upload_processor::upload_data(pool, &data_upload).await?;
    }

    Ok(())
}
