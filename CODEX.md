# NeuroNexus - Development Codex

> 🌐 **[Português](docs/pt/CODEX.md)** | **[中文](docs/zh/CODEX.md)**

## Overview
NeuroNexus is a multi-platform (web, desktop, mobile) educational platform built with Rust and Dioxus, focused on preparing for college entrance exams and ENEM. The platform combines generative AI, personalized learning paths, and an immersive cyberpunk-inspired interface (inspired by Cyberpunk 2077 and Blade Runner 2049).

## Design System: Neon Cyberpunk
### Visual Design
- **Base Theme**: Dark mode with deep dark gradients
- **Primary Colors**:
  - Background: `#000000` (absolute black) and `#0a0a0a` (soft black)
  - Neon Purple: `#9d4edd`, `#c77dff`, `#e0aaff`
  - Neon Pink: `#ff69b4`, `#ff10f0`, `#ff00ff`
  - Neon Blue: `#00ffff`, `#0ff0ff`, `#00d9ff`
  - Gold Accents: `#ffd700`, `#ffed4e`
  - Glow Effects: Bright shadows with blur for neon effect

### Dioxus UI Components
- **NeonButton**: Buttons with neon borders and glow hover
- **NeonInput**: Inputs with pulsating borders and glow
- **CyberCard**: Cards with dark gradients and neon borders
- **StatusBar**: Top bar with time and indicators (cyberpunk style)
- **TabBar**: Bottom navigation with neon icons
- **GlitchText**: Text with glitch/scanline effect (optional)
- **HologramEffect**: Holographic effects for important elements
- **NeonProgressBar**: Progress bars with neon animation

### Typography
- **Fonts**: Modern sans-serif, preferably monospace for technical elements
- **Hierarchy**: Titles in CAPS LOCK with glow, readable body text
- **Effects**: Neon text-shadow, increased tracking for titles

### Application Icon
- **Icon**: Cyberpunk representation of hammer and scythe over graduation cap and books
- **Icon Colors**:
  - Hammer and Scythe: Red neon (`#ff0000`) with blue neon outline (`#00ffff`)
  - Graduation Cap: Dark blue (`#000080`) with cyan neon outline (`#00ffff`)
  - Books: Dark blue (`#000080`) with cyan neon outline (`#00ffff`)
  - Background: Absolute black (`#000000`)
- **Style**: Neon cyberpunk aesthetic with glow effects
- **Location**: `assets/icon.png`, `assets/icon.ico` (Windows), `assets/icon.icns` (macOS), `assets/icon.png` (Android/iOS)

## Technical Architecture

### Technology Stack
- **Language**: Rust (edition 2021, minimum 1.75)
- **UI Framework**: Dioxus 0.4+
- **Architecture**: Clean Architecture (Domain → Data → Presentation)
- **Structure**: Workspace with multiple crates

### Crate Structure
```
crates/
├── domain/          # Pure business logic, models, use cases, traits
├── data/            # Repository implementations, database, seeders
├── app/             # Dioxus components, pages, routing, UI
├── shared/          # Shared utilities, common types
└── services/        # External services (AI, API, etc.)
```

### Compilation Targets
- **Web**: `wasm32-unknown-unknown` (Dioxus Web)
- **Desktop**: Wry renderer (WebView)
- **Mobile**: Platform-specific (Tauri mobile)

## Implementation Phases

### PHASE 1: Core MVP (Foundation)
#### Objective
Establish base architecture and essential navigation and display functionalities.

#### Core Features
1. **Navigation System**
   - Bottom TabBar with 4 tabs: Plan, Questions, Essays, Profile
   - Basic routing between pages
   - Top StatusBar with time and online indicator

2. **Study Plan (Home)**
   - Dashboard with subject categories
   - Quick access cards
   - Basic statistics (total essays, questions answered)
   - Cyberpunk theme applied

3. **Question Listing**
   - Viewing questions by category
   - Simple keyword search
   - Basic filters (subject, difficulty)
   - Question card with preview

4. **Essay Listing**
   - List of user essays
   - Status: In Progress / Corrected
   - Filter by status
   - Essay card with title and score

5. **Basic Profile**
   - User name and avatar
   - Basic information
   - Simple settings

#### Phase 1 Implementation
```
1. Setup Rust workspace with crates
2. Configure Dioxus for web/desktop/mobile
3. Create basic layer structure (domain, data, app)
4. Implement basic routing
5. Create base UI components (NeonButton, NeonInput, CyberCard, TabBar, StatusBar)
6. Implement cyberpunk theme (colors, effects, typography)
7. Pages: Home, Questions (list), Essays (list), Profile (basic)
8. In-memory repositories for development
9. Data test seeders
```

