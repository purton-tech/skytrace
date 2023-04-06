use crate::errors::CustomError;
use db::Pool;
use grpc_api::trace::UploadXmlDataRequest;

pub async fn upload_xml_data(
    pool: Pool,
    data_upload: &UploadXmlDataRequest,
) -> Result<(), CustomError> {
    let mut _client = pool.get().await?;

    let _cdm_upload = super::oem::convert_to_proto(&data_upload.msg);

    Ok(())
}
