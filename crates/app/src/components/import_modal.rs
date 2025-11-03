use dioxus::prelude::*;
use serde_json;
use uuid::Uuid;
use domain::{question::Question, knowledge_trail::KnowledgeTrail};
use crate::context::AppContext;
use domain::traits::{QuestionRepository, KnowledgeTrailRepository};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
pub enum ImportType {
    Questions,
    LearningTrails,
}

#[derive(Clone, PartialEq)]
pub enum ImportStatus {
    Idle,
    Processing,
    Success(String),
    Error(String),
}

#[derive(Props, Clone, PartialEq)]
pub struct ImportModalProps {
    pub show: bool,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn ImportModal(props: ImportModalProps) -> Element {
    let mut import_status = use_signal(|| ImportStatus::Idle);
    let mut selected_type = use_signal(|| None::<ImportType>);

    let handle_file_change = move |evt: Event<FormData>| {
        spawn(async move {
            import_status.set(ImportStatus::Processing);
            
            if let Some(file_engine) = evt.files() {
                if let Some(files) = file_engine.files() {
                    if let Some(file_name) = files.first() {
                        if let Some(content) = file_engine.read_file_to_string(file_name).await {
                            match selected_type() {
                                Some(ImportType::Questions) => {
                                    handle_questions_import(content, import_status).await;
                                }
                                Some(ImportType::LearningTrails) => {
                                    handle_trails_import(content, import_status).await;
                                }
                                None => {
                                    import_status.set(ImportStatus::Error("No import type selected".to_string()));
                                }
                            }
                        } else {
                            import_status.set(ImportStatus::Error("Failed to read file".to_string()));
                        }
                    }
                }
            }
        });
    };

    let close_modal = move |_| {
        import_status.set(ImportStatus::Idle);
        selected_type.set(None);
        props.on_close.call(());
    };

    if !props.show {
        return rsx! { div { style: "display: none;" } };
    }

    rsx! {
        div {
            class: "import-modal-overlay",
            onclick: close_modal,
            
            div {
                class: "import-modal-container",
                onclick: move |evt| evt.stop_propagation(),
                
                // Header
                div {
                    class: "import-modal-header",
                    h2 {
                        class: "import-modal-title",
                        "IMPORTAR DADOS"
                    }
                    button {
                        class: "import-modal-close",
                        onclick: close_modal,
                        "✕"
                    }
                }
                
                // Content
                div {
                    class: "import-modal-content",
                    
                    match import_status() {
                        ImportStatus::Idle => rsx! {
                            div {
                                class: "import-options",
                                p {
                                    class: "import-description",
                                    "Selecione o tipo de dados que deseja importar:"
                                }
                                
                                // Questions import
                                label {
                                    class: "import-option-button",
                                    r#for: "file-input-questions",
                                    div {
                                        class: "import-option-icon",
                                        "❓"
                                    }
                                    div {
                                        class: "import-option-text",
                                        h3 { "Questões de Exame" }
                                        p { "Importar questões no formato JSON" }
                                    }
                                    input {
                                        r#type: "file",
                                        id: "file-input-questions",
                                        accept: ".json",
                                        style: "display: none;",
                                        onchange: move |evt| {
                                            selected_type.set(Some(ImportType::Questions));
                                            handle_file_change(evt);
                                        }
                                    }
                                }
                                
                                // Learning trails import
                                label {
                                    class: "import-option-button",
                                    r#for: "file-input-trails",
                                    div {
                                        class: "import-option-icon",
                                        "🗺️"
                                    }
                                    div {
                                        class: "import-option-text",
                                        h3 { "Trilhas de Aprendizado" }
                                        p { "Importar trilhas no formato JSON" }
                                    }
                                    input {
                                        r#type: "file",
                                        id: "file-input-trails",
                                        accept: ".json",
                                        style: "display: none;",
                                        onchange: move |evt| {
                                            selected_type.set(Some(ImportType::LearningTrails));
                                            handle_file_change(evt);
                                        }
                                    }
                                }
                            }
                        },
                        ImportStatus::Processing => rsx! {
                            div {
                                class: "import-processing",
                                div {
                                    class: "import-spinner"
                                }
                                p { "Importando dados..." }
                            }
                        },
                        ImportStatus::Success(ref message) => rsx! {
                            div {
                                class: "import-feedback import-success",
                                div {
                                    class: "import-icon",
                                    "✓"
                                }
                                p { "{message}" }
                            }
                        },
                        ImportStatus::Error(ref message) => rsx! {
                            div {
                                class: "import-feedback import-error",
                                div {
                                    class: "import-icon",
                                    "✕"
                                }
                                p { "{message}" }
                            }
                        },
                    }
                }
            }
        }
    }
}

async fn handle_questions_import(json_text: String, mut status: Signal<ImportStatus>) {
    match serde_json::from_str::<Vec<Question>>(&json_text) {
        Ok(mut questions) => {
            let ctx = AppContext::new();
            let mut success_count = 0;
            let mut error_count = 0;
            
            for question in questions.iter_mut() {
                // Generate UUID if missing
                if question.id == Uuid::nil() {
                    question.id = Uuid::new_v4();
                }
                
                // Validate question
                if validate_question(question) {
                    match ctx.question_repo.insert(question.clone()).await {
                        Ok(_) => success_count += 1,
                        Err(_) => error_count += 1,
                    }
                } else {
                    error_count += 1;
                }
            }
            
            if error_count == 0 {
                status.set(ImportStatus::Success(
                    format!("Successfully imported {} questions", success_count)
                ));
            } else {
                status.set(ImportStatus::Success(
                    format!("Imported {} of {} items. {} errors encountered.", 
                        success_count, success_count + error_count, error_count)
                ));
            }
        }
        Err(e) => {
            status.set(ImportStatus::Error(
                format!("Invalid JSON format: {}", e)
            ));
        }
    }
}

async fn handle_trails_import(json_text: String, mut status: Signal<ImportStatus>) {
    match serde_json::from_str::<Vec<KnowledgeTrail>>(&json_text) {
        Ok(mut trails) => {
            let ctx = AppContext::new();
            let mut success_count = 0;
            let mut error_count = 0;
            
            for trail in trails.iter_mut() {
                // Generate UUID if missing
                if trail.id == Uuid::nil() {
                    trail.id = Uuid::new_v4();
                }
                
                // Generate UUIDs for modules if missing
                for module in trail.modules.iter_mut() {
                    if module.id == Uuid::nil() {
                        module.id = Uuid::new_v4();
                    }
                }
                
                // Validate trail
                if validate_trail(trail) {
                    match ctx.trail_repo.insert(trail.clone()).await {
                        Ok(_) => success_count += 1,
                        Err(_) => error_count += 1,
                    }
                } else {
                    error_count += 1;
                }
            }
            
            if error_count == 0 {
                status.set(ImportStatus::Success(
                    format!("Successfully imported {} learning trails", success_count)
                ));
            } else {
                status.set(ImportStatus::Success(
                    format!("Imported {} of {} items. {} errors encountered.", 
                        success_count, success_count + error_count, error_count)
                ));
            }
        }
        Err(e) => {
            status.set(ImportStatus::Error(
                format!("Invalid JSON format: {}", e)
            ));
        }
    }
}

fn validate_question(question: &Question) -> bool {
    // Validate statement is not empty
    if question.statement.trim().is_empty() {
        return false;
    }
    
    // Validate alternatives count (2-5)
    if question.alternatives.len() < 2 || question.alternatives.len() > 5 {
        return false;
    }
    
    // Validate correct_answer is valid index
    if question.correct_answer >= question.alternatives.len() {
        return false;
    }
    
    // Validate explanation is not empty
    if question.explanation.trim().is_empty() {
        return false;
    }
    
    true
}

fn validate_trail(trail: &KnowledgeTrail) -> bool {
    // Validate title is not empty
    if trail.title.trim().is_empty() {
        return false;
    }
    
    // Validate description is not empty
    if trail.description.trim().is_empty() {
        return false;
    }
    
    // Validate focus_areas is not empty
    if trail.focus_areas.is_empty() {
        return false;
    }
    
    // Validate progress is 0-100
    if trail.progress > 100 {
        return false;
    }
    
    // Validate modules is not empty
    if trail.modules.is_empty() {
        return false;
    }
    
    // Validate estimated_hours is positive
    if trail.estimated_hours == 0 {
        return false;
    }
    
    true
}
use dioxus::prelude::*;
use serde_json;
use uuid::Uuid;
use domain::{question::Question, knowledge_trail::KnowledgeTrail};
use crate::context::AppContext;
use domain::traits::{QuestionRepository, KnowledgeTrailRepository};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
pub enum ImportType {
    Questions,
    LearningTrails,
}

#[derive(Clone, PartialEq)]
pub enum ImportStatus {
    Idle,
    Processing,
    Success(String),
    Error(String),
}

#[derive(Props, Clone, PartialEq)]
pub struct ImportModalProps {
    pub show: bool,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn ImportModal(props: ImportModalProps) -> Element {
    let mut import_status = use_signal(|| ImportStatus::Idle);
    let mut selected_type = use_signal(|| None::<ImportType>);

    let handle_file_change = move |evt: Event<FormData>| {
        spawn(async move {
            import_status.set(ImportStatus::Processing);
            
            if let Some(file_engine) = evt.files() {
                if let Some(files) = file_engine.files() {
                    if let Some(file_name) = files.first() {
                        if let Some(content) = file_engine.read_file_to_string(file_name).await {
                            match selected_type() {
                                Some(ImportType::Questions) => {
                                    handle_questions_import(content, import_status).await;
                                }
                                Some(ImportType::LearningTrails) => {
                                    handle_trails_import(content, import_status).await;
                                }
                                None => {
                                    import_status.set(ImportStatus::Error("No import type selected".to_string()));
                                }
                            }
                        } else {
                            import_status.set(ImportStatus::Error("Failed to read file".to_string()));
                        }
                    }
                }
            }
        });
    };

    let close_modal = move |_| {
        import_status.set(ImportStatus::Idle);
        selected_type.set(None);
        props.on_close.call(());
    };

    if !props.show {
        return rsx! { div { style: "display: none;" } };
    }

    rsx! {
        div {
            class: "import-modal-overlay",
            onclick: close_modal,
            
            div {
                class: "import-modal-container",
                onclick: move |evt| evt.stop_propagation(),
                
                // Header
                div {
                    class: "import-modal-header",
                    h2 {
                        class: "import-modal-title",
                        "IMPORTAR DADOS"
                    }
                    button {
                        class: "import-modal-close",
                        onclick: close_modal,
                        "✕"
                    }
                }
                
                // Content
                div {
                    class: "import-modal-content",
                    
                    match import_status() {
                        ImportStatus::Idle => rsx! {
                            div {
                                class: "import-options",
                                p {
                                    class: "import-description",
                                    "Selecione o tipo de dados que deseja importar:"
                                }
                                
                                // Questions import
                                label {
                                    class: "import-option-button",
                                    r#for: "file-input-questions",
                                    div {
                                        class: "import-option-icon",
                                        "❓"
                                    }
                                    div {
                                        class: "import-option-text",
                                        h3 { "Questões de Exame" }
                                        p { "Importar questões no formato JSON" }
                                    }
                                    input {
                                        r#type: "file",
                                        id: "file-input-questions",
                                        accept: ".json",
                                        style: "display: none;",
                                        onchange: move |evt| {
                                            selected_type.set(Some(ImportType::Questions));
                                            handle_file_change(evt);
                                        }
                                    }
                                }
                                
                                // Learning trails import
                                label {
                                    class: "import-option-button",
                                    r#for: "file-input-trails",
                                    div {
                                        class: "import-option-icon",
                                        "🗺️"
                                    }
                                    div {
                                        class: "import-option-text",
                                        h3 { "Trilhas de Aprendizado" }
                                        p { "Importar trilhas no formato JSON" }
                                    }
                                    input {
                                        r#type: "file",
                                        id: "file-input-trails",
                                        accept: ".json",
                                        style: "display: none;",
                                        onchange: move |evt| {
                                            selected_type.set(Some(ImportType::LearningTrails));
                                            handle_file_change(evt);
                                        }
                                    }
                                }
                            }
                        },
                        ImportStatus::Processing => rsx! {
                            div {
                                class: "import-processing",
                                div {
                                    class: "import-spinner"
                                }
                                p { "Importando dados..." }
                            }
                        },
                        ImportStatus::Success(ref message) => rsx! {
                            div {
                                class: "import-feedback import-success",
                                div {
                                    class: "import-icon",
                                    "✓"
                                }
                                p { "{message}" }
                            }
                        },
                        ImportStatus::Error(ref message) => rsx! {
                            div {
                                class: "import-feedback import-error",
                                div {
                                    class: "import-icon",
                                    "✕"
                                }
                                p { "{message}" }
                            }
                        },
                    }
                }
            }
        }
    }
}

async fn handle_questions_import(json_text: String, mut status: Signal<ImportStatus>) {
    match serde_json::from_str::<Vec<Question>>(&json_text) {
        Ok(mut questions) => {
            let ctx = AppContext::new();
            let mut success_count = 0;
            let mut error_count = 0;
            
            for question in questions.iter_mut() {
                // Generate UUID if missing
                if question.id == Uuid::nil() {
                    question.id = Uuid::new_v4();
                }
                
                // Validate question
                if validate_question(question) {
                    match ctx.question_repo.insert(question.clone()).await {
                        Ok(_) => success_count += 1,
                        Err(_) => error_count += 1,
                    }
                } else {
                    error_count += 1;
                }
            }
            
            if error_count == 0 {
                status.set(ImportStatus::Success(
                    format!("Successfully imported {} questions", success_count)
                ));
            } else {
                status.set(ImportStatus::Success(
                    format!("Imported {} of {} items. {} errors encountered.", 
                        success_count, success_count + error_count, error_count)
                ));
            }
        }
        Err(e) => {
            status.set(ImportStatus::Error(
                format!("Invalid JSON format: {}", e)
            ));
        }
    }
}

async fn handle_trails_import(json_text: String, mut status: Signal<ImportStatus>) {
    match serde_json::from_str::<Vec<KnowledgeTrail>>(&json_text) {
        Ok(mut trails) => {
            let ctx = AppContext::new();
            let mut success_count = 0;
            let mut error_count = 0;
            
            for trail in trails.iter_mut() {
                // Generate UUID if missing
                if trail.id == Uuid::nil() {
                    trail.id = Uuid::new_v4();
                }
                
                // Generate UUIDs for modules if missing
                for module in trail.modules.iter_mut() {
                    if module.id == Uuid::nil() {
                        module.id = Uuid::new_v4();
                    }
                }
                
                // Validate trail
                if validate_trail(trail) {
                    match ctx.trail_repo.insert(trail.clone()).await {
                        Ok(_) => success_count += 1,
                        Err(_) => error_count += 1,
                    }
                } else {
                    error_count += 1;
                }
            }
            
            if error_count == 0 {
                status.set(ImportStatus::Success(
                    format!("Successfully imported {} learning trails", success_count)
                ));
            } else {
                status.set(ImportStatus::Success(
                    format!("Imported {} of {} items. {} errors encountered.", 
                        success_count, success_count + error_count, error_count)
                ));
            }
        }
        Err(e) => {
            status.set(ImportStatus::Error(
                format!("Invalid JSON format: {}", e)
            ));
        }
    }
}

fn validate_question(question: &Question) -> bool {
    // Validate statement is not empty
    if question.statement.trim().is_empty() {
        return false;
    }
    
    // Validate alternatives count (2-5)
    if question.alternatives.len() < 2 || question.alternatives.len() > 5 {
        return false;
    }
    
    // Validate correct_answer is valid index
    if question.correct_answer >= question.alternatives.len() {
        return false;
    }
    
    // Validate explanation is not empty
    if question.explanation.trim().is_empty() {
        return false;
    }
    
    true
}

fn validate_trail(trail: &KnowledgeTrail) -> bool {
    // Validate title is not empty
    if trail.title.trim().is_empty() {
        return false;
    }
    
    // Validate description is not empty
    if trail.description.trim().is_empty() {
        return false;
    }
    
    // Validate focus_areas is not empty
    if trail.focus_areas.is_empty() {
        return false;
    }
    
    // Validate progress is 0-100
    if trail.progress > 100 {
        return false;
    }
    
    // Validate modules is not empty
    if trail.modules.is_empty() {
        return false;
    }
    
    // Validate estimated_hours is positive
    if trail.estimated_hours == 0 {
        return false;
    }
    
    true
}
