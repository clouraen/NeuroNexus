# Testing Guide - NeuroNexus

> 🌐 **[Português](docs/pt/TESTING.md)** | **[中文](docs/zh/TESTING.md)**

## ✅ Build Status

- **Dioxus 0.7**: ✅ Updated and working
- **Compilation**: ✅ Success (only 3 minor warnings)
- **Release Build**: ✅ Working

## 🚀 How to Run

### Prerequisites

```bash
# 1. Install WASM target (required for web)
rustup target add wasm32-unknown-unknown

# 2. Install Dioxus CLI (if you don't have it yet)
cargo install dioxus-cli
```

### Development

#### Web Mode

```bash
# Method 1: Using Dioxus CLI (Recommended for web)
cd crates/app
dx serve

# Method 2: If dx is not in PATH
cd crates/app
~/.cargo/bin/dx serve

# Note: For web, don't use `cargo run` directly - use `dx serve`
```

#### Desktop Mode

```bash
# Run desktop application
cd crates/app
cargo run --features desktop

# Or from workspace root
cargo run --bin app --features desktop
```

The desktop app will open in a native window.

The application will be served automatically. Wait for the terminal message indicating the URL (usually `http://localhost:8080`).

**What happens:**
1. 🔨 Initial compilation (may take 30-60 seconds the first time)
2. 🌐 Web server starts automatically
3. 🔄 Hot-reload active (code changes update automatically)
4. 📱 Application available in browser

### Release Build

```bash
# Optimized build (slower, but optimized)
cargo build --bin app --release

# Run the release binary
./target/release/app
```

**When to use release:**
- ✅ Performance testing
- ✅ Production deployment
- ✅ Binary distribution

### Check Status

```bash
# Check if server is running
lsof -ti:8080 && echo "✅ Server active" || echo "❌ Server not found"

# View cargo processes
ps aux | grep cargo | grep -v grep
```

### Stop the Server

Press `Ctrl+C` in the terminal where cargo is running.

## 📋 Testing Checklist

### Navigation
- [ ] Home loads correctly
- [ ] Tab navigation works
- [ ] Router redirects correctly
- [ ] Links work

### Pages
- [ ] **Home**: Statistics load (essays, questions, trails)
- [ ] **Questions**: Question list appears
- [ ] **Questions**: Search works
- [ ] **Question Detail**: Loads correct question
- [ ] **Question Detail**: Alternatives work
- [ ] **Question Detail**: Explanation appears on answer
- [ ] **Essays**: Essay list appears
- [ ] **Essay Detail**: Loads correct essay
- [ ] **New Essay**: Editor works
- [ ] **New Essay**: Save works
- [ ] **Profile**: Loads information

### Functionalities
- [ ] Seeders populate data correctly
- [ ] Question search filters results
- [ ] Reactive states work (Signals)
- [ ] Input events work
- [ ] Navigation links work

### UI/UX
- [ ] Cyberpunk theme applied
- [ ] Neon colors visible
- [ ] Glow effects work
- [ ] Basic responsiveness
- [ ] Loading states appear

## 🐛 Known Issues

1. **NeonInput**: Input events may need fine-tuning
2. **Textarea**: Essay editor may need improvements
3. **Performance**: Large lists may need pagination

## 📝 Notes

- App uses in-memory repositories (data is lost on reload)
- Seeders run on initialization
- All data is loaded asynchronously

## 🔧 Debug

```bash
# View compilation logs
cargo build --bin app --verbose

# Check dependencies
cargo tree --bin app

# Clean and recompile
cargo clean && cargo build --bin app

# Install WASM target (required for web)
rustup target add wasm32-unknown-unknown

# Check installed targets
rustup target list --installed
```

