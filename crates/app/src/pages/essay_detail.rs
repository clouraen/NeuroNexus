use dioxus::prelude::*;
use crate::components::*;
use crate::context::AppContext;
use domain::traits::EssayRepository;
use uuid::Uuid;

#[component]
pub fn EssayDetail(id: String) -> Element {
    let mut essay = use_signal(|| None);
    
    // Carregar redação
    use_effect(move || {
        let id_clone = id.clone();
        spawn(async move {
            let essay_id = match Uuid::parse_str(&id_clone) {
                Ok(uuid) => uuid,
                Err(_) => return,
            };
            
            let ctx = AppContext::new();
            if let Ok(Some(e)) = ctx.essay_repo.find_by_id(essay_id).await {
                essay.set(Some(e));
            }
        });
    });
    
    rsx! {
        div {
            class: "page-container",
            if let Some(e) = essay() {
                div {
                    class: "essay-detail",
                    div {
                        class: "essay-header",
                        h1 {
                            class: "page-title",
                            {e.title.clone()}
                        }
                        div {
                            class: "essay-meta",
                            span {
                                class: "exam-type",
                                {e.exam_type.display_name()}
                            }
                            span {
                                class: "status",
                                {format!("{:?}", e.status)}
                            }
                            if let Some(score) = e.score {
                                span {
                                    class: "score",
                                    {format!("Score: {}/{}", score, e.max_score)}
                                }
                            }
                        }
                    }
                    div {
                        class: "essay-content",
                        h3 {
                            "Conteúdo:"
                        }
                        pre {
                            class: "essay-text",
                            {e.content.clone()}
                        }
                    }
                    if let Some(feedback) = &e.feedback {
                        div {
                            class: "essay-feedback",
                            h3 {
                                "Feedback:"
                            }
                            p {
                                {feedback.clone()}
                            }
                        }
                    }
                    if let Some(rubric) = &e.rubric_scores {
                        div {
                            class: "rubric-scores",
                            h3 {
                                "Notas por Competência:"
                            }
                            for (criterion, score) in rubric.scores.iter() {
                                div {
                                    class: "rubric-item",
                                    span {
                                        class: "criterion-name",
                                        {format!("{}:", criterion)}
                                    }
                                    span {
                                        class: "criterion-score",
                                        {score.to_string()}
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "loading",
                    "Carregando redação..."
                }
            }
        }
    }
}
