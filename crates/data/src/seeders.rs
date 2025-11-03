use chrono::{Utc, Duration};
use uuid::Uuid;
use domain::{
    essay::{Essay, EssayStatus, ExamType, RubricScores, ExamRubric, RubricCriterion},
    question::{Question, Subject, Difficulty, Alternative},
    user::{UserProfile, UserSettings, Theme, Language},
    knowledge_trail::{KnowledgeTrail, TrailModule, ContentType},
    traits::*,
};
use shared::Result;
use std::collections::HashMap;
use super::repositories::*;

/// Função principal para popular todos os repositórios com dados de teste
/// Aceita repositórios concretos para permitir inserção direta
pub async fn seed_all_data(
    essay_repo: &InMemoryEssayRepository,
    question_repo: &InMemoryQuestionRepository,
    user_repo: &InMemoryUserRepository,
    trail_repo: &InMemoryKnowledgeTrailRepository,
    rubric_repo: &InMemoryExamRubricRepository,
) -> Result<Uuid> {
    // Criar usuário de teste
    let user_id = seed_user(user_repo).await?;
    
    // Popular questões
    seed_questions(question_repo).await?;
    
    // Popular redações
    seed_essays(essay_repo, user_id).await?;
    
    // Popular trilhas
    seed_knowledge_trails(trail_repo, user_id).await?;
    
    // Popular rubricas de avaliação
    seed_rubrics(rubric_repo).await?;
    
    Ok(user_id)
}

/// Cria um usuário de teste
pub async fn seed_user(user_repo: &InMemoryUserRepository) -> Result<Uuid> {
    let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    
    let user = UserProfile {
        id: user_id,
        name: "Estudante Exemplo".to_string(),
        avatar_url: None,
        email: "estudante@neuronexus.app".to_string(),
        created_at: Utc::now() - Duration::days(30),
        settings: UserSettings {
            theme: Theme::Cyberpunk,
            notifications_enabled: true,
            study_reminders: true,
            language: Language::Portuguese,
        },
    };
    
    user_repo.update(user).await?;
    Ok(user_id)
}

