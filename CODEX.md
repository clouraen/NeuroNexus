# NeuroNexus - Codex de Desenvolvimento

## Visão Geral
NeuroNexus é uma plataforma educacional multiplataforma (web, desktop, mobile) construída com Rust e Dioxus, focada em preparação para vestibulares e ENEM. A plataforma combina IA generativa, trilhas de aprendizado personalizadas e uma interface imersiva com estética cyberpunk (inspirada em Cyberpunk 2077 e Blade Runner 2049).

## Design System: Neon Cyberpunk
### Estética Visual
- **Tema Base**: Dark mode com gradientes escuros profundos
- **Cores Principais**:
  - Fundo: `#000000` (preto absoluto) e `#0a0a0a` (preto suave)
  - Neon Roxo: `#9d4edd`, `#c77dff`, `#e0aaff`
  - Neon Rosa: `#ff69b4`, `#ff10f0`, `#ff00ff`
  - Neon Azul: `#00ffff`, `#0ff0ff`, `#00d9ff`
  - Acentos Dourados: `#ffd700`, `#ffed4e`
  - Glow Effects: Sombras brilhantes com blur para efeito neon

### Componentes UI Dioxus
- **NeonButton**: Botões com bordas neon e glow hover
- **NeonInput**: Inputs com bordas pulsantes e glow
- **CyberCard**: Cards com gradientes escuros e bordas neon
- **StatusBar**: Barra superior com hora e indicadores (estilo cyberpunk)
- **TabBar**: Navegação inferior com ícones neon
- **GlitchText**: Textos com efeito glitch/scanline (opcional)
- **HologramEffect**: Efeitos holográficos para elementos importantes
- **NeonProgressBar**: Barras de progresso com animação neon

### Tipografia
- **Fontes**: Sans-serif modernas, preferencialmente monospace para elementos técnicos
- **Hierarquia**: Títulos em CAPS LOCK com glow, corpo de texto legível
- **Efeitos**: Text-shadow neon, tracking aumentado para títulos

### Ícone do Aplicativo
- **Ícone**: Representação cyberpunk de martelo e foice sobre capelo de formatura e livros
- **Cores do Ícone**:
  - Martelo e Foice: Vermelho neon (`#ff0000`) com outline azul neon (`#00ffff`)
  - Capelo de Formatura: Azul escuro (`#000080`) com outline cyan neon (`#00ffff`)
  - Livros: Azul escuro (`#000080`) com outline cyan neon (`#00ffff`)
  - Fundo: Preto absoluto (`#000000`)
- **Estilo**: Estética cyberpunk neon com glow effects
- **Localização**: `assets/icon.png`, `assets/icon.ico` (Windows), `assets/icon.icns` (macOS), `assets/icon.png` (Android/iOS)

## Arquitetura Técnica

### Stack Tecnológico
- **Linguagem**: Rust (edition 2021, mínimo 1.75)
- **Framework UI**: Dioxus 0.4+
- **Arquitetura**: Clean Architecture (Domínio → Dados → Apresentação)
- **Estrutura**: Workspace com múltiplos crates

### Estrutura de Crates
```
crates/
├── domain/          # Lógica de negócio pura, modelos, casos de uso, traits
├── data/            # Implementações de repositórios, banco de dados, seeders
├── app/             # Componentes Dioxus, páginas, roteamento, UI
├── shared/          # Utilitários compartilhados, tipos comuns
└── services/        # Serviços externos (IA, API, etc.)
```

### Targets de Compilação
- **Web**: `wasm32-unknown-unknown` (Dioxus Web)
- **Desktop**: Wry renderer (WebView)
- **Mobile**: Plataforma específica (Tauri mobile)

## Fases de Implementação

### FASE 1: Core MVP (Fundação)
#### Objetivo
Estabelecer arquitetura base e funcionalidades essenciais de navegação e exibição.

#### Features Core
1. **Sistema de Navegação**
   - TabBar inferior com 4 abas: Plano, Perguntas, Redações, Perfil
   - Roteamento básico entre páginas
   - StatusBar superior com hora e indicador online

