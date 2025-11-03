# Documentation Translations

This directory contains translated versions of the NeuroNexus documentation.

## Available Languages

### English (Default)
Main documentation files are in the root directory:
- [README.md](../README.md)
- [QUICKSTART.md](../QUICKSTART.md)
- [TESTING.md](../TESTING.md)
- [STATUS.md](../STATUS.md)
- [PROGRESS.md](../PROGRESS.md)
- [SEEDERS.md](../SEEDERS.md)
- [IMPORT_GUIDE.md](../IMPORT_GUIDE.md)
- [CODEX.md](../CODEX.md) - *In progress, currently in Portuguese*

### Português (Portuguese)
Located in `docs/pt/`:
- [README.md](pt/README.md)
- [QUICKSTART.md](pt/QUICKSTART.md)
- Additional files to be added

### 中文 (Chinese)
Located in `docs/zh/`:
- [README.md](zh/README.md)
- [QUICKSTART.md](zh/QUICKSTART.md)
- Additional files to be added

## Translation Status

| Document | English | Português | 中文 |
|----------|---------|-----------|------|
| README.md | ✅ | ✅ | ✅ |
| QUICKSTART.md | ✅ | ✅ | ✅ |
| TESTING.md | ✅ | 🔄 Pending | 🔄 Pending |
| STATUS.md | ✅ | 🔄 Pending | 🔄 Pending |
| PROGRESS.md | ✅ | 🔄 Pending | 🔄 Pending |
| SEEDERS.md | ✅ | 🔄 Pending | 🔄 Pending |
| IMPORT_GUIDE.md | ✅ | 🔄 Pending | 🔄 Pending |
| CODEX.md | 🔄 In Progress | ✅ (Original) | 🔄 Pending |

## Contributing Translations

If you'd like to contribute translations:

1. Copy the English version from the root directory
2. Translate the content to your target language
3. Place the translated file in the appropriate language directory (`pt/` or `zh/`)
4. Update the language selector links at the top of the file
5. Submit a pull request

## Language Selector Format

Each document should include language selectors at the top:

```markdown
> 🌐 **[English](../../FILENAME.md)** | **[Português](../pt/FILENAME.md)** | **[中文](../zh/FILENAME.md)**
```

Adjust the paths based on the file location.

## Notes

- The CODEX.md file is currently in Portuguese and needs to be translated to English as the primary version
- All new documentation should be written in English first
- Translations should maintain the same structure and formatting as the English version
- Code examples and technical terms should generally remain in English