/// Cria questões de exemplo com dados reais
pub async fn seed_questions(question_repo: &InMemoryQuestionRepository) -> Result<()> {
    let questions = vec![
        // Matemática
        Question {
            id: Uuid::parse_str("10000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::Matematica,
            difficulty: Difficulty::Medio,
            statement: "Uma função quadrática f(x) = ax² + bx + c tem raízes em x = 2 e x = -3. Se f(0) = 6, qual é o valor de a?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "a = -1".to_string() },
                Alternative { id: 1, text: "a = 1".to_string() },
                Alternative { id: 2, text: "a = 2".to_string() },
                Alternative { id: 3, text: "a = -2".to_string() },
                Alternative { id: 4, text: "a = 3".to_string() },
            ],
            correct_answer: 0,
            explanation: "Sabendo que f(0) = c = 6 e que as raízes são 2 e -3, temos: f(x) = a(x-2)(x+3) = a(x² + x - 6). Como f(0) = -6a = 6, então a = -1.".to_string(),
            tags: vec!["função quadrática".to_string(), "raízes".to_string(), "álgebra".to_string()],
        },
        Question {
            id: Uuid::parse_str("10000000-0000-0000-0000-000000000002").unwrap(),
            subject: Subject::Matematica,
            difficulty: Difficulty::Facil,
            statement: "Qual é o valor de x na equação 2x + 5 = 15?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "x = 3".to_string() },
                Alternative { id: 1, text: "x = 5".to_string() },
                Alternative { id: 2, text: "x = 10".to_string() },
                Alternative { id: 3, text: "x = 7".to_string() },
                Alternative { id: 4, text: "x = 4".to_string() },
            ],
            correct_answer: 1,
            explanation: "Para resolver: 2x + 5 = 15, subtraímos 5 de ambos os lados: 2x = 10. Dividindo por 2: x = 5.".to_string(),
            tags: vec!["equação do primeiro grau".to_string(), "álgebra".to_string()],
        },
        Question {
            id: Uuid::parse_str("10000000-0000-0000-0000-000000000003").unwrap(),
            subject: Subject::Matematica,
            difficulty: Difficulty::Dificil,
            statement: "Se log₂(x) + log₂(x+2) = 3, qual é o valor de x?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "x = 2".to_string() },
                Alternative { id: 1, text: "x = 4".to_string() },
                Alternative { id: 2, text: "x = 6".to_string() },
                Alternative { id: 3, text: "x = 8".to_string() },
                Alternative { id: 4, text: "x = -4 (não válido)".to_string() },
            ],
            correct_answer: 0,
            explanation: "Usando a propriedade dos logaritmos: log₂(x) + log₂(x+2) = log₂(x(x+2)) = 3. Logo, x(x+2) = 2³ = 8. Resolvendo: x² + 2x - 8 = 0. As raízes são x = 2 e x = -4. Como x > 0 no domínio do logaritmo, x = 2.".to_string(),
            tags: vec!["logaritmos".to_string(), "equação logarítmica".to_string()],
        },
        
        // História
        Question {
            id: Uuid::parse_str("20000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::Historia,
            difficulty: Difficulty::Facil,
            statement: "Em que ano o Brasil foi descoberto por Pedro Álvares Cabral?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "1498".to_string() },
                Alternative { id: 1, text: "1500".to_string() },
                Alternative { id: 2, text: "1492".to_string() },
                Alternative { id: 3, text: "1502".to_string() },
                Alternative { id: 4, text: "1495".to_string() },
            ],
            correct_answer: 1,
            explanation: "O Brasil foi descoberto em 22 de abril de 1500 pela expedição comandada por Pedro Álvares Cabral, que estava a caminho das Índias.".to_string(),
            tags: vec!["história do brasil".to_string(), "descobrimento".to_string(), "colonização".to_string()],
        },
        Question {
            id: Uuid::parse_str("20000000-0000-0000-0000-000000000002").unwrap(),
            subject: Subject::Historia,
            difficulty: Difficulty::Medio,
            statement: "Qual foi o principal motivo da Inconfidência Mineira?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "Abolição da escravidão".to_string() },
                Alternative { id: 1, text: "Desejo de independência e insatisfação com os impostos".to_string() },
                Alternative { id: 2, text: "Proclamação da República".to_string() },
                Alternative { id: 3, text: "Expulsão dos jesuítas".to_string() },
                Alternative { id: 4, text: "Transferência da capital para o Rio de Janeiro".to_string() },
            ],
            correct_answer: 1,
            explanation: "A Inconfidência Mineira (1789) foi motivada principalmente pela insatisfação com os altos impostos portugueses, especialmente a derrama, e pelo desejo de independência da região de Minas Gerais.".to_string(),
            tags: vec!["história do brasil".to_string(), "inconfidência mineira".to_string(), "período colonial".to_string()],
        },
        
        // Física
        Question {
            id: Uuid::parse_str("30000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::Fisica,
            difficulty: Difficulty::Medio,
            statement: "Um objeto é lançado verticalmente para cima com velocidade inicial de 20 m/s. Desprezando a resistência do ar, qual é a altura máxima atingida? (Considere g = 10 m/s²)".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "10 m".to_string() },
                Alternative { id: 1, text: "20 m".to_string() },
                Alternative { id: 2, text: "30 m".to_string() },
                Alternative { id: 3, text: "40 m".to_string() },
                Alternative { id: 4, text: "50 m".to_string() },
            ],
            correct_answer: 1,
            explanation: "Usando a equação de Torricelli: v² = v₀² + 2aΔs. No ponto de altura máxima, v = 0. Logo: 0 = 20² - 2(10)h. Portanto, h = 400/20 = 20 m.".to_string(),
            tags: vec!["mecânica".to_string(), "lançamento vertical".to_string(), "cinemática".to_string()],
        },
        Question {
            id: Uuid::parse_str("30000000-0000-0000-0000-000000000002").unwrap(),
            subject: Subject::Fisica,
            difficulty: Difficulty::Dificil,
            statement: "Um capacitor de 10 μF é carregado até 100 V e depois desconectado da fonte. Se conectarmos a ele um capacitor de 5 μF descarregado em paralelo, qual será a tensão final?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "50 V".to_string() },
                Alternative { id: 1, text: "66,7 V".to_string() },
                Alternative { id: 2, text: "75 V".to_string() },
                Alternative { id: 3, text: "100 V".to_string() },
                Alternative { id: 4, text: "33,3 V".to_string() },
            ],
            correct_answer: 1,
            explanation: "A carga inicial é Q = CV = 10×10⁻⁶ × 100 = 10⁻³ C. A capacitância equivalente em paralelo é C_eq = 10 + 5 = 15 μF. Como a carga se conserva, V_final = Q/C_eq = 10⁻³/(15×10⁻⁶) = 66,7 V.".to_string(),
            tags: vec!["eletricidade".to_string(), "capacitores".to_string(), "circuitos".to_string()],
        },
        
        // Química
        Question {
            id: Uuid::parse_str("40000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::Quimica,
            difficulty: Difficulty::Medio,
            statement: "Qual é o pH de uma solução 0,01 mol/L de HCl?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "pH = 1".to_string() },
                Alternative { id: 1, text: "pH = 2".to_string() },
                Alternative { id: 2, text: "pH = 7".to_string() },
                Alternative { id: 3, text: "pH = 12".to_string() },
                Alternative { id: 4, text: "pH = 13".to_string() },
            ],
            correct_answer: 1,
            explanation: "HCl é um ácido forte que se dissocia completamente. Portanto, [H⁺] = 0,01 mol/L = 10⁻² mol/L. Logo, pH = -log[H⁺] = -log(10⁻²) = 2.".to_string(),
            tags: vec!["química analítica".to_string(), "pH".to_string(), "ácidos e bases".to_string()],
        },
        
        // Biologia
        Question {
            id: Uuid::parse_str("50000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::Biologia,
            difficulty: Difficulty::Medio,
            statement: "Durante a fotossíntese, em qual organela celular ocorre a produção de glicose?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "Mitocôndria".to_string() },
                Alternative { id: 1, text: "Cloroplasto".to_string() },
                Alternative { id: 2, text: "Núcleo".to_string() },
                Alternative { id: 3, text: "Retículo endoplasmático".to_string() },
                Alternative { id: 4, text: "Complexo de Golgi".to_string() },
            ],
            correct_answer: 1,
            explanation: "A fotossíntese ocorre nos cloroplastos, organelas que contêm clorofila e outros pigmentos necessários para capturar a energia luminosa e convertê-la em energia química armazenada na glicose.".to_string(),
            tags: vec!["biologia celular".to_string(), "fotossíntese".to_string(), "organelas".to_string()],
        },
        
        // Literatura
        Question {
            id: Uuid::parse_str("60000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::Literatura,
            difficulty: Difficulty::Medio,
            statement: "Qual é a obra mais conhecida de Machado de Assis?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "O Guarani".to_string() },
                Alternative { id: 1, text: "Dom Casmurro".to_string() },
                Alternative { id: 2, text: "Iracema".to_string() },
                Alternative { id: 3, text: "Macunaíma".to_string() },
                Alternative { id: 4, text: "O Cortiço".to_string() },
            ],
            correct_answer: 1,
            explanation: "Dom Casmurro, publicado em 1899, é uma das obras mais famosas de Machado de Assis e da literatura brasileira, narrando a história de Bentinho e Capitu com sua prosa irônica e psicológica característica.".to_string(),
            tags: vec!["literatura brasileira".to_string(), "machado de assis".to_string(), "romance".to_string()],
        },
        
        // Geografia
        Question {
            id: Uuid::parse_str("70000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::Geografia,
            difficulty: Difficulty::Facil,
            statement: "Qual é o maior bioma brasileiro?".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "Mata Atlântica".to_string() },
                Alternative { id: 1, text: "Amazônia".to_string() },
                Alternative { id: 2, text: "Cerrado".to_string() },
                Alternative { id: 3, text: "Caatinga".to_string() },
                Alternative { id: 4, text: "Pampa".to_string() },
            ],
            correct_answer: 1,
            explanation: "A Amazônia é o maior bioma brasileiro, ocupando cerca de 49% do território nacional, além de se estender por outros países da América do Sul.".to_string(),
            tags: vec!["geografia do brasil".to_string(), "biomas".to_string(), "amazônia".to_string()],
        },
        
        // Português
        Question {
            id: Uuid::parse_str("80000000-0000-0000-0000-000000000001").unwrap(),
            subject: Subject::LinguaPortuguesa,
            difficulty: Difficulty::Medio,
            statement: "Assinale a alternativa em que todas as palavras estão corretamente acentuadas:".to_string(),
            alternatives: vec![
                Alternative { id: 0, text: "ideia, jiboia, assembleia".to_string() },
                Alternative { id: 1, text: "ideia, jibóia, assembleia".to_string() },
                Alternative { id: 2, text: "idéia, jiboia, assembléia".to_string() },
                Alternative { id: 3, text: "ideia, jiboia, assembléia".to_string() },
                Alternative { id: 4, text: "idéia, jibóia, assembléia".to_string() },
            ],
            correct_answer: 0,
            explanation: "Segundo o Acordo Ortográfico de 2009, 'ideia' não tem mais acento (era 'idéia'), 'jiboia' não tem acento (era 'jibóia'), e 'assembleia' não tem mais acento (era 'assembléia').".to_string(),
            tags: vec!["português".to_string(), "ortografia".to_string(), "acentuação".to_string()],
        },
    ];
    
    // Inserir questões usando o método insert do repositório
    for question in questions {
        question_repo.insert(question).await?;
    }
    
    Ok(())
}

