use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Subject {
    LinguaPortuguesa,
    Literatura,
    Ingles,
    Espanhol,
    Artes,
    EducacaoFisica,
    Tic,
    Historia,
    Geografia,
    Filosofia,
    Sociologia,
    Fisica,
    Quimica,
    Biologia,
    Matematica,
    Redacao,
}

impl Subject {
    pub fn display_name(&self) -> &'static str {
        match self {
            Subject::LinguaPortuguesa => "Língua Portuguesa",
            Subject::Literatura => "Literatura",
            Subject::Ingles => "Inglês",
            Subject::Espanhol => "Espanhol",
            Subject::Artes => "Artes",
            Subject::EducacaoFisica => "Educação Física",
            Subject::Tic => "TIC",
            Subject::Historia => "História",
            Subject::Geografia => "Geografia",
            Subject::Filosofia => "Filosofia",
            Subject::Sociologia => "Sociologia",
            Subject::Fisica => "Física",
            Subject::Quimica => "Química",
            Subject::Biologia => "Biologia",
            Subject::Matematica => "Matemática",
            Subject::Redacao => "Redação",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Difficulty {
    Facil,
    Medio,
    Dificil,
}

impl Difficulty {
    pub fn display_name(&self) -> &'static str {
        match self {
            Difficulty::Facil => "Fácil",
            Difficulty::Medio => "Médio",
            Difficulty::Dificil => "Difícil",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub subject: Subject,
    pub difficulty: Difficulty,
    pub statement: String,
    pub alternatives: Vec<Alternative>,
    pub correct_answer: usize,
    pub explanation: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub id: usize,
    pub text: String,
}

