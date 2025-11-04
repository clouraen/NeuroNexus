# JSON Schema Index

Quick reference index for all NeuroNexus JSON schemas.

## 📋 Quick Links

- [Main README](./README.md) - Comprehensive documentation
- [Implementation Summary](../IMPLEMENTATION_SUMMARY.md) - Project overview
- [Visualization Examples](../VISUALIZATION_EXAMPLES.md) - Visual diagrams and examples

## 📁 Schema Files by Category

### Entity Schemas

| Schema | Path | Size | Description |
|--------|------|------|-------------|
| **Question** | [`entities/question.schema.json`](entities/question.schema.json) | 4.1 KB | Multiple-choice questions with alternatives |
| **Essay** | [`entities/essay.schema.json`](entities/essay.schema.json) | 6.4 KB | Student essays with evaluation data |
| **Knowledge Trail** | [`entities/knowledge_trail.schema.json`](entities/knowledge_trail.schema.json) | 5.4 KB | Learning paths with modules |

### Enumeration Schemas

| Schema | Path | Size | Values | Description |
|--------|------|------|--------|-------------|
| **Subject** | [`enums/subject.schema.json`](enums/subject.schema.json) | 2.5 KB | 16 | Academic subjects |
| **Difficulty** | [`enums/difficulty.schema.json`](enums/difficulty.schema.json) | 2.0 KB | 3 | Difficulty levels |
| **Content Type** | [`enums/content_type.schema.json`](enums/content_type.schema.json) | 2.8 KB | 5 | Module content types |
| **Exam Type** | [`enums/exam_type.schema.json`](enums/exam_type.schema.json) | 2.4 KB | 55+ | Brazilian exam types |

### Import Schemas

| Schema | Path | Size | Description |
|--------|------|------|-------------|
| **Question Import** | [`imports/question_import.schema.json`](imports/question_import.schema.json) | 4.2 KB | Batch question import format |
| **Trail Import** | [`imports/trail_import.schema.json`](imports/trail_import.schema.json) | 5.5 KB | Batch trail import format |

### Meta Schemas

| Schema | Path | Size | Definitions | Description |
|--------|------|------|-------------|-------------|
| **Common** | [`meta/common.schema.json`](meta/common.schema.json) | ~3 KB | 8 | Reusable type definitions |

## 🔗 Schema Dependencies

```
question.schema.json
├── common.schema.json (UUID, TagArray)
├── subject.schema.json
└── difficulty.schema.json

essay.schema.json
├── common.schema.json (UUID, DateTime)
└── exam_type.schema.json

knowledge_trail.schema.json
├── common.schema.json (UUID, PercentageScore, PositiveInteger)
├── subject.schema.json
├── difficulty.schema.json
└── content_type.schema.json

question_import.schema.json
├── common.schema.json (UUID)
├── subject.schema.json
└── difficulty.schema.json

trail_import.schema.json
├── common.schema.json (UUID, PositiveInteger)
├── subject.schema.json
├── difficulty.schema.json
└── content_type.schema.json
```

## 🎯 Common Use Cases

### Validate a Question File

```bash
# Using Python
python3 ../validate_schemas.py

# Or validate specific file with ajv
ajv validate -s entities/question.schema.json -d your_question.json
```

### Import Questions

Use schema: `imports/question_import.schema.json`

Example file format:
```json
[
  {
    "subject": "MATEMATICA",
    "difficulty": "Medio",
    "statement": "Your question...",
    "alternatives": [...],
    "correct_answer": 1,
    "explanation": "Your explanation...",
    "tags": ["tag1", "tag2"]
  }
]
```

### Import Knowledge Trails

Use schema: `imports/trail_import.schema.json`

Example file format:
```json
[
  {
    "title": "Trail Title",
    "description": "Trail description...",
    "focus_areas": ["MATEMATICA"],
    "estimated_hours": 40,
    "difficulty_level": "Medio",
    "modules": [...]
  }
]
```

## 📊 Schema Statistics

| Category | Files | Total Size | Lines |
|----------|-------|------------|-------|
| Entities | 3 | 15.9 KB | ~500 |
| Enums | 4 | 9.7 KB | ~280 |
| Imports | 2 | 9.7 KB | ~325 |
| Meta | 1 | ~3 KB | ~89 |
| **Total** | **10** | **~38 KB** | **~1,194** |

## ✅ Validation Checklist

When creating or modifying schemas:

- [ ] Includes `$schema` declaration
- [ ] Has unique `$id` following naming convention
- [ ] Contains `title` and `description`
- [ ] Specifies `type` for all properties
- [ ] Lists `required` fields
- [ ] Provides realistic `examples`
- [ ] Uses `$ref` for reusable definitions
- [ ] Includes validation constraints
- [ ] Documents all properties
- [ ] Validates successfully with validation script

## 🔍 Schema Lookup by Use Case

### Creating Questions

**Schemas needed:**
1. `entities/question.schema.json` - Question structure
2. `enums/subject.schema.json` - Valid subjects
3. `enums/difficulty.schema.json` - Valid difficulties

### Creating Essays

**Schemas needed:**
1. `entities/essay.schema.json` - Essay structure
2. `enums/exam_type.schema.json` - Valid exam types

### Creating Learning Trails

**Schemas needed:**
1. `entities/knowledge_trail.schema.json` - Trail structure
2. `enums/subject.schema.json` - Valid subjects
3. `enums/difficulty.schema.json` - Valid difficulties
4. `enums/content_type.schema.json` - Valid content types

### Batch Importing

**For Questions:**
- `imports/question_import.schema.json`

**For Trails:**
- `imports/trail_import.schema.json`

## 📚 Additional Resources

- [JSON Schema Official Docs](https://json-schema.org/)
- [Understanding JSON Schema](https://json-schema.org/understanding-json-schema/)
- [JSON Schema Validator](https://www.jsonschemavalidator.net/)
- [NeuroNexus Main Repository](https://github.com/clouraen/NeuroNexus)

## 🆘 Support

For issues or questions:
1. Check [README.md](./README.md) for common questions
2. Review [VISUALIZATION_EXAMPLES.md](../VISUALIZATION_EXAMPLES.md) for examples
3. Open an issue on GitHub

---

**Last Updated**: January 4, 2024  
**Schema Version**: 1.0.0  
**JSON Schema Draft**: 2020-12
