use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NeonInputProps {
    #[props(default)]
    value: String,
    #[props(default)]
    placeholder: String,
    on_input: EventHandler<String>,
    #[props(default)]
    class: String,
    #[props(default)]
    input_type: String,
}

#[component]
pub fn NeonInput(props: NeonInputProps) -> Element {
    rsx! {
        input {
            class: "neon-input {props.class}",
            r#type: "{props.input_type}",
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            oninput: move |evt| {
                props.on_input.call(evt.value());
            },
        }
    }
}
