use crate::authentication::Authentication;
use crate::errors::CustomError;
use db::Transaction;

// A helper function for setting the RLS user which is used by all the policies.
pub async fn set_row_level_security_user(
    transaction: &Transaction<'_>,
    current_user: &Authentication,
) -> Result<(), CustomError> {
    transaction
        .query(
            &format!(
                "SET LOCAL row_level_security.user_id = {}",
                current_user.user_id
            ),
            &[],
        )
        .await?;

    Ok(())
}
// A helper function for setting the RLS user which is used by all the policies.
pub async fn _set_row_level_security_ecdh_public_key(
    transaction: &Transaction<'_>,
    ecdh_public_key: &str,
) -> Result<(), CustomError> {
    transaction
        .query(
            &format!(
                "SET LOCAL row_level_security.ecdh_public_key = '{}'",
                ecdh_public_key
            ),
            &[],
        )
        .await?;

    Ok(())
}
