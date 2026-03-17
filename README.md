# DraconDev

**Local-first tooling. High-performance Rust. Build your own stack.**

---

## Projects

### [dracon-terminal-engine](https://github.com/DraconDev/dracon-terminal-engine)
High-performance TUI runtime with z-indexed compositing. Think game engine for the terminal.

- Full z-indexed layering (planes, overlays, modals)
- Kitty Keyboard Protocol — chords, modifiers, release events, discrete mouse
- In-terminal image rendering, procedural geometry, truecolor
- Syntax-highlighted editor with fuzzy filters, undo/redo, multi-selection

### [dracon-code](./SOVEREIGN_CORE.md)
Autonomous engineering runtime. Plans, executes, verifies, and reports against a Blueprint spec. Ships with a Tauri dashboard for supervising runs.

### [dracon-utilities](./SOVEREIGN_UTILITIES.md)
Background daemons for deterministic workflows — git automation, secret scrubbing, encryption-at-rest.

### [Junk-Runner](./SOVEREIGN_SIMULATION.md)
Procedural space sim built with Bevy 0.18. Zero traditional 3D assets — everything rendered via gizmos and math.

### [dracon-libs](./SOVEREIGN_ENGINEERING.md)
Shared toolkit: AI routing, SQLite/ONNX memory, TUI compositing primitives, secure git operations.

---

## Principles

- **Local-first** — runs offline
- **Deterministic** — same input, same output
- **Ownership** — your data, your history
