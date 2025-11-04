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
                        // Botão de avaliação
                        if e.status != EssayStatus::Corrigida {
                            div {
                                class: "evaluation-actions",
                                button {
                                    class: "neon-button",
                                    disabled: is_evaluating(),
                                    onclick: evaluate_essay,
                                    if is_evaluating() {
                                        "Avaliando..."
                                    } else {
                                        "✨ Avaliar Redação com IA"
                                    }
                                }
                            }
                        }
                        // Mensagem de erro
                        if let Some(error) = evaluation_error() {
                            div {
                                class: "error-message",
                                style: "color: #ff4444; margin-top: 10px;",
                                {error}
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
                                "Feedback Geral:"
                            }
                            p {
                                style: "white-space: pre-wrap;",
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
                                    style: "margin: 15px 0; padding: 15px; background: rgba(0, 255, 255, 0.1); border-left: 3px solid #00ffff;",
                                    div {
                                        style: "display: flex; justify-content: space-between; margin-bottom: 10px;",
                                        span {
                                            class: "criterion-name",
                                            style: "font-weight: bold; color: #00ffff;",
                                            {format!("Competência {}:", criterion)}
                                        }
                                        span {
                                            class: "criterion-score",
                                            style: "font-weight: bold; color: #ff00ff; font-size: 1.2em;",
                                            {format!("{} pontos", score)}
                                        }
                                    }
                                    if let Some(detailed) = rubric.detailed_feedback.get(criterion) {
                                        p {
                                            style: "color: #cccccc; white-space: pre-wrap; margin-top: 5px;",
                                            {detailed.clone()}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(corrections) = &e.corrections {
                        if !corrections.is_empty() {
                            div {
                                class: "essay-corrections",
                                h3 {
                                    "Sugestões de Melhoria:"
                                }
                                for correction in corrections {
                                    div {
                                        class: "correction-item",
                                        style: "margin: 10px 0; padding: 10px; background: rgba(255, 100, 100, 0.1); border-left: 3px solid #ff6464;",
                                        p {
                                            style: "color: #ff6464; font-weight: bold;",
                                            {format!("Competência {}", correction.rubric_criterion)}
                                        }
                                        p {
                                            style: "color: #cccccc;",
                                            {correction.reason.clone()}
                                        }
                                        p {
                                            style: "color: #aaaaaa; font-style: italic; margin-top: 5px;",
                                            {correction.suggested_text.clone()}
                                        }
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
