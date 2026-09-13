#!/usr/bin/env bash
set -euo pipefail

output=$(XDG_BIN_HOME=/tmp/paying-attention-bin XDG_CONFIG_HOME=/tmp/paying-attention-config \
  bash scripts/install-local.sh --dry-run --systemd-user)

grep -Fq "/tmp/paying-attention-bin/paying-attention" <<<"$output"
grep -Fq "/tmp/paying-attention-config/systemd/user/paying-attention.service" <<<"$output"
