use crate::{
    error::KernelError, events::EventMetadata, myaccount, myaccount::domain::pending_account, utils,
};
use chrono::Duration;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[derive(Clone, Debug)]
pub struct SendNewCode {
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug)]
pub struct SendNewCodeNonStored {
    pub code: String,
}

impl eventsourcing::Command for SendNewCode {
    type Aggregate = pending_account::PendingAccount;
    type Event = pending_account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation("Account not found.".to_string()));
        }

        let now = chrono::Utc::now();
        if now.signed_duration_since(aggregate.updated_at) < Duration::seconds(20) {
            return Err(KernelError::Validation(
                "Please wait at least for 20 seconds beffore requesting a new code".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let code = utils::random_digit_string(8);
        let token_hash = bcrypt::hash(&code, myaccount::PENDING_USER_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = pending_account::EventData::NewCodeSentV1(pending_account::NewCodeSentV1 {
            token_hash,
            code,
        });

        return Ok(pending_account::Event {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        });
    }
}