### PHASE 2: Essential Educational Features
#### Objective
Implement main learning and evaluation functionalities.

#### Features Phase 2

1. **Complete Essay Management**
   - Create new essay (text editor)
   - Select exam type/university (ENEM, USP, UNICAMP, etc.)
   - Specific correction by exam format
   - Save draft
   - Submit for evaluation
   - View detailed feedback based on exam criteria
   - Score with highlight (0-1000 for ENEM, 0-48 for Fuvest/USP, etc.)
   - Complete history with formatted dates
   - Filter essays by exam type

2. **Detailed Question System**
   - View full question
   - Answer alternatives
   - Explained resolution
   - Mark as answered
   - Navigate between questions

3. **Knowledge Trails**
   - List of available trails
   - View trail details
   - Progress tracking (percentage)
   - Start/continue trail
   - Focus areas by trail

4. **Basic Progress System**
   - Track answered questions
   - Track submitted essays
   - Calculate progress by category
   - Display metrics on dashboard

#### Phase 2 Implementation
```
1. Implement ExamType enum with all supported exams
2. Domain use cases for essays:
   - CreateEssay (with ExamType), SaveEssayDraft, SubmitEssayForEvaluation, GetEssayFeedback
   - ListEssaysByExamType, GetExamRubric

3. Domain use cases for questions:
   - GetQuestionDetails, MarkQuestionAsAnswered, GetQuestionExplanation

4. Domain use cases for trails:
   - StartKnowledgeTrail, UpdateTrailProgress, GetTrailDetails

5. Implement essay editor (textarea with cyberpunk styling)
6. Exam type selection component (cyberpunk dropdown/selector)
7. Implement ExamRubric and RubricScores models
8. Create ExamRubricRepository
9. Implement base rubrics for ENEM, Fuvest, UNICAMP, UNESP, UERJ
10. Question details page
11. Trail details page
12. Mock essay evaluation service (simulate AI with specific criteria)
13. Display scores in correct format per ExamType
14. Filter essays by exam type in UI
15. Update UI with neon animations
```

### PHASE 3: AI and Personalization
#### Objective
Add artificial intelligence and personalized learning.

#### Features Phase 3

1. **AI Tutor Chat**
   - Real-time chat with AI tutor
   - Contextualized responses
   - Subject support
   - Conversation history
   - Cyberpunk chat interface (messages with glow)

2. **Essay Evaluation with AI**
   - Detailed analysis by specific competencies of each exam
   - Correction based on official criteria of each exam
   - Specific improvement suggestions by exam type
   - Highlight errors/opportunities
   - Constructive feedback adapted to exam format
   - Comparison with historical averages of the specific exam

3. **Personalized Trails**
   - Trail generation based on performance
   - Content recommendations
   - Difficulty adaptation

4. **Achievement System**
   - Badges and achievements
   - Achievement unlock notifications
   - Achievement view in profile
   - Unlock animations

#### Phase 3 Implementation
```
1. Hugging Face Hub integration for LLM model downloads
2. LLM model download and management system (dlite-v2-1.5B)
3. Download interface on first execution
4. Local inference service using Candle (dlite-v2-1.5B)
5. Quantization optimizations for mobile (INT8/INT4)
6. P2P LLM service for sharing capacity
7. Tutor chat integration
8. Essay evaluation integration
9. Inference caching system
10. Implement assessment rubrics for each ExamType
11. Adaptive correction system by exam type
12. Database of official criteria for each exam
13. Basic recommendation system
14. Achievement system (domain + UI)
15. Cyberpunk-style toast notifications
16. Unlock animations
```

### PHASE 4: Advanced Features
#### Objective
Expand functionalities with additional study tools.

#### Features Phase 4

1. **Flashcards**
   - Create custom flashcards
   - Flashcard deck by subject
   - Review mode with spacing
   - Flip animation
   - Success statistics

2. **Study Notes**
   - Subject notes block
   - Code editor
   - Note search
   - Note export
   - Tags and categorization

3. **Simulations and Tests**
   - Create simulations by subject
   - Timed mode
   - Detailed results
   - Performance graphs
   - Comparison with average

4. **Advanced Progress Analysis**
   - Performance graphs over time
   - Activity heatmap
   - Subject statistics
   - Custom study goals
   - Weekly/monthly reports

5. **Multimedia Content**
   - Integration with educational videos
   - Links to external materials
   - Bibliography and references

