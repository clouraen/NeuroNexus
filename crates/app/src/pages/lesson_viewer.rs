use dioxus::prelude::*;
use dioxus_router::{navigator, components::Link};
use crate::components::*;
use crate::components::neon_button::ButtonVariant;
use crate::context::AppContext;
use crate::app::Route;
use domain::traits::{KnowledgeTrailRepository, ReadingContentRepository};
use domain::knowledge_trail::{ContentType, TrailModule};
use domain::reading_content::ReadingContent;
use uuid::Uuid;

#[component]
pub fn LessonViewer(trail_id: String, module_id: String) -> Element {
    let mut trail = use_signal(|| None);
    let mut current_module = use_signal(|| None::<TrailModule>);
    let mut reading_content = use_signal(|| None::<ReadingContent>);
    let mut loading = use_signal(|| true);
    let ctx = use_context::<AppContext>();
    let nav = navigator();
    let trail_repo = ctx.trail_repo.clone();
    let reading_repo = ctx.reading_repo.clone();
    let user_id = ctx.current_user_id;
    
    // Clone for use_effect
    let trail_id_for_effect = trail_id.clone();
    let module_id_for_effect = module_id.clone();
    let trail_repo_for_effect = trail_repo.clone();
    let reading_repo_for_effect = reading_repo.clone();
    
    // Clone for mark_complete
    let trail_id_for_mark = trail_id.clone();
    let module_id_for_mark = module_id.clone();
    let trail_repo_for_mark = trail_repo.clone();
    
    // Clone for view
    let trail_id_for_view = trail_id.clone();
    
    // Load trail and module data
    use_effect(move || {
        let trail_id_str = trail_id_for_effect.clone();
        let module_id_str = module_id_for_effect.clone();
        let t_repo = trail_repo_for_effect.clone();
        let r_repo = reading_repo_for_effect.clone();
        
        spawn(async move {
            loading.set(true);
            
            let trail_uuid = match Uuid::parse_str(&trail_id_str) {
                Ok(uuid) => uuid,
                Err(_) => {
                    loading.set(false);
                    return;
                }
            };
            
            let module_uuid = match Uuid::parse_str(&module_id_str) {
                Ok(uuid) => uuid,
                Err(_) => {
                    loading.set(false);
                    return;
                }
            };
            
            // Load trail
            if let Ok(Some(t)) = t_repo.find_by_id(trail_uuid).await {
                // Find the module
                if let Some(module) = t.modules.iter().find(|m| m.id == module_uuid) {
                    current_module.set(Some(module.clone()));
                    
                    // If it's a reading type, load the reading content
                    if module.content_type == ContentType::Reading {
                        if let Ok(Some(reading)) = r_repo.find_by_id(module.content_id).await {
                            reading_content.set(Some(reading));
                        }
                    } else if module.content_type == ContentType::Question {
                        // Navigate to question detail page
                        nav.push(Route::QuestionDetail {
                            id: module.content_id.to_string()
                        });
                        return;
                    }
                }
                trail.set(Some(t));
            }
            
            loading.set(false);
        });
    });
    
    // Mark lesson as complete
    let mut trail_mut = trail.clone();
    let mut current_module_mut = current_module.clone();
    let mark_complete = move |_| {
        let trail_repo = trail_repo_for_mark.clone();
        let trail_id_str = trail_id_for_mark.clone();
        let module_id_str = module_id_for_mark.clone();
        
        spawn(async move {
            let trail_uuid = Uuid::parse_str(&trail_id_str).unwrap();
            let module_uuid = Uuid::parse_str(&module_id_str).unwrap();
            
            let _ = trail_repo.mark_module_complete(trail_uuid, module_uuid, user_id).await;
            
            // Reload trail to get updated progress
            if let Ok(Some(t)) = trail_repo.find_by_id(trail_uuid).await {
                trail_mut.set(Some(t.clone()));
                if let Some(module) = t.modules.iter().find(|m| m.id == module_uuid) {
                    current_module_mut.set(Some(module.clone()));
                }
            }
        });
    };
    
    // Get next and previous modules
    let (next_module, prev_module) = if let (Some(t), Some(curr)) = (trail(), current_module()) {
        let curr_order = curr.order;
        let next = t.modules.iter().find(|m| m.order == curr_order + 1);
        let prev = if curr_order > 0 {
            t.modules.iter().find(|m| m.order == curr_order - 1)
        } else {
            None
        };
        (next.cloned(), prev.cloned())
    } else {
        (None, None)
    };
    
    rsx! {
        div {
            class: "app-container",
            StatusBar {}
            main {
                class: "main-content",
                if loading() {
                    div {
                        class: "loading",
                        "Carregando lição..."
                    }
                } else if let Some(module) = current_module() {
                    div {
                        class: "lesson-viewer-container",
                        // Top Navigation
                        div {
                            class: "lesson-nav-top",
                            Link {
                                to: Route::TrailDetail { trail_id: trail_id_for_view.clone() },
                                class: "back-to-trail",
                                "← Voltar para Trilha"
                            }
                            if module.completed {
                                div {
                                    class: "completed-badge",
                                    "✓ Completo"
                                }
                            }
                        }
                        
                        // Content Display
                        if module.content_type == ContentType::Reading {
                            if let Some(reading) = reading_content() {
                                ReadingContentViewer {
                                    content: reading,
                                    module: module.clone()
                                }
                            }
                        }
                        
                        // Bottom Navigation
                        div {
                            class: "lesson-nav-bottom",
                            div {
                                class: "lesson-controls",
                                if let Some(prev) = prev_module {
                                    Link {
                                        to: Route::LessonViewer {
                                            trail_id: trail_id_for_view.clone(),
                                            module_id: prev.id.to_string()
                                        },
                                        NeonButton {
                                            variant: ButtonVariant::Secondary,
                                            "← Lição Anterior"
                                        }
                                    }
                                }
                                
                                if !module.completed {
                                    NeonButton {
                                        variant: ButtonVariant::Primary,
                                        on_click: mark_complete,
                                        "Marcar como Completo"
                                    }
                                }
                                
                                if let Some(next) = next_module {
                                    Link {
                                        to: Route::LessonViewer {
                                            trail_id: trail_id_for_view.clone(),
                                            module_id: next.id.to_string()
                                        },
                                        NeonButton {
                                            variant: if module.completed { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                            "Próxima Lição →"
                                        }
                                    }
                                } else {
                                    Link {
                                        to: Route::TrailDetail { trail_id: trail_id_for_view.clone() },
                                        NeonButton {
                                            variant: ButtonVariant::Secondary,
                                            "Voltar para Trilha"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "error-state",
                        "Lição não encontrada."
                    }
                }
            }
            TabBar {}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ReadingContentViewerProps {
    content: ReadingContent,
    module: TrailModule,
}

#[component]
fn ReadingContentViewer(props: ReadingContentViewerProps) -> Element {
    rsx! {
        div {
            class: "reading-content-viewer",
            div {
                class: "reading-header",
                h1 {
                    class: "reading-title",
                    {props.content.title.clone()}
                }
                div {
                    class: "reading-meta",
                    span {
                        class: "reading-time",
                        {format!("⏱️ {} minutos de leitura", props.content.estimated_reading_time)}
                    }
                    if let Some(author) = &props.content.author {
                        span {
                            class: "reading-author",
                            {format!("✍️ {}", author)}
                        }
                    }
                }
            }
            div {
                class: "reading-content",
                dangerous_inner_html: "{props.content.content}"
            }
            if !props.content.references.is_empty() {
                div {
                    class: "reading-references",
                    h3 { "Referências:" }
                    ul {
                        for reference in props.content.references.iter() {
                            li { {reference.clone()} }
                        }
                    }
                }
            }
        }
    }
}
