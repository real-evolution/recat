use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key, MutableEntity};
use reddd_macros::MutableEntity;
use serde::{Deserialize, Serialize};

/// A struct representing a placeholder for a principle.
#[derive(Debug, MutableEntity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Principle {
    /// The unique identifier of the principle.
    pub id: Key<Self, uuid::Uuid>,

    /// The timestamp at which the principle was created.
    pub created_at: DateTime<Utc>,

    /// The timestamp that the latest modification was made.
    pub updated_at: DateTime<Utc>,
}
