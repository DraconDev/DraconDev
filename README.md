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

## Public Projects

### AI & Media
| Project | Description |
|:--------|:------------|
| [ai-vid-editor](https://github.com/DraconDev/ai-vid-editor) | Automated video editing. Silence removal, auto-reframe, watch folders. |
| [volume-and-video-pro](https://github.com/DraconDev/volume-and-video-pro) | Advanced audio and video browser controls. |

### Developer Tools
| Project | Description |
|:--------|:------------|
| [opencode-auto-continue](https://github.com/DraconDev/opencode-auto-continue) | Auto-recover stalled OpenCode sessions. |
| [opencode-auto-force-resume](https://github.com/DraconDev/opencode-auto-force-resume) | Detect and recover stalled AI sessions. |
| [opencode-auto-review-completed-todos](https://github.com/DraconDev/opencode-auto-review-completed-todos) | Auto-review sessions when all todos complete. |

### Browser Extensions
| Project | Description |
|:--------|:------------|
| [browser-extensions-shared](https://github.com/DraconDev/browser-extensions-shared) | Chrome extensions: API debugger, SamAI, and more. |
| [SamAI](https://github.com/DraconDev/SamAI) | AI-powered web experience: search, scraping, chat, forms. |

### Runtimes & Engines
| Project | Description |
|:--------|:------------|
| [dracon-terminal-engine-2](https://github.com/DraconDev/dracon-terminal-engine-2) | TUI runtime with z-indexed compositing, code editor. |
| [dracon-rust-ui](https://github.com/DraconDev/dracon-rust-ui) | ECS-based GPU UI engine. Vello rendering, 10 themes. |
| [dracon-code](https://github.com/DraconDev/dracon-code) | Autonomous engineering runtime. Blueprint-first execution. |

### Platform Services
| Project | Description |
|:--------|:------------|
| [dracon-demons](https://github.com/DraconDev/dracon-demons) | Service daemons: auth, billing, email, AI routing. |
| [dracon-platform](https://github.com/DraconDev/dracon-platform) | Web apps, AI API gateway, dashboard. |
| [dracon-utilities](https://github.com/DraconDev/dracon-utilities) | Background sync, security hardening. |
| [dracon-spark-and-director](https://github.com/DraconDev/dracon-spark-and-director) | Pull-based fleet management for VPS nodes. |

### Games
| Project | Description |
|:--------|:------------|
| [Junk-Runner-bevy](https://github.com/DraconDev/Junk-Runner-bevy) | 100% procedural space roguelike on Bevy 0.18. |

### Meta
| Project | Description |
|:--------|:------------|
| [DraconDev](https://github.com/DraconDev/DraconDev) | Public showcase of the Dracon ecosystem. |

---

## Private & Internal Projects

DraconDev hosts numerous private/internal projects including:
- `dracon-code-*` — Numbered variants for fleet scaling
- `dracon-demons-*` — Service daemon variants
- `dracon-libs-*` — Library variant development
- `dracon-terminal-engine-*` — TUI framework variants
- `test-repo-*` — Experimental repositories
- `tiles-*`, `browser-extensions-shared-*` — Version variants

These follow the same AGPLv3 + CLA licensing when public.

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