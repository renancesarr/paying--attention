# Manual Validation: Ticket 10

| Check | Outcome | Observed at | Command | Observation | Artifact |
| --- | --- | --- | --- | --- | --- |
| fullscreen-on-both-displays | passed | 2026-09-13T11:15:04-03:00 | `cargo run -p paying_attention_desktop` | Attention Block abriu em fullscreen nas duas telas. |  |
| alt-tab | failed | 2026-09-13T11:15:16-03:00 | `cargo run -p paying_attention_desktop` | Alt+Tab alternou para outra aplicacao e contornou a Attention Block. |  |
| super-tab | failed | 2026-09-13T11:17:23-03:00 | `cargo run -p paying_attention_desktop` | Super+Tab alternou para outra aplicacao e contornou a Attention Block. |  |
| initial-focus-and-keyboard | passed | 2026-09-13T11:25:01-03:00 | `cargo run -p paying_attention_desktop` | Without clicking, text entered directly into the Declared Task field. |  |
| development-safe-exit | passed | 2026-09-13T11:27:54-03:00 | `Ctrl+Shift+F` | All Attention Block windows closed and the terminal returned to the prompt without freezing the session. |  |
| workspace-switch | failed | 2026-09-13T11:29:25-03:00 | `Super+Right` | Super+Right switched to another GNOME workspace, bypassing the Attention Block. Super+Up and Super+Down had no effect in this environment. |  |
| other-application-interaction | failed | 2026-09-13T11:30:24-03:00 | `Alt+Tab / Super+Tab / Super+Right` | After bypassing the Attention Block, other applications remained interactive and usable. |  |