2. **Plano de Estudo (Home)**
   - Dashboard com categorias de assuntos
   - Cards de acesso rápido
   - Estatísticas básicas (total de redações, questões respondidas)
   - Tema cyberpunk aplicado

3. **Listagem de Questões**
   - Visualização de questões por categoria
   - Busca simples por palavras-chave
   - Filtros básicos (assunto, dificuldade)
   - Card de questão com preview

4. **Listagem de Redações**
   - Lista de redações do usuário
   - Status: Em Progresso / Corrigida
   - Filtro por status
   - Card de redação com título e score

5. **Perfil Básico**
   - Nome e avatar do usuário
   - Informações básicas
   - Configurações simples

#### Implementação Fase 1
```
1. Setup do workspace Rust com crates
2. Configurar Dioxus para web/desktop/mobile
3. Criar estrutura básica de camadas (domain, data, app)
4. Implementar roteamento básico
5. Criar componentes UI base (NeonButton, NeonInput, CyberCard, TabBar, StatusBar)
6. Implementar tema cyberpunk (cores, efeitos, tipografia)
7. Páginas: Home, Questões (lista), Redações (lista), Perfil (básico)
8. Repositórios em memória para desenvolvimento
9. Seeders de dados de teste
```

### FASE 2: Funcionalidades Educacionais Essenciais
#### Objetivo
Implementar as funcionalidades principais de aprendizado e avaliação.

#### Features Fase 2

1. **Gerenciamento de Redações Completo**
   - Criar nova redação (editor de texto)
   - Selecionar tipo de vestibular/universidade (ENEM, USP, UNICAMP, etc.)
   - Correção específica por formato de prova
   - Salvar rascunho
   - Enviar para avaliação
   - Visualizar feedback detalhado baseado nos critérios do vestibular
   - Score com destaque (0-1000 para ENEM, 0-48 para Fuvest/USP, etc.)
   - Histórico completo com datas formatadas
   - Filtrar redações por vestibular

2. **Sistema de Questões Detalhado**
   - Visualizar questão completa
   - Alternativas de resposta
   - Resolução explicada
   - Marcar como respondida
   - Navegação entre questões

3. **Trilhas de Conhecimento**
   - Lista de trilhas disponíveis
   - Visualizar detalhes da trilha
   - Rastreamento de progresso (porcentagem)
   - Iniciar/continuar trilha
   - Áreas de foco por trilha

4. **Sistema de Progresso Básico**
   - Rastrear questões respondidas
   - Rastrear redações enviadas
   - Calcular progresso por categoria
   - Exibir métricas no dashboard

#### Implementação Fase 2
```
1. Implementar enum ExamType com todos os vestibulares suportados
2. Casos de uso de domínio para redações:
   - CreateEssay (com ExamType), SaveEssayDraft, SubmitEssayForEvaluation, GetEssayFeedback
   - ListEssaysByExamType, GetExamRubric

3. Casos de uso de domínio para questões:
   - GetQuestionDetails, MarkQuestionAsAnswered, GetQuestionExplanation

4. Casos de uso de domínio para trilhas:
   - StartKnowledgeTrail, UpdateTrailProgress, GetTrailDetails

5. Implementar editor de redação (textarea com estilização cyberpunk)
6. Componente de seleção de vestibular (dropdown/selector cyberpunk)
7. Implementar modelo ExamRubric e RubricScores
8. Criar repositório ExamRubricRepository
9. Implementar rubricas base para ENEM, Fuvest, UNICAMP, UNESP, UERJ
10. Página de detalhes de questão
11. Página de detalhes de trilha
12. Serviço mock de avaliação de redação (simular IA com critérios específicos)
13. Exibir scores no formato correto conforme ExamType
14. Filtrar redações por tipo de vestibular na UI
15. Atualizar UI com animações neon
```

### FASE 3: IA e Personalização
#### Objetivo
Adicionar inteligência artificial e personalização do aprendizado.

#### Features Fase 3

