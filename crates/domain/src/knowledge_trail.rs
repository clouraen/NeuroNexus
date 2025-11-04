use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::question::{Subject, Difficulty};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTrail {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub focus_areas: Vec<Subject>,
    pub progress: u8, // 0-100
    pub modules: Vec<TrailModule>,
    pub estimated_hours: u16,
    pub difficulty_level: Difficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrailModule {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub content_type: ContentType,
    pub content_id: Uuid,
    pub order: usize,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContentType {
    Question,
    Essay,
    Video,
    Reading,
    PracticeTest,
}

