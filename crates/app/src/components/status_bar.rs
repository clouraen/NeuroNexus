use dioxus::prelude::*;
use dioxus::prelude::*;
use super::BrainIcon;
use crate::context::AppContext;

#[component]
pub fn StatusBar() -> Element {
    let ctx = use_context::<AppContext>();
    let timer_seconds = use_signal(|| 0u32);
    let is_active = use_signal(|| false);
    
    let format_time = |seconds: u32| -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    };
    
    let toggle_timer = move |_| {
        // Timer functionality to be implemented with platform-specific async
        // For now, just toggle the visual state
    };
    
    let reset_timer = move |_: Event<MouseData>| {
        // Reset functionality placeholder
    };
    
    let timer_class = if is_active() {
        "status-time active"
    } else {
        "status-time"
    };
    
    rsx! {
        div {
            class: "status-bar",
            
            // Left section: Logo + Brain Icon
            div {
                class: "status-bar-left",
                BrainIcon {
                    size: "1.5rem".to_string(),
                    class: "brain-icon".to_string(),
                }
                div {
                    class: "status-logo",
                    span {
                        class: "logo-icon",
                        "⚡"
                    }
                    span {
                        class: "logo-text",
                        "NEURONEXUS"
                    }
                }
            }
            
            // Center section: Timer
            div {
                class: "status-bar-center",
                span {
                    class: "{timer_class}",
                    onclick: toggle_timer,
                    oncontextmenu: reset_timer,
                    title: "{ctx.t(\"status-bar-timer-tooltip\")}",
                    "{format_time(timer_seconds())}"
                }
            }
            
            // Right section: Status + Version
            div {
                class: "status-bar-right",
                div {
                    class: "status-info",
                    span {
                        class: "info-item online-status",
                        "● ONLINE"
                    }
                    span {
                        class: "info-separator",
                        "|"
                    }
                    span {
                        class: "info-item version-text",
                        "v0.1.0"
                    }
                }
            }
        }
    }
}
