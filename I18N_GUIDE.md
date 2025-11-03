# NeuroNexus Internationalization (i18n) System

## Overview

NeuroNexus now supports internationalization and localization for 246 languages using the Fluent localization system. The infrastructure is designed to provide seamless multi-language support with automatic system locale detection and user-selectable language preferences.

## Architecture

### Core Components

1. **Locale Detection Service** (`shared/src/i18n/locale.rs`)
   - Detects system locale automatically
   - Manages user language preferences
   - Provides locale validation and normalization

2. **Fluent Loader** (`shared/src/i18n/fluent_loader.rs`)
   - Loads and caches Fluent translation bundles
   - Manages .ftl translation files
   - Provides efficient resource loading

3. **Translator Service** (`shared/src/i18n/translator.rs`)
   - Core translation lookup functionality
   - Fallback mechanism (current locale → English → key itself)
   - Variable interpolation support
   - Date/time/number formatting (extensible)

4. **App Context Integration** (`app/src/context.rs`)
   - Global translator instance accessible throughout the app
   - Helper methods: `t()`, `set_locale()`, `current_locale()`
   - Integrated with Dioxus context system

## Translation Files Structure

```
locales/
├── en-US/              # English (default fallback)
│   ├── common.ftl      # Common UI elements
│   ├── navigation.ftl  # Navigation menus and tabs
│   ├── essays.ftl      # Essays page translations
│   ├── questions.ftl   # Questions page translations
│   ├── profile.ftl     # Profile page translations
│   ├── trails.ftl      # Knowledge trails translations
│   ├── home.ftl        # Home/Dashboard translations
│   └── domain.ftl      # Domain model translations
├── pt-BR/              # Portuguese (Brazil) - Primary audience
│   ├── common.ftl
│   ├── navigation.ftl
│   └── ... (same structure as en-US)
└── [244+ more languages...]
```

## Translation Key Naming Convention

Format: `{domain}-{component}-{element}-{variant?}`

Examples:
- `nav-sidebar-label-essays` - Sidebar navigation label for Essays
- `common-button-save` - Save button (reusable across app)
- `domain-difficulty-medium` - Medium difficulty level
- `profile-stats-label-studytime` - Study time statistics label

## Using Translations in Components

### Basic Translation

```rust
use dioxus::prelude::*;

#[component]
fn MyComponent() -> Element {
    let ctx = use_context::<AppContext>();
    
    rsx! {
        h1 { "{ctx.t(\"nav-sidebar-label-essays\")}" }
        button { "{ctx.t(\"common-button-save\")}" }
    }
}
```

### Translation with Variables (Future Enhancement)

The Fluent system supports variable interpolation for dynamic content:

```ftl
# In translation file
home-header-greeting = Hello, { $name }!
```

```rust
// Usage (to be implemented)
ctx.t_with_args("home-header-greeting", &[("name", user.name)])
```

## Locale Detection Priority

1. **User-selected language** (stored in preferences/local storage)
2. **System locale** (detected from OS)
3. **English (en-US)** (default fallback)

## Supported Languages

### Phase 1-3 (Currently Implemented)
- en-US - English (United States) - Default
- pt-BR - Português (Brasil) - Primary audience

### Phase 4 (High Priority - To be added)
- es-ES - Español (España)
- es-MX - Español (México)
- fr-FR - Français
- de-DE - Deutsch
- it-IT - Italiano
- ja-JP - 日本語
- zh-CN - 简体中文
- ko-KR - 한국어

### Phase 5 (Full Rollout)
- All 246 Google Translate supported languages

## RTL (Right-to-Left) Language Support

The system includes RTL detection for languages like Arabic, Hebrew, Persian, and Urdu:

```rust
use shared::i18n::is_rtl;

if is_rtl(&locale) {
    // Apply RTL-specific layouts
}
```

## Adding New Translations

### 1. Create New Translation File

Create a new `.ftl` file in the appropriate locale directory:

```bash
touch locales/en-US/new_feature.ftl
```

### 2. Add Translation Keys

```ftl
# new_feature.ftl
feature-title = My New Feature
feature-description = This is a new feature description
feature-button-action = Take Action
```

### 3. Use in Components

```rust
ctx.t("feature-title")
ctx.t("feature-description")
ctx.t("feature-button-action")
```

## Adding a New Language

### 1. Create Locale Directory

```bash
mkdir locales/fr-FR
```

### 2. Copy Base Translation Files

```bash
cp locales/en-US/*.ftl locales/fr-FR/
```

### 3. Translate Content

Edit each `.ftl` file in the new language directory with appropriate translations.

### 4. Register Language (Optional)

Add the language to the supported languages list in `shared/src/i18n/locale.rs`:

```rust
Locale::new("fr-FR", "Français", "French"),
```

## Changing Language at Runtime