1. **Chat Tutor com IA**
   - Chat em tempo real com tutor IA
   - Respostas contextualizadas
   - Suporte por assunto
   - Histórico de conversas
   - Interface chat cyberpunk (mensagens com glow)

2. **Avaliação de Redação com IA**
   - Análise detalhada por competências específicas de cada vestibular
   - Correção baseada nos critérios oficiais de cada exame
   - Sugestões de melhoria específicas por tipo de prova
   - Highlight de erros/oportunidades
   - Feedback construtivo adaptado ao formato do vestibular
   - Comparação com médias históricas do exame específico

3. **Trilhas Personalizadas**
   - Geração de trilhas baseadas no desempenho
   - Recomendações de conteúdo
   - Adaptação de dificuldade

4. **Sistema de Conquistas**
   - Badges e conquistas
   - Notificações de conquistas desbloqueadas
   - Visualização de conquistas no perfil
   - Animações de desbloqueio

#### Implementação Fase 3
```
1. Integração com Hugging Face Hub para download de modelos LLM
2. Sistema de download e gestão de modelos LLM (dlite-v2-1.5B)
3. Interface de download na primeira execução
4. Serviço de inferência local usando Candle (dlite-v2-1.5B)
5. Otimizações de quantização para mobile (INT8/INT4)
6. Serviço P2P de LLM para compartilhamento de capacidade
7. Integração com chat tutor
8. Integração com avaliação de redação
9. Sistema de cache de inferências
10. Implementar rubricas de avaliação para cada ExamType
11. Sistema de correção adaptativo por tipo de vestibular
12. Banco de dados de critérios oficiais de cada exame
13. Sistema de recomendação básico
14. Sistema de conquistas (domínio + UI)
15. Notificações toast com estilo cyberpunk
16. Animações de desbloqueio
```

### FASE 4: Features Avançadas
#### Objetivo
Expandir funcionalidades com ferramentas de estudo adicionais.

#### Features Fase 4

1. **Flashcards**
   - Criar flashcards customizados
   - Deck de flashcards por assunto
   - Modo revisão com espaçamento
   - Animação de flip
   - Estatísticas de acerto

2. **Anotações de Estudo**
   - Bloco de notas por assunto
   - Editor markdown
   - Busca em anotações
   - Exportar anotações
   - Tags e categorização

3. **Simulados e Provas**
   - Criar simulados por assunto
   - Modo cronometrado
   - Resultados detalhados
   - Gráficos de desempenho
   - Comparação com média

4. **Análise de Progresso Avançada**
   - Gráficos de desempenho ao longo do tempo
   - Heatmap de atividades
   - Estatísticas por assunto
   - Metas de estudo personalizadas
   - Relatórios semanais/mensais

5. **Conteúdo Multimídia**
   - Integração com vídeos educacionais
   - Links para materiais externos
   - Bibliografia e referências

#### Implementação Fase 4
```
1. Sistema de flashcards (CRUD completo)
2. Editor de anotações markdown
3. Sistema de simulados (cronômetro, questões, resultados)
4. Biblioteca de gráficos (para análises)
5. Integração com API de vídeos (YouTube, etc.)
6. Sistema de exportação de dados
7. Componentes de visualização de dados (gráficos, heatmaps)
```

### FASE 5: Refinamento e Otimização
#### Objetivo
Melhorar performance, UX e adicionar polimento final.

#### Features Fase 5

1. **Performance**
   - Otimização de bundle size
   - Lazy loading de componentes
   - Cache inteligente
   - Otimização de re-renders

2. **Modo Offline**
   - Sincronização local
   - Cache de conteúdo
   - PWA para web

3. **Acessibilidade**
   - Suporte a leitores de tela
   - Navegação por teclado
   - Contraste adequado (mesmo em cyberpunk)

4. **Animações e Transições**
   - Transições suaves entre páginas
   - Micro-interações
   - Loading states estilizados
   - Feedback visual aprimorado

5. **Persistência de Dados**
   - Integração com SQLite
   - Migrações de banco
   - Backup e restore

