#!/bin/bash
# Publish script for dintero-rust-sdk
# This script publishes all crates in the correct order

set -e

echo "🚀 Publishing Dintero Rust SDK v0.1.0"
echo ""

# Step 1: Build and test everything
echo "📦 Building and testing all crates..."
cargo build --all-features
cargo test --all-features
cargo clippy --all-features -- -D warnings
echo "✅ All tests and checks passed"
echo ""

# Step 2: Publish feature crates (these have no dependencies on each other)
echo "📤 Publishing feature crates..."
echo ""

crates=("dintero-checkout" "dintero-orders" "dintero-payments" "dintero-accounts" "dintero-loyalty" "dintero-insights")

for crate in "${crates[@]}"; do
    echo "Publishing $crate..."
    cargo publish -p "$crate"
    echo "✅ $crate published"
    echo "⏳ Waiting 30 seconds for crates.io to index..."
    sleep 30
    echo ""
done

# Step 3: Publish main crate
echo "📤 Publishing main dintero crate..."
cargo publish -p dintero
echo "✅ dintero published"
echo ""

echo "🎉 All crates published successfully!"
echo ""
echo "Users can now install with:"
echo "  cargo add dintero"
echo ""
echo "Or add to Cargo.toml:"
echo '  [dependencies]'
echo '  dintero = "0.1.0"'
