use dioxus::prelude::*;
use crate::context::AppContext;
use crate::components::{NeonInput, NeonButton, NeonProgressBar, ButtonVariant, TokenGuide, ModelStatusIndicator};
use services::{AIService, AIConfigManager, ModelStatus};

#[component]
pub fn AIConfigPanel() -> Element {
    let ctx = use_context::<AppContext>();
    
    // State management
    let mut token_input = use_signal(|| String::new());
    let mut token_configured = use_signal(|| false);
    let mut model_cached = use_signal(|| false);
    let mut model_status = use_signal(|| ModelStatus::NotConfigured);
    let mut loading_progress = use_signal(|| 0.0_f32);
    let mut loading_message = use_signal(|| String::new());
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut cache_info = use_signal(|| String::new());
    
    // Initialize state on mount
    use_effect(move || {
        spawn(async move {
            // Check if token is configured
            let ai_service = ctx.ai_service.as_ref();
            let is_configured = ai_service.is_token_configured();
            token_configured.set(is_configured);
            
            // Check if model is cached
            if let Ok(cached) = ai_service.check_model_cache() {
                model_cached.set(cached);
            }
            
            // Get cache info
            if let Ok(info) = ai_service.get_cache_info() {
                cache_info.set(format!("{}: {}", 
                    ctx.t("profile-ai-info-cache-location"), 
                    info.location
                ));
            }
            
            // Set initial status
            if is_configured && model_cached() {
                model_status.set(ModelStatus::Ready);
            } else if is_configured {
                model_status.set(ModelStatus::TokenSaved);
            } else {
                model_status.set(ModelStatus::NotConfigured);
            }
        });
    });
    
    // Handle token save
    let handle_save_token = move |_| {
        let token_value = token_input();
        
        // Validate token format
        if !AIService::validate_token_format(&token_value) {
            error_message.set(Some(ctx.t("profile-ai-error-invalid-format")));
            return;
        }
        
        // Save token
        spawn(async move {
            if let Ok(_) = ctx.ai_service.set_token(&token_value) {
                token_configured.set(true);
                model_status.set(ModelStatus::TokenSaved);
                error_message.set(None);
                token_input.set(String::new()); // Clear input for security
            } else {
                error_message.set(Some(ctx.t("profile-ai-error-invalid-token")));
            }
        });
    };
    
    // Handle model download
    let handle_download_model = move |_| {
        error_message.set(None);
        model_status.set(ModelStatus::Downloading);
        
        let ai_service = ctx.ai_service.clone();
        
        spawn(async move {
            // Create progress callback
            let progress_cb = {
                let loading_progress = loading_progress.clone();
                let loading_message = loading_message.clone();
                std::sync::Arc::new(move |progress: f32, message: String| {
                    loading_progress.set(progress);
                    loading_message.set(message);
                }) as services::ProgressCallback
            };
            
            // Initialize with progress
            match ai_service.initialize_with_progress(Some(progress_cb)).await {
                Ok(_) => {
                    model_status.set(ModelStatus::Ready);
                    model_cached.set(true);
                    loading_progress.set(1.0);
                    loading_message.set(ctx.t("profile-ai-progress-complete"));
                },
                Err(e) => {
                    model_status.set(ModelStatus::Error(e.to_string()));
                    error_message.set(Some(format!("{}: {}", 
                        ctx.t("profile-ai-error-unknown"), 
                        e.to_string()
                    )));
                }
            }
        });
    };
    
    // Handle clear cache
    let handle_clear_cache = move |_| {
        spawn(async move {
            if let Ok(_) = ctx.ai_service.clear_cache() {
                model_cached.set(false);
                model_status.set(if token_configured() { 
                    ModelStatus::TokenSaved 
                } else { 
                    ModelStatus::NotConfigured 
                });
                loading_progress.set(0.0);
                loading_message.set(String::new());
            }
        });
    };
    
    rsx! {
        div {
            class: "ai-config-panel",
            style: "padding: 20px;",
            
            // Title
            h2 {
                style: "
                    color: #ff00ff;
                    margin: 0 0 8px 0;
                    font-size: 1.5em;
                    text-shadow: 0 0 10px #ff00ff;
                ",
                {ctx.t("profile-ai-title")}
            }
            
            p {
                style: "color: #cccccc; margin: 0 0 24px 0;",
                {ctx.t("profile-ai-subtitle")}
            }
            
            // Status indicator
            div {
                style: "margin-bottom: 24px;",
                ModelStatusIndicator {
                    status: model_status()
                }
            }
            
            // Token input section
            div {
                class: "token-input-section",
                style: "margin-bottom: 24px;",
                
                label {
                    style: "
                        display: block;
                        color: #00ffff;
                        margin-bottom: 8px;
                        font-weight: bold;
                    ",
                    {ctx.t("profile-ai-token-label")}
                }
                
                div {
                    style: "display: flex; gap: 12px; align-items: flex-start;",
                    
                    div {
                        style: "flex: 1;",
                        NeonInput {
                            value: token_input(),
                            placeholder: ctx.t("profile-ai-token-placeholder"),
                            input_type: "password",
                            on_change: move |val| token_input.set(val)
                        }
                    }
                    
                    NeonButton {
                        text: ctx.t("profile-ai-token-save"),
                        variant: ButtonVariant::Primary,
                        on_click: handle_save_token,
                        disabled: token_input().is_empty()
                    }
                }
                
                if token_configured() {
                    p {
                        style: "color: #00ff00; margin-top: 8px; font-size: 0.9em;",
                        "✓ {ctx.t(\"profile-ai-status-configured\")}"
                    }
                }
            }
            
            // Error display
            if let Some(err) = error_message() {
                div {
                    style: "
                        padding: 12px;
                        background: rgba(255, 0, 0, 0.1);
                        border: 1px solid rgba(255, 0, 0, 0.3);
                        border-radius: 8px;
                        color: #ff0000;
                        margin-bottom: 16px;
                    ",
                    "⚠ {err}"
                }
            }
            
            // Token guide
            TokenGuide {}
            
            // Model download section
            div {
                class: "model-download-section",
                style: "margin-top: 32px;",
                
                h3 {
                    style: "
                        color: #00ffff;
                        margin: 0 0 16px 0;
                        font-size: 1.2em;
                    ",
                    "Model Management"
                }
                
                // Progress bar (shown when downloading/loading)
                if matches!(model_status(), ModelStatus::Downloading | ModelStatus::Loading) {
                    div {
                        style: "margin-bottom: 16px;",
                        
                        NeonProgressBar {
                            progress: loading_progress(),
                            label: Some(loading_message())
                        }
                    }
                }
                
                // Action buttons
                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap; margin-bottom: 16px;",
                    
                    // Download button
                    if !model_cached() && token_configured() {
                        NeonButton {
                            text: ctx.t("profile-ai-download-model"),
                            variant: ButtonVariant::Primary,
                            on_click: handle_download_model,
                            disabled: matches!(model_status(), ModelStatus::Downloading | ModelStatus::Loading)
                        }
                    }
                    
                    // Retry button (on error)
                    if matches!(model_status(), ModelStatus::Error(_)) {
                        NeonButton {
                            text: ctx.t("profile-ai-retry-download"),
                            variant: ButtonVariant::Primary,
                            on_click: handle_download_model,
                            disabled: false
                        }
                    }
                    
                    // Clear cache button
                    if model_cached() {
                        NeonButton {
                            text: ctx.t("profile-ai-clear-cache"),
                            variant: ButtonVariant::Secondary,
                            on_click: handle_clear_cache,
                            disabled: matches!(model_status(), ModelStatus::Downloading | ModelStatus::Loading)
                        }
                    }
                }
                
                // Info messages
                div {
                    style: "
                        padding: 12px;
                        background: rgba(0, 255, 255, 0.05);
                        border-radius: 8px;
                        border: 1px solid rgba(0, 255, 255, 0.2);
                    ",
                    
                    p {
                        style: "color: #00ffff; margin: 0 0 8px 0; font-size: 0.9em;",
                        "ℹ️ {ctx.t(\"profile-ai-info-model-size\")}"
                    }
                    
                    if !model_cached() {
                        p {
                            style: "color: #00ffff; margin: 0 0 8px 0; font-size: 0.9em;",
                            "ℹ️ {ctx.t(\"profile-ai-info-first-download\")}"
                        }
                    }
                    
                    p {
                        style: "color: #00ffff; margin: 0 0 8px 0; font-size: 0.9em;",
                        "ℹ️ {ctx.t(\"profile-ai-info-offline-ready\")}"
                    }
                    
                    p {
                        style: "color: #888888; margin: 0; font-size: 0.85em;",
                        {cache_info()}
                    }
                }
            }
        }
    }
}
