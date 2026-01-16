---
inclusion: always
---

# Git & Rust/AWS Standards Checklist

This checklist must be completed by the AI agent before every git commit to ensure architectural integrity, security compliance, and high-performance Rust execution.

---

## 1. 🦀 Rust Excellence & Performance

- **[ ] Linting & Format:** Ran `cargo fmt --all` and `cargo clippy -- -D warnings`.
- **[ ] Error Handling:** No `.unwrap()` or `.expect()` used in production paths. Result-based error propagation is implemented.
- **[ ] Cold Start Optimization:** \* AWS SDK clients (S3, DynamoDB, etc.) are initialized **outside** the `function_handler`.
  - Used `once_cell` or `tokio::sync::OnceCell` for static resource initialization where appropriate.
- **[ ] Binary Size:** `Cargo.toml` release profile includes `lto = true`, `codegen-units = 1`, and `panic = "abort"`.
- **[ ] Dependencies:** Ran `cargo audit` to check for known vulnerabilities.
- **[ ] Lockfiles:** Verified that Cargo.lock and .terraform.lock.hcl are tracked and NOT ignored.
- **[ ] Artifacts**: Verified that the compiled bootstrap binary and target/ directories are ignored.
- **[ ] Environment:** Verified that no .env files containing real secrets are staged.

## 2. ☁️ AWS Lambda & Cloud Native

- **[ ] Logging:** Using `tracing` and `tracing-subscriber` for structured JSON logging compatible with CloudWatch.
- **[ ] Architecture:** Targeted `arm64` (Graviton2) for better price-performance unless specific x86 legacy requirements exist.
- **[ ] Runtime:** Using `cargo lambda` for building and cross-compilation to ensure Amazon Linux compatibility.

## 3. 🏗️ Infrastructure-as-Code (OpenTofu)

- **[ ] Modularization:** Infrastructure logic is abstracted into modules; the root `main.tf` is clean and declarative.
- **[ ] State Security:** Verified that no `.terraform/`, `.tofu/`, or `*.tfstate` files are staged for commit.
- **[ ] Validation:** Used `validation` blocks in `variables.tf` for inputs like AWS regions or resource naming.
- **[ ] Binary Paths:** Verified that `aws_lambda_function` points to the correct build artifact path (e.g., `target/lambda/release/`).

## 4. 🔒 Security & IAM

- **[ ] Least Privilege:** IAM policies use specific actions (e.g., `s3:GetObject`) and resource ARNs instead of wildcards (`*`).
- **[ ] Secret Management:** No hardcoded API keys or environment variables containing sensitive data. Verified usage of AWS Secrets Manager or SSM Parameter Store.
- **[ ] Network:** If the Lambda is in a VPC, verified that security groups follow the egress-only principle where possible.

## 5. 📂 Repository Hygiene

- **[ ] Directory Structure:**
  - Rust code is in `src/functions/` or `src/shared/`.
  - IaC is in `infrastructure/`.
  - Helper scripts are in `scripts/`.
- **[ ] Orchestration:** Updated or verified the `justfile` to include commands for building (via `cargo lambda`) and deploying (via `tofu`).
- **[ ] Scripts:** All scripts in `scripts/` have a proper shebang (`#!/bin/bash`), are executable, and use environment variables for AWS profiles/regions.
- **[ ] Commit Message:** Follows Conventional Commits (e.g., `feat(iam): restrict s3 access for processor lambda`).

---

## 🛠️ Recommended Build Command

Before committing, ensure the build succeeds via the project's task runner:

```bash
# Recommended build sequence
just build       # Runs cargo lambda build --release
just test        # Runs cargo test
just infra-plan  # Runs tofu plan
```
