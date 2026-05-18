# DraconDev

> **Performance-first tools. Compile-time guarantees.**

Building a sovereign toolchain where the compiler is the first line of defense.

No runtime surprises. No "works on my machine." Just software that ships correct.

---

## Flagship Projects

### [azumi — Rust web framework](https://github.com/DraconDev/azumi) ⭐
Server-rendered HTML with client interactivity — all Rust, zero custom JavaScript.
- CSS-HTML co-validation at compile time
- 3KB JS runtime handles what HTMX needs the network for
- 41 framework widgets (CommandPalette, Form, DataGrid, etc.)
- [crates.io](https://crates.io/crates/azumi)

### [dracon-terminal-engine — Rust TUI framework](https://github.com/DraconDev/dracon-terminal-engine) ⭐
Build terminal apps in Rust: widgets, compositor, themes, single binary.
- 15 built-in themes, 60FPS rendering
- Zero external dependencies
- [crates.io](https://crates.io/crates/dracon-terminal-engine)

### [obs-wayland-hotkey — OBS hotkeys for Wayland](https://github.com/DraconDev/obs-wayland-hotkey) ⭐⭐⭐⭐⭐⭐⭐
Rust daemon that gives you global OBS Studio hotkeys on Wayland (and X11). Uses evdev to bypass Wayland's input restrictions.
- Auto-reconnect, auto-start via systemd
- F13–F24 as stream deck buttons
- [crates.io](https://crates.io/crates/obs-hotkey)

### [git-ai-committer — AI auto-commit for VS Code](https://github.com/DraconDev/git-ai-committer) ⭐⭐⭐⭐⭐⭐
VS Code extension that watches your coding pauses and auto-commits with AI-generated messages. Your work is saved without interrupting your flow.

### [tiles — Terminal file manager](https://github.com/DraconDev/tiles) ⭐
Dual-pane file manager with built-in text editor, git integration, SSH browsing, and system monitor.
- Vim-style navigation, command palette
- Smart terminal spawning (Konsole/Kitty/Wezterm)
- [crates.io](https://crates.io/crates/tiles-tui-file-manager)

### [dracon-libs — Shared Rust crate collection](https://github.com/DraconDev/dracon-libs) ⭐
Reusable Rust libraries for git operations, system monitoring, media processing, AI routing, TUI, and memory/embeddings.
- `dracon-git`, `dracon-system`, `dracon-terminal-engine`, `dracon-memory-runtime`
- `dracon-ai-contracts` + `ai-routing-runtime` for AI provider selection
- `dracon-media-runtime`, `dracon-video-runtime`, `dracon-stt-runtime`

---

## Public Projects

### AI & Media
| Project | Description |
|:--------|:------------|
| [ai-vid-editor — AI video editor](https://github.com/DraconDev/ai-vid-editor) ⭐ | Drop raw footage, get polished output. CLI + GUI + headless watch-folder mode. Silence removal, auto-reframe, audio enhancement. |

### Developer Tools
| Project | Description |
|:--------|:------------|
| [git-seal — Git encryption filter](https://github.com/DraconDev/git-seal) ⭐⭐ | Transparent file encryption in Git. Encrypts on commit, decrypts on checkout. You work with plaintext locally; the repo stores ciphertext. |
| [opencode-auto-continue — OpenCode stall recovery](https://github.com/DraconDev/opencode-auto-continue) ⭐ | Detects stalled AI coding sessions, aborts, sends continue. 4-layer context compaction, todo nudging, hallucination loop detection. |
| [css-peek-pro — CSS navigation for VS Code](https://github.com/DraconDev/css-peek-pro) ⭐ | Hover to jump from HTML/JS/TS to CSS definitions. Smart scoping filters noise in large projects. Works with React, Vue, Svelte, Rust templates. |
| [opencode-auto-force-resume — OpenCode session recovery](https://github.com/DraconDev/opencode-auto-force-resume) | Detects stalled OpenCode sessions and recovers them. Stall pattern detection, terminal progress bar. |
| [opencode-auto-review-completed-todos — Auto-review on completion](https://github.com/DraconDev/opencode-auto-review-completed-todos) | When all OpenCode todos are completed, automatically sends a review message. Debounced to avoid false triggers. |

### Streaming & OBS
| Project | Description |
|:--------|:------------|
| [obs-wayland-hotkey — OBS hotkeys for Wayland](https://github.com/DraconDev/obs-wayland-hotkey) | Rust daemon for global OBS Studio hotkeys on Wayland and X11. |

### Browser Extensions
| Project | Description |
|:--------|:------------|
| [chrome-auto-fullscreen — Auto-fullscreen any page](https://github.com/DraconDev/chrome-auto-fullscreen) ⭐⭐ | Auto-fullscreen on page load. Click-and-hold to toggle. Handles SPA navigations. Not just videos — any page. |
| [api-debugger — HTTP debugger Chrome extension](https://github.com/DraconDev/api-debugger) ⭐ | Capture, replay, and modify HTTP requests in-browser. Import from Postman/Insomnia/OpenAPI. No account needed. |
| [volume-and-video-pro — Browser audio/video controls](https://github.com/DraconDev/volume-and-video-pro) | Per-tab volume and video playback controls in the browser. |

### Runtimes & Engines
| Project | Description |
|:--------|:------------|
| [terma — Terminal compositor engine](https://github.com/DraconDev/terma) ⭐ | Low-level terminal rendering: z-indexed layers, TrueColor, SGR mouse, Kitty keyboard protocol. Drop-in Ratatui support. |

### Meta
| Project | Description |
|:--------|:------------|
| [DraconDev — Ecosystem showcase](https://github.com/DraconDev/DraconDev) | This repo. Public overview of the Dracon ecosystem. |

---

## Private & Internal Projects

DraconDev hosts numerous private/internal projects including:
- `dracon-code` / `dracon-code-*` — Autonomous engineering runtime and fleet scaling variants
- `dracon-demons` / `dracon-demons-*` — Service daemons: auth, billing, email, AI routing
- `dracon-platform` — Web apps, AI API gateway, dashboard
- `dracon-utilities` — Background sync, security hardening
- `dracon-spark-and-director` — Pull-based fleet management for VPS nodes
- `dracon-rust-ui` — ECS-based GPU UI engine with Vello rendering
- `browser-extensions-shared` / `SamAI` — Chrome extensions and AI-powered web tools
- `Junk-Runner-bevy` — 100% procedural space roguelike on Bevy
- `dracon-libs-*`, `dracon-terminal-engine-*`, `tiles-*` — Version variants
- `test-repo-*` — Experimental repositories

These follow the same AGPLv3 + CLA licensing when public.

---

## Architecture Principles

1. **Correct by construction** — The compiler is the first line of defense
2. **Zero surprises** — Deterministic behavior, no hidden state
3. **Single binary** — No runtime dependencies, no environment drift
4. **Open source (AGPLv3)** — Transparency with commercial protection

## License

This project is dual-licensed:

- **AGPL-3.0-only** — See [LICENSE](LICENSE) for the full text. This is the default license for open source use.
- **Commercial License** — For organizations that prefer not to comply with AGPLv3's source disclosure requirements. See [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md) for details.

By contributing to this project, you agree to the terms in [CLA.md](CLA.md).
