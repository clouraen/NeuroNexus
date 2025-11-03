# NeuroNexus - Quick Start Guide

> 🌐 **[Português](docs/pt/QUICKSTART.md)** | **[中文](docs/zh/QUICKSTART.md)**

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ installed
- Cargo installed

### Run the Application

```bash
# Compile and run
cargo run --bin app

# Or just compile
cargo build --bin app

# Release build
cargo build --bin app --release
```

The application will be served at `http://localhost:8080` (or port configured by Dioxus).

## 📦 Project Structure

```
NeuroNexus/
├── CODEX.md              # Complete project specification
├── README.md             # Overview
├── SEEDERS.md            # Seeders documentation
├── STATUS.md             # Current development status
├── QUICKSTART.md         # This file
├── CARGO.toml            # Workspace configuration
└── crates/
    ├── domain/           # Business logic and models
    ├── data/             # Repositories and seeders
    ├── app/              # Dioxus interface
    ├── shared/           # Shared utilities
    └── services/         # External services (future)
```

## 🎨 Implemented Features

### Cyberpunk Neon Interface
- ✅ Dark theme with neon colors (purple, pink, blue, gold)
- ✅ Glow effects and shadows
- ✅ Styled components (buttons, cards, inputs)
- ✅ Tab navigation

### Functionalities
- ✅ Study plan dashboard
- ✅ Questions list (11 real questions)
- ✅ Essays list (4 essays)
- ✅ User profile
- ✅ Routing system

### Test Data
- ✅ 11 real questions from multiple subjects
- ✅ 4 essays with feedback
- ✅ 3 knowledge trails
- ✅ Test user configured

## 🔧 Useful Commands

```bash
# Check code
cargo check

# Format code
cargo fmt

# Linter
cargo clippy

# Tests (when implemented)
cargo test

# Clean build
cargo clean
```

## 📝 Upcoming Features

- Essay editor
- Detailed question view
- Functional search system
- Real-time repository integration
- AI tutor chat (Phase 3)
- Achievement system

## 🐛 Known Issues

- NeonInput doesn't fully capture input events yet (placeholder)
- Some lifetime warnings (non-critical)

## 📚 Documentation

- `CODEX.md` - Complete specification
- `SEEDERS.md` - Test data details
- `STATUS.md` - Current development status

