#!/bin/bash
# Essay AI Correction - Quick Test Script

echo "🧪 Testing Essay AI Correction Implementation"
echo "=============================================="
echo ""

echo "📦 Building services crate..."
cargo build --package services

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"
echo ""

echo "🧪 Running unit tests..."
cargo test --package services

if [ $? -ne 0 ]; then
    echo "❌ Tests failed"
    exit 1
fi

echo "✅ Tests passed"
echo ""

echo "📝 Testing rubric definitions..."
cargo test --package services rubrics::tests -- --nocapture

echo ""
echo "🤖 Testing AI service..."
cargo test --package services ai::tests -- --nocapture

echo ""
echo "📊 Testing evaluation service..."
cargo test --package services evaluation::tests -- --nocapture

echo ""
echo "=============================================="
echo "✨ All tests completed!"
echo ""
echo "Next steps:"
echo "1. Run: cargo run --bin app"
echo "2. Navigate to an essay detail page"
echo "3. Click 'Avaliar Redação com IA'"
echo "4. First run will download BERTimbau model (~420MB)"
echo "5. Subsequent runs will use cached model (offline)"