#### Phase 4 Implementation
```
1. Flashcard system (full CRUD)
2. Code note editor
3. Simulation system (timer, questions, results)
4. Graph library (for analyses)
5. Integration with video API (YouTube, etc.)
6. Data export system
7. Data visualization components (graphs, heatmaps)
```

### PHASE 5: Refinement and Optimization
#### Objective
Improve performance, UX, and add final polish.

#### Features Phase 5

1. **Performance**
   - Bundle size optimization
   - Lazy loading of components
   - Smart caching
   - Rerender optimization

2. **Offline Mode**
   - Local synchronization
   - Content caching
   - PWA for web

3. **Accessibility**
   - Screen reader support
   - Keyboard navigation
   - Adequate contrast (even in cyberpunk)

4. **Animations and Transitions**
   - Smooth transitions between pages
   - Micro-interactions
   - Styled loading states
   - Improved visual feedback

5. **Data Persistence**
   - Integration with SQLite
   - Database migrations
   - Backup and restore

#### Phase 5 Implementation
```
1. Profiling and optimization
2. Implement local persistence (SQLite)
3. Database migrations
4. Improve accessibility
5. Add transition animations
6. Implement PWA (for web)
7. Performance tests
```

## Domain Models

### Essays
```rust
Essay {
    id: Uuid,
    title: String,
    content: String,
    exam_type: ExamType, // Type of exam/university
    status: EssayStatus,
    score: Option<u16>, // Varies per exam (0-1000 ENEM, 0-48 Fuvest, etc.)
    max_score: u16, // Maximum score of the exam type
    feedback: Option<String>,
    corrections: Option<Vec<Correction>>,
    rubric_scores: Option<RubricScores>, // Scores by competency/criterion
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
}

ExamType {
    ENEM,           // 0-1000 (5 competencies, 200 each)
    Fuvest,         // USP - 0-48 (8 competencies, 0-6 each)
    Unicamp,        // 0-60 (10 competencies)
    Unesp,          // 0-40 (5 competencies, 0-8 each)
    Uerj,           // 0-20 (4 competencies, 0-5 each)
    Ita,            // 0-50 (10 specific competencies)
    Ime,            // 0-100 (20 technical competencies)
    Ufrj,           // 0-10 (5 competencies, 0-2 each)
    Ufmg,           // 0-1000 (similar ENEM)
    Ufsc,           // 0-100 (10 competencies)
    Ufrgs,          // 0-100 (10 competencies)
    Ufpr,           // 0-100 (10 competencies)
    Ufscar,         // 0-100 (10 competencies)
    Ufpe,           // 0-1000 (similar ENEM)
    Ufba,           // 0-1000 (similar ENEM)
    Ufc,            // 0-1000 (similar ENEM)
    Ufpa,           // 0-1000 (similar ENEM)
    Ufam,           // 0-1000 (similar ENEM)
    Ufac,           // 0-1000 (similar ENEM)
    Ufrn,           // 0-1000 (similar ENEM)
    Ufms,           // 0-1000 (similar ENEM)
    Ufg,            // 0-100 (10 competencies)
    Ufes,           // 0-1000 (similar ENEM)
    Ufjf,           // 0-100 (10 competencies)
    Ufv,            // 0-100 (10 competencies)
    Uftm,           // 0-100 (10 competencies)
    Ufpb,           // 0-1000 (similar ENEM)
    Ufpi,           // 0-1000 (similar ENEM)
    Ufma,           // 0-1000 (similar ENEM)
    Ufs,            // 0-100 (10 competencies)
    Uft,            // 0-100 (10 competencies)
    Ufopa,          // 0-1000 (similar ENEM)
    Unb,            // 0-100 (10 competencies)
    Ueg,            // 0-100 (10 competencies)
    Uema,           // 0-1000 (similar ENEM)
    Uece,           // 0-1000 (similar ENEM)
    Uern,           // 0-1000 (similar ENEM)
    Uepb,           // 0-100 (10 competencies)
    Uepg,           // 0-100 (10 competencies)
    Uel,            // 0-100 (10 competencies)
    Unemat,         // 0-1000 (similar ENEM)
    Uerr,           // 0-1000 (similar ENEM)
    Uea,            // 0-1000 (similar ENEM)
    Ufrr,           // 0-1000 (similar ENEM)
    Upe,            // 0-1000 (similar ENEM)
    Uenf,           // 0-100 (10 competencies)
    Unifesp,        // 0-100 (10 competencies)
    Uesc,           // 0-100 (10 competencies)
    Uemg,           // 0-100 (10 competencies)
    Uem,            // 0-100 (10 competencies)
    Uesb,           // 0-100 (10 competencies)
    Uespi,          // 0-100 (10 competencies)
    Uesf,           // 0-100 (10 competencies)
    Uesr,           // 0-100 (10 competencies)
    Uesg,           // 0-100 (10 competencies)
}

EssayStatus {
    EmProgresso,
    Corrigida,
    Enviada, // Waiting for correction
}

Correction {
    position: usize,
    original_text: String,
    suggested_text: String,
    reason: String,
    rubric_criterion: String, // Which competency/criterion was affected
}

RubricScores {
    // Variable structure per ExamType
    // Example for ENEM: 5 competencies (0-200 each)
    // Example for Fuvest: 8 competencies (0-6 each)
    scores: HashMap<String, u16>, // Key: competency name, Value: score
    detailed_feedback: HashMap<String, String>, // Feedback per competency
}

// Specific evaluation criteria per exam
ExamRubric {
    exam_type: ExamType,
    criteria: Vec<RubricCriterion>,
    max_score: u16,
    description: String, // Description of exam format
}

RubricCriterion {
    name: String,
    description: String,
    weight: f32, // Weight in total score
    max_score: u16,
    evaluation_points: Vec<String>, // Evaluation points
}
```

