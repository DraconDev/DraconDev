# 🧬 SOVEREIGN ENGINEERING: The DNA

The Interface Layer and the Underlying Libraries that power the Sovereign Stack.

## 🖼️ THE INTERFACE LAYER: High-Fidelity TUIs & GUIs

### 1. [Azumi](https://github.com/DraconDev/azumi) (The Web Engine)
A full-stack Rust engine with **zero hydration tax**.
- **Compiler-Driven:** CSS-HTML co-validation and optimistic UI updates handled at compile time.
- **Speed:** ~16kb runtime, outperforming modern frameworks like React and Leptos.

### 2. [Axiom UI](https://github.com/DraconDev/Axiom-UI) (The AI-First UI Framework)
The first framework built specifically for **AI agents** to build interfaces for humans.
- **Semantic Protocol:** AI describes *intent* ("A destructive button with breathing room"); Axiom handles the *pixels*.
- **GPU-Native:** Powered by **Vello + WGPU** for sub-millisecond frames and mathematically perfect typography.

### 3. [Tiles](https://github.com/DraconDev/tiles) (The Data Commander)
A 60FPS TUI "Data Commander" for high-performance file management.
- **Terma Engine:** A sophisticated engine wrapper for Ratatui that enables 60FPS grid rendering in the terminal.
- **Engage Mode:** Seamless transition between file navigation and full-screen editor/preview.

---

## 📚 dracon-libs: The Shared Foundations

`dracon-libs` is the repository of shared Rust crates that ensure every Dracon tool shares the same high standards.

- **`*-kit` (Infrastructure):** Stable crates for caching, rate-limiting, and retries.
- **`dracon-*` (Domain Tools):** Reusable logic for secure git operations, memory retrieval (SQLite/ONNX), and AI routing policies.
- **Contracts:** Pure traits and types that enforce the "Dracon Protocol" across the entire stack.

---

_Standardize the DNA; innovate on the manifesting._
