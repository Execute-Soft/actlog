#!/bin/bash

echo "🧪 Testing ActLog CLI..."

# Test help command
echo "Testing help command..."
cargo run -- --help

echo ""
echo "Testing authenticate help..."
cargo run -- authenticate --help

echo ""
echo "Testing config help..."
cargo run -- config --help

echo ""
echo "Testing report-costs help..."
cargo run -- report-costs --help

echo ""
echo "Testing scale-instances help..."
cargo run -- scale-instances --help

echo ""
echo "Testing cleanup help..."
cargo run -- cleanup --help

echo ""
echo "Testing list help..."
cargo run -- list --help

echo ""
echo "✅ CLI test completed!" 