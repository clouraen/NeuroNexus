use dioxus::prelude::*;
use dioxus_router::{Routable, Router};
use crate::pages::*;
use crate::theme::CSS;
use crate::context::AppContext;
use data::seed_all_data;

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
    
    // Popular dados de teste (async em background)
    use_effect(move || {
        let ctx = ctx.clone();
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
        Router::<Route> {}
    }
}
