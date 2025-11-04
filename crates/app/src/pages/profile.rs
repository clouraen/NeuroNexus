use dioxus::prelude::*;
use crate::components::*;
use crate::context::AppContext;
use shared::i18n::locale::get_supported_languages;

#[component]
pub fn Profile() -> Element {
    let ctx = use_context::<AppContext>();
    let mut notifications_enabled = use_signal(|| true);
    let mut show_import_modal = use_signal(|| false);
    let mut selected_locale = use_signal(|| ctx.current_locale());
    let supported_languages = get_supported_languages();
    
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
                                "{ctx.t(\"profile-header-title\")}"
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
                                            "{ctx.t(\"profile-personal-student\")}"
                                        }
                                        p {
                                            "{ctx.t(\"profile-personal-default-email\")}"
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
                                            "{ctx.t(\"profile-stats-streak\")}"
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
                                            "{ctx.t(\"profile-stats-study-time\")}"
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Right Panel: Settings with Tabs
                        div {
                            class: "panel-container",
                            div {
                                class: "panel-header-with-button",
                                h2 {
                                    class: "panel-title",
                                    "{ctx.t(\"profile-section-preferences\")}"
                                }
                                button {
                                    class: "import-button",
                                    onclick: move |_| show_import_modal.set(true),
                                    "{ctx.t(\"common-button-import\")}"
                                }
                            }
                            
                            // Tab Navigation
                            div {
                                class: "tab-navigation",
                                style: "display: flex; gap: 8px; margin-bottom: 16px; border-bottom: 2px solid rgba(0, 255, 255, 0.2);",
                                
                                button {
                                    class: if active_tab() == "preferences" { "tab-button active" } else { "tab-button" },
                                    style: "
                                        padding: 12px 24px;
                                        background: {if active_tab() == \"preferences\" { \"rgba(0, 255, 255, 0.2)\" } else { \"transparent\" }};
                                        border: none;
                                        border-bottom: 2px solid {if active_tab() == \"preferences\" { \"#00ffff\" } else { \"transparent\" }};
                                        color: {if active_tab() == \"preferences\" { \"#00ffff\" } else { \"#888888\" }};
                                        cursor: pointer;
                                        font-size: 1em;
                                        transition: all 0.3s ease;
                                    ",
                                    onclick: move |_| active_tab.set("preferences".to_string()),
                                    "{ctx.t(\"profile-section-preferences\")}"
                                }
                                
                                button {
                                    class: if active_tab() == "ai-config" { "tab-button active" } else { "tab-button" },
                                    style: "
                                        padding: 12px 24px;
                                        background: {if active_tab() == \"ai-config\" { \"rgba(255, 0, 255, 0.2)\" } else { \"transparent\" }};
                                        border: none;
                                        border-bottom: 2px solid {if active_tab() == \"ai-config\" { \"#ff00ff\" } else { \"transparent\" }};
                                        color: {if active_tab() == \"ai-config\" { \"#ff00ff\" } else { \"#888888\" }};
                                        cursor: pointer;
                                        font-size: 1em;
                                        transition: all 0.3s ease;
                                    ",
                                    onclick: move |_| active_tab.set("ai-config".to_string()),
                                    "🤖 {ctx.t(\"profile-ai-title\")}"
                                }
                            }
                            
                            // Tab Content
                            div {
                                class: "tab-content",
                                
                                // Preferences Tab
                                if active_tab() == "preferences" {
                                    div {
                                        class: "panel-card",
                                        // Language Selector
                                        div {
                                            class: "setting-item",
                                            label {
                                                class: "setting-label",
                                                "{ctx.t(\"profile-lang-select\")}"
                                            }
                                            select {
                                                class: "language-select",
                                                value: "{selected_locale()}",
                                                onchange: move |evt| {
                                                    let new_locale = evt.value();
                                                    if let Ok(_) = ctx.set_locale(&new_locale) {
                                                        selected_locale.set(new_locale.clone());
                                                    }
                                                },
                                                for lang in supported_languages.iter() {
                                                    option {
                                                        value: "{lang.code}",
                                                        selected: lang.code == selected_locale(),
                                                        "{lang.native_name} ({lang.english_name})"
                                                    }
                                                }
                                            }
                                        }
                                        Toggle {
                                            label: ctx.t("profile-pref-notifications"),
                                            checked: notifications_enabled(),
                                            on_change: move |value| {
                                                notifications_enabled.set(value);
                                            }
                                        }
                                    }
                                }
                                
                                // AI Configuration Tab
                                if active_tab() == "ai-config" {
                                    div {
                                        class: "panel-card",
                                        AIConfigPanel {}
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