#### Implementação Fase 5
```
1. Profiling e otimização
2. Implementar persistência local (SQLite)
3. Migrações de banco de dados
4. Melhorar acessibilidade
5. Adicionar animações de transição
6. Implementar PWA (para web)
7. Testes de performance
```

## Modelos de Domínio

### Redações
```rust
Essay {
    id: Uuid,
    title: String,
    content: String,
    exam_type: ExamType, // Tipo de vestibular/universidade
    status: EssayStatus,
    score: Option<u16>, // Varia conforme o exame (0-1000 ENEM, 0-48 Fuvest, etc.)
    max_score: u16, // Pontuação máxima do tipo de exame
    feedback: Option<String>,
    corrections: Option<Vec<Correction>>,
    rubric_scores: Option<RubricScores>, // Notas por competência/critério
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
}

ExamType {
    ENEM,           // 0-1000 (5 competências, 200 cada)
    Fuvest,         // USP - 0-48 (8 competências, 0-6 cada)
    Unicamp,        // 0-60 (10 competências)
    Unesp,          // 0-40 (5 competências, 0-8 cada)
    Uerj,           // 0-20 (4 competências, 0-5 cada)
    Ita,            // 0-50 (10 competências específicas)
    Ime,            // 0-100 (20 competências técnicas)
    Ufrj,           // 0-10 (5 competências, 0-2 cada)
    Ufmg,           // 0-1000 (similar ENEM)
    Ufsc,           // 0-100 (10 competências)
    Ufrgs,          // 0-100 (10 competências)
    Ufpr,           // 0-100 (10 competências)
    Ufscar,         // 0-100 (10 competências)
    Ufpe,           // 0-1000 (similar ENEM)
    Ufba,           // 0-1000 (similar ENEM)
    Ufc,            // 0-1000 (similar ENEM)
    Ufpa,           // 0-1000 (similar ENEM)
    Ufam,           // 0-1000 (similar ENEM)
    Ufac,           // 0-1000 (similar ENEM)
    Ufrn,           // 0-1000 (similar ENEM)
    Ufms,           // 0-1000 (similar ENEM)
    Ufg,            // 0-100 (10 competências)
    Ufes,           // 0-1000 (similar ENEM)
    Ufjf,           // 0-100 (10 competências)
    Ufv,            // 0-100 (10 competências)
    Uftm,           // 0-100 (10 competências)
    Ufpb,           // 0-1000 (similar ENEM)
    Ufpi,           // 0-1000 (similar ENEM)
    Ufma,           // 0-1000 (similar ENEM)
    Ufs,            // 0-100 (10 competências)
    Uft,            // 0-100 (10 competências)
    Ufopa,          // 0-1000 (similar ENEM)
    Unb,            // 0-100 (10 competências)
    Ueg,            // 0-100 (10 competências)
    Uema,           // 0-1000 (similar ENEM)
    Uece,           // 0-1000 (similar ENEM)
    Uern,           // 0-1000 (similar ENEM)
    Uepb,           // 0-100 (10 competências)
    Uepg,           // 0-100 (10 competências)
    Uel,            // 0-100 (10 competências)
    Unemat,         // 0-1000 (similar ENEM)
    Uerr,           // 0-1000 (similar ENEM)
    Uea,            // 0-1000 (similar ENEM)
    Ufrr,           // 0-1000 (similar ENEM)
    Upe,            // 0-1000 (similar ENEM)
    Uenf,           // 0-100 (10 competências)
    Unifesp,        // 0-100 (10 competências)
    Uesc,           // 0-100 (10 competências)
    Uemg,           // 0-100 (10 competências)
    Uem,            // 0-100 (10 competências)
    Uesb,           // 0-100 (10 competências)
    Uespi,          // 0-100 (10 competências)
    Uesf,           // 0-100 (10 competências)
    Uesr,           // 0-100 (10 competências)
    Uesg,           // 0-100 (10 competências)
}

EssayStatus {
    EmProgresso,
    Corrigida,
    Enviada, // Aguardando correção
}

Correction {
    position: usize,
    original_text: String,
    suggested_text: String,
    reason: String,
    rubric_criterion: String, // Qual competência/critério foi afetado
}

RubricScores {
    // Estrutura variável conforme ExamType
    // Exemplo para ENEM: 5 competências (0-200 cada)
    // Exemplo para Fuvest: 8 competências (0-6 cada)
    scores: HashMap<String, u16>, // Chave: nome da competência, Valor: pontuação
    detailed_feedback: HashMap<String, String>, // Feedback por competência
}

// Critérios de avaliação específicos por vestibular
ExamRubric {
    exam_type: ExamType,
    criteria: Vec<RubricCriterion>,
    max_score: u16,
    description: String, // Descrição do formato de prova
}

RubricCriterion {
    name: String,
    description: String,
    weight: f32, // Peso na pontuação total
    max_score: u16,
    evaluation_points: Vec<String>, // Pontos de avaliação
}
```

