# Pin vs README Feature Analysis

**Date:** 2026-06-12  
**Decision:** Treat GitHub pins and README/profile copy as different signal layers.

## Core rule

GitHub pins are constrained proof. They must be instantly understandable and valuable to a newcomer.

The README/profile copy has more freedom. It can include product destinations, usable tools, private-but-honest notes, and build-with foundations as secondary material.

## Pin layer

Pins should answer:

> “Why should a newcomer trust that I build real, usable systems?”

Good pins are:

- concrete,
- usable or clearly component-level,
- easy to explain in one line,
- stronger than a vague parent bucket,
- not just “foundations I used to build other things.”

Do **not** pin:

- `dracon-utilities` — too vague as a parent bucket.
- `dracon-terminal-engine` — strong build-with foundation, but low direct newcomer value.

## README/profile layer

The README can be broader. It should still lead with product/business signal, but it can include:

- verified product destinations first,
- usable tools,
- GitHub pin focus,
- build-with foundations in a secondary section,
- private/unverified work only with honest caveats.

This means `dracon-terminal-engine`, `azumi-live-ssr-framework`, and `ai-gui-auto-video-editor` can stay in the README as secondary build-with foundations without spending pin slots.

## Candidate evaluation

| Candidate | Pin fit | Reason |
|:----------|:--------|:---------|
| `dracon-sync` | Strong | Intricate AI/developer Git sync daemon; 30 Rust files / ~22.8K Rust LOC in current component tree. |
| `dracon-system` | Strong | Intricate disk/process guard; 12 Rust files / ~6.1K Rust LOC. |
| `dracon-warden` | Strong | Intricate Git encryption/repo hardening tool; 29 Rust files / ~10.2K Rust LOC. |
| `tiles-tui-file-manager` | Strong | Concrete usable TUI file manager; better newcomer signal than foundations. |
| `folder-auto-banner` | Strong | Concrete usable directory dashboard; `ls` plus git, TODO, ports, Docker, build, and cached size context. |
| `obs-wayland-hotkey` | Strong | Concrete OBS hotkey daemon; 8 stars, crates.io, CI, clear Wayland/X11 use case. |
| `dracon-terminal-engine` | README, not pin | Strong build-with foundation, but not something newcomers can immediately run/use. |
| `youtube-video-uploader` | README, not pin | Useful Rust CLI/library, but narrower creator-tool signal than the chosen pins. |
| `git-seal` | De-emphasize/omit | Older version of the `dracon-warden` story; useful in inventory, but not a primary README or pin item. |
| `dracon-utilities` | README, not pin | Parent bucket is too vague for a pin; use README to name the three component jobs. |

## Final pin set

1. `dracon-sync`
2. `dracon-system`
3. `dracon-warden`
4. `tiles-tui-file-manager`
5. `folder-auto-banner`
6. `obs-wayland-hotkey`

Important GitHub constraint:

- GitHub profile pins can pin repos, not subdirectories.
- `dracon-sync`, `dracon-system`, and `dracon-warden` currently exist as component directories inside `dracon-utilities`; separate public repos currently return 404.
- Until they are split/published as separate repos, link to the component subdirectories in the README and treat them as pin targets once they are pin-able repos.

## Final README guidance

Lead with:

1. verified Chrome extension/game destinations,
2. usable tools,
3. GitHub pin focus,
4. build-with foundations as secondary.

Current README pin focus:

- `dracon-sync`
- `dracon-system`
- `dracon-warden`
- `tiles-tui-file-manager`
- `folder-auto-banner`
- `obs-wayland-hotkey`

Current README secondary build-with foundations:

- `dracon-terminal-engine`
- `azumi-live-ssr-framework`
- `ai-gui-auto-video-editor`

## Final decision

Pins should be the six concrete items above. The README should remain broader and include the `dracon-utilities` parent suite, usable tools, and secondary build-with foundations. Do not pin the parent utilities repo or the terminal engine.

For future local-project candidates, see `LOCAL_PROJECT_PIN_README_CANDIDATES.md`.
