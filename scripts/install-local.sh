#!/usr/bin/env bash
set -euo pipefail

dry_run=false
systemd_user=false
for argument in "$@"; do
  case "$argument" in
    --dry-run) dry_run=true ;;
    --systemd-user) systemd_user=true ;;
    *) printf 'Unknown option: %s\n' "$argument" >&2; exit 2 ;;
  esac
done

bin_dir=${XDG_BIN_HOME:-"${HOME}/.local/bin"}
config_dir=${XDG_CONFIG_HOME:-"${HOME}/.config"}
desktop_file="${config_dir}/autostart/paying-attention.desktop"
service_file="${config_dir}/systemd/user/paying-attention.service"

run() {
  if "$dry_run"; then
    printf '%q ' "$@"
    printf '\n'
  else
    "$@"
  fi
}

run cargo build --release --workspace
run mkdir -p "$bin_dir"
run install -m 755 target/release/paying-attention "$bin_dir/paying-attention"
run install -m 755 target/release/paying-attention-desktop "$bin_dir/paying-attention-desktop"

if "$systemd_user"; then
  run mkdir -p "$(dirname "$service_file")"
  if "$dry_run"; then
    printf '%s\n' "$service_file"
  else
    cat > "$service_file" <<EOF
[Unit]
Description=Paying Attention

[Service]
ExecStart=${bin_dir}/paying-attention-desktop
Restart=on-failure

[Install]
WantedBy=default.target
EOF
    systemctl --user daemon-reload
    systemctl --user enable --now paying-attention.service
  fi
else
  run mkdir -p "$(dirname "$desktop_file")"
  if "$dry_run"; then
    printf '%s\n' "$desktop_file"
  else
    cat > "$desktop_file" <<EOF
[Desktop Entry]
Type=Application
Name=Paying Attention
Exec=${bin_dir}/paying-attention-desktop --autostart
X-GNOME-Autostart-enabled=true
EOF
  fi
fi
