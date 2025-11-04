use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Model loading status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    NotConfigured,
    TokenSaved,
    Connecting,
    Downloading,
    Loading,
    Ready,
    Error(String),
}

/// AI Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfiguration {
    /// Encrypted HuggingFace token (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huggingface_token_encrypted: Option<String>,
    
    /// Auto-load model on startup
    #[serde(default)]
    pub auto_load_on_startup: bool,
    
    /// Custom model cache path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_cache_path: Option<String>,
    
    /// Last successful model load
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_load: Option<DateTime<Utc>>,
    
    /// Model version identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    
    /// Download preferences
    #[serde(default)]
    pub download_preferences: DownloadPreferences,
}

/// Download preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadPreferences {
    /// Resume downloads on interruption
    #[serde(default = "default_true")]
    pub resume_on_interrupt: bool,
    
    /// Verify file integrity after download
    #[serde(default = "default_true")]
    pub verify_integrity: bool,
}

fn default_true() -> bool {
    true
}

impl Default for DownloadPreferences {
    fn default() -> Self {
        Self {
            resume_on_interrupt: true,
            verify_integrity: true,
        }
    }
}

impl Default for AIConfiguration {
    fn default() -> Self {
        Self {
            huggingface_token_encrypted: None,
            auto_load_on_startup: false,
            model_cache_path: None,
            last_successful_load: None,
            model_version: Some("neuralmind/bert-base-portuguese-cased".to_string()),
            download_preferences: DownloadPreferences::default(),
        }
    }
}

/// AI Configuration Manager
pub struct AIConfigManager {
    config_path: PathBuf,
}

