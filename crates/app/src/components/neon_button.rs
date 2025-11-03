use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NeonButtonProps {
    children: Element,
    #[props(default)]
    on_click: EventHandler<()>,
    #[props(default)]
    class: String,
    #[props(default)]
    variant: ButtonVariant,
}

#[derive(PartialEq, Clone, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
}

#[component]
pub fn NeonButton(props: NeonButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "neon-button-primary",
        ButtonVariant::Secondary => "neon-button-secondary",
        ButtonVariant::Danger => "neon-button-danger",
    };

    rsx! {
        button {
            class: "neon-button {variant_class} {props.class}",
            onclick: move |_| props.on_click.call(()),
            {props.children}
        }
    }
}
