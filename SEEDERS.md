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

### Questões (11 questões)

#### Matemática (3 questões)
1. **Função Quadrática** (Médio) - ID: `10000000-0000-0000-0000-000000000001`
   - Envolve raízes e coeficientes de função quadrática
   - Explicação detalhada da resolução

2. **Equação do Primeiro Grau** (Fácil) - ID: `10000000-0000-0000-0000-000000000002`
   - Equação simples: 2x + 5 = 15
   - Ideal para iniciantes

3. **Logaritmos** (Difícil) - ID: `10000000-0000-0000-0000-000000000003`
   - Equação logarítmica com propriedades
   - Requer conhecimento avançado

#### História (2 questões)
1. **Descobrimento do Brasil** (Fácil) - ID: `20000000-0000-0000-0000-000000000001`
   - Data do descobrimento por Cabral
   - Contexto histórico

2. **Inconfidência Mineira** (Médio) - ID: `20000000-0000-0000-0000-000000000002`
   - Motivos e contexto histórico
   - Período colonial brasileiro

#### Física (2 questões)
1. **Lançamento Vertical** (Médio) - ID: `30000000-0000-0000-0000-000000000001`
   - Cinemática: altura máxima
   - Uso da equação de Torricelli

2. **Capacitores** (Difícil) - ID: `30000000-0000-0000-0000-000000000002`
   - Eletricidade: associação de capacitores
   - Conservação de carga

#### Química (1 questão)
1. **pH de Ácido Forte** (Médio) - ID: `40000000-0000-0000-0000-000000000001`
   - Cálculo de pH
   - Dissociação completa

#### Biologia (1 questão)
1. **Fotossíntese** (Médio) - ID: `50000000-0000-0000-0000-000000000001`
   - Organelas celulares
   - Processo fotossintético

#### Literatura (1 questão)
1. **Machado de Assis** (Médio) - ID: `60000000-0000-0000-0000-000000000001`
   - Obra mais conhecida
   - Literatura brasileira

#### Geografia (1 questão)
1. **Biomas Brasileiros** (Fácil) - ID: `70000000-0000-0000-0000-000000000001`
   - Maior bioma do Brasil
   - Geografia física

#### Português (1 questão)
1. **Acentuação Ortográfica** (Médio) - ID: `80000000-0000-0000-0000-000000000001`
   - Acordo Ortográfico de 2009
   - Mudanças na acentuação

### Redações (4 redações)

1. **ENEM - Desafios da Educação Digital** (Em Progresso)
   - ID: `a0000000-0000-0000-0000-000000000001`
   - Tema atual sobre educação digital
   - Status: Em progresso

2. **ENEM - Evasão Escolar** (Corrigida)
   - ID: `a0000000-0000-0000-0000-000000000002`
   - Score: 820/1000
   - Feedback detalhado por competência
   - Rubric scores completos

3. **FUVEST - Tecnologia** (Em Progresso)
   - ID: `a0000000-0000-0000-0000-000000000003`
   - Redação iniciada
   - Max score: 48

4. **UNICAMP - Sustentabilidade** (Corrigida)
   - ID: `a0000000-0000-0000-0000-000000000004`
   - Score: 52/60
   - Artigo de opinião
   - Feedback positivo

### Trilhas de Conhecimento (3 trilhas)

1. **Fundamentos de Matemática**
   - ID: `b0000000-0000-0000-0000-000000000001`
   - Progresso: 0%
   - 2 módulos (Álgebra Básica, Funções Quadráticas)
   - Dificuldade: Médio
   - Estimativa: 20 horas

2. **História do Brasil**
   - ID: `b0000000-0000-0000-0000-000000000002`
   - Progresso: 30%
   - 2 módulos (1 completo, 1 pendente)
   - Dificuldade: Médio
   - Estimativa: 15 horas

3. **Física Mecânica**
   - ID: `b0000000-0000-0000-0000-000000000003`
   - Progresso: 0%
   - 1 módulo (Cinemática)
   - Dificuldade: Difícil
   - Estimativa: 25 horas

## Características dos Dados

- **Realistas**: Todas as questões têm enunciados, alternativas e explicações detalhadas baseadas em conteúdo real de vestibulares
- **Diversificados**: Cobrem múltiplas matérias e níveis de dificuldade
- **Completos**: Redações incluem feedbacks, scores e rubric scores quando corrigidas
- **Estruturados**: Trilhas têm módulos bem definidos com progresso

## Personalização

Você pode criar seus próprios seeders usando as funções auxiliares:

```rust
use data::seeders::{seed_user, seed_questions, seed_essays, seed_knowledge_trails};
```

Ou criar dados individuais usando as estruturas do domínio diretamente.

