use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use domain::{
    essay::{Essay, EssayStatus, ExamType, ExamRubric},
    question::{Question, Subject, Difficulty},
    user::{UserProfile, UserSettings, StudyProgress},
    knowledge_trail::KnowledgeTrail,
    traits::*,
};
use shared::Result;

// In-memory implementations for development

pub struct InMemoryEssayRepository {
    essays: Arc<RwLock<HashMap<Uuid, Essay>>>,
}

impl InMemoryEssayRepository {
    pub fn new() -> Self {
        Self {
            essays: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl EssayRepository for InMemoryEssayRepository {
    async fn save(&self, essay: Essay) -> Result<()> {
        let mut essays = self.essays.write().await;
        essays.insert(essay.id, essay);
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Essay>> {
        let essays = self.essays.read().await;
        Ok(essays.get(&id).cloned())
    }

    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Essay>> {
        let essays = self.essays.read().await;
        Ok(essays
            .values()
            .filter(|e| e.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn list_by_status(&self, user_id: Uuid, status: EssayStatus) -> Result<Vec<Essay>> {
        let essays = self.essays.read().await;
        Ok(essays
            .values()
            .filter(|e| e.user_id == user_id && e.status == status)
            .cloned()
            .collect())
    }

    async fn list_by_exam_type(&self, user_id: Uuid, exam_type: ExamType) -> Result<Vec<Essay>> {
        let essays = self.essays.read().await;
        Ok(essays
            .values()
            .filter(|e| e.user_id == user_id && e.exam_type == exam_type)
            .cloned()
            .collect())
    }

    async fn update(&self, essay: Essay) -> Result<()> {
        self.save(essay).await
    }
}

pub struct InMemoryExamRubricRepository {
    rubrics: Arc<RwLock<HashMap<ExamType, ExamRubric>>>,
}

impl InMemoryExamRubricRepository {
    pub fn new() -> Self {
        Self {
            rubrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Método auxiliar para inserir rubricas diretamente (usado por seeders)
    pub async fn insert(&self, rubric: ExamRubric) -> Result<()> {
        let mut rubrics = self.rubrics.write().await;
        rubrics.insert(rubric.exam_type.clone(), rubric);
        Ok(())
    }
}

#[async_trait::async_trait]
impl ExamRubricRepository for InMemoryExamRubricRepository {
    async fn get_rubric(&self, exam_type: ExamType) -> Result<Option<ExamRubric>> {
        let rubrics = self.rubrics.read().await;
        Ok(rubrics.get(&exam_type).cloned())
    }

    async fn list_all(&self) -> Result<Vec<ExamRubric>> {
        let rubrics = self.rubrics.read().await;
        Ok(rubrics.values().cloned().collect())
    }
}

pub struct InMemoryQuestionRepository {
    questions: Arc<RwLock<HashMap<Uuid, Question>>>,
}

impl InMemoryQuestionRepository {
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Método auxiliar para inserir questões diretamente (usado por seeders)
    pub async fn insert(&self, question: Question) -> Result<()> {
        let mut questions = self.questions.write().await;
        questions.insert(question.id, question);
        Ok(())
    }
}

#[async_trait::async_trait]
impl QuestionRepository for InMemoryQuestionRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Question>> {
        let questions = self.questions.read().await;
        Ok(questions.get(&id).cloned())
    }

    async fn list_by_subject(&self, subject: Subject) -> Result<Vec<Question>> {
        let questions = self.questions.read().await;
        Ok(questions
            .values()
            .filter(|q| q.subject == subject)
            .cloned()
            .collect())
    }

    async fn list_by_difficulty(&self, difficulty: Difficulty) -> Result<Vec<Question>> {
        let questions = self.questions.read().await;
        Ok(questions
            .values()
            .filter(|q| q.difficulty == difficulty)
            .cloned()
            .collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Question>> {
        let questions = self.questions.read().await;
        let query_lower = query.to_lowercase();
        Ok(questions
            .values()
            .filter(|q| {
                q.statement.to_lowercase().contains(&query_lower)
                    || q.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect())
    }
}

pub struct InMemoryKnowledgeTrailRepository {
    trails: Arc<RwLock<HashMap<Uuid, KnowledgeTrail>>>,
}

impl InMemoryKnowledgeTrailRepository {
    pub fn new() -> Self {
        Self {
            trails: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Método auxiliar para inserir trilhas diretamente (usado por seeders)
    pub async fn insert(&self, trail: KnowledgeTrail) -> Result<()> {
        let mut trails = self.trails.write().await;
        trails.insert(trail.id, trail);
        Ok(())
    }
}

#[async_trait::async_trait]
impl KnowledgeTrailRepository for InMemoryKnowledgeTrailRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeTrail>> {
        let trails = self.trails.read().await;
        Ok(trails.get(&id).cloned())
    }

    async fn list_available(&self) -> Result<Vec<KnowledgeTrail>> {
        let trails = self.trails.read().await;
        Ok(trails.values().cloned().collect())
    }

    async fn list_by_user(&self, _user_id: Uuid) -> Result<Vec<KnowledgeTrail>> {
        // For MVP, return all trails
        self.list_available().await
    }

    async fn update_progress(&self, trail_id: Uuid, _user_id: Uuid, progress: u8) -> Result<()> {
        let mut trails = self.trails.write().await;
        if let Some(trail) = trails.get_mut(&trail_id) {
            trail.progress = progress;
        }
        Ok(())
    }
}

pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<Uuid, UserProfile>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserProfile>> {
        let users = self.users.read().await;
        Ok(users.get(&id).cloned())
    }

    async fn update(&self, profile: UserProfile) -> Result<()> {
        let mut users = self.users.write().await;
        users.insert(profile.id, profile);
        Ok(())
    }

    async fn update_settings(&self, user_id: Uuid, settings: UserSettings) -> Result<()> {
        let mut users = self.users.write().await;
        if let Some(user) = users.get_mut(&user_id) {
            user.settings = settings;
        }
        Ok(())
    }
}

pub struct InMemoryProgressRepository {
    progress: Arc<RwLock<HashMap<Uuid, StudyProgress>>>,
}

impl InMemoryProgressRepository {
    pub fn new() -> Self {
        Self {
            progress: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl ProgressRepository for InMemoryProgressRepository {
    async fn get_progress(&self, user_id: Uuid) -> Result<StudyProgress> {
        let progress = self.progress.read().await;
        Ok(progress
            .get(&user_id)
            .cloned()
            .unwrap_or_else(|| StudyProgress {
                user_id,
                questions_answered: 0,
                essays_submitted: 0,
                study_streak: 0,
                total_study_hours: 0.0,
                achievements: vec![],
                subject_progress: std::collections::HashMap::new(),
            }))
    }

    async fn track_activity(&self, user_id: Uuid, activity: Activity) -> Result<()> {
        let mut progress = self.progress.write().await;
        let user_progress = progress.entry(user_id).or_insert_with(|| StudyProgress {
            user_id,
            questions_answered: 0,
            essays_submitted: 0,
            study_streak: 0,
            total_study_hours: 0.0,
            achievements: vec![],
            subject_progress: std::collections::HashMap::new(),
        });

        match activity {
            Activity::QuestionAnswered { .. } => {
                user_progress.questions_answered += 1;
            }
            Activity::EssaySubmitted { .. } => {
                user_progress.essays_submitted += 1;
            }
            Activity::TrailCompleted { .. } => {
                // Handle trail completion
            }
        }

        Ok(())
    }

    async fn update_streak(&self, user_id: Uuid, streak: u32) -> Result<()> {
        let mut progress = self.progress.write().await;
        let user_progress = progress.entry(user_id).or_insert_with(|| StudyProgress {
            user_id,
            questions_answered: 0,
            essays_submitted: 0,
            study_streak: 0,
            total_study_hours: 0.0,
            achievements: vec![],
            subject_progress: std::collections::HashMap::new(),
        });
        user_progress.study_streak = streak;
        Ok(())
    }
}

