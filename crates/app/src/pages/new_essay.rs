use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::*;
use crate::context::AppContext;
use crate::app::Route;
use domain::essay::{ExamType, EssayStatus};
use domain::traits::EssayRepository;
use uuid::Uuid;
use chrono::Utc;

#[component]
pub fn NewEssay() -> Element {
    let mut title = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    let mut exam_type = use_signal(|| ExamType::Enem);
    let mut is_saving = use_signal(|| false);
    
    let handle_save = move |_| {
        let title_val = title().clone();
        let content_val = content().clone();
        let exam_type_val = exam_type().clone();
        
        if title_val.is_empty() || content_val.is_empty() {
            return;
        }
        
        is_saving.set(true);
        
        spawn(async move {
            let ctx = AppContext::new();
            let user_id = ctx.current_user_id;
            
            let new_essay = domain::essay::Essay {
                id: Uuid::new_v4(),
                user_id,
                title: title_val,
                content: content_val,
                exam_type: exam_type_val.clone(),
                status: EssayStatus::EmProgresso,
                score: None,
                max_score: exam_type_val.max_score(),
                feedback: None,
                corrections: None,
                rubric_scores: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                submitted_at: None,
            };
            
            let _ = ctx.essay_repo.save(new_essay).await;
            is_saving.set(false);
        });
    };
    
    rsx! {
        div {
            class: "page-container",
            h1 {
                class: "page-title",
                "NOVA REDAÇÃO"
            }
            div {
                class: "essay-editor",
                div {
                    class: "editor-header",
                    NeonInput {
                        placeholder: "Título da redação...".to_string(),
                        value: title().to_string(),
                        on_input: move |value: String| {
                            title.set(value);
                        },
                        class: "essay-title-input".to_string()
                    }
                    select {
                        class: "neon-select",
                        onchange: move |evt| {
                            let value = evt.value();
                            match value.as_str() {
                                "enem" => exam_type.set(ExamType::Enem),
                                "fuvest" => exam_type.set(ExamType::Fuvest),
                                "unicamp" => exam_type.set(ExamType::Unicamp),
                                "unesp" => exam_type.set(ExamType::Unesp),
                                _ => {}
                            }
                        },
                        option {
                            value: "enem",
                            selected: exam_type() == ExamType::Enem,
                            "ENEM"
                        }
                        option {
                            value: "fuvest",
                            selected: exam_type() == ExamType::Fuvest,
                            "FUVEST"
                        }
                        option {
                            value: "unicamp",
                            selected: exam_type() == ExamType::Unicamp,
                            "UNICAMP"
                        }
                        option {
                            value: "unesp",
                            selected: exam_type() == ExamType::Unesp,
                            "UNESP"
                        }
                    }
                }
                textarea {
                    class: "essay-content-textarea neon-input",
                    placeholder: "Digite sua redação aqui...",
                    rows: "20",
                    value: content().to_string(),
                    oninput: move |evt| {
                        content.set(evt.value());
                    },
                }
                div {
                    class: "editor-actions",
                    NeonButton {
                        variant: crate::components::neon_button::ButtonVariant::Primary,
                        on_click: handle_save,
                        if is_saving() {
                            "Salvando..."
                        } else {
                            "Salvar Rascunho"
                        }
                    }
                    Link {
                        to: Route::Essays {},
                        NeonButton {
                            variant: crate::components::neon_button::ButtonVariant::Secondary,
                            "Cancelar"
                        }
                    }
                }
            }
        }
    }
}
