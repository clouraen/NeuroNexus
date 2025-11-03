# Import Feature - User Guide

> 🌐 **[Português](docs/pt/IMPORT_GUIDE.md)** | **[中文](docs/zh/IMPORT_GUIDE.md)**

## Overview

The Import feature allows you to import Exam Questions and Learning Trails from JSON files into the NeuroNexus application.

## How to Use

### 1. Access the Import Feature

1. Navigate to the **Profile** page (click the Profile icon in the bottom navigation bar)
2. Look for the **"📥 IMPORTAR"** button in the top-right corner of the Settings panel
3. Click the button to open the Import modal

### 2. Select Import Type

The Import modal presents two options:

- **❓ Questões de Exame**: Import exam questions
- **🗺️ Trilhas de Aprendizado**: Import learning trails

Click on the option you want to import.

### 3. Select JSON File

After clicking an import option:

1. A file picker dialog will open
2. Navigate to your JSON file
3. Select the file (must be a `.json` file)
4. Click "Open" or equivalent in your file picker

### 4. Wait for Import

The modal will show a loading spinner with the message "Importando dados..." while processing your file.

### 5. View Results

After processing, you'll see one of two outcomes:

**Success:**
- Green checkmark icon
- Message showing how many items were imported
- Example: "Successfully imported 10 questions"

**Error:**
- Red X icon
- Error message explaining what went wrong
- Common errors: Invalid JSON format, schema validation failure

### 6. Close the Modal

Click outside the modal or click the X button in the top-right to close it.

## JSON File Formats

### Questions JSON Format

```json
[
  {
    "id": "optional-uuid-will-generate-if-missing",
    "subject": "MATEMATICA",
    "difficulty": "Medio",
    "statement": "Your question text here",
    "alternatives": [
      {
        "id": 0,
        "text": "Option A"
      },
      {
        "id": 1,
        "text": "Option B"
      }
    ],
    "correct_answer": 1,
    "explanation": "Explanation of the correct answer",
    "tags": ["tag1", "tag2"]
  }
]
```

**Valid Subject Values:**
- LINGUA_PORTUGUESA
- LITERATURA
- INGLES
- ESPANHOL
- ARTES
- EDUCACAO_FISICA
- TIC
- HISTORIA
- GEOGRAFIA
- FILOSOFIA
- SOCIOLOGIA
- FISICA
- QUIMICA
- BIOLOGIA
- MATEMATICA
- REDACAO

**Valid Difficulty Values:**
- Facil
- Medio
- Dificil

### Learning Trails JSON Format

```json
[
  {
    "id": "optional-uuid-will-generate-if-missing",
    "title": "Trail Title",
    "description": "Trail description",
    "focus_areas": ["MATEMATICA", "FISICA"],
    "progress": 0,
    "modules": [
      {
        "id": "optional-uuid-will-generate-if-missing",
        "title": "Module Title",
        "description": "Module description",
        "content_type": "Question",
        "content_id": "uuid-of-content",
        "order": 0,
        "completed": false
      }
    ],
    "estimated_hours": 40,
    "difficulty_level": "Medio"
  }
]
```

**Valid ContentType Values:**
- Question
- Essay
- Video
- Reading
- PracticeTest

## Sample Files

Two sample files are provided in the repository root:

1. **sample_questions_import.json**: Contains 3 sample questions
2. **sample_trails_import.json**: Contains 2 sample learning trails

You can use these files to test the import functionality.

## Validation Rules

### Questions

- Statement must not be empty
- Must have 2-5 alternatives
- `correct_answer` must be a valid index (0 to alternatives.length - 1)
- Explanation must not be empty
- Subject must be a valid enum value
- Difficulty must be a valid enum value

### Learning Trails

- Title must not be empty
- Description must not be empty
- Must have at least one focus area
- Progress must be between 0-100
- Must have at least one module
- estimated_hours must be greater than 0
- difficulty_level must be a valid enum value

## Troubleshooting

### "Invalid JSON format" Error

**Cause**: The file is not valid JSON syntax.

**Solution**:
- Validate your JSON using an online JSON validator
- Check for missing commas, brackets, or quotes
- Ensure the file encoding is UTF-8

### "File does not match expected schema" Error

**Cause**: The JSON structure doesn't match the required format.

**Solution**:
- Compare your JSON with the format examples above
- Check that all required fields are present
- Verify enum values match exactly (case-sensitive)

### "Imported X of Y items. Z errors encountered" Message

**Cause**: Some items in your file are valid, others are not.

**Solution**:
- Valid items were imported successfully
- Review your JSON file for items that might be missing required fields
- Check validation rules above for each item

### File Size Limit

**Maximum file size**: 10MB

If your file exceeds this limit, consider splitting it into multiple smaller files.

## Tips

1. **Start Small**: Test with the provided sample files first
2. **Validate Before Import**: Use a JSON validator to check your files before importing
3. **Backup Data**: The import feature adds new data but doesn't modify existing data
4. **UUIDs Optional**: If you don't provide UUIDs, they will be automatically generated
5. **Tags Are Optional**: For questions, the tags array can be empty

## Future Enhancements

Planned improvements for future releases:

- Export functionality to download data as JSON
- Import progress bar for large files
- Data preview before confirming import
- Import from URLs
- Drag-and-drop file import
- Import history tracking
