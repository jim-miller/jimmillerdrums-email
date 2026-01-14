#!/bin/bash
set -e

echo "Building Rust Lambda function for AWS Lambda (Linux)..."

cd lambda

# Try cross-compilation to Linux using musl target
echo "Cross-compiling to Linux using musl target..."

# Build for Linux
cargo build --release --target x86_64-unknown-linux-musl

# Create the expected directory structure for OpenTofu
mkdir -p target/lambda/bootstrap
cp target/x86_64-unknown-linux-musl/release/bootstrap target/lambda/bootstrap/

echo "Build complete! Linux binary available at: lambda/target/lambda/bootstrap/bootstrap"
echo "Binary built for x86_64-unknown-linux-musl (compatible with AWS Lambda)"
