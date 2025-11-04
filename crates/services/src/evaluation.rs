use anyhow::{Context, Result};
use chrono::Utc;
use domain::essay::{
    Correction, Essay, EssayStatus, ExamType, RubricScores,
};
use std::collections::HashMap;

use crate::ai::AIService;
use crate::rubrics::{get_rubric, get_enem_score_level};

/// Evaluation Service for orchestrating essay evaluation
pub struct EvaluationService {
    ai_service: AIService,
}

impl EvaluationService {
    /// Create a new evaluation service
    pub fn new() -> Result<Self> {
        let ai_service = AIService::new()?;
        Ok(Self { ai_service })
    }

    /// Evaluate an essay and return updated essay with scores and feedback
    pub async fn evaluate_essay(&self, mut essay: Essay) -> Result<Essay> {
        // Get rubric for exam type
        let rubric = get_rubric(&essay.exam_type)
            .context("Rubric not found for exam type")?;

        // Extract theme from title (in production, this would be more sophisticated)
        let theme = &essay.title;

        // Score the essay using AI
        let competency_scores = self.ai_service
            .score_essay(theme, &essay.content)
            .await
            .context("Failed to score essay")?;

        // Build rubric scores
        let mut scores = HashMap::new();
        let mut detailed_feedback = HashMap::new();
        let mut total_score = 0u16;

        for (idx, criterion) in rubric.criteria.iter().enumerate() {
            let score = competency_scores.get(idx).copied().unwrap_or(0);
            total_score += score;
            
            scores.insert(criterion.name.clone(), score);
            
            // Generate feedback for this competency
            let feedback = self.generate_competency_feedback(
                &criterion.name,
                score,
                &essay.exam_type,
            );
            detailed_feedback.insert(criterion.name.clone(), feedback);
        }

        // Generate corrections (basic implementation)
        let corrections = self.generate_corrections(&essay.content, &competency_scores);

        // Generate overall feedback
        let overall_feedback = self.generate_overall_feedback(
            &essay.exam_type,
            &competency_scores,
        );

        // Update essay
        essay.score = Some(total_score);
        essay.max_score = rubric.max_score;
        essay.rubric_scores = Some(RubricScores {
            scores,
            detailed_feedback,
        });
        essay.corrections = Some(corrections);
        essay.feedback = Some(overall_feedback);
        essay.status = EssayStatus::Corrigida;
        essay.updated_at = Utc::now();

        Ok(essay)
    }

    /// Generate feedback for a specific competency
    fn generate_competency_feedback(
        &self,
        criterion: &str,
        score: u16,
        exam_type: &ExamType,
    ) -> String {
        match exam_type {
            ExamType::Enem => self.generate_enem_competency_feedback(criterion, score),
            _ => format!("Critério {}: {} pontos", criterion, score),
        }
    }