/// Cria redações de exemplo
pub async fn seed_essays(essay_repo: &InMemoryEssayRepository, user_id: Uuid) -> Result<()> {
    let now = Utc::now();
    
    let essays = vec![
        // Redação ENEM em progresso
        Essay {
            id: Uuid::parse_str("a0000000-0000-0000-0000-000000000001").unwrap(),
            user_id,
            title: "Redação ENEM - Desafios da Educação Digital".to_string(),
            content: "A educação digital tem se tornado cada vez mais presente na sociedade contemporânea. Com o avanço das tecnologias, novas formas de ensino e aprendizagem emergem, transformando o cenário educacional brasileiro.

No entanto, diversos desafios precisam ser superados para que a educação digital seja efetivamente implementada. A desigualdade de acesso à internet e aos dispositivos tecnológicos é um dos principais obstáculos enfrentados por estudantes de baixa renda.

Além disso, é necessário capacitar professores para utilizar adequadamente as ferramentas digitais em suas práticas pedagógicas. A formação continuada dos educadores é fundamental para garantir uma transição bem-sucedida para o ensino digital.

Portanto, é essencial que políticas públicas sejam desenvolvidas para democratizar o acesso à tecnologia e promover a inclusão digital, garantindo que todos os estudantes tenham oportunidades iguais de aprendizado.".to_string(),
            exam_type: ExamType::Enem,
            status: EssayStatus::EmProgresso,
            score: None,
            max_score: 1000,
            feedback: None,
            corrections: None,
            rubric_scores: None,
            created_at: now - Duration::days(5),
            updated_at: now - Duration::hours(2),
            submitted_at: None,
        },
        // Redação ENEM corrigida
        Essay {
            id: Uuid::parse_str("a0000000-0000-0000-0000-000000000002").unwrap(),
            user_id,
            title: "Redação ENEM - Caminhos para combater a evasão escolar".to_string(),
            content: "A evasão escolar é um problema grave que afeta milhares de estudantes brasileiros. Diversos fatores contribuem para esse fenômeno, como questões socioeconômicas, falta de interesse e dificuldades de acesso à escola.

Para combater a evasão escolar, é necessário implementar políticas públicas que garantam o acesso à educação, como programas de transferência de renda condicionados à frequência escolar. Além disso, é importante investir em melhorias na infraestrutura escolar e na formação de professores.

A participação da família no processo educacional também é fundamental. Programas que envolvam os pais na vida escolar dos filhos podem aumentar significativamente a permanência dos estudantes na escola.

Dessa forma, é possível construir caminhos para reduzir a evasão escolar e garantir que todos os jovens tenham acesso a uma educação de qualidade.".to_string(),
            exam_type: ExamType::Enem,
            status: EssayStatus::Corrigida,
            score: Some(820),
            max_score: 1000,
            feedback: Some("Boa estrutura argumentativa e domínio da norma padrão. Sugestão: aprofundar mais a proposta de intervenção com ações mais específicas.".to_string()),
            corrections: None,
            rubric_scores: Some(RubricScores {
                scores: {
                    let mut map = HashMap::new();
                    map.insert("Competência 1".to_string(), 160);
                    map.insert("Competência 2".to_string(), 160);
                    map.insert("Competência 3".to_string(), 160);
                    map.insert("Competência 4".to_string(), 180);
                    map.insert("Competência 5".to_string(), 160);
                    map
                },
                detailed_feedback: {
                    let mut map = HashMap::new();
                    map.insert("Competência 1".to_string(), "Bom domínio da escrita formal com poucos desvios.".to_string());
                    map.insert("Competência 2".to_string(), "Tema compreendido adequadamente.".to_string());
                    map.insert("Competência 3".to_string(), "Boa seleção e organização de informações.".to_string());
                    map.insert("Competência 4".to_string(), "Excelente demonstração de conhecimento linguístico.".to_string());
                    map.insert("Competência 5".to_string(), "Proposta de intervenção poderia ser mais detalhada.".to_string());
                    map
                },
            }),
            created_at: now - Duration::days(15),
            updated_at: now - Duration::days(10),
            submitted_at: Some(now - Duration::days(10)),
        },
        // Redação FUVEST
        Essay {
            id: Uuid::parse_str("a0000000-0000-0000-0000-000000000003").unwrap(),
            user_id,
            title: "Redação FUVEST - Argumentação sobre tecnologia".to_string(),
            content: "A tecnologia transformou profundamente a sociedade contemporânea, criando novas formas de comunicação, trabalho e relacionamento interpessoal.".to_string(),
            exam_type: ExamType::Fuvest,
            status: EssayStatus::EmProgresso,
            score: None,
            max_score: 48,
            feedback: None,
            corrections: None,
            rubric_scores: None,
            created_at: now - Duration::days(2),
            updated_at: now - Duration::hours(12),
            submitted_at: None,
        },
        // Redação UNICAMP corrigida
        Essay {
            id: Uuid::parse_str("a0000000-0000-0000-0000-000000000004").unwrap(),
            user_id,
            title: "Redação UNICAMP - Artigo de Opinião sobre sustentabilidade".to_string(),
            content: "A sustentabilidade ambiental tornou-se uma questão central no debate contemporâneo. É necessário repensar nossos padrões de consumo e produção para garantir um futuro viável para as próximas gerações.".to_string(),
            exam_type: ExamType::Unicamp,
            status: EssayStatus::Corrigida,
            score: Some(52),
            max_score: 60,
            feedback: Some("Excelente artigo de opinião com argumentação sólida e adequação ao gênero textual.".to_string()),
            corrections: None,
            rubric_scores: Some(RubricScores {
                scores: {
                    let mut map = HashMap::new();
                    map.insert("Análise e interpretação".to_string(), 6);
                    map.insert("Argumentação".to_string(), 6);
                    map.insert("Coerência textual".to_string(), 5);
                    map.insert("Coesão textual".to_string(), 5);
                    map.insert("Adequação ao gênero".to_string(), 6);
                    map.insert("Uso adequado da linguagem".to_string(), 5);
                    map.insert("Vocabulário".to_string(), 5);
                    map.insert("Estruturação".to_string(), 5);
                    map.insert("Criatividade".to_string(), 5);
                    map.insert("Ortografia e acentuação".to_string(), 4);
                    map
                },
                detailed_feedback: HashMap::new(),
            }),
            created_at: now - Duration::days(20),
            updated_at: now - Duration::days(18),
            submitted_at: Some(now - Duration::days(18)),
        },
        // Redação UNESP
        Essay {
            id: Uuid::parse_str("a0000000-0000-0000-0000-000000000005").unwrap(),
            user_id,
            title: "Redação UNESP - Desafios da urbanização no Brasil".to_string(),
            content: "O processo de urbanização no Brasil tem se intensificado nas últimas décadas, gerando transformações significativas na organização social e territorial do país. Com mais de 80% da população vivendo em áreas urbanas, os desafios relacionados ao crescimento das cidades tornam-se cada vez mais evidentes.

Um dos principais problemas decorrentes da urbanização acelerada é a formação de aglomerados urbanos desordenados, conhecidos como favelas e periferias. Essas áreas frequentemente carecem de infraestrutura básica, como saneamento, transporte e serviços públicos de qualidade.

A falta de planejamento urbano adequado resulta em problemas de mobilidade, habitação precária e desigualdade social. Muitas cidades brasileiras enfrentam dificuldades para garantir condições dignas de vida a todos os seus habitantes.

Diante desse cenário, é fundamental que o poder público desenvolva políticas urbanísticas que promovam a inclusão social e a sustentabilidade ambiental. Investimentos em habitação popular, transporte público eficiente e espaços públicos de qualidade são essenciais para melhorar a qualidade de vida nas cidades brasileiras.

Portanto, é necessário repensar o modelo de desenvolvimento urbano, priorizando a participação da sociedade civil no planejamento e a implementação de soluções que considerem as necessidades de todos os grupos sociais.".to_string(),
            exam_type: ExamType::Unesp,
            status: EssayStatus::EmProgresso,
            score: None,
            max_score: 40,
            feedback: None,
            corrections: None,
            rubric_scores: None,
            created_at: now - Duration::days(3),
            updated_at: now - Duration::hours(6),
            submitted_at: None,
        },
    ];
    
    for essay in essays {
        essay_repo.save(essay).await?;
    }
    
    Ok(())
}

