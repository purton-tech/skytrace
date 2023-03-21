use crate::errors::CustomError;
use db::queries;
use db::types::public::{NegotiationAction, NegotiationStatus};
use db::Pool;

/***
 * Find all conjunctions where the probability of collision is high
 * and we have at least one space object registered.
 *
 * If they don't already have a negotiations entry in the
 * DB then create one.
 */
pub async fn process_negotiations(pool: Pool) -> Result<(), CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", 1),
            &[],
        )
        .await?;

    let conjunctions = queries::conjunctions::conjunctions_with_no_negotiation()
        .bind(&transaction)
        .all()
        .await?;

    for conjunction in conjunctions {
        if conjunction.collision_probability > 0.005 {
            let possible_object_1s = queries::space_objects::find_object()
                .bind(&transaction, &conjunction.first_object_id.as_ref())
                .all()
                .await?;

            let possible_object_2s = queries::space_objects::find_object()
                .bind(&transaction, &conjunction.secondary_object_id.as_ref())
                .all()
                .await?;

            if let (Some(object_1), Some(object_2)) =
                (possible_object_1s.first(), possible_object_2s.first())
            {
                dbg!(&object_1, &object_2);
                // Whoah. It's about to hit.
                let negotiation_id = queries::negotiations::new_negotiation()
                    .bind(&transaction, object_1, object_2, &conjunction.id)
                    .one()
                    .await?;

                queries::negotiations::new_time_line()
                    .bind(
                        &transaction,
                        &negotiation_id,
                        &0,
                        &"High Interest Event Detected",
                        &format!(
                            "{} and {} with probability of collision at {}",
                            object_1, object_2, conjunction.collision_probability
                        )
                        .as_ref(),
                        &NegotiationAction::NegotiationTriggered,
                        &NegotiationStatus::ManoeuvreNegotiate,
                    )
                    .await?;
            }
        }
    }

    transaction.commit().await?;

    Ok(())
}
