#!/bin/bash
set -e

# Set default email if not provided
export TF_VAR_forward_to_email="${TF_VAR_forward_to_email:-miller.jimd@gmail.com}"

echo "Building Rust Lambda for ARM64..."
cd rust-lambda
cargo lambda build --release --arm64

echo "Deploying infrastructure with Terraform..."
cd ../infra
tofu init -upgrade
tofu apply -auto-approve

echo "Deployment complete!"
