use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::*;
use crate::context::AppContext;
use crate::app::Route;
use domain::traits::KnowledgeTrailRepository;
use domain::knowledge_trail::{ContentType, TrailModule};
use uuid::Uuid;

#[component]
pub fn TrailDetail(trail_id: String) -> Element {
    let mut trail = use_signal(|| None);
    let ctx = use_context::<AppContext>();
    let trail_id_for_view = trail_id.clone();
    
    // Load trail data
    use_effect(move || {
        let trail_id_str = trail_id.clone();
        let trail_repo = ctx.trail_repo.clone();
        
        spawn(async move {
            let trail_uuid = match Uuid::parse_str(&trail_id_str) {
                Ok(uuid) => uuid,
                Err(_) => return,
            };
            
            if let Ok(Some(t)) = trail_repo.find_by_id(trail_uuid).await {
                trail.set(Some(t));
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
                    class: "trail-detail-container",
                    if let Some(t) = trail() {
                        // Trail Header
                        div {
                            class: "trail-detail-header",
                            Link {
                                to: Route::KnowledgeTrails {},
                                class: "back-button",
                                "← Voltar para Trilhas"
                            }
                            h1 {
                                class: "trail-detail-title",
                                {t.title.clone()}
                            }
                            p {
                                class: "trail-detail-description",
                                {t.description.clone()}
                            }
                            div {
                                class: "trail-detail-meta",
                                div {
                                    class: "trail-meta-item",
                                    span { class: "meta-label", "Progresso:" }
                                    span { class: "meta-value", {format!("{}%", t.progress)} }
                                }
                                div {
                                    class: "trail-meta-item",
                                    span { class: "meta-label", "Dificuldade:" }
                                    span { class: "meta-value", {t.difficulty_level.display_name()} }
                                }
                                div {
                                    class: "trail-meta-item",
                                    span { class: "meta-label", "Horas estimadas:" }
                                    span { class: "meta-value", {format!("{} horas", t.estimated_hours)} }
                                }
                            }
                            NeonProgressBar {
                                progress: t.progress,
                                label: format!("{}% completo", t.progress)
                            }
                        }
                        
                        // Lessons List
                        div {
                            class: "lessons-container",
                            h2 {
                                class: "lessons-title",
                                "Lições"
                            }
                            div {
                                class: "lessons-list",
                                for module in t.modules.iter() {
                                    LessonItem {
                                        trail_id: trail_id_for_view.clone(),
                                        module: module.clone()
                                    }
                                }
                            }
                        }
                    } else {
                        div {
                            class: "loading",
                            "Carregando trilha..."
                        }
                    }
                }
            }
            TabBar {}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct LessonItemProps {
    trail_id: String,
    module: TrailModule,
}

#[component]
fn LessonItem(props: LessonItemProps) -> Element {
    let content_icon = match props.module.content_type {
        ContentType::Reading => "📖",
        ContentType::Video => "▶️",
        ContentType::Question => "❓",
        ContentType::Essay => "✍️",
        ContentType::PracticeTest => "📝",
    };
    
    let content_color = match props.module.content_type {
        ContentType::Reading => "content-reading",
        ContentType::Video => "content-video",
        ContentType::Question => "content-question",
        ContentType::Essay => "content-essay",
        ContentType::PracticeTest => "content-test",
    };
    
    rsx! {
        Link {
            to: Route::LessonViewer {
                trail_id: props.trail_id.clone(),
                module_id: props.module.id.to_string()
            },
            class: "lesson-item-link",
            div {
                class: format!("lesson-item {}", if props.module.completed { "completed" } else { "" }),
                div {
                    class: "lesson-order",
                    {format!("{}", props.module.order + 1)}
                }
                div {
                    class: format!("lesson-icon {}", content_color),
                    {content_icon}
                }
                div {
                    class: "lesson-info",
                    h3 {
                        class: "lesson-title",
                        {props.module.title.clone()}
                    }
                    p {
                        class: "lesson-description",
                        {props.module.description.clone()}
                    }
                }
                if props.module.completed {
                    div {
                        class: "lesson-status",
                        span {
                            class: "completion-badge",
                            "✓ Completo"
                        }
                    }
                }
            }
        }
    }
}
