# Internationalization and Localization Feature Design

## Overview

This design outlines the implementation of internationalization (i18n) and localization (l10n) support for NeuroNexus, enabling the application to support 246 official Google Translate languages with a system default fallback to English.

## Business Objectives

- Enable global accessibility by supporting all 246 official Google Translate languages
- Provide seamless language detection based on system locale
- Maintain English as the universal fallback language
- Preserve the cyberpunk neon UI aesthetics across all languages
- Ensure consistent user experience regardless of language selection

## Scope

### In Scope
- Translation infrastructure for all UI text elements
- System locale detection and automatic language selection
- Manual language switching capability
- Translation files management for 246 languages
- RTL (Right-to-Left) language support infrastructure
- Date, time, and number formatting per locale
- Translation of static content (UI labels, buttons, messages)

### Out of Scope
- Translation of user-generated content (essays, question answers)
- Translation of educational content (questions, explanations)
- Real-time machine translation of dynamic content
- Voice/speech localization
- Initial translation of all 246 languages (to be done incrementally)

## Architecture Strategy

### Localization Layer Structure

The i18n system will be integrated as a new shared service accessible across all crates:

```
crates/
├── shared/
│   ├── i18n/           (new module)
│   │   ├── mod.rs
│   │   ├── locale.rs   (locale detection and management)
│   │   ├── translator.rs (translation service)
│   │   └── fluent_loader.rs (resource bundle loader)
```

### Translation Resource Organization

Translation files will be structured by language code using the Fluent localization system format:

```
locales/
├── en-US/              (English - default)
│   ├── common.ftl
│   ├── navigation.ftl
│   ├── essays.ftl
│   ├── questions.ftl
│   └── profile.ftl
├── pt-BR/              (Portuguese - Brazil)
│   ├── common.ftl
│   ├── navigation.ftl
│   └── ...
├── es-ES/              (Spanish - Spain)
├── fr-FR/              (French - France)
├── de-DE/              (German - Germany)
└── [243 more language directories...]
```

## Core Components Design

### 1. Locale Detection Service

**Responsibility**: Detect and manage the application's active locale

**Key Functions**:
- Detect system locale on application startup
- Provide user-initiated locale switching
- Persist user language preference across sessions
- Validate requested locales against supported languages
- Fall back to English when locale is unavailable

**Locale Resolution Priority**:
1. User-selected language (stored in local storage or preferences)
2. System locale (detected from OS)
3. English (default fallback)

### 2. Translation Service

**Responsibility**: Provide translation lookup and formatting services

**Key Capabilities**:
- Load and cache translation resources per locale
- Resolve translation keys to localized strings
- Support variable interpolation in translations
- Handle pluralization rules per language
- Format dates, times, and numbers according to locale conventions

**Translation Key Structure**:
```
category-component-element-context

Examples:
nav-sidebar-label-essays
common-button-save
essays-card-status-corrected
profile-settings-toggle-notifications
```

### 3. Fluent Translation Format

All translation resources will use Project Fluent format for its advanced localization features:

**Sample Translation File** (en-US/navigation.ftl):
```
nav-sidebar-label-dashboard = Dashboard
nav-sidebar-label-trails = Knowledge Trails
nav-sidebar-label-questions = Questions
nav-sidebar-label-essays = Essays
nav-sidebar-label-profile = Profile

nav-tabbar-label-dashboard = Dashboard
nav-tabbar-label-trails = Trails
nav-tabbar-label-questions = Questions
nav-tabbar-label-essays = Essays
nav-tabbar-label-profile = Profile
```

### 4. Context Provider for Dioxus

**Responsibility**: Make translation service available throughout the component tree

**Integration Pattern**:
- Global i18n context accessible via use_context hook
- Reactive language switching that triggers UI re-renders
- Provide translation helper functions to components

**Usage in Components**:
Components will access translations through a dedicated hook that returns translation functions:

```
Component receives: translation function t()
Component calls: t("nav-sidebar-label-essays")
Component receives: "Essays" (or localized equivalent)
```

## Translation Coverage Areas

### Navigation Elements
- Sidebar navigation items and descriptions
- Tab bar labels
- Menu items
- Breadcrumbs

### Status Bar Components
- Online status indicator
- Timer labels
- Version information
- System notifications

### Page Titles and Headers
- All page titles (Dashboard, Trilhas, Questões, Redações, Perfil)
- Section headers
- Panel titles

