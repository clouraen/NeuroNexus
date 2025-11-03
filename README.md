# NeuroNexus

> 🌐 **[Português](docs/pt/README.md)** | **[中文](docs/zh/README.md)**

A cross-platform educational platform (web, desktop, mobile) built with Rust and Dioxus, focused on college entrance exam and ENEM preparation.

## 🎨 Design

Cyberpunk neon interface inspired by Cyberpunk 2077 and Blade Runner 2049, featuring dark theme and neon effects.

## 🏗️ Architecture

The project uses Clean Architecture with the following structure:

```
crates/
├── domain/     # Business logic, models, use cases, traits
├── data/       # Repository implementations, database, seeders
├── app/        # Dioxus components, pages, routing, UI
├── shared/     # Shared utilities, common types
└── services/   # External services (AI, APIs, etc.)
```

## 🚀 Development

### Prerequisites

- Rust 1.75 or higher
- Cargo

### Run (Web)

```bash
cargo run --bin app
```

### Build

```bash
cargo build --release
```

## 📋 Implementation Phases

### ✅ PHASE 1: Core MVP (In Progress)
- [x] Rust workspace setup
- [x] Crate structure
- [x] Basic domain models
- [x] Cyberpunk UI components
- [x] Basic routing
- [x] Main pages
- [x] In-memory repositories
- [ ] Test data seeders

### PHASE 2: Essential Educational Features
- [ ] Essay editor
- [ ] Essay evaluation
- [ ] Question viewing
- [ ] Trail system

### PHASE 3: AI and Personalization
- [ ] AI tutor chat
- [ ] AI-powered essay evaluation
- [ ] Personalized trails
- [ ] Achievement system

## 📚 Documentation

See `CODEX.md` for complete project documentation.

## 🎯 Technologies

- **Rust**: Primary language
- **Dioxus**: Cross-platform UI framework
- **Tokio**: Async runtime
- **Chrono**: Date handling
- **UUID**: Unique identifiers

## 📝 License

MIT
