# DraconDev

> **Rust-first tools. Compile-time guarantees.**

Building a sovereign toolchain where the compiler is the first line of defense.

No runtime surprises. No "works on my machine." Just software that ships correct.

---

## Flagship Projects

### [azumi](https://github.com/DraconDev/azumi) — Web Framework
Compile-time CSS validation. Zero hydration, ~3KB runtime.
- CSS-HTML co-validation at compile time
- No JavaScript runtime overhead
- [crates.io](https://crates.io/crates/azumi)

### [dracon-terminal-engine](https://github.com/DraconDev/dracon-terminal-engine) — TUI Framework
35-widget TUI framework. Ship in a single binary.
- 15 themes, 60FPS rendering
- Zero external dependencies
- [crates.io](https://crates.io/crates/dracon-terminal-engine)

### [tiles](https://github.com/DraconDev/tiles) — Terminal File Manager
Terminal file manager with editor integration and git awareness.
- 60FPS grid rendering
- Seamless editor/preview transitions
- SSH and sync capabilities

### [dracon-libs](https://github.com/DraconDev/dracon-libs) — Shared Foundations
Shared Rust crates for the entire Dracon ecosystem.
- `*-kit`: Infrastructure (caching, rate-limiting, retries)
- `dracon-*`: Domain tools (git, system, media, AI routing)
- Contracts: Pure traits enforcing the Dracon Protocol

---

## Complete Toolset

| Project | Language | Description |
|:--------|:---------|:------------|
| [ai-vid-editor](https://github.com/DraconDev/ai-vid-editor) | Rust | Automated video editing. Silence removal, auto-reframe. |
| [opencode-auto-continue](https://github.com/DraconDev/opencode-auto-continue) | JS | Auto-recover, nudge, and review OpenCode sessions. |
| [opencode-auto-review-completed-todos](https://github.com/DraconDev/opencode-auto-review-completed-todos) | JS | Auto-review sessions when todos finish. |
| [kittentts-showcase](https://github.com/DraconDev/kittentts-showcase) | JS | Real-time text-to-speech demo. |
| [browser-extensions-shared](https://github.com/DraconDev/browser-extensions-shared) | TS | Chrome extensions: API debugger, SamAI, and more. |
| [dracon-utilities](https://github.com/DraconDev/dracon-utilities) | Rust | Background sync, security, system maintenance. |
| [dracon-platform](https://github.com/DraconDev/dracon-platform) | Rust | Core platform services and APIs. |
| [Junk-Runner-bevy](https://github.com/DraconDev/Junk-Runner-bevy) | Rust | 100% procedural space roguelike on Bevy 0.18. |

---

## Architecture Principles

1. **Correct by construction** — The compiler is the first line of defense
2. **Zero surprises** — Deterministic behavior, no hidden state
3. **Single binary** — No runtime dependencies, no environment drift
4. **Open source (AGPLv3)** — Transparency with commercial protection

## License

All Dracon projects are licensed under **AGPLv3** with a Contributor License Agreement (CLA). This ensures:
- The entire ecosystem remains open and copyleft
- Competitive forks must contribute back or purchase a commercial license
- Contributors retain rights while granting relicense rights to Dracon

Commercial licenses available for proprietary use. Contact: **license@dracon.uk**
