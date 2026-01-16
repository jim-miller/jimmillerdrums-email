# Legacy Lambda Backup

This directory contains the old JavaScript/Node.js Lambda implementation that was replaced by the Rust implementation in `rust-lambda/`.

## Contents
- `index.js` - Node.js Lambda handler (deprecated)
- `package.json` - Node.js dependencies (deprecated)
- Old Rust attempt files (Cargo.toml, src/) - superseded by rust-lambda/

## Status
**DEPRECATED** - Kept for rollback purposes only

## Migration Date
January 14-16, 2026

## Rollback Instructions
If you need to rollback to the Node.js version:

1. Revert `infra/lambda.tf` to use:
   ```hcl
   runtime = "nodejs20.x"
   handler = "index.handler"
   architectures = ["x86_64"]
   source_file = "${path.module}/../lambda-legacy-backup/index.js"
   ```

2. Run `tofu apply` in infra/

## Safe to Delete After
February 16, 2026 (30 days after migration)

## New Implementation
See `rust-lambda/` directory for the current Rust implementation.
