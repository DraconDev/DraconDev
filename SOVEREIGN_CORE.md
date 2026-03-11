# 🧠 SOVEREIGN CORE: dracon-code

**"Supervision over Steering."**

`dracon-code` is an autonomous engineering runtime with a dark, operator-focused desktop GUI and a headless CLI control plane. It is the flagship of the Dracon ecosystem.

## 🏛️ Philosophy: The Blueprint-First Model

In `dracon-code`, the **Blueprint is the only source of truth**. We believe that "Chat-driven development" is too noisy and brittle. Instead:
1. **Human writes the objective** (in the GUI or a `do.md`).
2. **Runtime refines `[[slices]]`** in a structured `blueprint.toml`.
3. **Engine executes slices**, runs gate verifiers (`fmt`, `clippy`, `check`, `tests`), and checkpoints progress.
4. **Runtime recovers automatically** where policy allows.

## 🛠️ The Technology

- **Backend:** High-performance Rust engine with a "Commander/Worker" architecture.
- **GUI:** Built with **Tauri 2 + SolidJS**. It features a dark, keyboard-friendly "Hub" for managing fleet-wide progress and a "Project" view for deep supervision.
- **Autonomy:** Supports real token streaming and live "Thinking" blocks to build operator trust.
- **Gates:** Integrated verification pipeline that ensures every autonomous change is "Green" before it moves to the next slice.

## 🚀 Key Features

- **Fleet Hub:** A table showing all projects, phase, git status, and test results in real-time.
- **Last Run Rail:** A live progress bar with color-coded gate results.
- **Blueprint Rail:** A visual queue of upcoming tasks (slices).
- **Desktop Notifications:** Stay informed when a run completes, fails, or stalls.

---

_The blueprint is the contract. The engine is the execution._
