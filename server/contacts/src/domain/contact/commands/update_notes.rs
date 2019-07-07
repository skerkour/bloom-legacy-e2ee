use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNotes {
    pub notes: Option<String>,
}

impl eventsourcing::Command for UpdateNotes {
    type Aggregate = contact::Contact;
    type Event = NotesUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Note not found".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let data = contact::EventData::NotesUpdatedV1(contact::NotesUpdatedV1 {
            notes: self.notes.clone(),
        });

        return Ok(NotesUpdated {
            timestamp: chrono::Utc::now(),
            notes: self.notes.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct NotesUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub notes: Option<String>,
}

impl Event for NotesUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            notes: self.notes.clone(),
            ..aggregate
        };
    }
}
