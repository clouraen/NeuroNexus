# Guia de Testes - NeuroNexus

## ✅ Status da Compilação

- **Dioxus 0.7**: ✅ Atualizado e funcionando
- **Compilação**: ✅ Sucesso (apenas 3 warnings menores)
- **Build Release**: ✅ Funcionando

## 🚀 Como Executar

### Pré-requisitos

```bash
# 1. Instalar target WASM (necessário para web)
rustup target add wasm32-unknown-unknown

# 2. Instalar Dioxus CLI (se ainda não tiver)
cargo install dioxus-cli
```

### Desenvolvimento

#### Modo Web

```bash
# Método 1: Usando Dioxus CLI (Recomendado para web)
cd crates/app
dx serve

# Método 2: Se dx não estiver no PATH
cd crates/app
~/.cargo/bin/dx serve

# Nota: Para web, não use `cargo run` diretamente - use `dx serve`
```

#### Modo Desktop

```bash
# Executar aplicação desktop
cd crates/app
cargo run --features desktop

# Ou do workspace root
cargo run --bin app --features desktop
```

A aplicação desktop abrirá em uma janela nativa.

A aplicação será servida automaticamente. Aguarde a mensagem no terminal indicando a URL (geralmente `http://localhost:8080`).

**O que acontece:**
1. 🔨 Compilação inicial (pode levar 30-60 segundos na primeira vez)
2. 🌐 Servidor web inicia automaticamente
3. 🔄 Hot-reload ativo (mudanças no código atualizam automaticamente)
4. 📱 Aplicação disponível no navegador

### Build de Release

```bash
# Build otimizado (mais lento, mas otimizado)
cargo build --bin app --release

# Executar o binário de release
./target/release/app
```

**Quando usar release:**
- ✅ Teste de performance
- ✅ Deploy para produção
- ✅ Distribuição do binário

### Verificar Status

```bash
# Verificar se o servidor está rodando
lsof -ti:8080 && echo "✅ Servidor ativo" || echo "❌ Servidor não encontrado"

# Ver processos do cargo
ps aux | grep cargo | grep -v grep
```

### Parar o Servidor

Pressione `Ctrl+C` no terminal onde o cargo está executando.

## 📋 Checklist de Testes

### Navegação
- [ ] Home carrega corretamente
- [ ] Navegação entre tabs funciona
- [ ] Router redireciona corretamente
- [ ] Links funcionam

### Páginas
- [ ] **Home**: Estatísticas carregam (redações, questões, trilhas)
- [ ] **Questões**: Lista de questões aparece
- [ ] **Questões**: Busca funciona
- [ ] **Questão Detail**: Carrega questão correta
- [ ] **Questão Detail**: Alternativas funcionam
- [ ] **Questão Detail**: Explicação aparece ao responder
- [ ] **Redações**: Lista de redações aparece
- [ ] **Redação Detail**: Carrega redação correta
- [ ] **Nova Redação**: Editor funciona
- [ ] **Nova Redação**: Salvar funciona
- [ ] **Perfil**: Carrega informações

### Funcionalidades
- [ ] Seeders populam dados corretamente
- [ ] Busca de questões filtra resultados
- [ ] Estados reativos funcionam (Signals)
- [ ] Eventos de input funcionam
- [ ] Links de navegação funcionam

### UI/UX
- [ ] Tema cyberpunk aplicado
- [ ] Cores neon visíveis
- [ ] Efeitos glow funcionam
- [ ] Responsividade básica
- [ ] Loading states aparecem

## 🐛 Problemas Conhecidos

1. **NeonInput**: Eventos de input podem precisar ajustes finos
2. **Textarea**: Editor de redação pode precisar melhorias
3. **Performance**: Listas grandes podem precisar paginação

## 📝 Notas

- A aplicação usa repositórios em memória (dados são perdidos ao recarregar)
- Seeders são executados na inicialização
- Todos os dados são carregados de forma assíncrona

## 🔧 Debug

```bash
# Ver logs de compilação
cargo build --bin app --verbose

# Verificar dependências
cargo tree --bin app

# Limpar e recompilar
cargo clean && cargo build --bin app

# Instalar target WASM (necessário para web)
rustup target add wasm32-unknown-unknown

# Verificar targets instalados
rustup target list --installed
```

