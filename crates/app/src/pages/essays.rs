use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::*;
use crate::context::AppContext;
use crate::app::Route;
use domain::traits::EssayRepository;

#[component]
pub fn Essays() -> Element {
    let mut essays = use_signal(|| Vec::new());
    
    // Carregar redações na inicialização
    use_effect(move || {
        spawn(async move {
            let ctx = AppContext::new();
            let user_id = ctx.current_user_id;
            if let Ok(user_essays) = ctx.essay_repo.list_by_user(user_id).await {
                essays.set(user_essays);
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
                    div {
                        class: "page-header",
                        h1 {
                            class: "page-title",
                            "REDAÇÕES"
                        }
                        Link {
                            to: Route::NewEssay {},
                            NeonButton {
                                class: "new-essay-button".to_string(),
                                variant: crate::components::neon_button::ButtonVariant::Primary,
                                "Nova Redação"
                            }
                        }
                    }
                    div {
                        class: "essays-list",
                        if essays().is_empty() {
                            div {
                                class: "empty-state",
                                "Nenhuma redação ainda. Crie sua primeira redação!"
                            }
                        } else {
                            for essay in essays().iter() {
                                if essay.score.is_some() {
                                    EssayCardWithScore {
                                        id: essay.id.to_string(),
                                        title: essay.title.clone(),
                                        exam_type: essay.exam_type.display_name(),
                                        status: match essay.status {
                                            domain::essay::EssayStatus::EmProgresso => "Em Progresso",
                                            domain::essay::EssayStatus::Corrigida => "Corrigida",
                                            domain::essay::EssayStatus::Enviada => "Enviada",
                                        },
                                        score: essay.score.unwrap()
                                    }
                                } else {
                                    EssayCard {
                                        id: essay.id.to_string(),
                                        title: essay.title.clone(),
                                        exam_type: essay.exam_type.display_name(),
                                        status: match essay.status {
                                            domain::essay::EssayStatus::EmProgresso => "Em Progresso",
                                            domain::essay::EssayStatus::Corrigida => "Corrigida",
                                            domain::essay::EssayStatus::Enviada => "Enviada",
                                        },
                                    }
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
struct EssayCardProps {
    id: String,
    title: String,
    exam_type: &'static str,
    status: &'static str,
}

#[component]
fn EssayCard(props: EssayCardProps) -> Element {
    rsx! {
        Link {
            to: Route::EssayDetail { id: props.id.clone() },
            CyberCard {
                class: "essay-card".to_string(),
                div {
                    class: "essay-header",
                    h3 {
                        class: "essay-title",
                        {props.title}
                    }
                    span {
                        class: "essay-exam-type",
                        {props.exam_type}
                    }
                }
                div {
                    class: "essay-info",
                    span {
                        class: "essay-status",
                        {props.status}
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct EssayCardWithScoreProps {
    id: String,
    title: String,
    exam_type: &'static str,
    status: &'static str,
    score: u16,
}

#[component]
fn EssayCardWithScore(props: EssayCardWithScoreProps) -> Element {
    let score_text = format!("Score: {}", props.score);
    
    rsx! {
        Link {
            to: Route::EssayDetail { id: props.id.clone() },
            CyberCard {
                class: "essay-card".to_string(),
                div {
                    class: "essay-header",
                    h3 {
                        class: "essay-title",
                        {props.title}
                    }
                    span {
                        class: "essay-exam-type",
                        {props.exam_type}
                    }
                }
                div {
                    class: "essay-info",
                    span {
                        class: "essay-status",
                        {props.status}
                    }
                    span {
                        class: "essay-score",
                        {score_text}
                    }
                }
            }
        }
    }
}
