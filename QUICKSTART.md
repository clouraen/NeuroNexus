# NeuroNexus - Guia Rápido

## 🚀 Início Rápido

### Pré-requisitos
- Rust 1.75+ instalado
- Cargo instalado

### Executar a Aplicação

```bash
# Compilar e executar
cargo run --bin app

# Ou apenas compilar
cargo build --bin app

# Build de release
cargo build --bin app --release
```

A aplicação será servida em `http://localhost:8080` (ou porta configurada pelo Dioxus).

## 📦 Estrutura do Projeto

```
NeuroNexus/
├── CODEX.md              # Especificação completa do projeto
├── README.md              # Visão geral
├── SEEDERS.md            # Documentação dos seeders
├── STATUS.md             # Status atual do desenvolvimento
├── QUICKSTART.md         # Este arquivo
├── Cargo.toml            # Configuração do workspace
└── crates/
    ├── domain/           # Lógica de negócio e modelos
    ├── data/             # Repositórios e seeders
    ├── app/              # Interface Dioxus
    ├── shared/           # Utilitários compartilhados
    └── services/         # Serviços externos (futuro)
```

## 🎨 Recursos Implementados

### Interface Cyberpunk Neon
- ✅ Tema dark com cores neon (roxo, rosa, azul, dourado)
- ✅ Efeitos glow e sombras
- ✅ Componentes estilizados (botões, cards, inputs)
- ✅ Navegação por tabs

### Funcionalidades
- ✅ Dashboard de plano de estudo
- ✅ Lista de questões (11 questões reais)
- ✅ Lista de redações (4 redações)
- ✅ Perfil do usuário
- ✅ Sistema de rotas

### Dados de Teste
- ✅ 11 questões reais de múltiplas matérias
- ✅ 4 redações com feedbacks
- ✅ 3 trilhas de conhecimento
- ✅ Usuário de teste configurado

## 🔧 Comandos Úteis

```bash
# Verificar código
cargo check

# Formatar código
cargo fmt

# Linter
cargo clippy

# Testes (quando implementados)
cargo test

# Limpar build
cargo clean
```

## 📝 Próximas Funcionalidades

- Editor de redação
- Visualização detalhada de questões
- Sistema de busca funcional
- Integração com repositórios em tempo real
- Chat tutor com IA (Fase 3)
- Sistema de conquistas

## 🐛 Problemas Conhecidos

- NeonInput ainda não captura eventos de input completamente (placeholder)
- Alguns warnings de lifetime (não críticos)

## 📚 Documentação

- `CODEX.md` - Especificação completa
- `SEEDERS.md` - Detalhes dos dados de teste
- `STATUS.md` - Status atual do desenvolvimento

