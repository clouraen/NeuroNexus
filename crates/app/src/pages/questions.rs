use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::*;
use crate::context::AppContext;
use crate::app::Route;
use domain::traits::QuestionRepository;

#[component]
pub fn Questions() -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut questions = use_signal(|| Vec::new());
    
    // Carregar questões na inicialização
    use_effect(move || {
        spawn(async move {
            let ctx = AppContext::new();
            if let Ok(all_questions) = ctx.question_repo.search("").await {
                questions.set(all_questions);
            }
        });
    });
    
    // Filtrar questões baseado na busca
    let filtered_questions: Vec<_> = questions()
        .iter()
        .filter(|q| {
            search_query().is_empty() || 
            q.statement.to_lowercase().contains(&search_query().to_lowercase()) ||
            q.tags.iter().any(|tag| tag.to_lowercase().contains(&search_query().to_lowercase()))
        })
        .take(20)
        .cloned()
        .collect();
    
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
                        "QUESTÕES"
                    }
                    div {
                        class: "search-section",
                        NeonInput {
                            placeholder: "Buscar questões...".to_string(),
                            value: search_query().to_string(),
                            on_input: move |value: String| {
                                search_query.set(value);
                            },
                        }
                    }
                    div {
                        class: "questions-list",
                        if filtered_questions.is_empty() {
                            div {
                                class: "empty-state",
                                "Nenhuma questão encontrada"
                            }
                        } else {
                            for question in filtered_questions.iter() {
                                Link {
                                    to: Route::QuestionDetail { id: question.id.to_string() },
                                    QuestionCard {
                                        id: question.id.to_string(),
                                        subject: question.subject.display_name(),
                                        difficulty: question.difficulty.display_name(),
                                        preview: question.statement.chars().take(100).collect::<String>()
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
struct QuestionCardProps {
    id: String,
    subject: &'static str,
    difficulty: &'static str,
    preview: String,
}

#[component]
fn QuestionCard(props: QuestionCardProps) -> Element {
    rsx! {
        CyberCard {
            class: "question-card".to_string(),
            div {
                class: "question-header",
                span {
                    class: "question-subject",
                    {props.subject}
                }
                span {
                    class: "question-difficulty",
                    {props.difficulty}
                }
            }
            div {
                class: "question-preview",
                {format!("{}...", props.preview)}
            }
        }
    }
}
