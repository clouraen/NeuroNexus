use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::question::{Subject, Difficulty};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadingContent {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub estimated_reading_time: u16,
    pub subject: Subject,
    pub difficulty: Difficulty,
    pub author: Option<String>,
    pub references: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