### Questões
```rust
Question {
    id: Uuid,
    subject: Subject,
    difficulty: Difficulty,
    statement: String,
    alternatives: Vec<Alternative>,
    correct_answer: usize,
    explanation: String,
    tags: Vec<String>,
}

Alternative {
    id: usize,
    text: String,
}

Subject {
    LinguaPortuguesa,
    Literatura,
    Ingles,
    Espanhol,
    Artes,
    EducacaoFisica,
    TIC,
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

Difficulty {
    Facil,
    Medio,
    Dificil,
}
```

### Trilhas de Conhecimento
```rust
KnowledgeTrail {
    id: Uuid,
    title: String,
    description: String,
    focus_areas: Vec<Subject>,
    progress: u8, // 0-100
    modules: Vec<TrailModule>,
    estimated_hours: u16,
    difficulty_level: Difficulty,
}

TrailModule {
    id: Uuid,
    title: String,
    description: String,
    content_type: ContentType,
    content_id: Uuid,
    order: usize,
    completed: bool,
}

ContentType {
    Question,
    Essay,
    Video,
    Reading,
    PracticeTest,
}
```

### Perfil e Progresso
```rust
UserProfile {
    id: Uuid,
    name: String,
    avatar_url: Option<String>,
    email: String,
    created_at: DateTime<Utc>,
    settings: UserSettings,
}

UserSettings {
    theme: Theme,
    notifications_enabled: bool,
    study_reminders: bool,
    language: Language,
}

StudyProgress {
    user_id: Uuid,
    questions_answered: u32,
    essays_submitted: u32,
    study_streak: u32,
    total_study_hours: f32,
    achievements: Vec<Achievement>,
    subject_progress: HashMap<Subject, SubjectProgress>,
}

Achievement {
    id: Uuid,
    name: String,
    description: String,
    icon_url: Option<String>,
    unlocked_at: DateTime<Utc>,
}

SubjectProgress {
    subject: Subject,
    questions_answered: u32,
    average_score: f32,
    last_studied_at: Option<DateTime<Utc>>,
}
```

### Chat e IA
```rust
ChatMessage {
    id: Uuid,
    conversation_id: Uuid,
    role: MessageRole,
    content: String,
    timestamp: DateTime<Utc>,
    subject_context: Option<Subject>,
}

MessageRole {
    User,
    Assistant,
}

Conversation {
    id: Uuid,
    user_id: Uuid,
    subject: Option<Subject>,
    messages: Vec<ChatMessage>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### Flashcards
```rust
Flashcard {
    id: Uuid,
    deck_id: Uuid,
    front: String,
    back: String,
    difficulty: u8, // 0-100
    last_reviewed: Option<DateTime<Utc>>,
    review_count: u32,
    next_review: Option<DateTime<Utc>>,
}

