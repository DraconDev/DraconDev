# 💓 SOVEREIGN UTILITIES: The Heartbeat

**"Work securely. Save invisibly."**

The `dracon-utilities` suite provides the essential background services that turn a standard Linux machine into a **Sovereign Workstation**. These tools supersede all previous "Demon" and "Git-Seal" projects.

## 🏥 dracon-sync: The Pulse
**Background Git Automation.**
- **Deterministic Payloads:** Unlike old "AI committers" that produced noisy messages, `dracon-sync` focuses on deterministic, structured history.
- **Watch Roots:** Monitors your workspace and automatically handles pull/commit/push cycles.
- **Freeze Toggle:** Pause synchronization when you need absolute manual control over a repo.

## 🛡️ dracon-warden: The Guardian
**Security Hardening & Secret Management.**
- **Encryption-at-Rest:** Transparently handles your secret files using git filters.
- **Marker Scrubbing:** Automatically detects and scrubs leaked secret markers (`[DEMON_SECRET:...]`) before they can be committed.
- **Policy Enforcement:** Manages `.gitignore` and `.gitattributes` to ensure no sensitive paths are ever tracked as plaintext.

## 🏗️ dracon-system: The Architect
**Machine Maintenance & Symlink Reconciliation.**
- **Diagnostics:** Continuous health checks for your Sovereign Stack services.
- **Storage Cleanup:** Automated management of caches and temporary build artifacts.
- **Symlink Engine:** Reconciles your user configuration (`dotfiles`) against the Dracon system architecture.

## 🤖 dracon-ai: The Operator
**Interactive Execution Loop.**
- **Command-Line Assistant:** The primary way to run immediate tasks with AI guidance.
- **Unified Routing:** Consumes the shared AI routing policy from `dracon-libs`, ensuring consistent model selection across the stack.

---

_A Sovereign machine is a deterministic machine._
