# NeuroNexus JSON Schema Documentation

## Overview

This directory contains JSON Schema definitions for all data structures used in the NeuroNexus platform. These schemas follow the **JSON Schema Draft 2020-12** specification and provide:

- **Validation** - Ensure data integrity across import/export operations
- **Documentation** - Self-documenting API contracts and data formats
- **Tooling Support** - IDE autocomplete, type checking, and validation
- **Consistency** - Single source of truth for data structure definitions

## Directory Structure

```
schemas/
├── README.md                           # This file
├── entities/                           # Core domain entity schemas
│   ├── question.schema.json           # Question entity
│   ├── essay.schema.json              # Essay entity
│   └── knowledge_trail.schema.json    # Knowledge trail entity
├── enums/                              # Enumeration type schemas
│   ├── subject.schema.json            # Academic subjects
│   ├── difficulty.schema.json         # Difficulty levels
│   ├── exam_type.schema.json          # Exam types
│   └── content_type.schema.json       # Content types
├── imports/                            # Import format schemas
│   ├── question_import.schema.json    # Question batch import
│   └── trail_import.schema.json       # Trail batch import
└── meta/                               # Schema metadata
    └── common.schema.json             # Common type definitions
```

## Schema Categories

### Entity Schemas (`entities/`)

Core business domain entities that represent the main data structures:

| Schema | Description | Key Fields |
|--------|-------------|------------|
| **question.schema.json** | Multiple-choice questions | id, subject, difficulty, statement, alternatives, correct_answer |
| **essay.schema.json** | Student essays with evaluation | id, user_id, title, content, exam_type, status, score |
| **knowledge_trail.schema.json** | Learning paths with modules | id, title, focus_areas, modules, progress |

### Enumeration Schemas (`enums/`)

Controlled vocabularies and enumeration types:

| Schema | Description | Values |
|--------|-------------|--------|
| **subject.schema.json** | Academic subjects | MATEMATICA, FISICA, LINGUA_PORTUGUESA, etc. (16 subjects) |
| **difficulty.schema.json** | Difficulty levels | Facil, Medio, Dificil |
| **content_type.schema.json** | Module content types | Question, Essay, Video, Reading, PracticeTest |
| **exam_type.schema.json** | Brazilian exam types | ENEM, FUVEST, ITA, etc. (55+ exam types) |

### Import Schemas (`imports/`)

Formats for batch data import operations:

| Schema | Description | Use Case |
|--------|-------------|----------|
| **question_import.schema.json** | Batch question import | Import questions from JSON files or API |
| **trail_import.schema.json** | Batch trail import | Import learning trails with modules |

### Meta Schemas (`meta/`)

Reusable definitions and common patterns:

| Schema | Description | Definitions |
|--------|-------------|-------------|
| **common.schema.json** | Common types | UUID, DateTime, PositiveInteger, Email, PercentageScore, etc. |

## Quick Start

### Validating Data

To validate your JSON data against a schema, you can use various tools:

#### Using Node.js (ajv)

```bash
npm install -g ajv-cli
ajv validate -s schemas/entities/question.schema.json -d your_question.json
```

#### Using Python (jsonschema)

```python
import json
from jsonschema import validate

# Load schema
with open('schemas/entities/question.schema.json') as f:
    schema = json.load(f)

# Load data
with open('your_question.json') as f:
    data = json.load(f)

# Validate
validate(instance=data, schema=schema)
```

#### Using Online Validators

