use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::*;
use crate::context::AppContext;
use crate::app::Route;
use domain::traits::KnowledgeTrailRepository;

#[component]
pub fn KnowledgeTrails() -> Element {
    let mut trails = use_signal(|| Vec::new());
    
    // Carregar trilhas na inicialização
    use_effect(move || {
        spawn(async move {
            let ctx = AppContext::new();
            let user_id = ctx.current_user_id;
            if let Ok(user_trails) = ctx.trail_repo.list_by_user(user_id).await {
                trails.set(user_trails);
            }
        });
    });
    
    rsx! {
        div {
            class: "app-container",
            StatusBar {}
            main {
                class: "main-content",
                div {
                    class: "page-container",
                    h1 {
                        class: "page-title",
                        "TRILHAS DE CONHECIMENTO"
                    }
                    div {
                        class: "trails-list",
                        if trails().is_empty() {
                            div {
                                class: "empty-state",
                                "Nenhuma trilha disponível ainda."
                            }
                        } else {
                            for trail in trails().iter() {
                                TrailCard {
                                    id: trail.id.to_string(),
                                    title: trail.title.clone(),
                                    description: trail.description.clone(),
                                    progress: trail.progress,
                                }
                            }
                        }
                    }
                }
            }
            TabBar {}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct TrailCardProps {
    id: String,
    title: String,
    description: String,
    progress: u8,
}

#[component]
fn TrailCard(props: TrailCardProps) -> Element {
    rsx! {
        CyberCard {
            class: "trail-card".to_string(),
            div {
                class: "trail-header",
                h3 {
                    class: "trail-title",
                    {props.title.clone()}
                }
                div {
                    class: "trail-progress",
                    span {
                        class: "progress-label",
                        "Progresso:"
                    }
                    span {
                        class: "progress-value",
                        {format!("{}%", props.progress)}
                    }
                }
            }
            div {
                class: "trail-description",
                {props.description.clone()}
            }
            NeonProgressBar {
                progress: props.progress,
                label: "".to_string()
            }
        }
    }
}

