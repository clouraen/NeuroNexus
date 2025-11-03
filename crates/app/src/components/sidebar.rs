use dioxus::prelude::*;
use dioxus_router::Link;
use crate::app::Route;
use crate::context::AppContext;

#[component]
pub fn Sidebar() -> Element {
    let ctx = use_context::<AppContext>();
    
    rsx! {
        aside {
            class: "sidebar",
            div {
                class: "sidebar-header",
                h2 {
                    class: "sidebar-logo",
                    "NEURONEXUS"
                }
                div {
                    class: "sidebar-subtitle",
                    "SISTEMA DE APRENDIZADO"
                }
            }
            nav {
                class: "sidebar-nav",
                NavItem {
                    route: Route::Home {},
                    icon: "📊",
                    label: ctx.t("nav-sidebar-label-dashboard"),
                    description: ctx.t("nav-sidebar-desc-dashboard")
                }
                NavItem {
                    route: Route::KnowledgeTrails {},
                    icon: "🗺️",
                    label: ctx.t("nav-sidebar-label-trails"),
                    description: ctx.t("nav-sidebar-desc-trails")
                }
                NavItem {
                    route: Route::Questions {},
                    icon: "❓",
                    label: ctx.t("nav-sidebar-label-questions"),
                    description: ctx.t("nav-sidebar-desc-questions")
                }
                NavItem {
                    route: Route::Essays {},
                    icon: "✍️",
                    label: ctx.t("nav-sidebar-label-essays"),
                    description: ctx.t("nav-sidebar-desc-essays")
                }
                NavItem {
                    route: Route::Profile {},
                    icon: "👤",
                    label: ctx.t("nav-sidebar-label-profile"),
                    description: ctx.t("nav-sidebar-desc-profile")
                }
            }
            div {
                class: "sidebar-footer",
                div {
                    class: "sidebar-stats",
                    div {
                        class: "stat-mini",
                        span {
                            class: "stat-value-mini",
                            "0"
                        }
                        span {
                            class: "stat-label-mini",
                            "{ctx.t(\"profile-stats-sequences\")}"
                        }
                    }
                    div {
                        class: "stat-mini",
                        span {
                            class: "stat-value-mini",
                            "0h"
                        }
                        span {
                            class: "stat-label-mini",
                            "{ctx.t(\"common-time-today\")}"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct NavItemProps {
    route: Route,
    icon: &'static str,
    label: String,
    description: String,
}

#[component]
fn NavItem(props: NavItemProps) -> Element {
    rsx! {
        Link {
            to: props.route.clone(),
            class: "nav-item",
            div {
                class: "nav-item-icon",
                {props.icon}
            }
            div {
                class: "nav-item-content",
                div {
                    class: "nav-item-label",
                    {props.label}
                }
                div {
                    class: "nav-item-description",
                    {props.description}
                }
            }
        }
    }
}

