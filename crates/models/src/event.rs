use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Operation {
    Insert { pos: usize, text: String },
    Delete { pos: usize, len: usize },
    Replace { pos: usize, old_len: usize, text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EditEvent {
    pub id: Uuid,
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub operation: serde_json::Value,
    pub version: i64,
    pub timestamp: DateTime<Utc>,
}