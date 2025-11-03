//! Translation service for resolving localized strings
//!
//! This module provides the core translation functionality using Fluent.

use super::fluent_loader::{FluentLoader, LoaderError};
use fluent::FluentArgs;
use std::collections::HashMap;

/// Translation key type for type-safe translation lookups
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationKey(String);

impl TranslationKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for TranslationKey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for TranslationKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Main translation service
pub struct Translator {
    /// Current active locale
    current_locale: String,
    /// Fluent resource loader
    loader: FluentLoader,
    /// Fallback locale (typically "en-US")
    fallback_locale: String,
}

impl Translator {
    /// Create a new translator with the specified locale and base path
    pub fn new(locale: impl Into<String>, base_path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            current_locale: locale.into(),
            loader: FluentLoader::new(base_path),
            fallback_locale: crate::i18n::DEFAULT_LOCALE.to_string(),
        }
    }

    /// Get the current active locale
    pub fn current_locale(&self) -> &str {
        &self.current_locale
    }

    /// Switch to a different locale
    pub fn set_locale(&mut self, locale: impl Into<String>) -> Result<(), LoaderError> {
        let new_locale = locale.into();
        // Pre-load the bundle to validate the locale
        self.loader.load_bundle(&new_locale)?;
        self.current_locale = new_locale;
        Ok(())
    }

    /// Translate a key to a localized string
    /// 
    /// Falls back to English if translation not found in current locale
    /// Falls back to the key itself if not found in English either
    pub fn translate(&mut self, key: impl Into<TranslationKey>) -> String {
        self.translate_with_args(key, None)
    }

    /// Translate a key with variable arguments
    pub fn translate_with_args(
        &mut self,
        key: impl Into<TranslationKey>,
        args: Option<HashMap<String, String>>,
    ) -> String {
        let key = key.into();
        
        // Try current locale first
        if let Ok(translation) = self.get_translation(&self.current_locale.clone(), &key, args.as_ref()) {
            return translation;
        }

        // Try fallback locale
        if self.current_locale != self.fallback_locale {
            if let Ok(translation) = self.get_translation(&self.fallback_locale.clone(), &key, args.as_ref()) {
                return translation;
            }
        }

        // Final fallback: return the key itself
        key.0
    }

    /// Get translation from a specific locale bundle
    fn get_translation(
        &mut self,
        locale: &str,
        key: &TranslationKey,
        args: Option<&HashMap<String, String>>,
    ) -> Result<String, String> {
        let bundle = self.loader
            .load_bundle(locale)
            .map_err(|e| format!("Failed to load bundle: {}", e))?;

        let message = bundle
            .get_message(key.as_str())
            .ok_or_else(|| format!("Message not found: {}", key.as_str()))?;

        let pattern = message
            .value()
            .ok_or_else(|| format!("Message has no value: {}", key.as_str()))?;

        let mut errors = vec![];
        
        let formatted = if let Some(args_map) = args {
            let mut fluent_args = FluentArgs::new();
            for (k, v) in args_map {
                fluent_args.set(k, v.clone());
            }
            bundle.format_pattern(pattern, Some(&fluent_args), &mut errors)
        } else {
            bundle.format_pattern(pattern, None, &mut errors)
        };

        if !errors.is_empty() {
            return Err(format!("Formatting errors: {:?}", errors));
        }

        Ok(formatted.to_string())
    }

    /// Check if a translation exists for a key in the current locale
    pub fn has_translation(&mut self, key: impl Into<TranslationKey>) -> bool {
        let key = key.into();
        self.get_translation(&self.current_locale.clone(), &key, None).is_ok()
    }

    /// Format a number according to current locale
    pub fn format_number(&self, number: f64) -> String {
        // TODO: Implement proper locale-aware number formatting
        // For now, use basic formatting
        format!("{:.2}", number)
    }

    /// Format a date according to current locale
    pub fn format_date(&self, date: &chrono::DateTime<chrono::Utc>) -> String {
        // TODO: Implement proper locale-aware date formatting
        // For now, use ISO format
        date.format("%Y-%m-%d").to_string()
    }

    /// Format a date and time according to current locale
    pub fn format_datetime(&self, datetime: &chrono::DateTime<chrono::Utc>) -> String {
        // TODO: Implement proper locale-aware datetime formatting
        // For now, use ISO format
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_key_creation() {
        let key = TranslationKey::new("test-key");
        assert_eq!(key.as_str(), "test-key");
    }

    #[test]
    fn test_translation_key_from_str() {
        let key: TranslationKey = "test-key".into();
        assert_eq!(key.as_str(), "test-key");
    }
}
