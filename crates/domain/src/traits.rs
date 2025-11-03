use async_trait::async_trait;
use uuid::Uuid;
use super::essay::{Essay, EssayStatus, ExamType, ExamRubric};
use super::question::{Question, Subject, Difficulty};
use super::user::{UserProfile, UserSettings, StudyProgress};
use super::knowledge_trail::KnowledgeTrail;
use shared::Result;

#[async_trait]
pub trait EssayRepository: Send + Sync {
    async fn save(&self, essay: Essay) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Essay>>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Essay>>;
    async fn list_by_status(&self, user_id: Uuid, status: EssayStatus) -> Result<Vec<Essay>>;
    async fn list_by_exam_type(&self, user_id: Uuid, exam_type: ExamType) -> Result<Vec<Essay>>;
    async fn update(&self, essay: Essay) -> Result<()>;
}

#[async_trait]
pub trait ExamRubricRepository: Send + Sync {
    async fn get_rubric(&self, exam_type: ExamType) -> Result<Option<ExamRubric>>;
    async fn list_all(&self) -> Result<Vec<ExamRubric>>;
}

#[async_trait]
pub trait QuestionRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Question>>;
    async fn list_by_subject(&self, subject: Subject) -> Result<Vec<Question>>;
    async fn list_by_difficulty(&self, difficulty: Difficulty) -> Result<Vec<Question>>;
    async fn search(&self, query: &str) -> Result<Vec<Question>>;
}

#[async_trait]
pub trait KnowledgeTrailRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeTrail>>;
    async fn list_available(&self) -> Result<Vec<KnowledgeTrail>>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<KnowledgeTrail>>;
    async fn update_progress(&self, trail_id: Uuid, user_id: Uuid, progress: u8) -> Result<()>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserProfile>>;
    async fn update(&self, profile: UserProfile) -> Result<()>;
    async fn update_settings(&self, user_id: Uuid, settings: UserSettings) -> Result<()>;
}

#[async_trait]
pub trait ProgressRepository: Send + Sync {
    async fn get_progress(&self, user_id: Uuid) -> Result<StudyProgress>;
    async fn track_activity(&self, user_id: Uuid, activity: Activity) -> Result<()>;
    async fn update_streak(&self, user_id: Uuid, streak: u32) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum Activity {
    QuestionAnswered { question_id: Uuid, correct: bool },
    EssaySubmitted { essay_id: Uuid },
    TrailCompleted { trail_id: Uuid },
}

