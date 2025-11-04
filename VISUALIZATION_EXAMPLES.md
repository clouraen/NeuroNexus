# JSON Schema Visualization Examples

This document provides visual examples of how the JSON schemas relate to actual data and how they can be used for validation and documentation.

## Schema Relationship Diagram

```mermaid
graph TB
    subgraph "Meta Schemas"
        Common[common.schema.json<br/>Common Types]
    end
    
    subgraph "Enumeration Schemas"
        Subject[subject.schema.json<br/>16 Subjects]
        Difficulty[difficulty.schema.json<br/>3 Levels]
        ContentType[content_type.schema.json<br/>5 Types]
        ExamType[exam_type.schema.json<br/>55+ Exams]
    end
    
    subgraph "Entity Schemas"
        Question[question.schema.json<br/>Question Entity]
        Essay[essay.schema.json<br/>Essay Entity]
        Trail[knowledge_trail.schema.json<br/>Trail Entity]
    end
    
    subgraph "Import Schemas"
        QImport[question_import.schema.json<br/>Batch Questions]
        TImport[trail_import.schema.json<br/>Batch Trails]
    end
    
    Question -->|uses| Common
    Question -->|uses| Subject
    Question -->|uses| Difficulty
    
    Essay -->|uses| Common
    Essay -->|uses| ExamType
    
    Trail -->|uses| Common
    Trail -->|uses| Subject
    Trail -->|uses| Difficulty
    Trail -->|uses| ContentType
    
    QImport -->|validates| Question
    TImport -->|validates| Trail
    
    style Common fill:#e1f5ff
    style Subject fill:#fff4e1
    style Difficulty fill:#fff4e1
    style ContentType fill:#fff4e1
    style ExamType fill:#fff4e1
    style Question fill:#e8f5e9
    style Essay fill:#e8f5e9
    style Trail fill:#e8f5e9
    style QImport fill:#f3e5f5
    style TImport fill:#f3e5f5
```

## Data Flow: Question Import

```mermaid
flowchart LR
    A[JSON File<br/>sample_questions_import.json] --> B{Validate against<br/>question_import.schema.json}
    B -->|Valid| C[Extract Questions]
    B -->|Invalid| D[Error Report]
    
    C --> E{Validate each against<br/>question.schema.json}
    E -->|Valid| F[Import to Database]
    E -->|Invalid| D
    
    F --> G[✅ Questions Available]
    D --> H[❌ Fix and Retry]
    
    style A fill:#e3f2fd
    style B fill:#fff9c4
    style C fill:#f1f8e9
    style D fill:#ffebee
    style E fill:#fff9c4
    style F fill:#f1f8e9
    style G fill:#c8e6c9
    style H fill:#ffcdd2
```

## Validation Workflow

```mermaid
sequenceDiagram
    participant User
    participant File as JSON File
    participant Schema as JSON Schema
    participant Validator
    participant System
    
    User->>File: Create/Edit JSON
    User->>Validator: Validate File
    Validator->>File: Load JSON
    Validator->>Schema: Load Schema
    Validator->>Validator: Check Structure
    Validator->>Validator: Check Types
    Validator->>Validator: Check Constraints
    
    alt Validation Success
        Validator->>User: ✅ Valid
        User->>System: Import Data
        System->>User: ✅ Imported
    else Validation Failure
        Validator->>User: ❌ Errors
        User->>File: Fix Issues
        User->>Validator: Re-validate
    end
```

## Example: Question Schema Validation

### Valid Question

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "subject": "MATEMATICA",
  "difficulty": "Medio",
  "statement": "Qual é a soma de 5 + 3?",
  "alternatives": [
    {"id": 0, "text": "6"},
    {"id": 1, "text": "7"},
    {"id": 2, "text": "8"},
    {"id": 3, "text": "9"}
  ],
  "correct_answer": 2,
  "explanation": "A soma de 5 + 3 é igual a 8.",
  "tags": ["aritmetica", "soma"]
}
```

✅ **Validation Result**: PASS
- All required fields present
- UUID format valid
- Subject in enum
- Difficulty in enum
- 4 alternatives (within 2-5 range)
- correct_answer is valid index
- Statement ≥ 10 characters
- Tags are unique

### Invalid Question Examples

#### Missing Required Field

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "subject": "MATEMATICA",
  "difficulty": "Medio",
  "statement": "Qual é a soma de 5 + 3?",
  "alternatives": [
    {"id": 0, "text": "8"}
  ],
  "correct_answer": 0,
  "explanation": "A soma é 8."
  // ❌ Missing "tags" field
}
```

❌ **Error**: `Required property 'tags' is missing`

#### Invalid Enum Value

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "subject": "PROGRAMMING", // ❌ Invalid subject
  "difficulty": "Medio",
  "statement": "Qual é a soma de 5 + 3?",
  "alternatives": [
    {"id": 0, "text": "8"}
  ],
  "correct_answer": 0,
  "explanation": "A soma é 8.",
  "tags": ["aritmetica"]
}
```

❌ **Error**: `Value 'PROGRAMMING' is not in enum [MATEMATICA, FISICA, ...]`

#### Constraint Violation

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "subject": "MATEMATICA",
  "difficulty": "Medio",
  "statement": "Curta", // ❌ Too short (< 10 chars)
  "alternatives": [
    {"id": 0, "text": "8"}
  ],
  "correct_answer": 0,
  "explanation": "A soma é 8.",
  "tags": ["aritmetica"]
}
```

