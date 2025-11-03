use std::sync::Arc;
use data::{
    InMemoryEssayRepository, InMemoryQuestionRepository,
    InMemoryUserRepository, InMemoryKnowledgeTrailRepository,
    InMemoryExamRubricRepository,
};
use uuid::Uuid;

/// Contexto global com repositórios da aplicação
pub struct AppContext {
    pub essay_repo: Arc<InMemoryEssayRepository>,
    pub question_repo: Arc<InMemoryQuestionRepository>,
    pub user_repo: Arc<InMemoryUserRepository>,
    pub trail_repo: Arc<InMemoryKnowledgeTrailRepository>,
    pub rubric_repo: Arc<InMemoryExamRubricRepository>,
    pub current_user_id: Uuid,
}

impl AppContext {
    pub fn new() -> Self {
        let essay_repo = Arc::new(InMemoryEssayRepository::new());
        let question_repo = Arc::new(InMemoryQuestionRepository::new());
        let user_repo = Arc::new(InMemoryUserRepository::new());
        let trail_repo = Arc::new(InMemoryKnowledgeTrailRepository::new());
        let rubric_repo = Arc::new(InMemoryExamRubricRepository::new());
        
        // User ID padrão (do seeder)
        let current_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001")
            .unwrap();
        
        Self {
            essay_repo,
            question_repo,
            user_repo,
            trail_repo,
            rubric_repo,
            current_user_id,
        }
    }
}

