//! Fluent resource bundle loader
//!
//! This module handles loading and caching of Fluent translation files.

use fluent::{FluentBundle, FluentResource};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use unic_langid::LanguageIdentifier;

/// Fluent resource loader and cache manager
pub struct FluentLoader {
    /// Base directory for locale files (e.g., "./locales")
    base_path: PathBuf,
    /// Cached bundles by locale code
    bundles: HashMap<String, FluentBundle<FluentResource>>,
}

impl FluentLoader {
    /// Create a new FluentLoader with the specified base path
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            bundles: HashMap::new(),
        }
    }

    /// Load a Fluent bundle for the specified locale
    pub fn load_bundle(&mut self, locale: &str) -> Result<&FluentBundle<FluentResource>, LoaderError> {
        // Check if already cached
        if !self.bundles.contains_key(locale) {
            let bundle = self.create_bundle(locale)?;
            self.bundles.insert(locale.to_string(), bundle);
        }

        self.bundles
            .get(locale)
            .ok_or_else(|| LoaderError::BundleNotFound(locale.to_string()))
    }

    /// Create a new Fluent bundle by loading all .ftl files for a locale
    fn create_bundle(&self, locale: &str) -> Result<FluentBundle<FluentResource>, LoaderError> {
        let lang_id: LanguageIdentifier = locale
            .parse()
            .map_err(|e| LoaderError::InvalidLocale(locale.to_string(), format!("{:?}", e)))?;

        let mut bundle = FluentBundle::new(vec![lang_id]);

        // Load all .ftl files from the locale directory
        let locale_dir = self.base_path.join(locale);
        
        if !locale_dir.exists() {
            return Err(LoaderError::LocaleDirectoryNotFound(locale.to_string()));
        }

        let ftl_files = self.find_ftl_files(&locale_dir)?;

        for ftl_path in ftl_files {
            let content = fs::read_to_string(&ftl_path)
                .map_err(|e| LoaderError::FileReadError(ftl_path.clone(), e.to_string()))?;

            let resource = FluentResource::try_new(content)
                .map_err(|e| LoaderError::ParseError(ftl_path.clone(), format!("{:?}", e.1)))?;

            bundle
                .add_resource(resource)
                .map_err(|e| LoaderError::BundleAddError(ftl_path.clone(), format!("{:?}", e)))?;
        }

        Ok(bundle)
    }

    /// Find all .ftl files in a directory (non-recursive)
    fn find_ftl_files(&self, dir: &Path) -> Result<Vec<PathBuf>, LoaderError> {
        let mut files = Vec::new();

        let entries = fs::read_dir(dir)
            .map_err(|e| LoaderError::DirectoryReadError(dir.to_path_buf(), e.to_string()))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| LoaderError::DirectoryReadError(dir.to_path_buf(), e.to_string()))?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ftl") {
                files.push(path);
            }
        }

        Ok(files)
    }

    /// Clear cached bundles to free memory
    pub fn clear_cache(&mut self) {
        self.bundles.clear();
    }

    /// Remove a specific locale from cache
    pub fn remove_from_cache(&mut self, locale: &str) {
        self.bundles.remove(locale);
    }
}

/// Errors that can occur during Fluent resource loading
#[derive(Debug, thiserror::Error)]
pub enum LoaderError {
    #[error("Invalid locale identifier: {0} - {1}")]
    InvalidLocale(String, String),

    #[error("Locale directory not found: {0}")]
    LocaleDirectoryNotFound(String),

    #[error("Failed to read directory {0}: {1}")]
    DirectoryReadError(PathBuf, String),

    #[error("Failed to read file {0}: {1}")]
    FileReadError(PathBuf, String),

    #[error("Failed to parse Fluent file {0}: {1}")]
    ParseError(PathBuf, String),

    #[error("Failed to add resource to bundle from {0}: {1}")]
    BundleAddError(PathBuf, String),

    #[error("Bundle not found for locale: {0}")]
    BundleNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluent_loader_creation() {
        let loader = FluentLoader::new("./locales");
        assert_eq!(loader.base_path, PathBuf::from("./locales"));
    }
}