❌ **Error**: `String 'Curta' is too short (minimum: 10 characters)`

## Schema Hierarchy Visualization

```mermaid
classDiagram
    class CommonTypes {
        <<meta>>
        +UUID
        +DateTime
        +PositiveInteger
        +Email
        +PercentageScore
        +TagArray
    }
    
    class SubjectEnum {
        <<enum>>
        MATEMATICA
        FISICA
        LINGUA_PORTUGUESA
        +16 subjects total
    }
    
    class DifficultyEnum {
        <<enum>>
        Facil
        Medio
        Dificil
    }
    
    class Question {
        +UUID id
        +Subject subject
        +Difficulty difficulty
        +String statement
        +Alternative[] alternatives
        +int correct_answer
        +String explanation
        +String[] tags
    }
    
    class Alternative {
        +int id
        +String text
    }
    
    Question --> CommonTypes: uses UUID, TagArray
    Question --> SubjectEnum: uses Subject
    Question --> DifficultyEnum: uses Difficulty
    Question *-- Alternative: contains
```

## Type Reference Quick Guide

### Common Types Usage

| Type | JSON Schema Ref | Example Value |
|------|----------------|---------------|
| UUID | `{"$ref": "../meta/common.schema.json#/$defs/UUID"}` | `"550e8400-e29b-41d4-a716-446655440000"` |
| DateTime | `{"$ref": "../meta/common.schema.json#/$defs/DateTime"}` | `"2023-12-01T10:30:00Z"` |
| PercentageScore | `{"$ref": "../meta/common.schema.json#/$defs/PercentageScore"}` | `75` |
| TagArray | `{"$ref": "../meta/common.schema.json#/$defs/TagArray"}` | `["algebra", "geometry"]` |

### Enum Types Usage

| Type | JSON Schema Ref | Example Value |
|------|----------------|---------------|
| Subject | `{"$ref": "../enums/subject.schema.json"}` | `"MATEMATICA"` |
| Difficulty | `{"$ref": "../enums/difficulty.schema.json"}` | `"Medio"` |
| ContentType | `{"$ref": "../enums/content_type.schema.json"}` | `"Question"` |
| ExamType | `{"$ref": "../enums/exam_type.schema.json"}` | `"ENEM"` |

## Schema Versioning Example

```mermaid
gitGraph
    commit id: "v1.0.0 - Initial schemas"
    branch minor-updates
    commit id: "v1.1.0 - Add optional field"
    commit id: "v1.1.1 - Fix typos"
    checkout main
    merge minor-updates
    branch breaking-changes
    commit id: "v2.0.0 - Change field type"
    commit id: "v2.0.1 - Migration guide"
    checkout main
    merge breaking-changes tag: "Current"
```

## Import Format Comparison

### Single Question vs Batch Import

**Single Question (question.schema.json)**:
```json
{
  "id": "...",
  "subject": "MATEMATICA",
  "difficulty": "Medio",
  // ... other fields
}
```

**Batch Import (question_import.schema.json)**:
```json
[
  {
    "id": "...", // Optional
    "subject": "MATEMATICA",
    "difficulty": "Medio",
    // ... other fields
  },
  {
    "subject": "FISICA", // No ID - will be generated
    "difficulty": "Dificil",
    // ... other fields
  }
]
```

## Real-World Integration Example

```mermaid
flowchart TB
    subgraph "Frontend Application"
        UI[User Interface]
        Form[Question Form]
    end
    
    subgraph "Validation Layer"
        ClientVal[Client-side Validation<br/>Uses JSON Schema]
        ServerVal[Server-side Validation<br/>Uses JSON Schema]
    end
    
    subgraph "Backend"
        API[REST API]
        DB[(Database)]
    end
    
    subgraph "Documentation"
        Docs[API Docs<br/>Generated from Schema]
        Types[TypeScript Types<br/>Generated from Schema]
    end
    
    UI --> Form
    Form --> ClientVal
    ClientVal -->|Valid| API
    ClientVal -->|Invalid| UI
    
    API --> ServerVal
    ServerVal -->|Valid| DB
    ServerVal -->|Invalid| API
    
    Schema[JSON Schemas] -.->|Generates| Docs
    Schema -.->|Generates| Types
    Schema -.->|Powers| ClientVal
    Schema -.->|Powers| ServerVal
    
    style Schema fill:#ffd54f
    style ClientVal fill:#81c784
    style ServerVal fill:#81c784
    style Docs fill:#64b5f6
    style Types fill:#64b5f6
```

## Benefits Summary

### For Developers

✅ **IDE Support**
- Autocomplete in JSON files
- Real-time validation
- Inline documentation

✅ **Type Safety**
- Generate TypeScript/Rust types
- Consistent data structures
- Compile-time checks

✅ **Testing**
- Automated validation tests
- Contract testing
- Mock data generation

### For Documentation

✅ **Single Source of Truth**
- Schemas document themselves
- Always up-to-date
- Version controlled

✅ **API Documentation**
- Auto-generate API docs
- Interactive examples
- Clear constraints

### For Data Quality

✅ **Validation**
- Prevent invalid data
- Early error detection
- Clear error messages

✅ **Consistency**
- Uniform data format
- Standard constraints
- Reusable definitions

## Next Steps

1. **Integrate with CI/CD**: Add schema validation to build pipeline
2. **Generate Types**: Create Rust/TypeScript types from schemas
3. **API Documentation**: Generate OpenAPI specs
4. **IDE Configuration**: Set up schema associations
5. **Monitoring**: Track validation errors in production
