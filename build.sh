#!/bin/bash
set -e

echo "Building Rust Lambda function..."

# For now, we'll use a simple approach that works with current Rust version
# In production, you'd want to use cargo-lambda or Docker for proper cross-compilation

cd lambda

# Build for the current platform (we'll fix cross-compilation later)
cargo build --release

# Create the expected directory structure for OpenTofu
mkdir -p target/lambda/bootstrap
cp target/release/bootstrap target/lambda/bootstrap/

echo "Build complete! Binary available at: lambda/target/lambda/bootstrap/bootstrap"
echo "Note: This is built for the current platform. For production, use cargo-lambda or Docker."
