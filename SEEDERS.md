# Seeders de Dados de Teste

Este documento descreve os seeders implementados para popular o banco de dados com dados de teste realistas.

## Uso

```rust
use data::{InMemoryEssayRepository, InMemoryQuestionRepository, 
           InMemoryUserRepository, InMemoryKnowledgeTrailRepository, seed_all_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Criar repositórios
    let essay_repo = InMemoryEssayRepository::new();
    let question_repo = InMemoryQuestionRepository::new();
    let user_repo = InMemoryUserRepository::new();
    let trail_repo = InMemoryKnowledgeTrailRepository::new();
    
    // Popular todos os dados
    let user_id = seed_all_data(
        &essay_repo,
        &question_repo,
        &user_repo,
        &trail_repo,
    ).await?;
    
    println!("Dados seedados! User ID: {}", user_id);
    Ok(())
}
```

## Dados Criados

### Usuário
- **ID**: `00000000-0000-0000-0000-000000000001`
- **Nome**: "Estudante Exemplo"
- **Email**: estudante@neuronexus.app
- **Tema**: Cyberpunk
- **Configurações**: Notificações habilitadas, lembretes de estudo ativos

### Questions (11 questions)

#### Mathematics (3 questions)
1. **Quadratic Function** (Medium) - ID: `10000000-0000-0000-0000-000000000001`
   - Involves roots and coefficients of quadratic function
   - Detailed solution explanation

2. **First Degree Equation** (Easy) - ID: `10000000-0000-0000-0000-000000000002`
   - Simple equation: 2x + 5 = 15
   - Ideal for beginners

3. **Logarithms** (Hard) - ID: `10000000-0000-0000-0000-000000000003`
   - Logarithmic equation with properties
   - Requires advanced knowledge

#### History (2 questions)
1. **Discovery of Brazil** (Easy) - ID: `20000000-0000-0000-0000-000000000001`
   - Date of discovery by Cabral
   - Historical context

2. **Inconfidência Mineira** (Medium) - ID: `20000000-0000-0000-0000-000000000002`
   - Motives and historical context
   - Brazilian colonial period

#### Physics (2 questions)
1. **Vertical Launch** (Medium) - ID: `30000000-0000-0000-0000-000000000001`
   - Kinematics: maximum height
   - Use of Torricelli's equation

2. **Capacitors** (Hard) - ID: `30000000-0000-0000-0000-000000000002`
   - Electricity: capacitor association
   - Charge conservation

#### Chemistry (1 question)
1. **pH of Strong Acid** (Medium) - ID: `40000000-0000-0000-0000-000000000001`
   - pH calculation
   - Complete dissociation

#### Biology (1 question)
1. **Photosynthesis** (Medium) - ID: `50000000-0000-0000-0000-000000000001`
   - Cell organelles
   - Photosynthetic process

#### Literature (1 question)
1. **Machado de Assis** (Medium) - ID: `60000000-0000-0000-0000-000000000001`
   - Best-known work
   - Brazilian literature

#### Geography (1 question)
1. **Brazilian Biomes** (Easy) - ID: `70000000-0000-0000-0000-000000000001`
   - Largest biome in Brazil
   - Physical geography

#### Portuguese (1 question)
1. **Orthographic Accentuation** (Medium) - ID: `80000000-0000-0000-0000-000000000001`
   - 2009 Orthographic Agreement
   - Changes in accentuation

### Essays (4 essays)

1. **ENEM - Digital Education Challenges** (In Progress)
   - ID: `a0000000-0000-0000-0000-000000000001`
   - Current topic on digital education
   - Status: In progress

2. **ENEM - School Dropout** (Graded)
   - ID: `a0000000-0000-0000-0000-000000000002`
   - Score: 820/1000
   - Detailed feedback by competency
   - Complete rubric scores

3. **FUVEST - Technology** (In Progress)
   - ID: `a0000000-0000-0000-0000-000000000003`
   - Essay started
   - Max score: 48

4. **UNICAMP - Sustainability** (Graded)
   - ID: `a0000000-0000-0000-0000-000000000004`
   - Score: 52/60
   - Opinion article
   - Positive feedback

### Knowledge Trails (3 trails)

1. **Mathematics Fundamentals**
   - ID: `b0000000-0000-0000-0000-000000000001`
   - Progress: 0%
   - 2 modules (Basic Algebra, Quadratic Functions)
   - Difficulty: Medium
   - Estimated: 20 hours

2. **Brazilian History**
   - ID: `b0000000-0000-0000-0000-000000000002`
   - Progress: 30%
   - 2 modules (1 complete, 1 pending)
   - Difficulty: Medium
   - Estimated: 15 hours

3. **Mechanical Physics**
   - ID: `b0000000-0000-0000-0000-000000000003`
   - Progress: 0%
   - 1 module (Kinematics)
   - Difficulty: Hard
   - Estimated: 25 hours

## Data Characteristics

- **Realistic**: All questions have statements, alternatives and detailed explanations based on real college entrance exam content
- **Diverse**: Cover multiple subjects and difficulty levels
- **Complete**: Essays include feedback, scores and rubric scores when graded
- **Structured**: Trails have well-defined modules with progress

## Customization

You can create your own seeders using the helper functions:

```rust
use data::seeders::{seed_user, seed_questions, seed_essays, seed_knowledge_trails};
```

Or create individual data using domain structures directly.

