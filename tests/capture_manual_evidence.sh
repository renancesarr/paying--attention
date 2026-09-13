#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
temporary=$(mktemp -d)
fake_bin="$temporary/bin"
artifact_dir="$temporary/evidence"
arguments_file="$temporary/cargo-arguments"
mkdir -p "$fake_bin"

cat > "$fake_bin/gnome-screenshot" <<'SCRIPT'
#!/usr/bin/env bash
set -euo pipefail
test "$1" = "-f"
printf 'fake-png' > "$2"
SCRIPT

cat > "$fake_bin/cargo" <<SCRIPT
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "\$@" > "$arguments_file"
SCRIPT

chmod +x "$fake_bin/gnome-screenshot" "$fake_bin/cargo"

PATH="$fake_bin:$PATH" \
PAYING_ATTENTION_EVIDENCE_DIR="$artifact_dir" \
PAYING_ATTENTION_REPOSITORY_ROOT="$root" \
PAYING_ATTENTION_OBSERVED_AT="2026-09-13T13:40:00-03:00" \
"$root/scripts/capture-manual-evidence.sh" \
  --ticket 10 \
  --check fullscreen \
  --outcome passed \
  --command "cargo run -p paying_attention_desktop" \
  --observation "Fullscreen opened on both displays."

artifact=$(find "$artifact_dir" -type f -name '*.png')
test -s "$artifact"
grep -Fx -- "validation" "$arguments_file"
grep -Fx -- "record" "$arguments_file"
grep -Fx -- "--artifact" "$arguments_file"
grep -Fx -- "$artifact" "$arguments_file"