### Form Elements and Inputs
- Button labels (Save, Cancel, Submit, etc.)
- Input placeholders
- Form validation messages
- Checkbox and toggle labels

### Content Cards
- Question card metadata (difficulty, subject)
- Essay card metadata (status, exam type)
- Trail card progress indicators
- Empty state messages

### User Feedback Messages
- Success notifications
- Error messages
- Confirmation dialogs
- Loading states

### Settings and Configuration
- Configuration labels
- Preference descriptions
- Help text

### Metadata and Descriptive Text
- Stat labels (sequences, study time)
- Difficulty levels (Fácil, Médio, Difícil)
- Essay status (Em Progresso, Corrigida, Enviada)
- Subject names (Matemática, História, etc.)

## Language Support Matrix

### Initial Implementation Phase (High Priority)
Languages with significant Brazilian student populations:
- en-US (English - United States) - Default
- pt-BR (Portuguese - Brazil) - Primary audience
- es-ES (Spanish - Spain)
- es-MX (Spanish - Mexico)
- fr-FR (French - France)
- de-DE (German - Germany)
- it-IT (Italian - Italy)
- ja-JP (Japanese - Japan)
- zh-CN (Chinese - Simplified)
- ko-KR (Korean - South Korea)

### Full Implementation Phase
All 246 Google Translate supported languages organized by:
- Latin script languages
- Cyrillic script languages
- Arabic script languages (RTL support required)
- CJK languages (Chinese, Japanese, Korean)
- Indic scripts
- Other regional scripts

### RTL Language Support
Special handling required for:
- Arabic (ar)
- Hebrew (he)
- Persian/Farsi (fa)
- Urdu (ur)

UI layout adaptation:
- Mirror horizontal layouts
- Reverse text alignment
- Flip navigation directions
- Adjust icon positioning

## Data Formatting Strategy

### Date and Time Formatting
- Use locale-specific date formats (DD/MM/YYYY vs MM/DD/YYYY)
- Time format adaptation (12h vs 24h)
- Timezone display conventions
- Relative time expressions (e.g., "2 days ago")

### Number Formatting
- Decimal separator conventions (. vs ,)
- Thousand separator conventions
- Currency display (when applicable)
- Percentage formatting

### Text Direction and Layout
- LTR (Left-to-Right) for most languages
- RTL (Right-to-Left) for Arabic-script languages
- Mixed direction handling for multilingual content

## Translation Workflow

### Translation File Management
1. Developer extracts translatable strings to en-US base files
2. Translation keys added to Fluent resource files
3. Translation service distributes files to localization teams
4. Translated files returned and validated
5. Translations integrated into locale directories
6. Quality assurance testing per language

### Translation Key Naming Convention
Format: `{domain}-{component}-{element}-{variant?}`

Examples:
- `common-button-save` - Save button across application
- `essays-status-inprogress` - Essay status "In Progress"
- `questions-difficulty-medium` - Medium difficulty label
- `profile-stats-label-studytime` - Study time stat label

### Missing Translation Handling
1. Attempt to load requested locale translation
2. If missing, fall back to key's English translation
3. If English missing, display translation key itself
4. Log missing translation for future addition

## User Language Selection

### Settings Integration
Location: Profile page settings panel

**UI Elements**:
- Language selector dropdown
- Current language display
- System default option
- Language search/filter for 246 languages

**Behavior**:
- Display language names in their native script
- Show English translation alongside native name
- Highlight currently selected language
- Apply changes immediately upon selection
- Persist selection to local storage

### System Default Detection
- Read system locale from OS environment
- Map OS locale codes to supported language codes
- Handle locale variants (e.g., en-GB vs en-US)
- Default to en-US if system locale unsupported

## Technical Considerations

### Performance Optimization
- Lazy load translation resources per language
- Cache loaded translation bundles in memory
- Preload only active language and fallback
- Minimize bundle size through code splitting

### Bundle Size Management
- Each language bundle approximately 10-50 KB
- Only load active language resources
- Implement on-demand loading for language switching
- Use compression for translation file storage

### Accessibility Compliance
- Maintain ARIA labels in active language
- Ensure screen reader compatibility across languages
- Preserve semantic HTML structure
- Support keyboard navigation in all languages

### Cultural Adaptation
- Review cultural appropriateness of content
- Adapt examples and references to local context where needed
- Consider date/time preferences by region
- Respect naming conventions (family name first vs last)

