use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BrainIconProps {
    #[props(default = "1.5rem".to_string())]
    size: String,
    #[props(default = "brain-icon".to_string())]
    class: String,
}

/// Brain icon component with SVG representation
/// Shows a stylized geometric brain with neural connections
#[component]
pub fn BrainIcon(props: BrainIconProps) -> Element {
    rsx! {
        svg {
            class: "{props.class}",
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 100 100",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            
            // Brain outline
            path {
                d: "M50 10 C30 10, 15 20, 15 35 C15 40, 16 45, 18 49 C15 52, 13 56, 13 60 C13 70, 20 78, 30 82 C35 88, 42 92, 50 92 C58 92, 65 88, 70 82 C80 78, 87 70, 87 60 C87 56, 85 52, 82 49 C84 45, 85 40, 85 35 C85 20, 70 10, 50 10 Z",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linejoin": "round",
            }
            
            // Neural connection lines
            path {
                d: "M30 35 Q40 40, 50 35",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M50 35 Q60 40, 70 35",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M35 55 Q45 58, 50 55",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M50 55 Q55 58, 65 55",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M40 70 Q50 68, 60 70",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            
            // Neural nodes
            circle {
                cx: "30",
                cy: "35",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "50",
                cy: "35",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "70",
                cy: "35",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "35",
                cy: "55",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "50",
                cy: "55",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "65",
                cy: "55",
                r: "2",
                fill: "currentColor",
            }
        }
    }
}

/// Large brain watermark for background
#[component]
pub fn BrainWatermark() -> Element {
    rsx! {
        div {
            class: "brain-watermark",
            "aria-hidden": "true",
            BrainIcon {
                size: "100%".to_string(),
                class: "brain-watermark-svg".to_string(),
            }
        }
    }
}
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BrainIconProps {
    #[props(default = "1.5rem".to_string())]
    size: String,
    #[props(default = "brain-icon".to_string())]
    class: String,
}

/// Brain icon component with SVG representation
/// Shows a stylized geometric brain with neural connections
#[component]
pub fn BrainIcon(props: BrainIconProps) -> Element {
    rsx! {
        svg {
            class: "{props.class}",
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 100 100",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            
            // Brain outline
            path {
                d: "M50 10 C30 10, 15 20, 15 35 C15 40, 16 45, 18 49 C15 52, 13 56, 13 60 C13 70, 20 78, 30 82 C35 88, 42 92, 50 92 C58 92, 65 88, 70 82 C80 78, 87 70, 87 60 C87 56, 85 52, 82 49 C84 45, 85 40, 85 35 C85 20, 70 10, 50 10 Z",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linejoin": "round",
            }
            
            // Neural connection lines
            path {
                d: "M30 35 Q40 40, 50 35",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M50 35 Q60 40, 70 35",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M35 55 Q45 58, 50 55",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M50 55 Q55 58, 65 55",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            path {
                d: "M40 70 Q50 68, 60 70",
                stroke: "currentColor",
                "stroke-width": "1.5",
                opacity: "0.6",
            }
            
            // Neural nodes
            circle {
                cx: "30",
                cy: "35",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "50",
                cy: "35",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "70",
                cy: "35",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "35",
                cy: "55",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "50",
                cy: "55",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "65",
                cy: "55",
                r: "2",
                fill: "currentColor",
            }
        }
    }
}

/// Large brain watermark for background
#[component]
pub fn BrainWatermark() -> Element {
    rsx! {
        div {
            class: "brain-watermark",
            "aria-hidden": "true",
            BrainIcon {
                size: "100%".to_string(),
                class: "brain-watermark-svg".to_string(),
            }
        }
    }
}
