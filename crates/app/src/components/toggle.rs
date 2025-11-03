use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ToggleProps {
    #[props(default = false)]
    checked: bool,
    #[props(default)]
    on_change: EventHandler<bool>,
    #[props(default = "".to_string())]
    label: String,
}

/// Toggle/Checkbox component with neon styling
/// Displays a checkmark (✓) when enabled, empty circle (○) when disabled
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let toggle_class = if props.checked {
        "setting-toggle"
    } else {
        "setting-toggle disabled"
    };
    
    let symbol = if props.checked { "✓" } else { "○" };
    
    rsx! {
        div {
            class: "setting-item",
            span {
                class: "setting-label",
                "{props.label}"
            }
            span {
                class: "{toggle_class}",
                onclick: move |_| {
                    props.on_change.call(!props.checked);
                },
                title: if props.checked { "Clique para desativar" } else { "Clique para ativar" },
                "{symbol}"
            }
        }
    }
}
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ToggleProps {
    #[props(default = false)]
    checked: bool,
    #[props(default)]
    on_change: EventHandler<bool>,
    #[props(default = "".to_string())]
    label: String,
}

/// Toggle/Checkbox component with neon styling
/// Displays a checkmark (✓) when enabled, empty circle (○) when disabled
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let toggle_class = if props.checked {
        "setting-toggle"
    } else {
        "setting-toggle disabled"
    };
    
    let symbol = if props.checked { "✓" } else { "○" };
    
    rsx! {
        div {
            class: "setting-item",
            span {
                class: "setting-label",
                "{props.label}"
            }
            span {
                class: "{toggle_class}",
                onclick: move |_| {
                    props.on_change.call(!props.checked);
                },
                title: if props.checked { "Clique para desativar" } else { "Clique para ativar" },
                "{symbol}"
            }
        }
    }
}