### Questions
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

### Knowledge Trails
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

### Profile and Progress
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

### Chat and AI
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

### Notes
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

## Use Cases

### Essays
- `CreateEssay`: Create new essay (with exam type selection)
- `SaveEssayDraft`: Save draft
- `SubmitEssayForEvaluation`: Submit for evaluation (with specific exam correction)
- `GetEssayFeedback`: Get detailed feedback based on exam criteria
- `ListEssaysByStatus`: List by status
- `ListEssaysByExamType`: List by exam type
- `GetEssayScore`: Get score
- `UpdateEssay`: Update essay
- `GetExamRubric`: Get exam type evaluation criteria
- `EvaluateEssay`: Evaluate essay using specific ExamType criteria

### Questions
- `SearchQuestions`: Search questions
- `FilterQuestionsBySubject`: Filter by subject
- `FilterQuestionsByDifficulty`: Filter by difficulty
- `GetQuestionDetails`: Get details
- `MarkQuestionAsAnswered`: Mark as answered
- `GetQuestionExplanation`: Get explanation

### Trails
- `StartKnowledgeTrail`: Start trail
- `UpdateTrailProgress`: Update progress
- `GetTrailDetails`: Get details
- `CompleteTrailModule`: Complete module
- `ListAvailableTrails`: List available trails

### Chat Tutor
- `SendChatMessage`: Send message
- `GetConversationHistory`: Get history
- `StartNewConversation`: Start new conversation
- `GetAIResponse`: Get AI response

### Progress
- `TrackStudyProgress`: Track progress
- `GetUserAchievements`: Get achievements
- `CalculateProgressPercentage`: Calculate percentage
- `GetSubjectProgress`: Get progress by subject
- `UpdateStudyStreak`: Update study streak

### Flashcards
- `CreateFlashcardDeck`: Create deck
- `AddFlashcard`: Add flashcard
- `ReviewFlashcard`: Review flashcard
- `GetFlashcardsForReview`: Get for review

### Notes
- `CreateNote`: Create note
- `UpdateNote`: Update note
- `SearchNotes`: Search notes
- `DeleteNote`: Delete note

## Repository Traits

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

## Evaluation Criteria by Exam

### ENEM
- **Score**: 0-1000
- **Competencies** (5 competencies, 200 points each):
  1. Domain of formal writing
  2. Understanding of the proposition
  3. Selection and organization of information
  4. Demonstration of linguistic knowledge
  5. Elaboration of intervention proposal
- **Characteristics**: Dissertative-argumentative text, minimum 30 lines

### Fuvest (USP)
- **Score**: 0-48
- **Competencies** (8 competencies, 0-6 points each):
  1. Appropriateness to theme and genre
  2. Coherence and cohesion
  3. Mastery of standard norms
  4. Expressive resources
  5. Vocabulary
  6. Textual structure
  7. Argumentation
  8. Originality
- **Characteristics**: Argumentative text, focus on reasoning and originality

### UNICAMP
- **Score**: 0-60
- **Competencies** (10 competencies, 0-6 points each):
  - Analysis and interpretation
  - Argumentation
  - Textual coherence
  - Textual cohesion
  - Appropriateness to genre
  - Proper use of language
  - Vocabulary
  - Structuring
  - Creativity
  - Orthography and accentuation
