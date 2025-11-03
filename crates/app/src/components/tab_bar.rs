use dioxus::prelude::*;
use dioxus_router::Link;
use crate::app::Route;

#[component]
pub fn TabBar() -> Element {
    rsx! {
        nav {
            class: "tab-bar",
            TabItem {
                route: Route::Home {},
                icon: "📊",
                label: "Dashboard"
            }
            TabItem {
                route: Route::KnowledgeTrails {},
                icon: "🗺️",
                label: "Trilhas"
            }
            TabItem {
                route: Route::Questions {},
                icon: "❓",
                label: "Questões"
            }
            TabItem {
                route: Route::Essays {},
                icon: "✍️",
                label: "Redações"
            }
            TabItem {
                route: Route::Profile {},
                icon: "👤",
                label: "Perfil"
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct TabItemProps {
    route: Route,
    icon: &'static str,
    label: &'static str,
}

#[component]
fn TabItem(props: TabItemProps) -> Element {
    rsx! {
        Link {
            to: props.route.clone(),
            class: "tab-item",
            div {
                class: "tab-item-content",
                span {
                    class: "tab-icon",
                    {props.icon}
                }
                span {
                    class: "tab-label",
                    {props.label}
                }
            }
        }
    }
}
