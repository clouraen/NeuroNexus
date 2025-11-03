use dioxus::prelude::*;
use crate::components::*;
use crate::context::AppContext;
use domain::traits::QuestionRepository;
use uuid::Uuid;

#[component]
pub fn QuestionDetail(id: String) -> Element {
    let mut question = use_signal(|| None);
    let mut selected_answer = use_signal(|| None::<usize>);
    let mut show_explanation = use_signal(|| false);
    
    // Carregar questão
    use_effect(move || {
        let id_clone = id.clone();
        spawn(async move {
            let question_id = match Uuid::parse_str(&id_clone) {
                Ok(uuid) => uuid,
                Err(_) => return,
            };
            
            let ctx = AppContext::new();
            if let Ok(Some(q)) = ctx.question_repo.find_by_id(question_id).await {
                question.set(Some(q));
            }
        });
    });
    
    rsx! {
        div {
            class: "page-container",
            if let Some(q) = question() {
                div {
                    class: "question-detail",
                    div {
                        class: "question-header",
                        h1 {
                            class: "page-title",
                            {q.subject.display_name()}
                        }
                        span {
                            class: "question-difficulty",
                            {format!("Dificuldade: {}", q.difficulty.display_name())}
                        }
                    }
                    div {
                        class: "question-statement",
                        {q.statement}
                    }
                    div {
                        class: "alternatives-section",
                        h3 {
                            "Alternativas:"
                        }
                        for (idx, alt) in q.alternatives.iter().enumerate() {
                            div {
                                class: "alternative-item",
                                onclick: move |_| {
                                    selected_answer.set(Some(idx));
                                    show_explanation.set(true);
                                },
                                input {
                                    r#type: "radio",
                                    name: "answer",
                                    checked: selected_answer() == Some(idx),
                                }
                                label {
                                    {alt.text.clone()}
                                }
                            }
                        }
                    }
                    if show_explanation() {
                        div {
                            class: "explanation-section",
                            h3 {
                                "Explicação:"
                            }
                            p {
                                {q.explanation.clone()}
                            }
                            if selected_answer() == Some(q.correct_answer) {
                                div {
                                    class: "correct-answer",
                                    "✓ Resposta Correta!"
                                }
                            } else if selected_answer().is_some() {
                                div {
                                    class: "incorrect-answer",
                                    {format!("✗ Resposta Incorreta. A resposta correta é a alternativa {}.", q.correct_answer + 1)}
                                }
                            }
                        }
                    }
                    div {
                        class: "question-tags",
                        for tag in q.tags.iter() {
                            span {
                                class: "tag",
                                {tag.clone()}
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "loading",
                    "Carregando questão..."
                }
            }
        }
    }
}
