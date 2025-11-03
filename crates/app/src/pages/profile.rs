use dioxus::prelude::*;
use crate::components::*;

#[component]
pub fn Profile() -> Element {
    let mut notifications_enabled = use_signal(|| true);
    let mut show_import_modal = use_signal(|| false);
    
    rsx! {
        div {
            class: "app-container",
            StatusBar {}
            main {
                class: "main-content",
                div {
                    class: "page-container",
                    
                    // Two-panel layout
                    div {
                        class: "two-panel-layout",
                        
                        // Left Panel: Profile Information
                        div {
                            class: "panel-container",
                            h2 {
                                class: "panel-title",
                                "PERFIL"
                            }
                            div {
                                class: "panel-card",
                                div {
                                    class: "profile-header",
                                    div {
                                        class: "profile-avatar",
                                        "👤"
                                    }
                                    div {
                                        class: "profile-info",
                                        h2 {
                                            "Estudante"
                                        }
                                        p {
                                            "estudante@neuronexus.app"
                                        }
                                    }
                                }
                                div {
                                    class: "profile-stats",
                                    div {
                                        class: "stat-item",
                                        span {
                                            class: "stat-number",
                                            "0"
                                        }
                                        span {
                                            class: "stat-desc",
                                            "Dias de sequência"
                                        }
                                    }
                                    div {
                                        class: "stat-item",
                                        span {
                                            class: "stat-number",
                                            "0h"
                                        }
                                        span {
                                            class: "stat-desc",
                                            "Tempo de estudo"
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Right Panel: Settings
                        div {
                            class: "panel-container",
                            div {
                                class: "panel-header-with-button",
                                h2 {
                                    class: "panel-title",
                                    "CONFIGURAÇÕES"
                                }
                                button {
                                    class: "import-button",
                                    onclick: move |_| show_import_modal.set(true),
                                    "📥 IMPORTAR"
                                }
                            }
                            div {
                                class: "panel-card",
                                Toggle {
                                    label: "Notificações".to_string(),
                                    checked: notifications_enabled(),
                                    on_change: move |value| {
                                        notifications_enabled.set(value);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            TabBar {}
            ImportModal {
                show: show_import_modal(),
                on_close: move |_| show_import_modal.set(false)
            }
        }
    }
}