impl AIConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        Ok(Self { config_path })
    }
    
    /// Get the platform-specific configuration file path
    fn get_config_path() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/NeuroNexus
            dirs::home_dir()
                .context("Failed to get home directory")?
                .join("Library")
                .join("Application Support")
                .join("NeuroNexus")
        } else if cfg!(target_os = "windows") {
            // Windows: %APPDATA%\NeuroNexus
            dirs::config_dir()
                .context("Failed to get config directory")?
                .join("NeuroNexus")
        } else {
            // Linux: ~/.config/neuronexus
            dirs::config_dir()
                .context("Failed to get config directory")?
                .join("neuronexus")
        };
        
        Ok(config_dir.join("ai_config.json"))
    }
    
    /// Load configuration from disk
    pub fn load(&self) -> Result<AIConfiguration> {
        if !self.config_path.exists() {
            return Ok(AIConfiguration::default());
        }
        
        let content = fs::read_to_string(&self.config_path)
            .context("Failed to read configuration file")?;
        
        let config: AIConfiguration = serde_json::from_str(&content)
            .context("Failed to parse configuration file")?;
        
        Ok(config)
    }
    
    /// Save configuration to disk
    pub fn save(&self, config: &AIConfiguration) -> Result<()> {
        let content = serde_json::to_string_pretty(config)
            .context("Failed to serialize configuration")?;
        
        fs::write(&self.config_path, content)
            .context("Failed to write configuration file")?;
        
        Ok(())
    }
    
    /// Get the HuggingFace token (decrypted)
    pub fn get_token(&self) -> Result<Option<String>> {
        let config = self.load()?;
        
        if let Some(encrypted) = config.huggingface_token_encrypted {
            // Simple base64 decoding for now
            // In production, use proper encryption
            let decoded = general_purpose::STANDARD.decode(&encrypted)
                .context("Failed to decode token")?;
            let token = String::from_utf8(decoded)
                .context("Failed to parse token")?;
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
    
    /// Set the HuggingFace token (encrypted)
    pub fn set_token(&self, token: &str) -> Result<()> {
        let mut config = self.load()?;
        
        // Simple base64 encoding for now
        // In production, use proper encryption
        let encrypted = general_purpose::STANDARD.encode(token);
        config.huggingface_token_encrypted = Some(encrypted);
        
        self.save(&config)?;
        Ok(())
    }
    
    /// Validate token format
    pub fn validate_token_format(token: &str) -> bool {
        token.starts_with("hf_") && token.len() > 10
    }
    
    /// Clear the stored token
    pub fn clear_token(&self) -> Result<()> {
        let mut config = self.load()?;
        config.huggingface_token_encrypted = None;
        self.save(&config)?;
        Ok(())
    }
    
    /// Check if token is configured
    pub fn is_token_configured(&self) -> bool {
        self.load()
            .ok()
            .and_then(|c| c.huggingface_token_encrypted)
            .is_some()
    }
    
    /// Get the default HuggingFace cache directory
    pub fn get_default_cache_dir() -> Result<PathBuf> {
        let cache_dir = dirs::home_dir()
            .context("Failed to get home directory")?
            .join(".cache")
            .join("huggingface")
            .join("hub");
        
        Ok(cache_dir)
    }
    
    /// Get the model-specific cache directory
    pub fn get_model_cache_dir() -> Result<PathBuf> {
        Ok(Self::get_default_cache_dir()?
            .join("models--neuralmind--bert-base-portuguese-cased"))
    }
    
    /// Check if model is cached locally
    pub fn is_model_cached() -> Result<bool> {
        let cache_dir = Self::get_model_cache_dir()?;
        
        if !cache_dir.exists() {
            return Ok(false);
        }
        
        // Check for essential model files
        let snapshots_dir = cache_dir.join("snapshots");
        if !snapshots_dir.exists() {
            return Ok(false);
        }
        
        // Check if there's at least one snapshot with required files
        if let Ok(entries) = fs::read_dir(&snapshots_dir) {
            for entry in entries.flatten() {
                let snapshot_dir = entry.path();
                if snapshot_dir.is_dir() {
                    let has_config = snapshot_dir.join("config.json").exists();
                    let has_tokenizer = snapshot_dir.join("tokenizer.json").exists();
                    let has_model = snapshot_dir.join("pytorch_model.bin").exists();
                    
                    if has_config && has_tokenizer && has_model {
                        return Ok(true);
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    /// Get cache information (size, location)
    pub fn get_cache_info() -> Result<CacheInfo> {
        let cache_dir = Self::get_model_cache_dir()?;
        let exists = cache_dir.exists();
        
        let size_bytes = if exists {
            Self::calculate_dir_size(&cache_dir)?
        } else {
            0
        };
        
        Ok(CacheInfo {
            location: cache_dir.to_string_lossy().to_string(),
            exists,
            size_bytes,
            size_human: Self::format_bytes(size_bytes),
        })
    }
    
    /// Calculate directory size recursively
    fn calculate_dir_size(path: &PathBuf) -> Result<u64> {
        let mut total = 0;
        
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    total += Self::calculate_dir_size(&path)?;
                } else {
                    total += entry.metadata()?.len();
                }
            }
        }
        
        Ok(total)
    }
    
    /// Format bytes to human-readable string
    fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_idx = 0;
        
        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }
        
        if unit_idx == 0 {
            format!("{} {}", size as u64, UNITS[unit_idx])
        } else {
            format!("{:.2} {}", size, UNITS[unit_idx])
        }
    }
    
    /// Clear model cache
    pub fn clear_cache() -> Result<()> {
        let cache_dir = Self::get_model_cache_dir()?;
        
        if cache_dir.exists() {
            fs::remove_dir_all(&cache_dir)
                .context("Failed to remove cache directory")?;
        }
        
        Ok(())
    }
    
    /// Update last successful load timestamp
    pub fn update_last_load(&self) -> Result<()> {
        let mut config = self.load()?;
        config.last_successful_load = Some(Utc::now());
        self.save(&config)?;
        Ok(())
    }
}

/// Cache information
#[derive(Debug, Clone)]
pub struct CacheInfo {
    pub location: String,
    pub exists: bool,
    pub size_bytes: u64,
    pub size_human: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_validation() {
        assert!(AIConfigManager::validate_token_format("hf_1234567890abcdef"));
        assert!(!AIConfigManager::validate_token_format("invalid_token"));
        assert!(!AIConfigManager::validate_token_format("hf_"));
        assert!(!AIConfigManager::validate_token_format(""));
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(AIConfigManager::format_bytes(100), "100 B");
        assert_eq!(AIConfigManager::format_bytes(1024), "1.00 KB");
        assert_eq!(AIConfigManager::format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(AIConfigManager::format_bytes(420 * 1024 * 1024), "420.00 MB");
    }
    
    #[test]
    fn test_default_config() {
        let config = AIConfiguration::default();
        assert_eq!(config.huggingface_token_encrypted, None);
        assert_eq!(config.auto_load_on_startup, false);
        assert_eq!(config.model_version, Some("neuralmind/bert-base-portuguese-cased".to_string()));
        assert!(config.download_preferences.resume_on_interrupt);
        assert!(config.download_preferences.verify_integrity);
    }
}