## Domain Model Localization

### Enum Display Names
Enums requiring translation:
- Subject (16 subjects: Mathematics, History, etc.)
- Difficulty (3 levels: Easy, Medium, Hard)
- EssayStatus (3 states: In Progress, Corrected, Submitted)
- ExamType (60+ Brazilian university exams)

**Strategy**: Replace hardcoded display_name methods with translation key lookups

### Translation Keys for Enums
Examples:
- `domain-subject-mathematics` → "Matemática" (pt-BR) / "Mathematics" (en-US)
- `domain-difficulty-medium` → "Médio" (pt-BR) / "Medium" (en-US)
- `domain-essaystatus-corrected` → "Corrigida" (pt-BR) / "Corrected" (en-US)

## Testing Strategy

### Translation Coverage Testing
- Verify all UI elements have translation keys
- Ensure no hardcoded strings remain in components
- Check translation file completeness per language

### Locale Switching Testing
- Validate smooth switching between languages
- Confirm UI re-renders correctly
- Verify no layout breaks with different text lengths

### RTL Layout Testing
- Test Arabic and Hebrew language layouts
- Verify proper text alignment and direction
- Ensure icons and graphics flip appropriately

### Fallback Mechanism Testing
- Test missing translation scenarios
- Verify English fallback works correctly
- Validate graceful degradation

## Migration Strategy

### Phase 1: Infrastructure Setup
- Add i18n dependencies to workspace
- Create shared i18n module
- Implement locale detection service
- Create translation loader infrastructure
- Set up Dioxus context provider

### Phase 2: English Baseline Extraction
- Extract all hardcoded strings from components
- Create en-US translation files
- Organize keys by domain and component
- Replace hardcoded strings with translation calls

### Phase 3: Portuguese Localization
- Create pt-BR translation files
- Translate all English keys to Portuguese
- Test Brazilian Portuguese implementation
- Gather feedback from native speakers

### Phase 4: High-Priority Languages
- Implement 10 most requested languages
- Focus on languages with significant user base
- Test and validate each language
- Deploy incrementally

### Phase 5: Complete Language Rollout
- Implement remaining 236 languages
- Leverage machine translation as baseline
- Schedule human review for quality
- Deploy in batches of 20-30 languages

## Success Criteria

### Functional Requirements
- All UI text elements successfully localized
- System locale detected correctly on all platforms
- User can manually select from all 246 languages
- Language preference persists across sessions
- RTL languages render correctly
- Dates and numbers format according to locale

### Quality Requirements
- No hardcoded user-facing strings in codebase
- Translation coverage at 100% for English baseline
- Translation coverage at 100% for Portuguese
- Translation coverage at minimum 95% for high-priority languages
- Zero layout breaks across all supported languages
- Sub-100ms translation lookup performance

### User Experience Requirements
- Language switching takes effect immediately
- No flickering or layout shifts during switch
- Consistent cyberpunk aesthetic in all languages
- Native-like text rendering for all scripts
- Intuitive language selection interface

## Dependencies and Constraints

### Technical Dependencies
- fluent-rs: Fluent localization system for Rust
- sys-locale: System locale detection
- unic-langid: Language identifier parsing
- intl-memoizer: Internationalization memoization

### Platform Constraints
- Web platform: Browser locale detection via navigator API
- Desktop platform: OS locale from environment variables
- Mobile platform: Device locale from system settings

### Content Constraints
- Initial focus on UI localization only
- Educational content remains in Portuguese
- User-generated content not translated
- Machine translation used as baseline for lesser-known languages

## Risk Mitigation

### Risk: Translation Quality Variability
**Mitigation**: 
- Use professional translation services for top 20 languages
- Implement community review system for corrections
- Maintain translation quality guidelines
- Regular audits of translation accuracy

### Risk: Layout Breaking with Long Translations
**Mitigation**:
- Design flexible UI components with text overflow handling
- Test with longest known translations (e.g., German, Finnish)
- Implement text truncation with tooltips where needed
- Use responsive design patterns

### Risk: Performance Degradation
**Mitigation**:
- Lazy load translation bundles
- Implement efficient caching strategy
- Monitor bundle size for each language
- Optimize translation lookup algorithms

### Risk: Maintenance Burden for 246 Languages
**Mitigation**:
- Automate translation file validation
- Use version control for translation files
- Implement translation management platform
- Prioritize languages by user demand

