use domain::essay::{ExamRubric, ExamType, RubricCriterion};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Global rubric definitions for all exam types
pub static EXAM_RUBRICS: Lazy<HashMap<ExamType, ExamRubric>> = Lazy::new(|| {
    let mut rubrics = HashMap::new();
    
    // ENEM Rubric - 5 competencies, each 0-200 points
    rubrics.insert(
        ExamType::Enem,
        ExamRubric {
            exam_type: ExamType::Enem,
            max_score: 1000,
            description: "Redação do ENEM avaliada em 5 competências".to_string(),
            criteria: vec![
                RubricCriterion {
                    name: "C1".to_string(),
                    description: "Domínio da modalidade escrita formal da língua portuguesa".to_string(),
                    weight: 0.2,
                    max_score: 200,
                    evaluation_points: vec![
                        "Gramática e ortografia".to_string(),
                        "Sintaxe e concordância".to_string(),
                        "Pontuação e acentuação".to_string(),
                        "Uso adequado de conectivos".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "C2".to_string(),
                    description: "Compreensão da proposta de redação e aplicação de conhecimentos".to_string(),
                    weight: 0.2,
                    max_score: 200,
                    evaluation_points: vec![
                        "Adequação ao tema proposto".to_string(),
                        "Estrutura dissertativo-argumentativa".to_string(),
                        "Uso de conhecimentos de diferentes áreas".to_string(),
                        "Não fuga ao tema".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "C3".to_string(),
                    description: "Seleção, relação, organização e interpretação de informações".to_string(),
                    weight: 0.2,
                    max_score: 200,
                    evaluation_points: vec![
                        "Coerência lógica".to_string(),
                        "Qualidade dos argumentos".to_string(),
                        "Repertório sociocultural".to_string(),
                        "Defesa de ponto de vista".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "C4".to_string(),
                    description: "Conhecimento dos mecanismos linguísticos de argumentação".to_string(),
                    weight: 0.2,
                    max_score: 200,
                    evaluation_points: vec![
                        "Coesão textual".to_string(),
                        "Uso de conectivos e operadores argumentativos".to_string(),
                        "Progressão temática".to_string(),
                        "Articulação entre parágrafos".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "C5".to_string(),
                    description: "Proposta de intervenção respeitando direitos humanos".to_string(),
                    weight: 0.2,
                    max_score: 200,
                    evaluation_points: vec![
                        "Proposta clara e viável".to_string(),
                        "Detalhamento da ação".to_string(),
                        "Identificação de agentes sociais".to_string(),
                        "Respeito aos direitos humanos".to_string(),
                    ],
                },
            ],
        },
    );
    
    // FUVEST Rubric - Different structure
    rubrics.insert(
        ExamType::Fuvest,
        ExamRubric {
            exam_type: ExamType::Fuvest,
            max_score: 48,
            description: "Redação da FUVEST avaliada em múltiplos critérios".to_string(),
            criteria: vec![
                RubricCriterion {
                    name: "Estrutura".to_string(),
                    description: "Organização textual e estrutura argumentativa".to_string(),
                    weight: 0.33,
                    max_score: 16,
                    evaluation_points: vec![
                        "Introdução, desenvolvimento e conclusão".to_string(),
                        "Progressão de ideias".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Conteúdo".to_string(),
                    description: "Qualidade argumentativa e repertório".to_string(),
                    weight: 0.33,
                    max_score: 16,
                    evaluation_points: vec![
                        "Argumentos consistentes".to_string(),
                        "Conhecimento do tema".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Linguagem".to_string(),
                    description: "Norma culta e coesão".to_string(),
                    weight: 0.34,
                    max_score: 16,
                    evaluation_points: vec![
                        "Gramática e ortografia".to_string(),
                        "Coesão e coerência".to_string(),
                    ],
                },
            ],
        },
    );
    
    rubrics
});

/// Get rubric for a specific exam type
pub fn get_rubric(exam_type: &ExamType) -> Option<&'static ExamRubric> {
    EXAM_RUBRICS.get(exam_type)
}

/// Get ENEM score level descriptions
pub fn get_enem_score_level(score: u16) -> &'static str {
    match score {
        0 => "Ausência completa da competência ou desclassificação",
        40 => "Demonstração muito fraca",
        80 => "Demonstração fraca com deficiências significativas",
        120 => "Demonstração razoável com algumas deficiências",
        160 => "Boa demonstração com deficiências menores",
        200 => "Excelente demonstração da competência",
        _ => "Pontuação inválida",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enem_rubric_exists() {
        let rubric = get_rubric(&ExamType::Enem);
        assert!(rubric.is_some());
        let rubric = rubric.unwrap();
        assert_eq!(rubric.max_score, 1000);
        assert_eq!(rubric.criteria.len(), 5);
    }

    #[test]
    fn test_enem_criteria_scores() {
        let rubric = get_rubric(&ExamType::Enem).unwrap();
        for criterion in &rubric.criteria {
            assert_eq!(criterion.max_score, 200);
        }
    }
}
