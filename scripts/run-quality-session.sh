#!/usr/bin/env bash
set -euo pipefail

print_usage() {
  cat <<'USAGE'
Usage:
  scripts/run-quality-session.sh <chapter-or-directory> [quality-orchestrator flags...]

Runs the full quality pipeline (cliche detection, voice consistency, continuity, and
length checks) via the quality-orchestrator binary.

Examples:
  scripts/run-quality-session.sh chapters/chapter-01.md
  scripts/run-quality-session.sh chapters --fail_on blocker
  scripts/run-quality-session.sh chapters/chapter-03.md --no_artifact
USAGE
}

if [[ $# -lt 1 ]]; then
  print_usage
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${PROJECT_ROOT}"

TARGET="$1"
shift

cargo run --manifest-path rust/Cargo.toml --package quality-orchestrator -- "$TARGET" "$@"
