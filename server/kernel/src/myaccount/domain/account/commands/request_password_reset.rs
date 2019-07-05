use crate::{
    error::KernelError, events::EventMetadata, myaccount, myaccount::domain::account, utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct RequestPasswordReset {
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug)]
pub struct RequestPasswordResetNonStored {
    pub plaintext_token: String,
}

impl eventsourcing::Command for RequestPasswordReset {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let password_reset_id = uuid::Uuid::new_v4();
        let mut rng = rand::thread_rng();
        let token_length = rng.gen_range(
            myaccount::PASSWORD_RESET_TOKEN_MIN_LENGTH,
            myaccount::PASSWORD_RESET_TOKEN_MAX_LENGTH,
        );
        let token = utils::random_hex_string(token_length as usize);
        let hashed_token = bcrypt::hash(&token, myaccount::PASSWORD_RESET_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data =
            account::EventData::PasswordResetRequestedV1(account::PasswordResetRequestedV1 {
                password_reset_id,
                password_reset_token: hashed_token.clone(),
                token_plaintext: token,
            });

        return Ok(
            account::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            });
    }
}
