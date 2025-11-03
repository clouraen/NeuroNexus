use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NeonProgressBarProps {
    progress: u8, // 0-100
    #[props(default)]
    label: String,
}

#[component]
pub fn NeonProgressBar(props: NeonProgressBarProps) -> Element {
    let progress = props.progress.min(100);
    let progress_str = format!("{}%", progress);
    let width_style = format!("width: {}%", progress);
    let label_text = props.label.clone();
    
    rsx! {
        div {
            class: "neon-progress-container",
            if !label_text.is_empty() {
                div {
                    class: "progress-label",
                    {label_text}
                }
            }
            div {
                class: "neon-progress-bar",
                div {
                    class: "neon-progress-fill",
                    style: "{width_style}",
                    ""
                }
            }
            div {
                class: "progress-text",
                {progress_str}
            }
        }
    }
}
