use dioxus::prelude::*;
use dioxus_router::{Routable, Router};
use crate::pages::*;
use crate::components::LoadingScreen;
use crate::theme::CSS;
use crate::context::AppContext;
use data::seed_all_data;
use std::sync::Arc;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/trilhas")]
    KnowledgeTrails {},
    #[route("/trilha/:trail_id")]
    TrailDetail { trail_id: String },
    #[route("/trilha/:trail_id/licao/:module_id")]
    LessonViewer { trail_id: String, module_id: String },
    #[route("/questoes")]
    Questions {},
    #[route("/questao/:id")]
    QuestionDetail { id: String },
    #[route("/redacoes")]
    Essays {},
    #[route("/redacao/:id")]
    EssayDetail { id: String },
    #[route("/redacao/nova")]
    NewEssay {},
    #[route("/perfil")]
    Profile {},
}

#[component]
pub fn App() -> Element {
    // Inicializar contexto e fornecer para a árvore de componentes
    let ctx = use_context_provider(|| AppContext::new());
    
    // Clone context for different use_effects
    let ctx_for_ai = ctx.clone();
    let ctx_for_seeding = ctx.clone();
    
    // Track AI model loading state
    let mut is_loading = use_signal(|| true);
    let mut loading_progress = use_signal(|| 0.0_f32);
    let mut loading_message = use_signal(|| "Initializing AI model...".to_string());
    
    // Initialize AI model on startup
    use_effect(move || {
        let ctx_clone = ctx_for_ai.clone();
        spawn(async move {
            // Check if model is already loaded
            if ctx_clone.ai_service.is_initialized().await {
                loading_progress.set(1.0);
                loading_message.set("Model already loaded!".to_string());
                // Small delay to show success state
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                is_loading.set(false);
                return;
            }
            
            // Update loading states
            loading_message.set("Downloading AI model...".to_string());
            loading_progress.set(0.2);
            
            // Start initialization in background
            let ai_service = ctx_clone.ai_service.clone();
            let init_task = tokio::spawn(async move {
                ai_service.initialize().await
            });
            
            // Simulate progress while initializing
            let mut progress: f32 = 0.2;
            while !init_task.is_finished() {
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                progress = f32::min(progress + 0.05, 0.95);
                loading_progress.set(progress);
                
                if progress < 0.4 {
                    loading_message.set("Downloading model files...".to_string());
                } else if progress < 0.7 {
                    loading_message.set("Loading model weights...".to_string());
                } else {
                    loading_message.set("Initializing model...".to_string());
                }
            }
            
            // Check result
            match init_task.await {
                Ok(Ok(_)) => {
                    loading_progress.set(1.0);
                    loading_message.set("Model loaded successfully!".to_string());
                    // Small delay to show success state
                    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                    is_loading.set(false);
                }
                Ok(Err(e)) => {
                    let error_msg = format!("{}", e);
                    tracing::error!("Failed to initialize AI model: {}", error_msg);
                    loading_message.set("Failed to load model. Continuing without AI...".to_string());
                    loading_progress.set(1.0);
                    // Continue anyway after showing error
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    is_loading.set(false);
                }
                Err(e) => {
                    let error_msg = format!("{}", e);
                    tracing::error!("Task failed: {}", error_msg);
                    loading_message.set("Failed to load model. Continuing without AI...".to_string());
                    loading_progress.set(1.0);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    is_loading.set(false);
                }
            }
        });
    });
    
    // Popular dados de teste (async em background)
    use_effect(move || {
        let ctx = ctx_for_seeding.clone();
        spawn(async move {
            let _ = seed_all_data(
                &*ctx.essay_repo,
                &*ctx.question_repo,
                &*ctx.user_repo,
                &*ctx.trail_repo,
                &*ctx.rubric_repo,
            ).await;
        });
    });
    
    rsx! {
        style { {CSS} }
        
        if is_loading() {
            LoadingScreen {
                progress: loading_progress(),
                message: loading_message(),
            }
        } else {
            Router::<Route> {}
        }
    }
}
