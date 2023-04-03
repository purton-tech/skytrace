// Implement the XML upload for the API, parse the incoming XML and validate it
// against the schema. Then convert the XML to protobuf and store as json in the dtaabase.
use crate::errors::CustomError;
use db::Pool;
use grpc_api::trace::UploadXmlDataRequest;
/***
 * Find all conjunctions where the probability of collision is high
 * and we have at least one space object registered.
 *
 * If they don't already have a negotiations entry in the
 * DB then create one.
 */
pub async fn upload_xml_data(
    pool: Pool,
    data_upload: &UploadXmlDataRequest,
) -> Result<(), CustomError> {
    let mut _client = pool.get().await?;

    let _parser = xml::reader::EventReader::from_str(data_upload.msg.as_str());

    Ok(())
}
