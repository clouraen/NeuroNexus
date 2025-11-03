use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExamType {
    Enem,
    Fuvest,
    Unicamp,
    Unesp,
    Uerj,
    Ita,
    Ime,
    Ufrj,
    Ufmg,
    Ufsc,
    Ufrgs,
    Ufpr,
    Ufscar,
    Ufpe,
    Ufba,
    Ufc,
    Ufpa,
    Ufam,
    Ufac,
    Ufrn,
    Ufms,
    Ufg,
    Ufes,
    Ufjf,
    Ufv,
    Uftm,
    Ufpb,
    Ufpi,
    Ufma,
    Ufs,
    Uft,
    Ufopa,
    Unb,
    Ueg,
    Uema,
    Uece,
    Uern,
    Uepb,
    Uepg,
    Uel,
    Unemat,
    Uerr,
    Uea,
    Ufrr,
    Upe,
    Uenf,
    Unifesp,
    Uesc,
    Uemg,
    Uem,
    Uesb,
    Uespi,
    Uesf,
    Uesr,
    Uesg,
}

impl ExamType {
    pub fn max_score(&self) -> u16 {
        match self {
            ExamType::Enem | ExamType::Ufmg | ExamType::Ufpe | ExamType::Ufba
            | ExamType::Ufc | ExamType::Ufpa | ExamType::Ufam | ExamType::Ufac
            | ExamType::Ufrn | ExamType::Ufms | ExamType::Ufes | ExamType::Ufpb
            | ExamType::Ufpi | ExamType::Ufma | ExamType::Ufopa | ExamType::Uema
            | ExamType::Uece | ExamType::Uern | ExamType::Unemat | ExamType::Uerr
            | ExamType::Uea | ExamType::Ufrr | ExamType::Upe => 1000,
            ExamType::Fuvest => 48,
            ExamType::Unicamp => 60,
            ExamType::Unesp => 40,
            ExamType::Uerj => 20,
            ExamType::Ita => 50,
            ExamType::Ime => 100,
            ExamType::Ufrj => 10,
            _ => 100,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ExamType::Enem => "ENEM",
            ExamType::Fuvest => "FUVEST (USP)",
            ExamType::Unicamp => "UNICAMP",
            ExamType::Unesp => "UNESP",
            ExamType::Uerj => "UERJ",
            ExamType::Ita => "ITA",
            ExamType::Ime => "IME",
            ExamType::Ufrj => "UFRJ",
            ExamType::Ufmg => "UFMG",
            ExamType::Ufsc => "UFSC",
            ExamType::Ufrgs => "UFRGS",
            ExamType::Ufpr => "UFPR",
            ExamType::Ufscar => "UFSCAR",
            ExamType::Ufpe => "UFPE",
            ExamType::Ufba => "UFBA",
            ExamType::Ufc => "UFC",
            ExamType::Ufpa => "UFPA",
            ExamType::Ufam => "UFAM",
            ExamType::Ufac => "UFAC",
            ExamType::Ufrn => "UFRN",
            ExamType::Ufms => "UFMS",
            ExamType::Ufg => "UFG",
            ExamType::Ufes => "UFES",
            ExamType::Ufjf => "UFJF",
            ExamType::Ufv => "UFV",
            ExamType::Uftm => "UFTM",
            ExamType::Ufpb => "UFPB",
            ExamType::Ufpi => "UFPI",
            ExamType::Ufma => "UFMA",
            ExamType::Ufs => "UFS",
            ExamType::Uft => "UFT",
            ExamType::Ufopa => "UFOPA",
            ExamType::Unb => "UNB",
            ExamType::Ueg => "UEG",
            ExamType::Uema => "UEMA",
            ExamType::Uece => "UECE",
            ExamType::Uern => "UERN",
            ExamType::Uepb => "UEPB",
            ExamType::Uepg => "UEPG",
            ExamType::Uel => "UEL",
            ExamType::Unemat => "UNEMAT",
            ExamType::Uerr => "UERR",
            ExamType::Uea => "UEA",
            ExamType::Ufrr => "UFRR",
            ExamType::Upe => "UPE",
            ExamType::Uenf => "UENF",
            ExamType::Unifesp => "UNIFESP",
            ExamType::Uesc => "UESC",
            ExamType::Uemg => "UEMG",
            ExamType::Uem => "UEM",
            ExamType::Uesb => "UESB",
            ExamType::Uespi => "UESPI",
            ExamType::Uesf => "UESF",
            ExamType::Uesr => "UESR",
            ExamType::Uesg => "UESG",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EssayStatus {
    EmProgresso,
    Corrigida,
    Enviada,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Essay {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub exam_type: ExamType,
    pub status: EssayStatus,
    pub score: Option<u16>,
    pub max_score: u16,
    pub feedback: Option<String>,
    pub corrections: Option<Vec<Correction>>,
    pub rubric_scores: Option<RubricScores>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    pub position: usize,
    pub original_text: String,
    pub suggested_text: String,
    pub reason: String,
    pub rubric_criterion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricScores {
    pub scores: HashMap<String, u16>,
    pub detailed_feedback: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamRubric {
    pub exam_type: ExamType,
    pub criteria: Vec<RubricCriterion>,
    pub max_score: u16,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricCriterion {
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub max_score: u16,
    pub evaluation_points: Vec<String>,
}