    /// Generate ENEM-specific competency feedback
    fn generate_enem_competency_feedback(&self, criterion: &str, score: u16) -> String {
        let level_desc = get_enem_score_level(score);
        
        let specific_feedback = match criterion {
            "C1" => match score {
                200 => "Excelente domínio da norma culta da língua portuguesa. Uso adequado de gramática, ortografia e pontuação.",
                160 => "Bom domínio da norma culta, com poucos desvios gramaticais.",
                120 => "Domínio razoável, mas com alguns desvios que comprometem parcialmente a qualidade.",
                80 => "Domínio insuficiente da norma culta, com muitos desvios gramaticais.",
                40 => "Domínio muito precrário da norma culta.",
                _ => "Ausência de domínio da norma culta.",
            },
            "C2" => match score {
                200 => "Excelente compreensão do tema e desenvolvimento argumentativo consistente.",
                160 => "Boa compreensão do tema com desenvolvimento adequado.",
                120 => "Compreensão razoável do tema, desenvolvimento parcial.",
                80 => "Compreensão superficial do tema.",
                40 => "Compreensão muito limitada do tema.",
                _ => "Não demonstrou compreensão do tema.",
            },
            "C3" => match score {
                200 => "Excelente seleção e organização de argumentos. Defesa consistente do ponto de vista.",
                160 => "Boa organização argumentativa com argumentos relevantes.",
                120 => "Organização razoável dos argumentos.",
                80 => "Organização insuficiente dos argumentos.",
                40 => "Organização muito precrária.",
                _ => "Ausência de organização argumentativa.",
            },
            "C4" => match score {
                200 => "Excelente articulação de ideias com uso adequado de conectivos e mecanismos coesivos.",
                160 => "Boa articulação com uso adequado de conectivos.",
                120 => "Articulação razoável entre as ideias.",
                80 => "Articulação insuficiente das ideias.",
                40 => "Articulação muito precrária.",
                _ => "Ausência de articulação.",
            },
            "C5" => match score {
                200 => "Proposta de intervenção completa e detalhada, respeitando os direitos humanos.",
                160 => "Boa proposta de intervenção com detalhamento adequado.",
                120 => "Proposta razoável, mas com falta de detalhamento.",
                80 => "Proposta insuficiente ou pouco detalhada.",
                40 => "Proposta muito precrária.",
                _ => "Ausência de proposta de intervenção.",
            },
            _ => "Feedback não disponível.",
        };

        format!("{} ({} pontos)\n{}", level_desc, score, specific_feedback)
    }

    /// Generate overall feedback for the essay
    fn generate_overall_feedback(
        &self,
        exam_type: &ExamType,
        scores: &[u16],
    ) -> String {
        let total: u16 = scores.iter().sum();
        let avg = if !scores.is_empty() {
            total / scores.len() as u16
        } else {
            0
        };

        let performance_level = match avg {
            180..=200 => "excelente",
            140..=179 => "bom",
            100..=139 => "razoável",
            60..=99 => "insuficiente",
            _ => "precário",
        };

        match exam_type {
            ExamType::Enem => {
                format!(
                    "Pontuação total: {}/1000\n\n\
                    Desempenho geral: {}\n\n\
                    Sua redação demonstrou um desempenho {} nas competências avaliadas. \
                    Continue praticando e atenção aos pontos que precisam de melhoria em cada competência.\n\n\
                    Dicas gerais:\n\
                    - Revise a estrutura dissertativo-argumentativa\n\
                    - Pratique o uso de conectivos para melhorar a coesão\n\
                    - Desenvolva propostas de intervenção mais detalhadas\n\
                    - Amplie seu repertório sociocultural com leituras diversas",
                    total,
                    performance_level,
                    performance_level
                )
            }
            _ => format!("Pontuação total: {}. Desempenho: {}", total, performance_level),
        }
    }

    /// Generate basic corrections (placeholder for more sophisticated analysis)
    fn generate_corrections(
        &self,
        content: &str,
        _scores: &[u16],
    ) -> Vec<Correction> {
        let mut corrections = Vec::new();

        // Simple example: check for common issues
        if !content.to_lowercase().contains("portanto") &&
           !content.to_lowercase().contains("assim") &&
           !content.to_lowercase().contains("dessa forma") {
            corrections.push(Correction {
                position: 0,
                original_text: "[Início do texto]".to_string(),
                suggested_text: "Considere usar conectivos conclusivos como 'portanto', 'assim' ou 'dessa forma'".to_string(),
                reason: "Conectivos ajudam na articulação de ideias".to_string(),
                rubric_criterion: "C4".to_string(),
            });
        }

        corrections
    }
}

impl Default for EvaluationService {
    fn default() -> Self {
        Self::new().expect("Failed to create evaluation service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_enem_feedback() {
        let service = EvaluationService::new().unwrap();
        let feedback = service.generate_enem_competency_feedback("C1", 160);
        assert!(feedback.contains("Bom"));
        assert!(feedback.contains("160"));
    }

    #[test]
    fn test_generate_overall_feedback() {
        let service = EvaluationService::new().unwrap();
        let scores = vec![160, 160, 160, 160, 160];
        let feedback = service.generate_overall_feedback(&ExamType::Enem, &scores);
        assert!(feedback.contains("800/1000"));
    }
}

