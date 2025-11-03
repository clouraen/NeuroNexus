use dioxus::prelude::*;
use dioxus_router::Link;
use crate::app::Route;
use crate::context::AppContext;

#[component]
pub fn TabBar() -> Element {
    let ctx = use_context::<AppContext>();
    
    rsx! {
        nav {
            class: "tab-bar",
            TabItem {
                route: Route::Home {},
                icon: "📊",
                label: ctx.t("nav-tabbar-label-dashboard")
            }
            TabItem {
                route: Route::KnowledgeTrails {},
                icon: "🗺️",
                label: ctx.t("nav-tabbar-label-trails")
            }
            TabItem {
                route: Route::Questions {},
                icon: "❓",
                label: ctx.t("nav-tabbar-label-questions")
            }
            TabItem {
                route: Route::Essays {},
                icon: "✍️",
                label: ctx.t("nav-tabbar-label-essays")
            }
            TabItem {
                route: Route::Profile {},
                icon: "👤",
                label: ctx.t("nav-tabbar-label-profile")
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct TabItemProps {
    route: Route,
    icon: &'static str,
    label: String,
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
