# DraconDev

> **Rust-first tools. Compile-time guarantees.**

Building software where the compiler is the first line of defense.

No runtime surprises. No "works on my machine." Just software that ships correct.

---

## Flagship

| # | Project | What it does |
|---|:---|:---|
| 1 | **[azumi](https://github.com/DraconDev/azumi)** | Compile-time CSS validation. Zero hydration, ~3KB runtime. |
| 2 | **[dracon-terminal-engine](https://github.com/DraconDev/dracon-terminal-engine)** | 35-widget TUI framework. Ship in a single binary. |
| 3 | **[tiles](https://github.com/DraconDev/tiles)** | Terminal file manager. Edit, browse, sync. |
| 4 | **[dracon-libs](https://github.com/DraconDev/dracon-libs)** | Shared Rust: git, system, media, AI. |

---

## More Tools

| Project | Description |
|:---|:---|
| [ai-vid-editor](https://github.com/DraconDev/ai-vid-editor) | Automated video editing. Drop footage, get results. |
| [opencode-auto-continue](https://github.com/DraconDev/opencode-auto-continue) | Auto-recover, nudge, and review OpenCode sessions. |
| [opencode-auto-review-completed-todos](https://github.com/DraconDev/opencode-auto-review-completed-todos) | Auto-review sessions when todos finish. |
| [kittentts-showcase](https://github.com/DraconDev/kittentts-showcase) | Text-to-speech demo. |
| [browser-extensions-shared](https://github.com/DraconDev/browser-extensions-shared) | Chrome extensions: API debugger, SamAI, and more. |
| [dracon-utilities](https://github.com/DraconDev/dracon-utilities) | Background sync, security, system maintenance. |
| [dracon-platform](https://github.com/DraconDev/dracon-platform) | Core platform services and APIs. |

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Dracon Stack                       │
├─────────────────────────────────────────────────────┤
│  Interface Layer                                     │
│  ├── azumi (Web)                                    │
│  ├── dracon-terminal-engine (TUI)                   │
│  └── tiles (File Manager)                           │
├─────────────────────────────────────────────────────┤
│  Core Engine                                        │
│  ├── dracon-libs (Shared Rust)                     │
│  ├── dracon-code (Autonomous Runtime)              │
│  └── dracon-rust-ui (GPU UI)                       │
├─────────────────────────────────────────────────────┤
│  Infrastructure                                     │
│  ├── dracon-utilities (Sync, Security, System)      │
│  ├── dracon-spark-and-director (Fleet Management)   │
│  └── dracon-platform (Platform Services)             │
└─────────────────────────────────────────────────────┘
```

---

## Principles

**Correct by construction, not tested into correctness.**

- The compiler is the first line of defense
- Deterministic behavior, no hidden state
- Single binary, no runtime dependencies

---

## License

Source-available: [Dracon License](https://github.com/DraconDev/azumi/blob/master/LICENSE).

Free for small teams. Paid for organizations. Some tools MIT.

[dracon.uk](https://dracon.uk)