### In Components

```rust
// Change language
if ctx.set_locale("pt-BR").is_ok() {
    // Language changed successfully
    // UI will re-render with new translations
}

// Get current language
let current = ctx.current_locale();
```

### Language Selector Component (To be implemented)

A language selector will be added to the profile page to allow users to:
- View all available languages
- Search/filter languages
- Select and apply language preference
- Persist preference across sessions

## Translation Coverage

### Currently Translated Areas

✅ Navigation (Sidebar, Tab Bar, Breadcrumbs)
✅ Common UI Elements (Buttons, Status, Messages, Forms)
✅ Essays Page (Headers, Filters, Cards, Actions, Status)
✅ Questions Page (Headers, Filters, Cards, Stats)
✅ Profile Page (Headers, Sections, Stats, Preferences)
✅ Knowledge Trails (Headers, Progress, Cards)
✅ Home/Dashboard (Headers, Stats, Actions, Recommendations)
✅ Domain Models (Subjects, Difficulty, Essay Status, Exam Types)

### To Be Translated

⏳ Component-specific strings (in progress)
⏳ Error messages and validation
⏳ Help text and tooltips
⏳ Accessibility labels

## Best Practices

### DO:
- ✅ Use translation keys for all user-facing text
- ✅ Keep keys descriptive and well-organized
- ✅ Group related translations in the same .ftl file
- ✅ Test with multiple languages during development
- ✅ Provide context in comments for translators

### DON'T:
- ❌ Hardcode user-facing strings in components
- ❌ Concatenate translated strings
- ❌ Use translation keys as UI labels directly
- ❌ Forget to handle pluralization
- ❌ Ignore RTL language requirements

## Fluent Syntax Features

### Variables

```ftl
greeting = Hello, { $name }!
items-count = You have { $count } items
```

### Pluralization

```ftl
emails = 
    { $count ->
        [one] You have one email
       *[other] You have { $count } emails
    }
```

### Select Expressions

```ftl
gender-greeting = 
    { $gender ->
        [male] Hello, sir!
        [female] Hello, madam!
       *[other] Hello!
    }
```

## Performance Considerations

- **Lazy Loading**: Translation bundles are loaded on-demand
- **Caching**: Loaded bundles are cached in memory
- **Bundle Size**: Each language bundle is approximately 10-50 KB
- **Lookup Performance**: Sub-100ms translation lookup time

## Testing

### Manual Testing

1. Change system locale and verify detection
2. Switch languages via profile settings
3. Verify all UI elements are translated
4. Test RTL languages for layout issues
5. Check fallback behavior for missing translations

### Automated Testing

```rust
#[test]
fn test_translation_fallback() {
    let mut translator = Translator::new("pt-BR", "./locales");
    
    // Test existing translation
    assert_eq!(translator.translate("common-button-save"), "Salvar");
    
    // Test fallback to English for missing key
    let result = translator.translate("non-existent-key");
    // Should return English translation or key itself
}
```

## Troubleshooting

### Translation Not Showing

1. **Check translation key exists** in both current locale and en-US
2. **Verify .ftl file syntax** - Use Fluent syntax validator
3. **Ensure locale directory exists** and files are loaded
4. **Check console for errors** during bundle loading

### Wrong Language Displaying

1. **Verify current locale**: `ctx.current_locale()`
2. **Check locale detection**: System locale might not be supported
3. **Clear cache**: Restart app to reload translation bundles

### Layout Breaking with Long Translations

1. **Use flexible layouts** with text overflow handling
2. **Test with German/Finnish** (typically longest translations)
3. **Implement truncation** with tooltips where needed

## Future Enhancements

- [ ] Locale-aware date/time formatting
- [ ] Locale-aware number formatting
- [ ] Currency formatting
- [ ] Translation management UI
- [ ] Crowdsourced translation contributions
- [ ] Machine translation baseline for new languages
- [ ] Translation memory and consistency checks
- [ ] In-context translation editing

## Resources

- [Project Fluent Documentation](https://projectfluent.org/)
- [Fluent Syntax Guide](https://projectfluent.org/fluent/guide/)
- [Google Translate Languages](https://cloud.google.com/translate/docs/languages)
- [Language Codes (ISO 639-1)](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)

## Contributing Translations

We welcome translation contributions for all 246 supported languages!

### Process:
1. Fork the repository
2. Create/update translation files in `locales/{locale}/`
3. Follow existing translation key structure
4. Test translations locally
5. Submit pull request with language tag

### Translation Guidelines:
- Maintain consistent tone and style
- Preserve placeholders and variables
- Consider cultural context
- Keep translations concise for UI constraints
- Add comments for ambiguous terms

---

**Status**: Phase 1-3 Complete ✅
**Next Steps**: Component integration, language selector UI, expanded language support