- **Characteristics**: Diversity of textual genres

### UNESP
- **Score**: 0-40
- **Competencies** (5 competencies, 0-8 points each):
  1. Appropriateness to theme
  2. Textual genre
  3. Coherence
  4. Cohesion
  5. Grammatical correctness
- **Characteristics**: Focus on linguistic domain

### UERJ
- **Score**: 0-20
- **Competencies** (4 competencies, 0-5 points each):
  1. Thematic content
  2. Textual structure
  3. Language
  4. Cohesion mechanisms
- **Characteristics**: Dissertative-argumentative text

### ITA and IME
- **Score**: ITA (0-50), IME (0-100)
- **Competencies**: Focus on technical and scientific argumentation
- **Characteristics**: Technical essays with focus on exact sciences

### Federal Universities (UFMG, UFPE, UFBA, etc.)
- **Score**: 0-1000 (similar ENEM) or 0-100 (own system)
- **Competencies**: Vary by university, generally based on ENEM
- **Characteristics**: Adaptation of ENEM criteria to regional context

### Important Notes for Implementation
1. Each exam has specific documented criteria
2. The AI must evaluate using the correct criteria for each exam
3. Feedback should specifically mention the evaluated competencies
4. The interface should display the score in the correct format (0-1000, 0-48, 0-60, etc.)
5. The database should store official rubrics for each exam

## Application Routes

```
/                          # Home (Study Plan)
/questoes                  # Question list
/questao/:id               # Question details
/redacoes                  # Essay list
/redacao/:id               # Essay details
/redacao/nova              # Create new essay
/redacao/nova/:exam_type   # Create essay for specific exam
/redacoes/:exam_type       # List essays by exam type
/trilhas                   # Trail list
/trilha/:id                # Trail details
/chat                      # Tutor chat (conversation list)
/chat/:id                  # Specific conversation
/flashcards                # Flashcard deck list
/flashcards/deck/:id       # Specific deck
/notas                     # Notes list
/nota/:id                  # Specific note
/nota/nova                 # Create new note
/simulados                 # Simulation list
/simulado/:id              # Simulation details
/analise                   # Progress analysis
/perfil                    # User profile
/configuracoes             # Settings
```

## Implementation Patterns

### Use Case Structure
``rust
pub struct CreateEssayUseCase {
    essay_repo: Arc<dyn EssayRepository>,
}

impl CreateEssayUseCase {
    pub async fn execute(&self, user_id: Uuid, title: String) -> Result<Essay> {
        // Use case logic
    }
}
```

### Dioxus Component Structure
``rust
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

### Cyberpunk Theme in CSS
```
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



## Development Commands

### Makefile Targets
```makefile
dev-web:       # Develop for web (wasm)
dev-desktop:   # Develop for desktop
dev-mobile:    # Develop for mobile
test:          # Run tests
check:         # cargo check
fmt:           # cargo fmt
clippy:        # cargo clippy
build:         # Production build
```

## Development Principles

1. **Clean Architecture**: Domain → Data → Presentation
2. **TDD**: Unit tests for domain logic
3. **Async/Await**: All I/O operations are asynchronous
4. **Portuguese Language**: UI and content in Portuguese
5. **Multiplatform**: Work on web, desktop, and mobile
6. **Accessibility**: Even in cyberpunk, maintain accessibility
7. **Performance**: Optimize for mobile devices
8. **Visual Consistency**: Maintain cyberpunk theme in all components
9. **Reusable Components**: Create UI component library
10. **Documentation**: Keep code documented

## Implementation Checklist by Phase

### Phase 1 ✅
- [ ] Setup workspace Rust
- [ ] Configure Dioxus
- [ ] Create crate structure
- [ ] Implement base UI components
- [ ] Complete cyberpunk theme
- [ ] Basic routing
- [ ] Main pages (list)

### Phase 2 ✅
- [ ] Essay editor
- [ ] Essay evaluation (mock)
- [ ] Question viewing
- [ ] Basic trail system
- [ ] Progress tracking

### Phase 3 ✅
- [ ] AI integration
- [ ] Tutor chat
- [ ] Real AI evaluation
- [ ] Achievement system
- [ ] Recommendations

### Phase 4 ✅
- [ ] Flashcard system
- [ ] Note editor
- [ ] Simulation system
- [ ] Advanced progress analysis
- [ ] Multimedia integration

### Phase 5 ✅
- [ ] Performance optimization
- [ ] Offline mode
- [ ] SQLite persistence
- [ ] Complete accessibility
- [ ] Refined animations
