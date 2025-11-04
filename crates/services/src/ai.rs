use anyhow::{Context, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
use hf_hub::{api::sync::Api, Repo, RepoType};
use std::sync::Arc;
use tokenizers::Tokenizer;
use tokio::sync::RwLock;

/// Progress callback for model loading
pub type ProgressCallback = Arc<dyn Fn(f32, String) + Send + Sync>;

/// AI Service for essay evaluation using BERTimbau model
#[derive(Clone)]
pub struct AIService {
    model: Arc<RwLock<Option<BertModel>>>,
    tokenizer: Arc<RwLock<Option<Tokenizer>>>,
    device: Device,
}

impl AIService {
    /// Create a new AI service instance
    pub fn new() -> Result<Self> {
        let device = Device::Cpu;
        
        Ok(Self {
            model: Arc::new(RwLock::new(None)),
            tokenizer: Arc::new(RwLock::new(None)),
            device,
        })
    }

    /// Initialize the model by loading from local cache or downloading from HuggingFace
    pub async fn initialize(&self) -> Result<()> {
        self.initialize_with_progress(None).await
    }

    /// Initialize the model with progress callback
    pub async fn initialize_with_progress(
        &self,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<()> {
        // Check if already initialized
        if self.model.read().await.is_some() {
            if let Some(cb) = progress_callback {
                cb(1.0, "Model already loaded".to_string());
            }
            return Ok(());
        }

        let report_progress = |progress: f32, message: String| {
            if let Some(ref cb) = progress_callback {
                cb(progress, message);
            }
        };

        report_progress(0.0, "Starting model initialization...".to_string());
        tracing::info!("Initializing BERTimbau model...");

        // Download or load from cache
        report_progress(0.1, "Connecting to model repository...".to_string());
        let api = Api::new()?;
        let repo = api.repo(Repo::new(
            "neuralmind/bert-base-portuguese-cased".to_string(),
            RepoType::Model,
        ));

        // Get model files
        report_progress(0.2, "Downloading configuration...".to_string());
        let config_path = repo.get("config.json")?;
        
        report_progress(0.3, "Downloading tokenizer...".to_string());
        let tokenizer_path = repo.get("tokenizer.json")?;
        
        report_progress(0.4, "Downloading model weights...".to_string());
        let weights_path = repo.get("pytorch_model.bin")?;

        // Load tokenizer
        report_progress(0.6, "Loading tokenizer...".to_string());
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;

        // Load model configuration
        report_progress(0.7, "Loading model configuration...".to_string());
        let config: BertConfig = serde_json::from_str(
            &std::fs::read_to_string(config_path)?
        )?;

        // Load model weights
        report_progress(0.8, "Loading model weights...".to_string());
        let vb = VarBuilder::from_pth(&weights_path, candle_core::DType::F32, &self.device)?;
        
        report_progress(0.9, "Initializing model...".to_string());
        let model = BertModel::load(vb, &config)?;

        // Store model and tokenizer
        *self.model.write().await = Some(model);
        *self.tokenizer.write().await = Some(tokenizer);

        report_progress(1.0, "Model loaded successfully!".to_string());
        tracing::info!("BERTimbau model initialized successfully");
        Ok(())
    }

    /// Check if the model is already initialized
    pub async fn is_initialized(&self) -> bool {
        self.model.read().await.is_some()
    }

    /// Score an essay using the AI model
    /// Returns scores for each of the 5 ENEM competencies (0-200 each)
    pub async fn score_essay(
        &self,
        theme: &str,
        content: &str,
    ) -> Result<Vec<u16>> {
        // Ensure model is initialized
        if self.model.read().await.is_none() {
            self.initialize().await?;
        }

        let model_guard = self.model.read().await;
        let tokenizer_guard = self.tokenizer.read().await;

        let model = model_guard.as_ref()
            .context("Model not initialized")?;
        let tokenizer = tokenizer_guard.as_ref()
            .context("Tokenizer not initialized")?;

        // Prepare input text with theme separator
        let input_text = format!("{} <SEP> {}", theme, content);

        // Tokenize input
        let encoding = tokenizer
            .encode(input_text, true)
            .map_err(|e| anyhow::anyhow!("Tokenization failed: {}", e))?;

        let tokens = encoding.get_ids();
        let token_ids = Tensor::new(tokens, &self.device)?
            .unsqueeze(0)?; // Add batch dimension

        // Run inference
        let token_type_ids = Tensor::zeros(token_ids.shape(), candle_core::DType::U32, &self.device)?;
        let outputs = model.forward(&token_ids, &token_type_ids, None)?;
        
        // Extract CLS token embedding (first token)
        let cls_embedding = outputs.get(0)?.get(0)?;

        // For now, use a simple heuristic-based scoring
        // In production, this would use a fine-tuned regression head
        let scores = self.heuristic_scoring(&cls_embedding, content).await?;

        Ok(scores)
    }

    /// Heuristic-based scoring (placeholder for fine-tuned model)
    /// This is a simplified version that will be replaced with actual model output
    async fn heuristic_scoring(
        &self,
        _embedding: &Tensor,
        content: &str,
    ) -> Result<Vec<u16>> {
        // Simple heuristics based on essay characteristics
        let word_count = content.split_whitespace().count();
        let sentence_count = content.split(&['.', '!', '?'][..]).count();
        let paragraph_count = content.split("\n\n").count();

        // C1: Grammar and formal writing (based on length and structure)
        let c1 = if word_count > 250 && word_count < 350 {
            160
        } else if word_count >= 200 {
            120
        } else {
            80
        };

        // C2: Theme comprehension (based on structure)
        let c2 = if paragraph_count >= 4 {
            160
        } else if paragraph_count >= 3 {
            120
        } else {
            80
        };

        // C3: Argument organization
        let c3 = if sentence_count >= 15 && word_count > 250 {
            160
        } else {
            120
        };

        // C4: Linguistic mechanisms (based on average sentence length)
        let avg_sentence_len = if sentence_count > 0 {
            word_count / sentence_count
        } else {
            0
        };
        let c4 = if avg_sentence_len >= 15 && avg_sentence_len <= 25 {
            160
        } else {
            120
        };

        // C5: Intervention proposal (check for proposal indicators)
        let has_proposal = content.to_lowercase().contains("proposta") ||
                           content.to_lowercase().contains("solução") ||
                           content.to_lowercase().contains("necessário");
        let c5 = if has_proposal {
            160
        } else {
            80
        };

        Ok(vec![c1, c2, c3, c4, c5])
    }
}

impl Default for AIService {
    fn default() -> Self {
        Self::new().expect("Failed to create AI service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_service_creation() {
        let service = AIService::new();
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_heuristic_scoring() {
        let service = AIService::new().unwrap();
        let content = "Este é um exemplo de redação com várias palavras para testar \
                      a pontuação heurística. A redação tem múltiplos parágrafos.

\
                      Este é o segundo parágrafo com mais conteúdo.

\
                      Terceiro parágrafo com uma proposta de solução.

\
                      Conclusão do texto.";
        
        let device = Device::Cpu;
        let dummy_tensor = Tensor::zeros((768,), candle_core::DType::F32, &device).unwrap();
        let scores = service.heuristic_scoring(&dummy_tensor, content).await;
        
        assert!(scores.is_ok());
        let scores = scores.unwrap();
        assert_eq!(scores.len(), 5);
        assert!(scores.iter().all(|&s| s <= 200));
    }
}

