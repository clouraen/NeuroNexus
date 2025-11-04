use dioxus::prelude::*;
use crate::components::BrainWatermark;

#[derive(Props, Clone, PartialEq)]
pub struct LoadingScreenProps {
    pub progress: f32,
    pub message: String,
}

#[component]
pub fn LoadingScreen(props: LoadingScreenProps) -> Element {
    let progress_percent = (props.progress * 100.0).round() as u32;
    
    rsx! {
        div {
            class: "loading-screen-overlay",
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: #0a0a0f; display: flex; align-items: center; justify-content: center; z-index: 9999;",
            
            div {
                class: "loading-content",
                style: "text-align: center; max-width: 500px; padding: 40px;",
                
                // Brain watermark
                div {
                    style: "margin-bottom: 40px;",
                    BrainWatermark {}
                }
                
                // Title
                h1 {
                    style: "color: #00ffff; font-size: 2em; margin-bottom: 20px; text-shadow: 0 0 10px #00ffff;",
                    "NeuroNexus"
                }
                
                // Loading message
                p {
                    style: "color: #ffffff; font-size: 1.1em; margin-bottom: 30px; min-height: 30px;",
                    {props.message.clone()}
                }
                
                // Progress bar container
                div {
                    class: "progress-bar-container",
                    style: "width: 100%; height: 8px; background: rgba(0, 255, 255, 0.2); border-radius: 4px; overflow: hidden; margin-bottom: 10px; box-shadow: inset 0 0 10px rgba(0, 255, 255, 0.3);",
                    
                    // Progress bar fill
                    div {
                        class: "progress-bar-fill",
                        style: "height: 100%; background: linear-gradient(90deg, #00ffff, #ff00ff); width: {progress_percent}%; transition: width 0.3s ease; box-shadow: 0 0 10px #00ffff;",
                    }
                }
                
                // Progress percentage
                p {
                    style: "color: #00ffff; font-size: 0.9em; font-family: monospace;",
                    {format!("{}%", progress_percent)}
                }
                
                // Animated dots
                if props.progress < 1.0 {
                    div {
                        class: "loading-dots",
                        style: "margin-top: 20px; color: #ff00ff; font-size: 1.5em;",
                        "●●●"
                    }
                }
                
                // Success checkmark
                if props.progress >= 1.0 {
                    div {
                        style: "margin-top: 20px; color: #00ff00; font-size: 3em;",
                        "✓"
                    }
                }
            }
        }
        
        style {
            r#"
            @keyframes pulse {{
                0%, 100% {{ opacity: 0.4; }}
                50% {{ opacity: 1; }}
            }}
            
            .loading-dots {{
                animation: pulse 1.5s ease-in-out infinite;
            }}
            "#
        }
    }
}
