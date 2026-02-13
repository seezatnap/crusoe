# Rust Workspace Layout

The `rust/` directory is a workspace for narrative-analysis tooling.

- `crates/analysis-contracts` contains shared domain contracts for all analysis binaries.
  - Severity levels
  - Source location model
  - Finding/report payloads
  - Analyzer input contract
  - Analysis runner trait and error type
- `crates/excerpt-extract` currently houses the style-reference extraction utility.
- `crates/dr-joe-support` holds support helpers.
- `crates/cliche-detection` contains the literary cliché and sentence-pattern CLI.

All new binaries should consume `analysis-contracts` for:

- Deterministic severity handling
- Common report shape
- Shared working-directory and target-resolution behavior
