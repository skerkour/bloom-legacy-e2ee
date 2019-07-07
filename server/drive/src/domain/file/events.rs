use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use kernel::{db::schema::drive_files_events, events::EventMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "drive_files_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadedV1 {
    pub id: uuid::Uuid,
    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_: String, // MIME type
    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadedV1 {
    pub presigned_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovedV1 {
    pub to: uuid::Uuid, // new parent
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrashedV1 {
    pub explicitly_trashed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CopiedV1 {
    pub to: uuid::Uuid, // new parent
    pub new_file: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RenamedV1 {
    pub name: String,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // UploadedV1
            EventData::UploadedV1(ref data) => super::File {
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                explicitly_trashed: false,
                name: data.name.clone(),
                parent_id: data.parent_id,
                size: data.size,
                type_: data.type_.clone(),
                trashed_at: None,

                owner_id: data.owner_id,
            },
            // DownloadedV1
            EventData::DownloadedV1(_) => super::File { ..aggregate },
            // MovedV1
            EventData::MovedV1(ref data) => super::File {
                parent_id: Some(data.to),
                ..aggregate
            },
            // TrashedV1
            EventData::TrashedV1(ref data) => super::File {
                explicitly_trashed: data.explicitly_trashed,
                trashed_at: Some(self.timestamp),
                ..aggregate
            },
            // RestoredV1
            EventData::RestoredV1 => super::File {
                explicitly_trashed: false,
                trashed_at: None,
                ..aggregate
            },
            // CopiedV1
            // RenamedV1
            EventData::RenamedV1(ref data) => super::File {
                name: data.name.clone(),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
