use dioxus::prelude::*;
use crate::context::AppContext;

#[component]
pub fn TokenGuide() -> Element {
    let ctx = use_context::<AppContext>();
    let mut expanded = use_signal(|| false);

    rsx! {
        div {
            class: "token-guide",
            style: "margin: 20px 0;",
            
            // Collapsible header
            div {
                class: "guide-header",
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 12px 16px;
                    background: rgba(0, 255, 0.1);
                    border: 1px solid rgba(0, 255, 255, 0.3);
                    border-radius: 8px;
                    cursor: pointer;
                    margin-bottom: {if expanded() { \"16px\" } else { \"0\" }};
                ",
                onclick: move |_| expanded.set(!expanded()),
                
                h3 {
                    style: "
                        color: #00ffff;
                        margin: 0;
                        font-size: 1.1em;
                        text-shadow: 0 0 10px #00ffff;
                    ",
                    {ctx.t("profile-ai-guide-title")}
                }
                
                span {
                    style: "color: #00ffff; font-size: 1.2em;",
                    {if expanded() { "▼" } else { "▶" }}
                }
            }
            
            // Collapsible content
            if expanded() {
                div {
                    class: "guide-content",
                    style: "
                        background: rgba(10, 10, 15, 0.8);
                        border: 1px solid rgba(0, 255, 255, 0.2);
                        border-radius: 8px;
                        padding: 20px;
                    ",
                    
                    p {
                        style: "color: #ffffff; margin-bottom: 20px;",
                        {ctx.t("profile-ai-guide-intro")}
                    }
                    
                    // Step 1
                    div {
                        class: "guide-step",
                        style: "margin-bottom: 16px; padding-left: 12px; border-left: 2px solid #00ffff;",
                        
                        h4 {
                            style: "color: #ff00ff; margin: 0 0 8px 0;",
                            {ctx.t("profile-ai-guide-step1")}
                        }
                        p {
                            style: "color: #cccccc; margin: 0 0 8px 0;",
                            {ctx.t("profile-ai-guide-step1-desc")}
                        }
                        a {
                            href: "https://huggingface.co/join",
                            target: "_blank",
                            style: "
                                color: #00ffff;
                                text-decoration: none;
                                font-weight: bold;
                            ",
                            "🔗 {ctx.t(\"profile-ai-guide-step1-link\")}"
                        }
                    }
                    
                    // Step 2
                    div {
                        class: "guide-step",
                        style: "margin-bottom: 16px; padding-left: 12px; border-left: 2px solid #00ffff;",
                        
                        h4 {
                            style: "color: #ff00ff; margin: 0 0 8px 0;",
                            {ctx.t("profile-ai-guide-step2")}
                        }
                        p {
                            style: "color: #cccccc; margin: 0 0 8px 0;",
                            {ctx.t("profile-ai-guide-step2-desc")}
                        }
                        a {
                            href: "https://huggingface.co/settings/tokens",
                            target: "_blank",
                            style: "
                                color: #00ffff;
                                text-decoration: none;
                                font-weight: bold;
                            ",
                            "🔗 {ctx.t(\"profile-ai-guide-step2-link\")}"
                        }
                    }
                    
                    // Step 3
                    div {
                        class: "guide-step",
                        style: "margin-bottom: 16px; padding-left: 12px; border-left: 2px solid #00ffff;",
                        
                        h4 {
                            style: "color: #ff00ff; margin: 0 0 8px 0;",
                            {ctx.t("profile-ai-guide-step3")}
                        }
                        p {
                            style: "color: #cccccc; margin: 0 0 4px 0;",
                            {ctx.t("profile-ai-guide-step3-desc")}
                        }
                        p {
                            style: "color: #00ffff; margin: 0; font-size: 0.9em; font-style: italic;",
                            "💡 {ctx.t(\"profile-ai-guide-step3-name\")}"
                        }
                    }
                    
                    // Step 4
                    div {
                        class: "guide-step",
                        style: "margin-bottom: 16px; padding-left: 12px; border-left: 2px solid #00ffff;",
                        
                        h4 {
                            style: "color: #ff00ff; margin: 0 0 8px 0;",
                            {ctx.t("profile-ai-guide-step4")}
                        }
                        p {
                            style: "color: #cccccc; margin: 0;",
                            {ctx.t("profile-ai-guide-step4-desc")}
                        }
                    }
                    
                    // Step 5
                    div {
                        class: "guide-step",
                        style: "padding-left: 12px; border-left: 2px solid #00ffff;",
                        
                        h4 {
                            style: "color: #ff00ff; margin: 0 0 8px 0;",
                            {ctx.t("profile-ai-guide-step5")}
                        }
                        p {
                            style: "color: #cccccc; margin: 0;",
                            {ctx.t("profile-ai-guide-step5-desc")}
                        }
                    }
                    
                    // Info section
                    div {
                        style: "
                            margin-top: 20px;
                            padding: 12px;
                            background: rgba(0, 255, 255, 0.05);
                            border-radius: 8px;
                            border: 1px solid rgba(0, 255, 255, 0.2);
                        ",
                        
                        p {
                            style: "color: #00ffff; margin: 0 0 8px 0; font-size: 0.9em;",
                            "ℹ️ {ctx.t(\"profile-ai-info-token-permissions\")}"
                        }
                        p {
                            style: "color: #00ffff; margin: 0; font-size: 0.9em;",
                            "ℹ️ {ctx.t(\"profile-ai-info-requires-token\")}"
                        }
                    }
                }
            }
        }
    }
}
