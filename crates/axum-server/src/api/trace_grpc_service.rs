use db::Pool;
use grpc_api::trace::*;
use tonic::{Request, Response, Status};

pub struct TraceService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl grpc_api::trace::trace_server::Trace for TraceService {
    async fn process_negotiations(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let response = EmptyResponse {};

        super::negotiations_processor::process_negotiations(self.pool.clone()).await?;

        return Ok(Response::new(response));
    }

    async fn upload_data(
        &self,
        request: Request<UploadDataRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let mut client = self
            .pool
            .get()
            .await
            .map_err(|_| Status::internal("Problem getting DB connection"))?;
        let transaction = client
            .transaction()
            .await
            .map_err(|_| Status::internal("Problem getting DB connection"))?;

        let user_id = super::authentication::check_api_key(&transaction, &request).await?;
        let data_upload = request.into_inner();

        super::upload_processor::upload_data(transaction, &data_upload, user_id).await?;

        let response = EmptyResponse {};

        return Ok(Response::new(response));
    }

    async fn upload_xml_data(
        &self,
        request: Request<UploadXmlDataRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let mut client = self
            .pool
            .get()
            .await
            .map_err(|_| Status::internal("Problem getting DB connection"))?;
        let transaction = client
            .transaction()
            .await
            .map_err(|_| Status::internal("Problem getting DB connection"))?;

        let user_id = super::authentication::check_api_key(&transaction, &request).await?;
        let data_upload = request.into_inner();

        super::xml_upload_processor::upload_xml_data(transaction, &data_upload, user_id).await?;

        let response = EmptyResponse {};

        return Ok(Response::new(response));
    }
}
