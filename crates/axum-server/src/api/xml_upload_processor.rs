use crate::errors::CustomError;
use db::Pool;
use grpc_api::trace::UploadXmlDataRequest;

pub async fn upload_xml_data(
    pool: Pool,
    _data_upload: &UploadXmlDataRequest,
) -> Result<(), CustomError> {
    let mut _client = pool.get().await?;

    Ok(())
}
