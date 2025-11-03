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
    // Inicializar contexto e seeders
    use_effect(move || {
        spawn(async move {
            let ctx = AppContext::new();
            
            // Popular dados de teste (async em background)
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
