#!/usr/bin/env bash
set -euo pipefail

usage() {
  printf '%s\n' "Usage: $0 --ticket <number> --check <name> --outcome <passed|failed|blocked> --command <command> --observation <text>"
}

required_value() {
  local flag=$1
  local value=${2:-}
  if [[ -z "$value" ]]; then
    printf '%s\n' "$flag requires a value." >&2
    exit 2
  fi
  printf '%s' "$value"
}

ticket=""
check=""
outcome=""
command=""
observation=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --ticket)
      ticket=$(required_value "$1" "${2:-}")
      shift 2
      ;;
    --check)
      check=$(required_value "$1" "${2:-}")
      shift 2
      ;;
    --outcome)
      outcome=$(required_value "$1" "${2:-}")
      shift 2
      ;;
    --command)
      command=$(required_value "$1" "${2:-}")
      shift 2
      ;;
    --observation)
      observation=$(required_value "$1" "${2:-}")
      shift 2
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$ticket" || -z "$check" || -z "$outcome" || -z "$command" || -z "$observation" ]]; then
  usage >&2
  exit 2
fi

if ! command -v gnome-screenshot >/dev/null 2>&1; then
  printf '%s\n' "gnome-screenshot is required for automatic capture. Capture the screen manually, then use paying-attention validation record with --artifact." >&2
  exit 3
fi

repository_root=${PAYING_ATTENTION_REPOSITORY_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}
evidence_directory=${PAYING_ATTENTION_EVIDENCE_DIR:-/tmp/paying-attention-evidence}
observed_at=${PAYING_ATTENTION_OBSERVED_AT:-$(date --iso-8601=seconds)}
safe_check=$(printf '%s' "$check" | tr -cs '[:alnum:]_-' '-')
timestamp=$(date +%Y%m%dT%H%M%S)
artifact="$evidence_directory/ticket-${ticket}-${safe_check}-${timestamp}.png"

mkdir -p "$evidence_directory"
gnome-screenshot -f "$artifact"

if [[ ! -s "$artifact" ]]; then
  printf '%s\n' "Screenshot command completed without creating an artifact: $artifact" >&2
  exit 4
fi

cd "$repository_root"
cargo run -p paying_attention_cli -- validation record \
  --ticket "$ticket" \
  --check "$check" \
  --outcome "$outcome" \
  --observed-at "$observed_at" \
  --command "$command" \
  --observation "$observation" \
  --artifact "$artifact"

printf '%s\n' "Captured and recorded: $artifact"
