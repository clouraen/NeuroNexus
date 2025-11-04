# Translation Keys Quick Reference

## 🔍 How to Find Translation Keys

All translation keys follow the pattern: `{domain}-{component}-{element}`

### Common UI (common.ftl)
```
Buttons:        common-button-{action}
Status:         common-status-{state}
Messages:       common-message-{type}
Forms:          common-form-{element}
Time:           common-time-{period}
Pagination:     common-pagination-{element}
Validation:     common-validation-{rule}
Import Modal:   import-modal-{element}
Status Bar:     status-bar-{element}
Toggle:         toggle-{action}-title
```

### Navigation (navigation.ftl)
```
Sidebar:        nav-sidebar-label-{page}
                nav-sidebar-desc-{page}
Tab Bar:        nav-tabbar-label-{page}
Breadcrumbs:    nav-breadcrumb-{element}
Menu:           nav-menu-{element}
```

### Pages
```
Home:           home-{section}-{element}
Essays:         essays-{section}-{element}
                essay-{section}-{element}
Questions:      questions-{section}-{element}
                question-{section}-{element}
Trails:         trails-{section}-{element}
                trail-{section}-{element}
Profile:        profile-{section}-{element}
```

### Domain (domain.ftl)
```
Subjects:       domain-subject-{name}
Difficulty:     domain-difficulty-{level}
Essay Status:   domain-essay-status-{state}
Exam Types:     domain-exam-{institution}
```

## 📋 Complete Key List by File

### common.ftl (66 keys)
**Buttons (17)**: save, cancel, submit, delete, edit, add, remove, close, confirm, back, next, finish, start, upload, download, import, export

**Status (9)**: loading, saving, success, error, warning, info, online, offline, processing

**Messages (6)**: no-data, empty-state, confirm-delete, unsaved-changes, operation-success, operation-failed

**Forms (6)**: required, optional, placeholder-search, placeholder-filter, select-option, no-results

**Time (7)**: now, today, yesterday, days-ago, hours-ago, minutes-ago, just-now

**Pagination (5)**: previous, next, page, of, showing

**Validation (5)**: email, required, min-length, max-length, numeric

**Import Modal (13)**: title, close, description, questions-title, questions-desc, trails-title, trails-desc, processing, success, partial-success, error-no-type, error-read-file, error-invalid-json

**Status Bar (4)**: logo, timer-title, online, version

**Toggle (2)**: enable-title, disable-title

### navigation.ftl (25 keys)
**Sidebar Labels (5)**: dashboard, trails, questions, essays, profile

**Sidebar Descriptions (5)**: dashboard, trails, questions, essays, profile

**Tab Bar Labels (5)**: dashboard, trails, questions, essays, profile

**Breadcrumbs (2)**: home, back

**Navigation Menu (3)**: main-section, tools-section, coming-soon

### home.ftl (19 keys)
**Headers (3)**: title, subtitle, greeting

**Quick Stats (4)**: questions, essays, study-time, streak

**Recent Activity (5)**: title, empty, question-answered, essay-created, trail-started

**Quick Actions (5)**: title, new-essay, practice, continue-trail, view-progress

**Recommendations (2)**: title, empty

### essays.ftl (38 keys)
**Headers (2)**: title, subtitle

**Filters (3)**: all, status, exam

**Card Labels (8)**: exam, status, theme, created, updated, view, edit, delete

**Empty State (3)**: title, message, action

**Actions (2)**: new, import

**Detail Page (11)**: title, theme, exam, status, created, content, feedback, score, save, submit, back

**Status (4)**: draft, inprogress, submitted, corrected

**New Essay (5)**: title, select-exam, enter-theme, placeholder-theme, start-writing

### questions.ftl (37 keys)
**Headers (2)**: title, subtitle

**Filters (5)**: all, subject, difficulty, year, institution

**Card Labels (8)**: subject, difficulty, year, institution, answered, correct, view, practice

**Stats (4)**: total, answered, correct, accuracy

**Empty State (3)**: title, message, action

**Detail Page (14)**: title, statement, options, option-a, option-b, option-c, option-d, option-e, answer, correct-answer, explanation, submit, next, back

**Difficulty Levels (3)**: easy, medium, hard

### trails.ftl (20 keys)
**Headers (2)**: title, subtitle

**Card Labels (7)**: title, progress, questions, completed, start, continue, view

**Progress (2)**: total, percentage

**Empty State (3)**: title, message, action

**Detail Page (6)**: title, description, questions-list, start, reset, back

### profile.ftl (35 keys)
**Headers (2)**: title, subtitle

**Sections (4)**: personal, stats, preferences, language

**Personal Info (4)**: name, email, joined, edit

**Statistics (7)**: total-questions, answered, correct, essays, study-time, streak, sequences

**Preferences (4)**: notifications, dark-mode, auto-save, show-explanations

**Language (4)**: current, select, system-default, apply

**Actions (4)**: save, cancel, logout, delete-account

**Import/Export (6)**: import-title, import-questions, import-trails, import-select-file, export-title, export-all

### domain.ftl (40 keys)
**Subjects (16)**: mathematics, portuguese, history, geography, physics, chemistry, biology, philosophy, sociology, english, spanish, arts, physical-education, literature, general-knowledge, interdisciplinary

**Difficulty (3)**: easy, medium, hard

**Essay Status (4)**: draft, inprogress, submitted, corrected

**Exam Types (17)**: enem, fuvest, unicamp, unesp, uerj, ufrj, ufmg, ufrgs, ufpr, ufsc, unb, ufba, ufpe, ufc, ufpa, ufam, other

## 🌍 Available Languages

- **en-US**: English (United States) - 285 keys
- **pt-BR**: Portuguese (Brazil) - 285 keys  
- **zh-CN**: Chinese (Simplified) - 285 keys

## 💡 Usage Examples

```rust
// Simple translation
ctx.t("common-button-save")           // "Save" / "Salvar" / "保存"

// With variables
ctx.t("home-header-greeting")         // "Hello, {name}!"
ctx.t("trails-progress-total")        // "{completed} of {total} questions"

// Status messages
ctx.t("common-status-loading")        // "Loading..." / "Carregando..." / "加载中..."

// Navigation
ctx.t("nav-sidebar-label-essays")     // "Essays" / "Redações" / "作文"

// Domain models
ctx.t("domain-subject-mathematics")   // "Mathematics" / "Matemática" / "数学"
ctx.t("domain-difficulty-medium")     // "Medium" / "Médio" / "中等"
```

## 📖 Key Naming Conventions

1. **Domain**: Where is this used? (nav, home, essays, common, domain, etc.)
2. **Component**: What component/section? (button, header, card, stats, etc.)
3. **Element**: What specific element? (title, save, next, etc.)
4. **Variant** (optional): Any variation? (enable-title, disable-title)

### Examples
- `common-button-save` - Reusable save button
- `essays-header-title` - Title in essays page header
- `profile-stats-study-time` - Study time stat in profile
- `nav-sidebar-label-dashboard` - Dashboard label in sidebar
- `domain-exam-enem` - ENEM exam type

## 🔄 Adding New Translations

1. Choose appropriate file based on domain
2. Follow naming convention
3. Add to all three languages (en-US, pt-BR, zh-CN)
4. Use in component with `ctx.t("your-new-key")`

---

**Total Keys**: 285 per language
**Total Translations**: 855 (285 × 3 languages)
**Status**: ✅ Complete
