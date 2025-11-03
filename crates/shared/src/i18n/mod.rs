//! Internationalization and Localization Module
//!
//! This module provides comprehensive i18n/l10n support for NeuroNexus,
//! enabling the application to support 246 official Google Translate languages
//! with a system default fallback to English.

pub mod locale;
pub mod translator;
pub mod fluent_loader;

pub use locale::{Locale, LocaleDetector, SUPPORTED_LANGUAGES};
pub use translator::{Translator, TranslationKey};
pub use fluent_loader::FluentLoader;

/// Default fallback language (English - United States)
pub const DEFAULT_LOCALE: &str = "en-US";

/// Priority languages for initial implementation
pub const PRIORITY_LANGUAGES: &[&str] = &[
    "en-US", // English - United States (Default)
    "pt-BR", // Portuguese - Brazil (Primary audience)
    "es-ES", // Spanish - Spain
    "es-MX", // Spanish - Mexico
    "fr-FR", // French - France
    "de-DE", // German - Germany
    "it-IT", // Italian - Italy
    "ja-JP", // Japanese - Japan
    "zh-CN", // Chinese - Simplified
    "ko-KR", // Korean - South Korea
];

/// RTL (Right-to-Left) languages requiring special layout handling
pub const RTL_LANGUAGES: &[&str] = &[
    "ar",    // Arabic
    "he",    // Hebrew
    "fa",    // Persian/Farsi
    "ur",    // Urdu
];

/// Check if a language code requires RTL layout
pub fn is_rtl(locale: &str) -> bool {
    RTL_LANGUAGES.iter().any(|&lang| locale.starts_with(lang))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtl_detection() {
        assert!(is_rtl("ar"));
        assert!(is_rtl("ar-SA"));
        assert!(is_rtl("he-IL"));
        assert!(is_rtl("fa-IR"));
        assert!(!is_rtl("en-US"));
        assert!(!is_rtl("pt-BR"));
    }
}
