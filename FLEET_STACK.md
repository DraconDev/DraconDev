# ⚡ FLEET STACK: Autonomous Infrastructure

A local-first, pull-based alternative to cloud infrastructure management. Manage a fleet of servers using nothing but Git and local binaries.

## 🛰️ dracon-spark
**The Node Reconciler.**
A standalone agent that lives on your servers. It "reconciles" the node's current state against the **Desired State** defined in a central Git "Control Repo."
- **Pull-Based:** Servers don't need inbound SSH access. They pull their own instructions.
- **Generations:** Versioned state management with easy rollbacks to previous stable generations.
- **Zero-Trust:** Every reconcile is verified against local machine-decryption keys.

## 🕹️ dracon-director
**The Fleet Control Plane.**
The optional orchestration layer for managing multiple Spark nodes.
- **Fleet Summary:** A high-level view of rollout health across all nodes.
- **Generation Bumps:** Deploy a new artifact pin or configuration change to an entire "Role" (e.g., `momo-prod`) with a single command.
- **External Sync:** Automatically handles DNS updates (Cloudflare) and load balancer pools based on node health.

---

_The infrastructure is the control repo. The fleet is the manifestation._
