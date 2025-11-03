use dioxus::prelude::*;
use dioxus_router::Link;
use crate::app::Route;

#[component]
pub fn NavigationMenu() -> Element {
    let mut is_open = use_signal(|| false);
    
    rsx! {
        div {
            class: "nav-menu-container",
            button {
                class: "nav-menu-toggle",
                onclick: move |_| {
                    is_open.set(!is_open());
                },
                if is_open() {
                    "✕"
                } else {
                    "☰"
                }
            }
            if is_open() {
                div {
                    class: "nav-menu-overlay",
                    onclick: move |_| {
                        is_open.set(false);
                    }
                }
                nav {
                    class: "nav-menu",
                    div {
                        class: "nav-menu-header",
                        h3 {
                            "MENU"
                        }
                        button {
                            class: "nav-menu-close",
                            onclick: move |_| {
                                is_open.set(false);
                            },
                            "✕"
                        }
                    }
                    MenuLink {
                        route: Route::Home {},
                        icon: "📊",
                        label: "Dashboard"
                    }
                    MenuLink {
                        route: Route::KnowledgeTrails {},
                        icon: "🗺️",
                        label: "Trilhas de Conhecimento"
                    }
                    MenuLink {
                        route: Route::Questions {},
                        icon: "❓",
                        label: "Questões"
                    }
                    MenuLink {
                        route: Route::Essays {},
                        icon: "✍️",
                        label: "Redações"
                    }
                    MenuLink {
                        route: Route::Profile {},
                        icon: "👤",
                        label: "Perfil"
                    }
                    div {
                        class: "nav-menu-divider"
                    }
                    div {
                        class: "nav-menu-section",
                        h4 {
                            "TRILHAS"
                        }
                        div {
                            class: "nav-menu-placeholder",
                            "Em breve..."
                        }
                    }
                    div {
                        class: "nav-menu-section",
                        h4 {
                            "FERRAMENTAS"
                        }
                        div {
                            class: "nav-menu-placeholder",
                            "Em breve..."
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct MenuLinkProps {
    route: Route,
    icon: &'static str,
    label: &'static str,
}

#[component]
fn MenuLink(props: MenuLinkProps) -> Element {
    rsx! {
        Link {
            to: props.route.clone(),
            class: "menu-link",
            span {
                class: "menu-link-icon",
                {props.icon}
            }
            span {
                class: "menu-link-label",
                {props.label}
            }
            span {
                class: "menu-link-arrow",
                "→"
            }
        }
    }
}