## Open Questions

1. Should we support multiple language variants (e.g., pt-BR vs pt-PT, en-US vs en-GB)?
   - Recommendation: Support variants for top 10 languages only

2. How to handle language-specific content that doesn't translate well?
   - Recommendation: Create alternative content for specific locales when necessary

3. Should educational content (questions, essays) be translatable in the future?
   - Recommendation: Design architecture to support this, implement later

4. What's the strategy for handling user-generated content in multiple languages?
   - Recommendation: Phase 2 feature - display original with optional translation toggle

5. How to manage translation updates when UI changes?
   - Recommendation: Implement translation key versioning and deprecation strategy
### Phase 1: Infrastructure Setup
- Add i18n dependencies to workspace
- Create shared i18n module
- Implement locale detection service
- Create translation loader infrastructure
- Set up Dioxus context provider

### Phase 2: English Baseline Extraction
- Extract all hardcoded strings from components
- Create en-US translation files
- Organize keys by domain and component
- Replace hardcoded strings with translation calls

### Phase 3: Portuguese Localization
- Create pt-BR translation files
- Translate all English keys to Portuguese
- Test Brazilian Portuguese implementation
- Gather feedback from native speakers

### Phase 4: High-Priority Languages
- Implement 10 most requested languages
- Focus on languages with significant user base
- Test and validate each language
- Deploy incrementally

### Phase 5: Complete Language Rollout
- Implement remaining 236 languages
- Leverage machine translation as baseline
- Schedule human review for quality
- Deploy in batches of 20-30 languages

## Success Criteria

### Functional Requirements
- All UI text elements successfully localized
- System locale detected correctly on all platforms
- User can manually select from all 246 languages
- Language preference persists across sessions
- RTL languages render correctly
- Dates and numbers format according to locale

### Quality Requirements
- No hardcoded user-facing strings in codebase
- Translation coverage at 100% for English baseline
- Translation coverage at 100% for Portuguese
- Translation coverage at minimum 95% for high-priority languages
- Zero layout breaks across all supported languages
- Sub-100ms translation lookup performance

### User Experience Requirements
- Language switching takes effect immediately
- No flickering or layout shifts during switch
- Consistent cyberpunk aesthetic in all languages
- Native-like text rendering for all scripts
- Intuitive language selection interface

## Dependencies and Constraints

### Technical Dependencies
- fluent-rs: Fluent localization system for Rust
- sys-locale: System locale detection
- unic-langid: Language identifier parsing
- intl-memoizer: Internationalization memoization

### Platform Constraints
- Web platform: Browser locale detection via navigator API
- Desktop platform: OS locale from environment variables
- Mobile platform: Device locale from system settings

### Content Constraints
- Initial focus on UI localization only
- Educational content remains in Portuguese
- User-generated content not translated
- Machine translation used as baseline for lesser-known languages

## Risk Mitigation

### Risk: Translation Quality Variability
**Mitigation**: 
- Use professional translation services for top 20 languages
- Implement community review system for corrections
- Maintain translation quality guidelines
- Regular audits of translation accuracy

### Risk: Layout Breaking with Long Translations
**Mitigation**:
- Design flexible UI components with text overflow handling
- Test with longest known translations (e.g., German, Finnish)
- Implement text truncation with tooltips where needed
- Use responsive design patterns

### Risk: Performance Degradation
**Mitigation**:
- Lazy load translation bundles
- Implement efficient caching strategy
- Monitor bundle size for each language
- Optimize translation lookup algorithms

### Risk: Maintenance Burden for 246 Languages
**Mitigation**:
- Automate translation file validation
- Use version control for translation files
- Implement translation management platform
- Prioritize languages by user demand

## Open Questions

1. Should we support multiple language variants (e.g., pt-BR vs pt-PT, en-US vs en-GB)?
   - Recommendation: Support variants for top 10 languages only

2. How to handle language-specific content that doesn't translate well?
   - Recommendation: Create alternative content for specific locales when necessary

3. Should educational content (questions, essays) be translatable in the future?
   - Recommendation: Design architecture to support this, implement later

4. What's the strategy for handling user-generated content in multiple languages?
   - Recommendation: Phase 2 feature - display original with optional translation toggle

5. How to manage translation updates when UI changes?
   - Recommendation: Implement translation key versioning and deprecation strategy

