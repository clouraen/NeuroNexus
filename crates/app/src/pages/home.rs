use dioxus::prelude::*;
use crate::components::*;
use crate::context::AppContext;
use domain::traits::{EssayRepository, QuestionRepository, KnowledgeTrailRepository};

#[component]
pub fn Home() -> Element {
    let ctx = use_context::<AppContext>();
    let mut stats = use_signal(|| (0u32, 0u32, 0u32));
    
    // Carregar estatísticas
    use_effect(move || {
        spawn(async move {
            let ctx = AppContext::new();
            let user_id = ctx.current_user_id;
            
            let essays_count = ctx.essay_repo.list_by_user(user_id).await.unwrap_or_default().len() as u32;
            let questions_count = ctx.question_repo.search("").await.unwrap_or_default().len() as u32;
            let trails_count = ctx.trail_repo.list_by_user(user_id).await.unwrap_or_default().len() as u32;
            
            stats.set((essays_count, questions_count, trails_count));
        });
    });
    
    rsx! {
        div {
            class: "app-container",
            BrainWatermark {}
            StatusBar {}
            main {
                class: "main-content",
                div {
                    class: "page-container",
                    h1 {
                        class: "page-title",
                        "{ctx.t(\"home-header-title\")}"
                    }
                    div {
                        class: "dashboard-grid",
                        CyberCard {
                            class: "stat-card".to_string(),
                            div {
                                class: "stat-value",
                                {stats().0.to_string()}
                            }
                            div {
                                class: "stat-label",
                                "{ctx.t(\"home-stats-essays\")}"
                            }
                        }
                        CyberCard {
                            class: "stat-card".to_string(),
                            div {
                                class: "stat-value",
                                {stats().1.to_string()}
                            }
                            div {
                                class: "stat-label",
                                "{ctx.t(\"home-stats-questions\")}"
                            }
                        }
                        CyberCard {
                            class: "stat-card".to_string(),
                            div {
                                class: "stat-value",
                                {stats().2.to_string()}
                            }
                            div {
                                class: "stat-label",
                                "Trilhas Ativas"
                            }
                        }
                    }
                }
            }
            TabBar {}
        }
    }
}