FlashcardDeck {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: String,
    subject: Subject,
    flashcards: Vec<Flashcard>,
    created_at: DateTime<Utc>,
}
```

### Anotações
```rust
Note {
    id: Uuid,
    user_id: Uuid,
    title: String,
    content: String, // Markdown
    subject: Subject,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

## Casos de Uso (Use Cases)

### Redações
- `CreateEssay`: Criar nova redação (com seleção de ExamType)
- `SaveEssayDraft`: Salvar rascunho
- `SubmitEssayForEvaluation`: Enviar para avaliação (com correção específica do tipo de exame)
- `GetEssayFeedback`: Obter feedback detalhado baseado nos critérios do vestibular
- `ListEssaysByStatus`: Listar por status
- `ListEssaysByExamType`: Listar por tipo de vestibular
- `GetEssayScore`: Obter pontuação
- `UpdateEssay`: Atualizar redação
- `GetExamRubric`: Obter critérios de avaliação do tipo de exame
- `EvaluateEssay`: Avaliar redação usando critérios específicos do ExamType

### Questões
- `SearchQuestions`: Buscar questões
- `FilterQuestionsBySubject`: Filtrar por assunto
- `FilterQuestionsByDifficulty`: Filtrar por dificuldade
- `GetQuestionDetails`: Obter detalhes
- `MarkQuestionAsAnswered`: Marcar como respondida
- `GetQuestionExplanation`: Obter explicação

### Trilhas
- `StartKnowledgeTrail`: Iniciar trilha
- `UpdateTrailProgress`: Atualizar progresso
- `GetTrailDetails`: Obter detalhes
- `CompleteTrailModule`: Completar módulo
- `ListAvailableTrails`: Listar trilhas disponíveis

### Chat Tutor
- `SendChatMessage`: Enviar mensagem
- `GetConversationHistory`: Obter histórico
- `StartNewConversation`: Iniciar nova conversa
- `GetAIResponse`: Obter resposta da IA

### Progresso
- `TrackStudyProgress`: Rastrear progresso
- `GetUserAchievements`: Obter conquistas
- `CalculateProgressPercentage`: Calcular porcentagem
- `GetSubjectProgress`: Obter progresso por assunto
- `UpdateStudyStreak`: Atualizar sequência de estudo

### Flashcards
- `CreateFlashcardDeck`: Criar deck
- `AddFlashcard`: Adicionar flashcard
- `ReviewFlashcard`: Revisar flashcard
- `GetFlashcardsForReview`: Obter para revisão

### Anotações
- `CreateNote`: Criar anotação
- `UpdateNote`: Atualizar anotação
- `SearchNotes`: Buscar anotações
- `DeleteNote`: Deletar anotação

## Traits de Repositório

```rust
trait EssayRepository {
    async fn save(&self, essay: Essay) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Essay>>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Essay>>;
    async fn list_by_status(&self, user_id: Uuid, status: EssayStatus) -> Result<Vec<Essay>>;
    async fn list_by_exam_type(&self, user_id: Uuid, exam_type: ExamType) -> Result<Vec<Essay>>;
    async fn update(&self, essay: Essay) -> Result<()>;
}

trait ExamRubricRepository {
    async fn get_rubric(&self, exam_type: ExamType) -> Result<Option<ExamRubric>>;
    async fn list_all(&self) -> Result<Vec<ExamRubric>>;
}

trait QuestionRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Question>>;
    async fn list_by_subject(&self, subject: Subject) -> Result<Vec<Question>>;
    async fn list_by_difficulty(&self, difficulty: Difficulty) -> Result<Vec<Question>>;
    async fn search(&self, query: &str) -> Result<Vec<Question>>;
}

trait KnowledgeTrailRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeTrail>>;
    async fn list_available(&self) -> Result<Vec<KnowledgeTrail>>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<KnowledgeTrail>>;
    async fn update_progress(&self, trail_id: Uuid, user_id: Uuid, progress: u8) -> Result<()>;
}

trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserProfile>>;
    async fn update(&self, profile: UserProfile) -> Result<()>;
    async fn update_settings(&self, user_id: Uuid, settings: UserSettings) -> Result<()>;
}

trait ProgressRepository {
    async fn get_progress(&self, user_id: Uuid) -> Result<StudyProgress>;
    async fn track_activity(&self, user_id: Uuid, activity: Activity) -> Result<()>;
    async fn update_streak(&self, user_id: Uuid, streak: u32) -> Result<()>;
}

trait ConversationRepository {
    async fn create(&self, conversation: Conversation) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Conversation>>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Conversation>>;
    async fn add_message(&self, conversation_id: Uuid, message: ChatMessage) -> Result<()>;
}

trait FlashcardRepository {
    async fn create_deck(&self, deck: FlashcardDeck) -> Result<()>;
    async fn find_deck(&self, id: Uuid) -> Result<Option<FlashcardDeck>>;
    async fn add_flashcard(&self, flashcard: Flashcard) -> Result<()>;
    async fn update_flashcard(&self, flashcard: Flashcard) -> Result<()>;
    async fn get_due_for_review(&self, deck_id: Uuid) -> Result<Vec<Flashcard>>;
}

trait NoteRepository {
    async fn create(&self, note: Note) -> Result<()>;
    async fn update(&self, note: Note) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Note>>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Note>>;
    async fn search(&self, user_id: Uuid, query: &str) -> Result<Vec<Note>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
```

## Critérios de Avaliação por Vestibular

### ENEM
- **Pontuação**: 0-1000
- **Competências** (5 competências, 200 pontos cada):
  1. Domínio da escrita formal
  2. Compreensão da proposta
  3. Seleção e organização de informações
  4. Demonstração de conhecimento linguístico
  5. Elaboração de proposta de intervenção
- **Características**: Texto dissertativo-argumentativo, 30 linhas mínimo

### Fuvest (USP)
- **Pontuação**: 0-48
- **Competências** (8 competências, 0-6 pontos cada):
  1. Adequação ao tema e gênero
  2. Coerência e coesão
  3. Domínio da norma padrão
  4. Recursos expressivos
  5. Vocabulário
  6. Estrutura textual
  7. Argumentação
  8. Originalidade
- **Características**: Texto argumentativo, foco em raciocínio e originalidade

### UNICAMP
- **Pontuação**: 0-60
- **Competências** (10 competências, 0-6 pontos cada):
  - Análise e interpretação
  - Argumentação
  - Coerência textual
  - Coesão textual
  - Adequação ao gênero
  - Uso adequado da linguagem
  - Vocabulário
  - Estruturação
  - Criatividade
  - Ortografia e acentuação
- **Características**: Diversidade de gêneros textuais

### UNESP
- **Pontuação**: 0-40
- **Competências** (5 competências, 0-8 pontos cada):
  1. Adequação ao tema
  2. Gênero textual
  3. Coerência
  4. Coesão
  5. Correção gramatical
- **Características**: Foco em domínio linguístico

### UERJ
- **Pontuação**: 0-20
- **Competências** (4 competências, 0-5 pontos cada):
  1. Conteúdo temático
  2. Estrutura textual
  3. Linguagem
  4. Mecanismos de coesão
- **Características**: Texto dissertativo-argumentativo

### ITA e IME
- **Pontuação**: ITA (0-50), IME (0-100)
- **Competências**: Foco em argumentação técnica e científica
- **Características**: Redações técnicas com foco em exatas

### Universidades Federais (UFMG, UFPE, UFBA, etc.)
- **Pontuação**: 0-1000 (similar ENEM) ou 0-100 (sistema próprio)
- **Competências**: Variam por universidade, geralmente baseadas no ENEM
- **Características**: Adaptação dos critérios ENEM para contexto regional

### Notas Importantes para Implementação
1. Cada vestibular tem critérios específicos documentados
2. A IA deve avaliar usando os critérios corretos de cada exame
3. Feedback deve mencionar especificamente as competências avaliadas
4. Interface deve exibir score no formato correto (0-1000, 0-48, 0-60, etc.)
5. Banco de dados deve armazenar rubricas oficiais de cada vestibular

## Rotas da Aplicação

```
/                          # Home (Plano de Estudo)
/questoes                  # Lista de questões
/questao/:id               # Detalhes da questão
/redacoes                  # Lista de redações
/redacao/:id               # Detalhes da redação
/redacao/nova              # Criar nova redação
/redacao/nova/:exam_type   # Criar redação para vestibular específico
/redacoes/:exam_type       # Listar redações por tipo de vestibular
/trilhas                   # Lista de trilhas
/trilha/:id                # Detalhes da trilha
/chat                      # Chat tutor (lista de conversas)
/chat/:id                  # Conversa específica
/flashcards                # Lista de decks de flashcards
/flashcards/deck/:id       # Deck específico
/notas                     # Lista de anotações
/nota/:id                  # Anotação específica
/nota/nova                 # Criar nova anotação
/simulados                 # Lista de simulados
/simulado/:id              # Detalhes do simulado
/analise                   # Análise de progresso
/perfil                    # Perfil do usuário
/configuracoes             # Configurações
```

## Padrões de Implementação

### Estrutura de Caso de Uso
```rust
pub struct CreateEssayUseCase {
    essay_repo: Arc<dyn EssayRepository>,
}

impl CreateEssayUseCase {
    pub async fn execute(&self, user_id: Uuid, title: String) -> Result<Essay> {
        // Lógica do caso de uso
    }
}
```

### Estrutura de Componente Dioxus
```rust
#[component]
fn NeonButton(cx: Scope, props: NeonButtonProps) -> Element {
    render! {
        button {
            class: "neon-button",
            onclick: move |_| props.on_click.call(()),
            "{props.children}"
        }
    }
}
```

### Tema Cyberpunk em CSS
```css
.neon-button {
    background: transparent;
    border: 2px solid #9d4edd;
    color: #9d4edd;
    text-shadow: 0 0 10px #9d4edd, 0 0 20px #9d4edd;
    box-shadow: 0 0 10px #9d4edd, inset 0 0 10px #9d4edd;
    transition: all 0.3s ease;
}

.neon-button:hover {
    border-color: #c77dff;
    box-shadow: 0 0 20px #c77dff, 0 0 30px #c77dff;
}
```



## Comandos de Desenvolvimento

### Makefile Targets
```makefile
dev-web:       # Desenvolver para web (wasm)
dev-desktop:   # Desenvolver para desktop
dev-mobile:    # Desenvolver para mobile
test:          # Rodar testes
check:         # cargo check
fmt:           # cargo fmt
clippy:        # cargo clippy
build:         # Build de produção
```

## Princípios de Desenvolvimento

1. **Arquitetura Limpa**: Domínio → Dados → Apresentação
2. **TDD**: Testes unitários para lógica de domínio
3. **Async/Await**: Todas operações I/O são assíncronas
4. **Idioma Português**: UI e conteúdo em português
5. **Multiplataforma**: Funcionar em web, desktop e mobile
6. **Acessibilidade**: Mesmo em cyberpunk, manter acessibilidade
7. **Performance**: Otimizar para dispositivos móveis
8. **Consistência Visual**: Manter tema cyberpunk em todos os componentes
9. **Componentes Reutilizáveis**: Criar biblioteca de componentes UI
10. **Documentação**: Manter código documentado

## Checklist de Implementação por Fase

### Fase 1 ✅
- [ ] Setup workspace Rust
- [ ] Configurar Dioxus
- [ ] Criar estrutura de crates
- [ ] Implementar componentes UI base
- [ ] Tema cyberpunk completo
- [ ] Roteamento básico
- [ ] Páginas principais (lista)

### Fase 2 ✅
- [ ] Editor de redação
- [ ] Avaliação de redação (mock)
- [ ] Visualização de questão
- [ ] Sistema de trilhas básico
- [ ] Rastreamento de progresso

### Fase 3 ✅
- [ ] Integração com IA
- [ ] Chat tutor
- [ ] Avaliação IA real
- [ ] Sistema de conquistas
- [ ] Recomendações

### Fase 4 ✅
- [ ] Sistema de flashcards
- [ ] Editor de anotações
- [ ] Sistema de simulados
- [ ] Análise de progresso avançada
- [ ] Integração multimídia

### Fase 5 ✅
- [ ] Otimização de performance
- [ ] Modo offline
- [ ] Persistência SQLite
- [ ] Acessibilidade completa
- [ ] Animações refinadas
