# DraconDev

Rust-first tools that catch bugs before they ship.

## What's Being Built

A sovereign toolchain for developers who believe software should be correct by construction — not "tested into correctness." Every project here shares one property: **the compiler is the first line of defense.**

### Flagship Projects

**[azumi](https://github.com/DraconDev/azumi)** — Full-stack Rust web framework with compile-time CSS-HTML co-validation. Catches typos before deployment. Zero hydration, ~3KB runtime.

**[dracon-terminal-engine](https://github.com/DraconDev/dracon-terminal-engine)** — TUI framework shipping 35 widgets and 15 themes in a single binary. Published on [crates.io](https://crates.io/crates/dracon-terminal-engine).

### Active Development *(source-available)*

**[dracon-code](https://github.com/DraconDev/dracon-code)** *(private)* — Autonomous engineering runtime. Blueprint-first execution, fleet management, gate verification. Dark GUI + CLI.

**[dracon-rust-ui](https://github.com/DraconDev/dracon-rust-ui)** *(private)* — ECS-based GPU UI engine. Vello rendering, 10 themes, 44 interactive showcases.

### Public Tools

| Project | What It Does |
|:---|:---|
| **[tiles](https://github.com/DraconDev/tiles)** | Terminal file manager with dual-pane nav, editor, git integration, SSH |
| **[dracon-libs](https://github.com/DraconDev/dracon-libs)** | Shared Rust libraries: git, system, media, memory, AI routing |
| **[ai-vid-editor](https://github.com/DraconDev/ai-vid-editor)** | Automated video editing: silence removal, auto-reframe, audio enhancement |
| **[api-debugger](https://github.com/DraconDev/api-debugger)** | Chrome extension for HTTP debugging. No account required |
| **[opencode-auto-review-completed-todos](https://github.com/DraconDev/opencode-auto-review-completed-todos)** | OpenCode plugin: auto-reviews sessions when todos complete |
| **[opencode-auto-force-resume](https://github.com/DraconDev/opencode-auto-force-resume)** | OpenCode plugin: recovers stalled AI sessions |
| **[kittentts-showcase](https://github.com/DraconDev/kittentts-showcase)** | Real-time text-to-speech demo |

## Stack

Every project above is built with **Rust**, **determinism**, and **zero tolerance for runtime surprises**.

The source-available projects use the [Dracon License](https://github.com/DraconDev/azumi/blob/master/LICENSE) — free for small teams, paid for larger organizations. Some tools are MIT. Some services are closed.
