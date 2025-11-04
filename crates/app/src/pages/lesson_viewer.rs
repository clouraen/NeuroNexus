use dioxus::prelude::*;
use dioxus_router::navigator;
use crate::components::*;
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
    
    // Load trail and module data
    use_effect(move || {
        let trail_id_clone = trail_id.clone();
        let module_id_clone = module_id.clone();
        
        spawn(async move {
            loading.set(true);
            
            let trail_uuid = match Uuid::parse_str(&trail_id_clone) {
                Ok(uuid) => uuid,
                Err(_) => {
                    loading.set(false);
                    return;
                }
            };
            
            let module_uuid = match Uuid::parse_str(&module_id_clone) {
                Ok(uuid) => uuid,
                Err(_) => {
                    loading.set(false);
                    return;
                }
            };
            
            // Load trail
            if let Ok(Some(t)) = ctx.trail_repo.find_by_id(trail_uuid).await {
                // Find the module
                if let Some(module) = t.modules.iter().find(|m| m.id == module_uuid) {
                    current_module.set(Some(module.clone()));
                    
                    // If it's a reading type, load the reading content
                    if module.content_type == ContentType::Reading {
                        if let Ok(Some(reading)) = ctx.reading_repo.find_by_id(module.content_id).await {
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
    let mark_complete = move |_| {
        let trail_id_clone = trail_id.clone();
        let module_id_clone = module_id.clone();
        
        spawn(async move {
            let trail_uuid = Uuid::parse_str(&trail_id_clone).unwrap();
            let module_uuid = Uuid::parse_str(&module_id_clone).unwrap();
            let user_id = ctx.current_user_id;
            
            let _ = ctx.trail_repo.mark_module_complete(trail_uuid, module_uuid, user_id).await;
            
            // Reload trail to get updated progress
            if let Ok(Some(t)) = ctx.trail_repo.find_by_id(trail_uuid).await {
                trail.set(Some(t.clone()));
                if let Some(module) = t.modules.iter().find(|m| m.id == module_uuid) {
                    current_module.set(Some(module.clone()));
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
                                to: Route::TrailDetail { trail_id: trail_id.clone() },
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
                                            trail_id: trail_id.clone(),
                                            module_id: prev.id.to_string()
                                        },
                                        NeonButton {
                                            label: "← Lição Anterior".to_string(),
                                            variant: "secondary".to_string(),
                                            onclick: move |_| {}
                                        }
                                    }
                                }
                                
                                if !module.completed {
                                    NeonButton {
                                        label: "Marcar como Completo".to_string(),
                                        variant: "primary".to_string(),
                                        onclick: mark_complete
                                    }
                                }
                                
                                if let Some(next) = next_module {
                                    Link {
                                        to: Route::LessonViewer {
                                            trail_id: trail_id.clone(),
                                            module_id: next.id.to_string()
                                        },
                                        NeonButton {
                                            label: "Próxima Lição →".to_string(),
                                            variant: if module.completed { "primary".to_string() } else { "secondary".to_string() },
                                            onclick: move |_| {}
                                        }
                                    }
                                } else {
                                    Link {
                                        to: Route::TrailDetail { trail_id: trail_id.clone() },
                                        NeonButton {
                                            label: "Voltar para Trilha".to_string(),
                                            variant: "secondary".to_string(),
                                            onclick: move |_| {}
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
