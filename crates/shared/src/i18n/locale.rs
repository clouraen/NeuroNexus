//! Locale detection and management service
//!
//! This module handles system locale detection, user preference management,
//! and locale validation.

use serde::{Deserialize, Serialize};
use std::fmt;
use unic_langid::LanguageIdentifier;

/// Represents a locale/language setting
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Locale {
    /// Language identifier (e.g., "en-US", "pt-BR")
    pub code: String,
    /// Native language name (e.g., "English", "Português")
    pub native_name: String,
    /// English language name (e.g., "English", "Portuguese")
    pub english_name: String,
}

impl Locale {
    pub fn new(code: impl Into<String>, native_name: impl Into<String>, english_name: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            native_name: native_name.into(),
            english_name: english_name.into(),
        }
    }

    /// Parse language identifier from locale code
    pub fn language_id(&self) -> Result<LanguageIdentifier, String> {
        self.code.parse().map_err(|e| format!("{:?}", e))
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.native_name, self.code)
    }
}

/// Service for detecting and managing locale settings
pub struct LocaleDetector;

impl LocaleDetector {
    /// Detect the system locale
    /// 
    /// Resolution priority:
    /// 1. User-selected language (from storage)
    /// 2. System locale (from OS)
    /// 3. English (default fallback)
    pub fn detect_system_locale() -> String {
        sys_locale::get_locale()
            .and_then(|locale| Self::normalize_locale(&locale))
            .unwrap_or_else(|| crate::i18n::DEFAULT_LOCALE.to_string())
    }

    /// Normalize locale code to standard format (e.g., "en_US" -> "en-US")
    fn normalize_locale(locale: &str) -> Option<String> {
        // Replace underscore with hyphen
        let normalized = locale.replace('_', "-");
        
        // Validate against supported languages
        if Self::is_supported(&normalized) {
            Some(normalized)
        } else {
            // Try to match language without region
            let lang_only = normalized.split('-').next()?;
            Self::find_default_variant(lang_only)
        }
    }

    /// Check if a locale is in the supported languages list
    fn is_supported(locale: &str) -> bool {
        SUPPORTED_LANGUAGES.iter().any(|l| l.code == locale)
    }

    /// Find default variant for a language (e.g., "en" -> "en-US")
    fn find_default_variant(lang: &str) -> Option<String> {
        SUPPORTED_LANGUAGES
            .iter()
            .find(|l| l.code.starts_with(&format!("{}-", lang)))
            .map(|l| l.code.clone())
    }

    /// Get user's preferred locale from storage
    /// (To be implemented with actual storage mechanism)
    pub fn get_user_preference() -> Option<String> {
        // TODO: Implement storage retrieval
        None
    }

    /// Save user's locale preference to storage
    /// (To be implemented with actual storage mechanism)
    pub fn save_user_preference(_locale: &str) -> Result<(), String> {
        // TODO: Implement storage persistence
        Ok(())
    }
}

/// Supported languages list (Priority languages for initial phase)
/// Full 246 language list to be added in Phase 5
pub const SUPPORTED_LANGUAGES: &[Locale] = &[
    Locale {
        code: String::new(),
        native_name: String::new(),
        english_name: String::new(),
    },
];

// Helper function to create static locale list
pub fn get_supported_languages() -> Vec<Locale> {
    vec![
        // Priority languages (Phase 1-3)
        Locale::new("en-US", "English", "English"),
        Locale::new("pt-BR", "Português (Brasil)", "Portuguese (Brazil)"),
        Locale::new("es-ES", "Español (España)", "Spanish (Spain)"),
        Locale::new("es-MX", "Español (México)", "Spanish (Mexico)"),
        Locale::new("fr-FR", "Français", "French"),
        Locale::new("de-DE", "Deutsch", "German"),
        Locale::new("it-IT", "Italiano", "Italian"),
        Locale::new("ja-JP", "日本語", "Japanese"),
        Locale::new("zh-CN", "简体中文", "Chinese (Simplified)"),
        Locale::new("ko-KR", "한국어", "Korean"),
        
        // Additional common languages (Phase 4)
        Locale::new("ru-RU", "Русский", "Russian"),
        Locale::new("ar-SA", "العربية", "Arabic"),
        Locale::new("hi-IN", "हिन्दी", "Hindi"),
        Locale::new("tr-TR", "Türkçe", "Turkish"),
        Locale::new("nl-NL", "Nederlands", "Dutch"),
        Locale::new("pl-PL", "Polski", "Polish"),
        Locale::new("sv-SE", "Svenska", "Swedish"),
        Locale::new("da-DK", "Dansk", "Danish"),
        Locale::new("fi-FI", "Suomi", "Finnish"),
        Locale::new("no-NO", "Norsk", "Norwegian"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_display() {
        let locale = Locale::new("pt-BR", "Português (Brasil)", "Portuguese (Brazil)");
        assert_eq!(locale.to_string(), "Português (Brasil) (pt-BR)");
    }

    #[test]
    fn test_normalize_locale() {
        assert_eq!(
            LocaleDetector::normalize_locale("en_US"),
            Some("en-US".to_string())
        );
    }

    #[test]
    fn test_detect_system_locale() {
        let locale = LocaleDetector::detect_system_locale();
        assert!(!locale.is_empty());
    }
}
