use dioxus::prelude::*;
use services::ModelStatus;

#[derive(Props, Clone, PartialEq)]
pub struct ModelStatusIndicatorProps {
    pub status: ModelStatus,
}

#[component]
pub fn ModelStatusIndicator(props: ModelStatusIndicatorProps) -> Element {
    let (status_text, status_color, status_icon, should_pulse) = match &props.status {
        ModelStatus::NotConfigured => (
            "profile-ai-status-not-configured",
            "#666666",
            "⚠",
            false,
        ),
        ModelStatus::TokenSaved => (
            "profile-ai-status-configured",
            "#ffaa00",
            "✓",
            false,
        ),
        ModelStatus::Connecting => (
            "profile-ai-status-connected",
            "#00ffff",
            "⟳",
            true,
        ),
        ModelStatus::Downloading => (
            "profile-ai-status-downloading",
            "#ff00ff",
            "↓",
            true,
        ),
        ModelStatus::Loading => (
            "profile-ai-status-loading",
            "#00ffff",
            "⟳",
            true,
        ),
        ModelStatus::Ready => (
            "profile-ai-status-model-ready",
            "#00ff00",
            "✓",
            false,
        ),
        ModelStatus::Error(err) => (
            "profile-ai-status-error",
            "#ff0000",
            "✗",
            false,
        ),
    };

    let pulse_animation = if should_pulse {
        "animation: pulse 1.5s ease-in-out infinite;"
    } else {
        ""
    };

    rsx! {
        div {
            class: "model-status-indicator",
            style: "display: inline-flex; align-items: center; gap: 8px;",
            
            div {
                class: "status-icon",
                style: "
                    width: 24px;
                    height: 24px;
                    border-radius: 50%;
                    background: {status_color};
                    color: #0a0a0f;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-weight: bold;
                    box-shadow: 0 0 10px {status_color};
                    {pulse_animation}
                ",
                {status_icon}
            }
            
            span {
                style: "color: {status_color}; font-size: 0.9em;",
                // Translation key - will be translated by context
                {status_text}
            }
            
            if let ModelStatus::Error(err) = &props.status {
                div {
                    style: "margin-left: 8px; color: #ff0000; font-size: 0.85em;",
                    {err}
                }
            }
        }
        
        style {
            r#"
            @keyframes pulse {
                0%, 100% { opacity: 0.6; transform: scale(1); }
                50% { opacity: 1; transform: scale(1.1); }
            }
            "#
        }
    }
}
