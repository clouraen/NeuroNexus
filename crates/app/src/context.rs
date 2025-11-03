use std::sync::{Arc, Mutex};
use data::{
    InMemoryEssayRepository, InMemoryQuestionRepository,
    InMemoryUserRepository, InMemoryKnowledgeTrailRepository,
    InMemoryExamRubricRepository,
};
use shared::{Translator, LocaleDetector};
use uuid::Uuid;

/// Contexto global com repositórios da aplicação
#[derive(Clone)]
pub struct AppContext {
    pub essay_repo: Arc<InMemoryEssayRepository>,
    pub question_repo: Arc<InMemoryQuestionRepository>,
    pub user_repo: Arc<InMemoryUserRepository>,
    pub trail_repo: Arc<InMemoryKnowledgeTrailRepository>,
    pub rubric_repo: Arc<InMemoryExamRubricRepository>,
    pub current_user_id: Uuid,
    pub translator: Arc<Mutex<Translator>>,
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
        
        // Initialize translator with detected system locale
        let locale = LocaleDetector::detect_system_locale();
        let translator = Arc::new(Mutex::new(
            Translator::new(locale, "./locales")
        ));
        
        Self {
            essay_repo,
            question_repo,
            user_repo,
            trail_repo,
            rubric_repo,
            current_user_id,
            translator,
        }
    }
    
    /// Get a translated string for the given key
    pub fn t(&self, key: &str) -> String {
        self.translator
            .lock()
            .unwrap()
            .translate(key)
    }
    
    /// Change the application locale
    pub fn set_locale(&self, locale: &str) -> Result<(), String> {
        self.translator
            .lock()
            .unwrap()
            .set_locale(locale)
            .map_err(|e| format!("{}", e))
    }
    
    /// Get the current locale
    pub fn current_locale(&self) -> String {
        self.translator
            .lock()
            .unwrap()
            .current_locale()
            .to_string()
    }
}

