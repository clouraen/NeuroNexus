use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::question::Subject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub settings: UserSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub study_reminders: bool,
    pub language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    Cyberpunk,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Language {
    Portuguese,
    English,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyProgress {
    pub user_id: Uuid,
    pub questions_answered: u32,
    pub essays_submitted: u32,
    pub study_streak: u32,
    pub total_study_hours: f32,
    pub achievements: Vec<Achievement>,
    pub subject_progress: HashMap<Subject, SubjectProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectProgress {
    pub subject: Subject,
    pub questions_answered: u32,
    pub average_score: f32,
    pub last_studied_at: Option<DateTime<Utc>>,
}

