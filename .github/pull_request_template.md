## 📝 Description
Fixes # (issue)

## 🦀 Rust & Lambda Quality Checklist
- [ ] **Linting:** I have run `cargo clippy -- -D warnings` and it passes.
- [ ] **Formatting:** I have run `cargo fmt --all -- --check`.
- [ ] **Tests:** I have added/updated unit and integration tests (Local or via `cargo-lambda watch`).
- [ ] **Binary Size:** I have verified that the binary size is optimized (LTO enabled, symbols stripped).
- [ ] **Cold Start:** I have considered the impact of this change on Lambda initialization/cold start times.

## ☁️ AWS & Infrastructure Impact
- [ ] **IAM:** Does this PR require new permissions? (If yes, please list them below).
- [ ] **Environment Variables:** Are new variables required in the Lambda environment?
- [ ] **Architecture:** Does this change require updates to the CDK/SAM/Terraform code?
- [ ] **Observability:** Have relevant logs, metrics, or tracing (AWS X-Ray) been added?

## 📖 Documentation
- [ ] I have updated the `README.md` or internal documentation.
- [ ] I have updated API documentation (e.g., OpenAPI/Swagger) if applicable.
- [ ] I have commented on complex or non-obvious logic within the Rust source.

## 🚀 Deployment & Rollback
- [ ] **Migration:** Does this change require a database migration or stateful change?
- [ ] **Rollback Strategy:** If the canary fails, is there a clear path to revert?
- [ ] **Feature Flags:** Is this change protected by a feature toggle?

## 📸 Screenshots / Logs (if applicable)