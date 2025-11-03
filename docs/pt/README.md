# NeuroNexus

> 🌐 **[English](../../README.md)** | **[中文](../zh/README.md)**

Plataforma educacional multiplataforma (web, desktop, mobile) construída com Rust e Dioxus, focada em preparação para vestibulares e ENEM.

## 🎨 Design

Interface cyberpunk neon inspirada em Cyberpunk 2077 e Blade Runner 2049, com tema dark e efeitos neon.

## 🏗️ Arquitetura

O projeto utiliza Clean Architecture com a seguinte estrutura:

```
crates/
├── domain/     # Lógica de negócio, modelos, casos de uso, traits
├── data/       # Implementações de repositórios, banco de dados, seeders
├── app/        # Componentes Dioxus, páginas, roteamento, UI
├── shared/     # Utilitários compartilhados, tipos comuns
└── services/   # Serviços externos (IA, API, etc.)
```

## 🚀 Desenvolvimento

### Pré-requisitos

- Rust 1.75 ou superior
- Cargo

### Executar (Web)

```bash
cargo run --bin app
```

### Build

```bash
cargo build --release
```

## 📋 Fases de Implementação

### ✅ FASE 1: Core MVP (Em andamento)
- [x] Setup do workspace Rust
- [x] Estrutura de crates
- [x] Modelos de domínio básicos
- [x] Componentes UI cyberpunk
- [x] Roteamento básico
- [x] Páginas principais
- [x] Repositórios em memória
- [ ] Seeders de dados de teste

### FASE 2: Funcionalidades Educacionais Essenciais
- [ ] Editor de redação
- [ ] Avaliação de redação
- [ ] Visualização de questão
- [ ] Sistema de trilhas

### FASE 3: IA e Personalização
- [ ] Chat tutor com IA
- [ ] Avaliação de redação com IA
- [ ] Trilhas personalizadas
- [ ] Sistema de conquistas

## 📚 Documentação

Consulte `CODEX.md` para documentação completa do projeto.

## 🎯 Tecnologias

- **Rust**: Linguagem principal
- **Dioxus**: Framework UI multiplataforma
- **Tokio**: Runtime assíncrono
- **Chrono**: Manipulação de datas
- **UUID**: Identificadores únicos

## 📝 Licença

MIT
