# Documentation Structure

## Overview

The NeuroNexus project documentation is now **English-first** with translations available in Portuguese (Português) and Chinese (中文).

## Structure

```
NeuroNexus/
├── README.md                  # English (primary)
├── QUICKSTART.md              # English (primary)
├── TESTING.md                 # English (primary)
├── STATUS.md                  # English (primary)
├── PROGRESS.md                # English (primary)
├── SEEDERS.md                 # English (primary)
├── IMPORT_GUIDE.md            # English (primary)
├── CODEX.md                   # English (primary, header added)
├── DOCS_STRUCTURE.md          # This file
└── docs/
    ├── TRANSLATIONS.md        # Translation status and guidelines
    ├── pt/                    # Portuguese translations
    │   ├── README.md
    │   ├── QUICKSTART.md
    │   └── CODEX_TRANSLATION_NEEDED.md
    └── zh/                    # Chinese translations
        ├── README.md
        └── QUICKSTART.md
```

## Language Navigation

Each documentation file includes language selector links at the top:

```markdown
> 🌐 **[Português](docs/pt/FILENAME.md)** | **[中文](docs/zh/FILENAME.md)**
```

Users can easily switch between languages using these links.

## Documentation Files

### Core Documentation (Root Directory - English)

1. **README.md** - Project overview, quick introduction
2. **QUICKSTART.md** - Quick start guide for developers
3. **TESTING.md** - Testing and running the application
4. **STATUS.md** - Current development status
5. **PROGRESS.md** - Project progress tracking
6. **SEEDERS.md** - Test data seeders documentation
7. **IMPORT_GUIDE.md** - Import feature user guide
8. **CODEX.md** - Complete project specification (detailed)

### Translated Documentation

#### Portuguese (`docs/pt/`)
- ✅ README.md
- ✅ QUICKSTART.md
- 🔄 Other files pending translation

#### Chinese (`docs/zh/`)
- ✅ README.md
- ✅ QUICKSTART.md
- 🔄 Other files pending translation

## Translation Guidelines

### For Contributors

1. **Write new documentation in English first** in the root directory
2. Add language selector links at the top of each file
3. Create translations in respective language directories
4. Maintain the same file structure and formatting
5. Keep code examples and technical terms in English
6. Update `docs/TRANSLATIONS.md` with translation status

### File Naming Convention

- English files: `FILENAME.md` (root directory)
- Portuguese files: `docs/pt/FILENAME.md`
- Chinese files: `docs/zh/FILENAME.md`

## Current Status

| Document | English | Português | 中文 |
|----------|---------|-----------|------|
| README.md | ✅ | ✅ | ✅ |
| QUICKSTART.md | ✅ | ✅ | ✅ |
| TESTING.md | ✅ | 🔄 Pending | 🔄 Pending |
| STATUS.md | ✅ | 🔄 Pending | 🔄 Pending |
| PROGRESS.md | ✅ | 🔄 Pending | 🔄 Pending |
| SEEDERS.md | ✅ | 🔄 Pending | 🔄 Pending |
| IMPORT_GUIDE.md | ✅ | 🔄 Pending | 🔄 Pending |
| CODEX.md | ✅ Header | ✅ Original | 🔄 Pending |

**Note:** CODEX.md is currently primarily in Portuguese and needs full English translation. The header with language links has been added.

## Benefits

1. **Accessibility**: Documentation available in multiple languages
2. **Consistency**: English-first approach ensures consistency
3. **Organization**: Clear separation of languages in directories
4. **Navigation**: Easy switching between languages via links
5. **Maintenance**: Easy to track translation status

## Future Work

- Complete Portuguese translations of remaining files
- Complete Chinese translations of remaining files
- Translate CODEX.md fully to English
- Add automated translation workflow
- Consider additional languages based on user base

## Contributing

To contribute translations:

1. Check `docs/TRANSLATIONS.md` for pending translations
2. Copy the English version
3. Translate while maintaining structure
4. Submit a pull request
5. Update translation status in `docs/TRANSLATIONS.md`

For detailed guidelines, see [docs/TRANSLATIONS.md](docs/TRANSLATIONS.md).