/// Cria rubricas de avaliação para exames
pub async fn seed_rubrics(rubric_repo: &InMemoryExamRubricRepository) -> Result<()> {
    // Rubrica UNESP - 5 competências, 0-8 pontos cada, total 0-40
    let unesp_rubric = ExamRubric {
        exam_type: ExamType::Unesp,
        max_score: 40,
        description: "Avaliação de redação dissertativa-argumentativa conforme critérios da UNESP. Total de 40 pontos distribuídos em 5 competências (0-8 pontos cada).".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Adequação ao tema".to_string(),
                description: "Avalia se o texto aborda adequadamente o tema proposto e se há fuga parcial ou total do assunto.".to_string(),
                weight: 0.2,
                max_score: 8,
                evaluation_points: vec![
                    "Abordagem completa e adequada ao tema (7-8 pontos)".to_string(),
                    "Abordagem parcial ou com leve desvio (4-6 pontos)".to_string(),
                    "Abordagem insuficiente ou desvio significativo (1-3 pontos)".to_string(),
                    "Fuga total do tema ou texto não desenvolvido (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Gênero textual".to_string(),
                description: "Avalia se o texto está adequado ao gênero dissertativo-argumentativo, com introdução, desenvolvimento e conclusão.".to_string(),
                weight: 0.2,
                max_score: 8,
                evaluation_points: vec![
                    "Gênero adequado com estrutura completa e bem organizada (7-8 pontos)".to_string(),
                    "Gênero adequado com estrutura parcial ou organização razoável (4-6 pontos)".to_string(),
                    "Gênero inadequado ou estrutura confusa (1-3 pontos)".to_string(),
                    "Gênero completamente inadequado (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coerência".to_string(),
                description: "Avalia a consistência lógica do texto, a progressão das ideias e a argumentação.".to_string(),
                weight: 0.2,
                max_score: 8,
                evaluation_points: vec![
                    "Texto coerente com argumentação sólida e lógica (7-8 pontos)".to_string(),
                    "Texto geralmente coerente com pequenas inconsistências (4-6 pontos)".to_string(),
                    "Textos com inconsistências lógicas significativas (1-3 pontos)".to_string(),
                    "Texto incoerente ou sem lógica (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coesão".to_string(),
                description: "Avalia os mecanismos de coesão textual: conectivos, referências, paralelismos e articulação entre parágrafos.".to_string(),
                weight: 0.2,
                max_score: 8,
                evaluation_points: vec![
                    "Coesão adequada com uso correto de conectivos e articulações (7-8 pontos)".to_string(),
                    "Coesão razoável com alguns problemas de articulação (4-6 pontos)".to_string(),
                    "Coesão insuficiente com problemas significativos (1-3 pontos)".to_string(),
                    "Ausência de coesão ou texto fragmentado (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Correção gramatical".to_string(),
                description: "Avalia aspectos gramaticais: ortografia, acentuação, pontuação, concordância, regência e colocação pronominal.".to_string(),
                weight: 0.2,
                max_score: 8,
                evaluation_points: vec![
                    "Domínio excelente da norma padrão com poucos ou nenhum erro (7-8 pontos)".to_string(),
                    "Bom domínio com alguns desvios da norma padrão (4-6 pontos)".to_string(),
                    "Dominio precário com muitos desvios (1-3 pontos)".to_string(),
                    "Dominio muito precário com erros graves e frequentes (0 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(unesp_rubric).await?;
    
    // Rubrica ENEM - 5 competências, 200 pontos cada, total 0-1000
    let enem_rubric = ExamRubric {
        exam_type: ExamType::Enem,
        max_score: 1000,
        description: "Avaliação de redação dissertativa-argumentativa conforme critérios do ENEM. Total de 1000 pontos distribuídos em 5 competências (0-200 pontos cada). Texto mínimo de 30 linhas.".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Competência 1: Domínio da escrita formal".to_string(),
                description: "Avalia o domínio da modalidade escrita formal da Língua Portuguesa, incluindo ortografia, acentuação, pontuação, concordância e regência.".to_string(),
                weight: 0.2,
                max_score: 200,
                evaluation_points: vec![
                    "Domínio excelente da norma padrão (160-200 pontos)".to_string(),
                    "Bom domínio com poucos desvios (120-159 pontos)".to_string(),
                    "Dominio adequado com alguns desvios (80-119 pontos)".to_string(),
                    "Dominio precário com muitos desvios (40-79 pontos)".to_string(),
                    "Dominio muito precário (0-39 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Competência 2: Compreensão da proposta".to_string(),
                description: "Avalia se o candidato compreendeu a proposta de redação e se desenvolveu o tema dentro dos limites estruturais do texto dissertativo-argumentativo.".to_string(),
                weight: 0.2,
                max_score: 200,
                evaluation_points: vec![
                    "Desenvolvimento completo do tema com fuga parcial inexistente (160-200 pontos)".to_string(),
                    "Desenvolvimento adequado com leve fuga parcial (120-159 pontos)".to_string(),
                    "Desenvolvimento adequado com fuga parcial (80-119 pontos)".to_string(),
                    "Desenvolvimento mínimo ou fuga parcial significativa (40-79 pontos)".to_string(),
                    "Fuga total do tema ou texto não desenvolvido (0-39 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Competência 3: Seleção e organização de informações".to_string(),
                description: "Avalia a capacidade de selecionar, relacionar, organizar e interpretar informações, fatos, opiniões e argumentos em defesa de um ponto de vista.".to_string(),
                weight: 0.2,
                max_score: 200,
                evaluation_points: vec![
                    "Informações selecionadas e organizadas de forma excelente (160-200 pontos)".to_string(),
                    "Informações bem selecionadas e organizadas (120-159 pontos)".to_string(),
                    "Informações adequadamente selecionadas e organizadas (80-119 pontos)".to_string(),
                    "Informações selecionadas de forma precária (40-79 pontos)".to_string(),
                    "Informações desorganizadas ou ausentes (0-39 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Competência 4: Demonstração de conhecimento linguístico".to_string(),
                description: "Avalia o conhecimento dos mecanismos linguísticos necessários para a construção da argumentação, incluindo coesão e coerência.".to_string(),
                weight: 0.2,
                max_score: 200,
                evaluation_points: vec![
                    "Demonstração excelente dos mecanismos linguísticos (160-200 pontos)".to_string(),
                    "Demonstração adequada dos mecanismos linguísticos (120-159 pontos)".to_string(),
                    "Demonstração razoável dos mecanismos linguísticos (80-119 pontos)".to_string(),
                    "Demonstração precária dos mecanismos linguísticos (40-79 pontos)".to_string(),
                    "Ausência ou uso inadequado dos mecanismos linguísticos (0-39 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Competência 5: Elaboração de proposta de intervenção".to_string(),
                description: "Avalia a proposta de intervenção para o problema abordado, respeitando os direitos humanos e apresentando ações específicas.".to_string(),
                weight: 0.2,
                max_score: 200,
                evaluation_points: vec![
                    "Proposta de intervenção excelente e detalhada, respeitando direitos humanos (160-200 pontos)".to_string(),
                    "Proposta de intervenção adequada e respeitando direitos humanos (120-159 pontos)".to_string(),
                    "Proposta de intervenção simples mas respeitando direitos humanos (80-119 pontos)".to_string(),
                    "Proposta de intervenção precária ou desrespeitando direitos humanos (40-79 pontos)".to_string(),
                    "Ausência de proposta de intervenção ou desrespeitando direitos humanos (0-39 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(enem_rubric).await?;
    
    // Rubrica FUVEST - 8 competências, 0-6 pontos cada, total 0-48
    let fuvest_rubric = ExamRubric {
        exam_type: ExamType::Fuvest,
        max_score: 48,
        description: "Avaliação de redação argumentativa conforme critérios da FUVEST/USP. Total de 48 pontos distribuídos em 8 competências (0-6 pontos cada). Foco em raciocínio e originalidade.".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Adequação ao tema e gênero".to_string(),
                description: "Avalia se o texto está adequado ao tema proposto e ao gênero textual solicitado.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Adequação completa ao tema e gênero (5-6 pontos)".to_string(),
                    "Adequação adequada com pequenos desvios (3-4 pontos)".to_string(),
                    "Adequação parcial ou desvios significativos (1-2 pontos)".to_string(),
                    "Inadequação total (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coerência e coesão".to_string(),
                description: "Avalia a consistência lógica do texto e os mecanismos de coesão textual.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Excelente coerência e coesão (5-6 pontos)".to_string(),
                    "Boa coerência e coesão (3-4 pontos)".to_string(),
                    "Coerência e coesão razoáveis (1-2 pontos)".to_string(),
                    "Ausência de coerência e coesão (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Domínio da norma padrão".to_string(),
                description: "Avalia o domínio da escrita formal da Língua Portuguesa.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Domínio excelente da norma padrão (5-6 pontos)".to_string(),
                    "Bom domínio com poucos desvios (3-4 pontos)".to_string(),
                    "Domínio adequado com alguns desvios (1-2 pontos)".to_string(),
                    "Domínio precário (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Recursos expressivos".to_string(),
                description: "Avalia o uso adequado de recursos expressivos e figuras de linguagem.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Uso excelente de recursos expressivos (5-6 pontos)".to_string(),
                    "Uso adequado de recursos expressivos (3-4 pontos)".to_string(),
                    "Uso razoável de recursos expressivos (1-2 pontos)".to_string(),
                    "Ausência ou uso inadequado (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Vocabulário".to_string(),
                description: "Avalia a riqueza e precisão do vocabulário utilizado.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Vocabulário rico e preciso (5-6 pontos)".to_string(),
                    "Vocabulário adequado (3-4 pontos)".to_string(),
                    "Vocabulário básico mas adequado (1-2 pontos)".to_string(),
                    "Vocabulário insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Estrutura textual".to_string(),
                description: "Avalia a organização e estruturação do texto.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Estrutura textual excelente (5-6 pontos)".to_string(),
                    "Estrutura textual adequada (3-4 pontos)".to_string(),
                    "Estrutura textual razoável (1-2 pontos)".to_string(),
                    "Estrutura textual inadequada (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Argumentação".to_string(),
                description: "Avalia a qualidade e força dos argumentos apresentados.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Argumentação sólida e convincente (5-6 pontos)".to_string(),
                    "Argumentação adequada (3-4 pontos)".to_string(),
                    "Argumentação básica (1-2 pontos)".to_string(),
                    "Argumentação insuficiente ou ausente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Originalidade".to_string(),
                description: "Avalia a criatividade e originalidade na abordagem do tema.".to_string(),
                weight: 0.125,
                max_score: 6,
                evaluation_points: vec![
                    "Abordagem muito original e criativa (5-6 pontos)".to_string(),
                    "Abordagem original (3-4 pontos)".to_string(),
                    "Abordagem convencional mas adequada (1-2 pontos)".to_string(),
                    "Abordagem pouco original (0 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(fuvest_rubric).await?;
    
    // Rubrica UNICAMP - 10 competências, 0-6 pontos cada, total 0-60
    let unicamp_rubric = ExamRubric {
        exam_type: ExamType::Unicamp,
        max_score: 60,
        description: "Avaliação de redação conforme critérios da UNICAMP. Total de 60 pontos distribuídos em 10 competências (0-6 pontos cada). Diversidade de gêneros textuais.".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Análise e interpretação".to_string(),
                description: "Avalia a capacidade de analisar e interpretar informações e contextos.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Análise e interpretação excelentes (5-6 pontos)".to_string(),
                    "Análise e interpretação adequadas (3-4 pontos)".to_string(),
                    "Análise e interpretação razoáveis (1-2 pontos)".to_string(),
                    "Análise e interpretação insuficientes (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Argumentação".to_string(),
                description: "Avalia a qualidade e consistência dos argumentos apresentados.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Argumentação sólida e bem fundamentada (5-6 pontos)".to_string(),
                    "Argumentação adequada (3-4 pontos)".to_string(),
                    "Argumentação básica (1-2 pontos)".to_string(),
                    "Argumentação insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coerência textual".to_string(),
                description: "Avalia a consistência lógica e a progressão das ideias no texto.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Coerência textual excelente (5-6 pontos)".to_string(),
                    "Coerência textual adequada (3-4 pontos)".to_string(),
                    "Coerência textual razoável (1-2 pontos)".to_string(),
                    "Coerência textual insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coesão textual".to_string(),
                description: "Avalia os mecanismos de coesão textual utilizados.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Coesão textual excelente (5-6 pontos)".to_string(),
                    "Coesão textual adequada (3-4 pontos)".to_string(),
                    "Coesão textual razoável (1-2 pontos)".to_string(),
                    "Coesão textual insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Adequação ao gênero".to_string(),
                description: "Avalia se o texto está adequado ao gênero textual solicitado.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Adequação completa ao gênero (5-6 pontos)".to_string(),
                    "Adequação adequada ao gênero (3-4 pontos)".to_string(),
                    "Adequação parcial ao gênero (1-2 pontos)".to_string(),
                    "Inadequação ao gênero (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Uso adequado da linguagem".to_string(),
                description: "Avalia o uso adequado da linguagem conforme o contexto e gênero.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Uso muito adequado da linguagem (5-6 pontos)".to_string(),
                    "Uso adequado da linguagem (3-4 pontos)".to_string(),
                    "Uso razoável da linguagem (1-2 pontos)".to_string(),
                    "Uso inadequado da linguagem (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Vocabulário".to_string(),
                description: "Avalia a riqueza, precisão e adequação do vocabulário.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Vocabulário rico e preciso (5-6 pontos)".to_string(),
                    "Vocabulário adequado (3-4 pontos)".to_string(),
                    "Vocabulário básico mas adequado (1-2 pontos)".to_string(),
                    "Vocabulário insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Estruturação".to_string(),
                description: "Avalia a organização e estruturação do texto.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Estruturação excelente (5-6 pontos)".to_string(),
                    "Estruturação adequada (3-4 pontos)".to_string(),
                    "Estruturação razoável (1-2 pontos)".to_string(),
                    "Estruturação insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Criatividade".to_string(),
                description: "Avalia a criatividade e originalidade na abordagem do tema.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Muito criativo e original (5-6 pontos)".to_string(),
                    "Criativo e adequado (3-4 pontos)".to_string(),
                    "Criatividade básica (1-2 pontos)".to_string(),
                    "Pouca criatividade (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Ortografia e acentuação".to_string(),
                description: "Avalia a correção ortográfica e acentual do texto.".to_string(),
                weight: 0.1,
                max_score: 6,
                evaluation_points: vec![
                    "Ortografia e acentuação corretas (5-6 pontos)".to_string(),
                    "Poucos erros de ortografia e acentuação (3-4 pontos)".to_string(),
                    "Alguns erros de ortografia e acentuação (1-2 pontos)".to_string(),
                    "Muitos erros de ortografia e acentuação (0 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(unicamp_rubric).await?;
    
    // Rubrica UERJ - 4 competências, 0-5 pontos cada, total 0-20
    let uerj_rubric = ExamRubric {
        exam_type: ExamType::Uerj,
        max_score: 20,
        description: "Avaliação de redação dissertativa-argumentativa conforme critérios da UERJ. Total de 20 pontos distribuídos em 4 competências (0-5 pontos cada).".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Conteúdo temático".to_string(),
                description: "Avalia a abordagem do tema, a qualidade das informações e argumentos apresentados.".to_string(),
                weight: 0.25,
                max_score: 5,
                evaluation_points: vec![
                    "Conteúdo temático completo e bem desenvolvido (4-5 pontos)".to_string(),
                    "Conteúdo temático adequado (3 pontos)".to_string(),
                    "Conteúdo temático parcial (1-2 pontos)".to_string(),
                    "Conteúdo temático insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Estrutura textual".to_string(),
                description: "Avalia a organização, estruturação e progressão do texto dissertativo-argumentativo.".to_string(),
                weight: 0.25,
                max_score: 5,
                evaluation_points: vec![
                    "Estrutura textual excelente (4-5 pontos)".to_string(),
                    "Estrutura textual adequada (3 pontos)".to_string(),
                    "Estrutura textual razoável (1-2 pontos)".to_string(),
                    "Estrutura textual insuficiente (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Linguagem".to_string(),
                description: "Avalia o domínio da linguagem formal, vocabulário e expressão adequados ao gênero.".to_string(),
                weight: 0.25,
                max_score: 5,
                evaluation_points: vec![
                    "Linguagem excelente e adequada (4-5 pontos)".to_string(),
                    "Linguagem adequada (3 pontos)".to_string(),
                    "Linguagem razoável (1-2 pontos)".to_string(),
                    "Linguagem inadequada (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Mecanismos de coesão".to_string(),
                description: "Avalia o uso adequado de mecanismos de coesão textual: conectivos, referências e articulações.".to_string(),
                weight: 0.25,
                max_score: 5,
                evaluation_points: vec![
                    "Mecanismos de coesão excelentes (4-5 pontos)".to_string(),
                    "Mecanismos de coesão adequados (3 pontos)".to_string(),
                    "Mecanismos de coesão razoáveis (1-2 pontos)".to_string(),
                    "Mecanismos de coesão insuficientes (0 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(uerj_rubric).await?;
    
    // Rubricas para universidades com vestibular próprio (sistema 0-100 pontos, 10 competências)
    // UFRJ - sistema próprio (0-10 pontos, 5 competências, 0-2 cada)
    let ufrj_rubric = ExamRubric {
        exam_type: ExamType::Ufrj,
        max_score: 10,
        description: "Avaliação de redação conforme critérios da UFRJ. Total de 10 pontos distribuídos em 5 competências (0-2 pontos cada).".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Adequação ao tema e gênero".to_string(),
                description: "Avalia se o texto está adequado ao tema proposto e ao gênero dissertativo-argumentativo.".to_string(),
                weight: 0.2,
                max_score: 2,
                evaluation_points: vec![
                    "Adequação completa (2 pontos)".to_string(),
                    "Adequação parcial (1 ponto)".to_string(),
                    "Inadequação (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Estrutura textual".to_string(),
                description: "Avalia a organização e estruturação do texto.".to_string(),
                weight: 0.2,
                max_score: 2,
                evaluation_points: vec![
                    "Estrutura excelente (2 pontos)".to_string(),
                    "Estrutura adequada (1 ponto)".to_string(),
                    "Estrutura inadequada (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coerência e coesão".to_string(),
                description: "Avalia a consistência lógica e os mecanismos de coesão.".to_string(),
                weight: 0.2,
                max_score: 2,
                evaluation_points: vec![
                    "Excelente coerência e coesão (2 pontos)".to_string(),
                    "Coerência e coesão adequadas (1 ponto)".to_string(),
                    "Coerência e coesão insuficientes (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Domínio linguístico".to_string(),
                description: "Avalia o domínio da norma padrão e o vocabulário.".to_string(),
                weight: 0.2,
                max_score: 2,
                evaluation_points: vec![
                    "Domínio excelente (2 pontos)".to_string(),
                    "Domínio adequado (1 ponto)".to_string(),
                    "Domínio precário (0 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Argumentação".to_string(),
                description: "Avalia a qualidade dos argumentos apresentados.".to_string(),
                weight: 0.2,
                max_score: 2,
                evaluation_points: vec![
                    "Argumentação sólida (2 pontos)".to_string(),
                    "Argumentação adequada (1 ponto)".to_string(),
                    "Argumentação insuficiente (0 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(ufrj_rubric).await?;
    
    // Função auxiliar para criar rubricas padrão de universidades com sistema 0-100 (10 competências, 0-10 cada)
    fn create_standard_university_rubric(exam_type: ExamType, university_name: &str) -> ExamRubric {
        ExamRubric {
            exam_type: exam_type.clone(),
            max_score: 100,
            description: format!("Avaliação de redação conforme critérios da {}. Total de 100 pontos distribuídos em 10 competências (0-10 pontos cada).", university_name),
            criteria: vec![
                RubricCriterion {
                    name: "Adequação ao tema".to_string(),
                    description: "Avalia se o texto aborda adequadamente o tema proposto.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Abordagem completa e adequada (8-10 pontos)".to_string(),
                        "Abordagem adequada (5-7 pontos)".to_string(),
                        "Abordagem parcial (2-4 pontos)".to_string(),
                        "Fuga do tema (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Gênero textual".to_string(),
                    description: "Avalia se o texto está adequado ao gênero solicitado.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Gênero adequado com estrutura completa (8-10 pontos)".to_string(),
                        "Gênero adequado com estrutura parcial (5-7 pontos)".to_string(),
                        "Gênero parcialmente adequado (2-4 pontos)".to_string(),
                        "Gênero inadequado (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Coerência".to_string(),
                    description: "Avalia a consistência lógica e progressão das ideias.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Texto muito coerente (8-10 pontos)".to_string(),
                        "Texto coerente (5-7 pontos)".to_string(),
                        "Texto parcialmente coerente (2-4 pontos)".to_string(),
                        "Texto incoerente (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Coesão".to_string(),
                    description: "Avalia os mecanismos de coesão textual.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Coesão excelente (8-10 pontos)".to_string(),
                        "Coesão adequada (5-7 pontos)".to_string(),
                        "Coesão parcial (2-4 pontos)".to_string(),
                        "Coesão insuficiente (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Argumentação".to_string(),
                    description: "Avalia a qualidade e força dos argumentos.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Argumentação sólida e convincente (8-10 pontos)".to_string(),
                        "Argumentação adequada (5-7 pontos)".to_string(),
                        "Argumentação básica (2-4 pontos)".to_string(),
                        "Argumentação insuficiente (0-1 pontos)".to_string(),
                ],
                },
                RubricCriterion {
                    name: "Domínio da norma padrão".to_string(),
                    description: "Avalia o domínio da escrita formal da Língua Portuguesa.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Domínio excelente (8-10 pontos)".to_string(),
                        "Domínio adequado (5-7 pontos)".to_string(),
                        "Domínio parcial (2-4 pontos)".to_string(),
                        "Domínio precário (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Vocabulário".to_string(),
                    description: "Avalia a riqueza e precisão do vocabulário.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Vocabulário rico e preciso (8-10 pontos)".to_string(),
                        "Vocabulário adequado (5-7 pontos)".to_string(),
                        "Vocabulário básico (2-4 pontos)".to_string(),
                        "Vocabulário insuficiente (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Estruturação textual".to_string(),
                    description: "Avalia a organização e estruturação do texto.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Estruturação excelente (8-10 pontos)".to_string(),
                        "Estruturação adequada (5-7 pontos)".to_string(),
                        "Estruturação parcial (2-4 pontos)".to_string(),
                        "Estruturação inadequada (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Originalidade e criatividade".to_string(),
                    description: "Avalia a criatividade e originalidade na abordagem.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Muito original e criativo (8-10 pontos)".to_string(),
                        "Original e adequado (5-7 pontos)".to_string(),
                        "Convencional mas adequado (2-4 pontos)".to_string(),
                        "Pouco original (0-1 pontos)".to_string(),
                    ],
                },
                RubricCriterion {
                    name: "Ortografia e acentuação".to_string(),
                    description: "Avalia a correção ortográfica e acentual.".to_string(),
                    weight: 0.1,
                    max_score: 10,
                    evaluation_points: vec![
                        "Ortografia e acentuação corretas (8-10 pontos)".to_string(),
                        "Poucos erros (5-7 pontos)".to_string(),
                        "Alguns erros (2-4 pontos)".to_string(),
                        "Muitos erros (0-1 pontos)".to_string(),
                    ],
                },
            ],
        }
    }
    
    // Adicionar rubricas para todas as universidades com vestibular próprio (sistema 0-100)
    let universities_with_own_exam = vec![
        (ExamType::Ufsc, "UFSC"),
        (ExamType::Ufrgs, "UFRGS"),
        (ExamType::Ufpr, "UFPR"),
        (ExamType::Ufscar, "UFSCar"),
        (ExamType::Ufg, "UFG"),
        (ExamType::Ufjf, "UFJF"),
        (ExamType::Ufv, "UFV"),
        (ExamType::Uftm, "UFTM"),
        (ExamType::Ufs, "UFS"),
        (ExamType::Uft, "UFT"),
        (ExamType::Unb, "UNB"),
        (ExamType::Ueg, "UEG"),
        (ExamType::Uepb, "UEPB"),
        (ExamType::Uepg, "UEPG"),
        (ExamType::Uel, "UEL"),
        (ExamType::Uenf, "UENF"),
        (ExamType::Unifesp, "UNIFESP"),
        (ExamType::Uesc, "UESC"),
        (ExamType::Uemg, "UEMG"),
        (ExamType::Uem, "UEM"),
        (ExamType::Uesb, "UESB"),
        (ExamType::Uespi, "UESPI"),
        (ExamType::Uesf, "UESF"),
        (ExamType::Uesr, "UESR"),
        (ExamType::Uesg, "UESG"),
    ];
    
    for (exam_type, name) in universities_with_own_exam {
        let rubric = create_standard_university_rubric(exam_type, name);
        rubric_repo.insert(rubric).await?;
    }
    
    // ITA e IME - vestibulares técnicos
    let ita_rubric = ExamRubric {
        exam_type: ExamType::Ita,
        max_score: 50,
        description: "Avaliação de redação técnica conforme critérios do ITA. Total de 50 pontos. Foco em argumentação técnica e científica.".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Argumentação técnica".to_string(),
                description: "Avalia a qualidade da argumentação técnica e científica.".to_string(),
                weight: 0.3,
                max_score: 15,
                evaluation_points: vec![
                    "Argumentação técnica excelente (12-15 pontos)".to_string(),
                    "Argumentação técnica adequada (8-11 pontos)".to_string(),
                    "Argumentação técnica básica (4-7 pontos)".to_string(),
                    "Argumentação técnica insuficiente (0-3 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Rigor científico".to_string(),
                description: "Avalia o rigor científico e precisão técnica.".to_string(),
                weight: 0.25,
                max_score: 12,
                evaluation_points: vec![
                    "Rigor científico excelente (10-12 pontos)".to_string(),
                    "Rigor científico adequado (6-9 pontos)".to_string(),
                    "Rigor científico básico (3-5 pontos)".to_string(),
                    "Rigor científico insuficiente (0-2 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coerência e estrutura".to_string(),
                description: "Avalia a coerência lógica e estruturação do texto.".to_string(),
                weight: 0.2,
                max_score: 10,
                evaluation_points: vec![
                    "Coerência e estrutura excelentes (8-10 pontos)".to_string(),
                    "Coerência e estrutura adequadas (5-7 pontos)".to_string(),
                    "Coerência e estrutura parciais (2-4 pontos)".to_string(),
                    "Coerência e estrutura insuficientes (0-1 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Domínio linguístico".to_string(),
                description: "Avalia o domínio da linguagem formal e técnica.".to_string(),
                weight: 0.15,
                max_score: 8,
                evaluation_points: vec![
                    "Domínio linguístico excelente (6-8 pontos)".to_string(),
                    "Domínio linguístico adequado (4-5 pontos)".to_string(),
                    "Domínio linguístico parcial (2-3 pontos)".to_string(),
                    "Domínio linguístico precário (0-1 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Originalidade técnica".to_string(),
                description: "Avalia a originalidade e criatividade na abordagem técnica.".to_string(),
                weight: 0.1,
                max_score: 5,
                evaluation_points: vec![
                    "Originalidade técnica excelente (4-5 pontos)".to_string(),
                    "Originalidade técnica adequada (2-3 pontos)".to_string(),
                    "Originalidade técnica básica (1 ponto)".to_string(),
                    "Pouca originalidade técnica (0 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(ita_rubric).await?;
    
    let ime_rubric = ExamRubric {
        exam_type: ExamType::Ime,
        max_score: 100,
        description: "Avaliação de redação técnica conforme critérios do IME. Total de 100 pontos. Foco em argumentação técnica e científica.".to_string(),
        criteria: vec![
            RubricCriterion {
                name: "Argumentação técnica".to_string(),
                description: "Avalia a qualidade da argumentação técnica e científica.".to_string(),
                weight: 0.25,
                max_score: 25,
                evaluation_points: vec![
                    "Argumentação técnica excelente (20-25 pontos)".to_string(),
                    "Argumentação técnica adequada (13-19 pontos)".to_string(),
                    "Argumentação técnica básica (6-12 pontos)".to_string(),
                    "Argumentação técnica insuficiente (0-5 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Rigor científico".to_string(),
                description: "Avalia o rigor científico e precisão técnica.".to_string(),
                weight: 0.2,
                max_score: 20,
                evaluation_points: vec![
                    "Rigor científico excelente (16-20 pontos)".to_string(),
                    "Rigor científico adequado (10-15 pontos)".to_string(),
                    "Rigor científico básico (5-9 pontos)".to_string(),
                    "Rigor científico insuficiente (0-4 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Aplicação prática".to_string(),
                description: "Avalia a capacidade de aplicar conhecimentos técnicos.".to_string(),
                weight: 0.2,
                max_score: 20,
                evaluation_points: vec![
                    "Aplicação prática excelente (16-20 pontos)".to_string(),
                    "Aplicação prática adequada (10-15 pontos)".to_string(),
                    "Aplicação prática básica (5-9 pontos)".to_string(),
                    "Aplicação prática insuficiente (0-4 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Coerência e estrutura".to_string(),
                description: "Avalia a coerência lógica e estruturação do texto.".to_string(),
                weight: 0.15,
                max_score: 15,
                evaluation_points: vec![
                    "Coerência e estrutura excelentes (12-15 pontos)".to_string(),
                    "Coerência e estrutura adequadas (7-11 pontos)".to_string(),
                    "Coerência e estrutura parciais (3-6 pontos)".to_string(),
                    "Coerência e estrutura insuficientes (0-2 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Domínio linguístico técnico".to_string(),
                description: "Avalia o domínio da linguagem técnica e formal.".to_string(),
                weight: 0.1,
                max_score: 10,
                evaluation_points: vec![
                    "Domínio linguístico técnico excelente (8-10 pontos)".to_string(),
                    "Domínio linguístico técnico adequado (5-7 pontos)".to_string(),
                    "Domínio linguístico técnico parcial (2-4 pontos)".to_string(),
                    "Domínio linguístico técnico precário (0-1 pontos)".to_string(),
                ],
            },
            RubricCriterion {
                name: "Originalidade técnica".to_string(),
                description: "Avalia a originalidade e criatividade na abordagem técnica.".to_string(),
                weight: 0.1,
                max_score: 10,
                evaluation_points: vec![
                    "Originalidade técnica excelente (8-10 pontos)".to_string(),
                    "Originalidade técnica adequada (5-7 pontos)".to_string(),
                    "Originalidade técnica básica (2-4 pontos)".to_string(),
                    "Pouca originalidade técnica (0-1 pontos)".to_string(),
                ],
            },
        ],
    };
    
    rubric_repo.insert(ime_rubric).await?;
    
    Ok(())
}

/// Cria trilhas de conhecimento de exemplo
pub async fn seed_knowledge_trails(
    trail_repo: &InMemoryKnowledgeTrailRepository,
    _user_id: Uuid,
) -> Result<()> {
    let trails = vec![
        KnowledgeTrail {
            id: Uuid::parse_str("b0000000-0000-0000-0000-000000000001").unwrap(),
            title: "Trilha: Fundamentos de Matemática".to_string(),
            description: "Domine os conceitos fundamentais de matemática para vestibulares, incluindo álgebra, geometria e trigonometria.".to_string(),
            focus_areas: vec![Subject::Matematica],
            progress: 0,
            modules: vec![
                TrailModule {
                    id: Uuid::parse_str("b0000000-0000-0000-0000-000000000011").unwrap(),
                    title: "Álgebra Básica".to_string(),
                    description: "Equações do primeiro e segundo grau".to_string(),
                    content_type: ContentType::Question,
                    content_id: Uuid::parse_str("10000000-0000-0000-0000-000000000002").unwrap(),
                    order: 1,
                    completed: false,
                },
                TrailModule {
                    id: Uuid::parse_str("b0000000-0000-0000-0000-000000000012").unwrap(),
                    title: "Funções Quadráticas".to_string(),
                    description: "Análise de funções do segundo grau".to_string(),
                    content_type: ContentType::Question,
                    content_id: Uuid::parse_str("10000000-0000-0000-0000-000000000001").unwrap(),
                    order: 2,
                    completed: false,
                },
            ],
            estimated_hours: 20,
            difficulty_level: Difficulty::Medio,
        },
        KnowledgeTrail {
            id: Uuid::parse_str("b0000000-0000-0000-0000-000000000002").unwrap(),
            title: "Trilha: História do Brasil".to_string(),
            description: "Períodos históricos do Brasil desde o descobrimento até a República.".to_string(),
            focus_areas: vec![Subject::Historia],
            progress: 30,
            modules: vec![
                TrailModule {
                    id: Uuid::parse_str("b0000000-0000-0000-0000-000000000021").unwrap(),
                    title: "Período Colonial".to_string(),
                    description: "Descobrimento e colonização do Brasil".to_string(),
                    content_type: ContentType::Question,
                    content_id: Uuid::parse_str("20000000-0000-0000-0000-000000000001").unwrap(),
                    order: 1,
                    completed: true,
                },
                TrailModule {
                    id: Uuid::parse_str("b0000000-0000-0000-0000-000000000022").unwrap(),
                    title: "Movimentos de Independência".to_string(),
                    description: "Inconfidências e processo de independência".to_string(),
                    content_type: ContentType::Question,
                    content_id: Uuid::parse_str("20000000-0000-0000-0000-000000000002").unwrap(),
                    order: 2,
                    completed: false,
                },
            ],
            estimated_hours: 15,
            difficulty_level: Difficulty::Medio,
        },
        KnowledgeTrail {
            id: Uuid::parse_str("b0000000-0000-0000-0000-000000000003").unwrap(),
            title: "Trilha: Física Mecânica".to_string(),
            description: "Conceitos fundamentais de mecânica: cinemática, dinâmica e energia.".to_string(),
            focus_areas: vec![Subject::Fisica],
            progress: 0,
            modules: vec![
                TrailModule {
                    id: Uuid::parse_str("b0000000-0000-0000-0000-000000000031").unwrap(),
                    title: "Cinemática".to_string(),
                    description: "Movimento uniforme e uniformemente variado".to_string(),
                    content_type: ContentType::Question,
                    content_id: Uuid::parse_str("30000000-0000-0000-0000-000000000001").unwrap(),
                    order: 1,
                    completed: false,
                },
            ],
            estimated_hours: 25,
            difficulty_level: Difficulty::Dificil,
        },
    ];
    
    for trail in trails {
        trail_repo.insert(trail).await?;
    }
    
    Ok(())
}
