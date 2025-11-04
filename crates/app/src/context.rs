use std::sync::{Arc, Mutex, RwLock};
use data::{
    InMemoryEssayRepository, InMemoryQuestionRepository,
    InMemoryUserRepository, InMemoryKnowledgeTrailRepository,
    InMemoryExamRubricRepository, InMemoryReadingContentRepository,
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
    pub reading_repo: Arc<InMemoryReadingContentRepository>,
    pub current_user_id: Uuid,
    pub translator: Arc<Mutex<Translator>>,
    pub current_locale: Arc<RwLock<String>>,
}

impl AppContext {
    pub fn new() -> Self {
        let essay_repo = Arc::new(InMemoryEssayRepository::new());
        let question_repo = Arc::new(InMemoryQuestionRepository::new());
        let user_repo = Arc::new(InMemoryUserRepository::new());
        let trail_repo = Arc::new(InMemoryKnowledgeTrailRepository::new());
        let rubric_repo = Arc::new(InMemoryExamRubricRepository::new());
        let reading_repo = Arc::new(InMemoryReadingContentRepository::new());
        
        // User ID padrão (do seeder)
        let current_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001")
            .unwrap();
        
        // Initialize translator with detected system locale
        let locale = LocaleDetector::detect_system_locale();
        let translator = Arc::new(Mutex::new(
            Translator::new(locale.clone(), "./locales")
        ));
        
        let current_locale = Arc::new(RwLock::new(locale));
        
        Self {
            essay_repo,
            question_repo,
            user_repo,
            trail_repo,
            rubric_repo,
            reading_repo,
            current_user_id,
            translator,
            current_locale,
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
            .map_err(|e| format!("{}", e))?;
        
        *self.current_locale.write().unwrap() = locale.to_string();
        Ok(())
    }
    
    /// Get the current locale
    pub fn current_locale(&self) -> String {
        self.current_locale.read().unwrap().clone()
    }
}

