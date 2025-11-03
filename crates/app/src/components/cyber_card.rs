use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CyberCardProps {
    children: Element,
    #[props(default)]
    class: String,
    #[props(default)]
    on_click: Option<EventHandler<()>>,
}

#[component]
pub fn CyberCard(props: CyberCardProps) -> Element {
    rsx! {
        div {
            class: "cyber-card {props.class}",
            onclick: move |_| {
                if let Some(handler) = props.on_click.as_ref() {
                    handler.call(());
                }
            },
            {props.children}
        }
    }
}