- [JSON Schema Validator](https://www.jsonschemavalidator.net/)
- [JSON Schema Lint](https://jsonschemalint.com/)

### Importing Questions

Example question import file:

```json
[
  {
    "subject": "MATEMATICA",
    "difficulty": "Medio",
    "statement": "Qual é o valor de x na equação 2x + 5 = 15?",
    "alternatives": [
      {"id": 0, "text": "x = 3"},
      {"id": 1, "text": "x = 5"},
      {"id": 2, "text": "x = 7"},
      {"id": 3, "text": "x = 10"}
    ],
    "correct_answer": 1,
    "explanation": "Para resolver: 2x + 5 = 15 → 2x = 10 → x = 5",
    "tags": ["algebra", "equacoes", "primeiro-grau"]
  }
]
```

Validate against: `schemas/imports/question_import.schema.json`

### Importing Knowledge Trails

Example trail import file:

```json
[
  {
    "title": "ENEM - Matemática Básica",
    "description": "Fundamentos de matemática para o ENEM",
    "focus_areas": ["MATEMATICA"],
    "estimated_hours": 40,
    "difficulty_level": "Facil",
    "modules": [
      {
        "title": "Introdução à Álgebra",
        "description": "Conceitos básicos de álgebra",
        "content_type": "Reading",
        "content_id": "00000000-0000-0000-0000-000000000301",
        "order": 0
      }
    ]
  }
]
```

Validate against: `schemas/imports/trail_import.schema.json`

## Schema Reference

### Common Types

All schemas can reference common type definitions from `meta/common.schema.json`:

| Type | Format | Description | Example |
|------|--------|-------------|---------|
| **UUID** | string (uuid) | RFC 4122 UUID | `"550e8400-e29b-41d4-a716-446655440000"` |
| **DateTime** | string (date-time) | ISO 8601 timestamp | `"2023-12-01T10:30:00Z"` |
| **PositiveInteger** | integer (≥0) | Non-negative integer | `0`, `42`, `1000` |
| **Email** | string (email) | Email address | `"user@example.com"` |
| **PercentageScore** | integer (0-100) | Percentage value | `75` |
| **TagArray** | array[string] | Unique tags | `["algebra", "geometry"]` |

### Subject Enumeration

Academic subjects supported by the platform:

```
LINGUA_PORTUGUESA, LITERATURA, INGLES, ESPANHOL, ARTES, 
EDUCACAO_FISICA, TIC, HISTORIA, GEOGRAFIA, FILOSOFIA, 
SOCIOLOGIA, FISICA, QUIMICA, BIOLOGIA, MATEMATICA, REDACAO
```

### Difficulty Enumeration

Three difficulty levels:

- **Facil** - Easy difficulty
- **Medio** - Medium difficulty  
- **Dificil** - Hard difficulty

### Content Type Enumeration

Types of learning content:

- **Question** - Multiple-choice question
- **Essay** - Writing assignment
- **Video** - Video content
- **Reading** - Reading material
- **PracticeTest** - Practice examination

## Validation Rules

### Question Validation

| Field | Rule | Error if Violated |
|-------|------|-------------------|
| statement | minLength: 10 | Statement too short |
| alternatives | minItems: 2, maxItems: 5 | Must have 2-5 alternatives |
| correct_answer | Valid index into alternatives | Index out of range |
| tags | Unique items | Duplicate tags not allowed |

### Essay Validation

| Field | Rule | Error if Violated |
|-------|------|-------------------|
| title | 3-200 characters | Title length invalid |
| content | minLength: 100 | Content too short |
| score | ≤ max_score | Score exceeds maximum |
| status | One of: EmProgresso, Enviada, Corrigida | Invalid status |

### Trail Validation

| Field | Rule | Error if Violated |
|-------|------|-------------------|
| title | 3-150 characters | Title length invalid |
| focus_areas | minItems: 1, unique | At least one subject required |
| modules | minItems: 1 | At least one module required |
| progress | 0-100 | Invalid percentage |

## Error Handling

### Common Validation Errors

#### Type Mismatch

```json
{
  "error": "TYPE_MISMATCH",
  "path": "/difficulty",
  "expected": "string (enum)",
  "actual": "integer",
  "message": "Expected string from enum [Facil, Medio, Dificil], got integer"
}
```

#### Missing Required Field

```json
{
  "error": "REQUIRED_FIELD_MISSING",
  "path": "/subject",
  "message": "Field 'subject' is required but was not provided"
}
```

#### Constraint Violation

```json
{
  "error": "CONSTRAINT_VIOLATION",
  "path": "/alternatives",
  "constraint": "minItems",
  "expected": 2,
  "actual": 1,
  "message": "Array must contain at least 2 items, found 1"
}
```

## IDE Integration

### Visual Studio Code

Install the [JSON Schema extension](https://marketplace.visualstudio.com/items?itemName=YAML.vscode-yaml) and configure:

```json
{
  "json.schemas": [
    {
      "fileMatch": ["**/questions/*.json"],
      "url": "./schemas/imports/question_import.schema.json"
    },
    {
      "fileMatch": ["**/trails/*.json"],
      "url": "./schemas/imports/trail_import.schema.json"
    }
  ]
}
```

### JetBrains IDEs

Right-click a JSON file → "JSON Schema" → "Add Schema Mapping" → Select schema file.

## Best Practices

### Creating Valid Questions

✅ **Do:**
- Provide clear, unambiguous statements
- Ensure all alternatives are plausible
- Write detailed explanations
- Use relevant, specific tags
- Validate before importing

❌ **Don't:**
- Use vague or ambiguous language
- Include duplicate alternatives
- Leave explanations empty or too brief
- Use the same tag multiple times

### Creating Valid Trails

✅ **Do:**
- Organize modules in logical order
- Use descriptive module titles
- Specify appropriate difficulty levels
- Include estimated completion times
- Reference existing content IDs

❌ **Don't:**
- Skip module ordering (use sequential order values)
- Reference non-existent content IDs
- Mix incompatible difficulty levels
- Create trails without clear learning objectives

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2024-01-04 | Initial schema definitions |

## Schema Versioning

Schemas follow semantic versioning:

- **Major** (X.0.0) - Breaking changes requiring data migration
- **Minor** (0.X.0) - New optional fields, backward compatible
- **Patch** (0.0.X) - Documentation updates, no schema changes

## Contributing

When modifying schemas:

1. Update the schema file
2. Increment version appropriately
3. Update this README
4. Add examples demonstrating new features
5. Test with existing data for backward compatibility

## Related Documentation

- [JSON Schema Specification](https://json-schema.org/draft/2020-12/json-schema-core.html)
- [Understanding JSON Schema](https://json-schema.org/understanding-json-schema/)
- [NeuroNexus Import Guide](../IMPORT_GUIDE.md)
- [API Documentation](../API.md)

## Support

For questions or issues:
- GitHub Issues: [NeuroNexus Issues](https://github.com/clouraen/NeuroNexus/issues)
- Documentation: Check design documents in `.qoder/quests/`

## License

These schemas are part of the NeuroNexus platform and follow the same license as the main project.
