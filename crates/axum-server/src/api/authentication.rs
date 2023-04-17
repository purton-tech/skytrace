use crate::errors;
use db::Transaction;
use tonic::Request;

pub async fn check_api_key<T>(
    transaction: &Transaction<'_>,
    req: &Request<T>,
) -> Result<i32, errors::CustomError> {
    if let Some(api_key) = req.metadata().get("x-api-key") {
        let ak = api_key
            .to_str()
            .map_err(|_| errors::CustomError::Unauthorized("Problem parsing API key".into()))?;

        let api_keys = db::queries::api_keys::find_api_key()
            .bind(transaction, &ak)
            .all()
            .await?;

        if api_keys.len() == 1 {
            if let Some(api_key) = api_keys.get(0) {
                Ok(api_key.user_id)
            } else {
                Err(errors::CustomError::Unauthorized(
                    "Got key but could do anything with it".into(),
                ))
            }
        } else {
            Err(errors::CustomError::Unauthorized(
                "Problem with API key".into(),
            ))
        }
    } else {
        Err(errors::CustomError::Unauthorized(
            "You need to set an API Key".into(),
        ))
    }
}
