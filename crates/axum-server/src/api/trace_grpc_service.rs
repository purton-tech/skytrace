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
        let data_upload = request.into_inner();

        super::upload_processor::upload_data(self.pool.clone(), &data_upload).await?;

        let response = EmptyResponse {};

        return Ok(Response::new(response));
    }
}
