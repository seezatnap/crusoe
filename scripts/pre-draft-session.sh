#!/usr/bin/env bash
set -euo pipefail

print_usage() {
  cat <<'USAGE'
Usage:
  scripts/pre-draft-session.sh [chapter-or-directory] [quality-orchestrator flags...]

Runs contributor reminders before drafting:
- samples 5 narrative examples
- samples 5 dialogue examples
- runs the full quality pipeline on the target (defaults to `chapters`)

Examples:
  scripts/pre-draft-session.sh
  scripts/pre-draft-session.sh chapters/chapter-01.md
  scripts/pre-draft-session.sh chapters --fail-on blocker
USAGE
}

if [[ "${#}" -gt 0 && "$1" == "--help" ]]; then
  print_usage
  exit 0
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${PROJECT_ROOT}"

TARGET="chapters"
if [[ "${#}" -gt 0 && ! "$1" == --* ]]; then
  TARGET="$1"
  shift
fi

echo "==> Style reference reminders (run these samples before starting drafting)"
echo "Narrative sample command:"
cargo run --manifest-path rust/Cargo.toml --package style-reference -- narrative --count 5
echo
echo "Dialogue sample command:"
cargo run --manifest-path rust/Cargo.toml --package style-reference -- dialogue --count 5

echo
echo "==> Running full quality pipeline for ${TARGET}"
./scripts/run-quality-session.sh "${TARGET}" "$@"
