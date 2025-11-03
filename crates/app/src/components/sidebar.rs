use dioxus::prelude::*;
use dioxus_router::Link;
use crate::app::Route;

#[component]
pub fn Sidebar() -> Element {
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
                    label: "Dashboard",
                    description: "Visão geral e métricas"
                }
                NavItem {
                    route: Route::KnowledgeTrails {},
                    icon: "🗺️",
                    label: "Trilhas de Conhecimento",
                    description: "Caminhos de aprendizado"
                }
                NavItem {
                    route: Route::Questions {},
                    icon: "❓",
                    label: "Questões",
                    description: "Pratique exercícios"
                }
                NavItem {
                    route: Route::Essays {},
                    icon: "✍️",
                    label: "Redações",
                    description: "Escreva e corrija"
                }
                NavItem {
                    route: Route::Profile {},
                    icon: "👤",
                    label: "Perfil",
                    description: "Configurações e progresso"
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
                            "Sequência"
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
                            "Hoje"
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
    label: &'static str,
    description: &'static str,
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

