# dracon-utilities Feature Decision

**Date:** 2026-06-12  
**Decision:** Feature `dracon-utilities` as a prominent usable-tools product, but present the three components clearly instead of treating the monorepo as one vague utility.

## Evidence inspected

### Repo and release state

- Public GitHub repo exists: `https://github.com/DraconDev/dracon-utilities`.
- GitHub API reports the repo is public and defaults to `main`.
- Local copy inspected: `/home/dracon/Dev/dracon-utilities/`.
- Local branch inspected: `main`.
- Remote default branch inspected via GitHub raw README.
- Remote README points to `dracon-utilities-public.git`, which redirects to `DraconDev/dracon-utilities`.
- The repo exposes three installable CLI utilities:
  - `dracon-sync`
  - `dracon-system`
  - `dracon-warden`

### Validation run locally

From `/home/dracon/Dev/dracon-utilities/`:

- `cargo fmt --manifest-path /home/dracon/Dev/dracon-utilities/Cargo.toml -p dracon-sync -p dracon-system -p dracon-warden -- --check` passed.
- `cargo clippy --manifest-path /home/dracon/Dev/dracon-utilities/Cargo.toml -p dracon-sync -p dracon-system -p dracon-warden --all-targets --no-deps` passed with no issues.
- `cargo test --manifest-path /home/dracon/Dev/dracon-utilities/Cargo.toml --workspace -- --test-threads=1` passed: 715 passed, 9 ignored.
- `cargo deny --manifest-path /home/dracon/Dev/dracon-utilities/Cargo.toml check` passed.
- `./scripts/verify-spec.sh` passed from the repo root.

Important caveat: `./scripts/verify-spec.sh` fails if run from the wrong working directory because it expects to run inside the repo.

## Product summary

`dracon-utilities` is a public Rust monorepo of three installable CLI binaries for developer/system workflows.

The three components are:

| Component | Positioning | Best audience | Feature strength |
|:----------|:------------|:--------------|:-----------------|
| `dracon-sync` | Invisible Git sync for AI-powered development. Auto-commits, multi-mirrors, deterministic commit messages. | AI-assisted developers, multi-forge repo maintainers, small teams | Very high |
| `dracon-system` | Proactive disk/process guard for development machines and servers. Cleans Rust targets, monitors disk, renices heavy processes. | Developers, Linux users, small infra operators | High |
| `dracon-warden` | Git filter and repo hardening tool. Encrypts secrets at rest in Git while keeping working tree plaintext. | Developers, teams, security-conscious maintainers | Very high |

## Feature recommendation

### Feature as prominent usable tools

Yes. `dracon-utilities` should move from “private/unverified blocker” to “public usable-tools feature.”

Recommended public wording:

> **dracon-utilities** — three installable Rust CLI utilities: `dracon-sync` for invisible Git sync, `dracon-system` for disk/process protection, and `dracon-warden` for Git secret encryption and repo hardening.

This is stronger than:

> “Infrastructure monorepo.”

And stronger than:

> “Generic utilities.”

The feature should emphasize the three concrete jobs.

## Component feature order

### 1. Feature `dracon-sync` as the headline component

Why:

- Most distinctive product story.
- Strong AI/developer workflow angle.
- Multi-forge sync and deterministic commit messages are concrete.
- It is a repeatable tool, not a resume signal.

Suggested one-liner:

> Invisible Git sync for AI-powered development: auto-commit, multi-mirror, deterministic commit messages.

### 2. Feature `dracon-warden` as the security component

Why:

- Clear security value.
- Strong technical depth.
- Useful to developers and teams.
- Complements `git-seal` as the stronger successor.

Suggested one-liner:

> Git secret encryption and repo hardening while keeping your working tree plaintext.

### 3. Feature `dracon-system` as the practical ops component

Why:

- Easy to understand.
- Useful for developers and small servers.
- Disk full prevention is a concrete pain.
- Less unique than sync/warden, but still strong.

Suggested one-liner:

> Disk/process guard for development machines and servers.

## How this changes public docs

### README.md / README_DRAFT.md

Add `dracon-utilities` as a public usable-tools feature, not as a private note.

Recommended placement: under **Usable tools**, near the top.

Recommended bullet:

```markdown
- [**dracon-utilities**](https://github.com/DraconDev/dracon-utilities) — three installable Rust CLI utilities: `dracon-sync` for invisible Git sync, `dracon-system` for disk/process protection, and `dracon-warden` for Git secret encryption and repo hardening.
```

Remove the old private note from the usable-tools section.

### DIRECTION.md

Update the private/unverified blockers section:

- Remove `dracon-utilities` from private blockers.
- Add it to featured usable tools as a public installable-utilities product.
- Change the utilities decision from “public release?” to “feature rollout and component positioning.”

### PRODUCT_INVENTORY.md

Update the infrastructure tier:

- Mark `dracon-utilities` as public release candidate/public repo.
- Keep the three component paths distinct.
- Update GitHub profile strategy to feature `dracon-utilities` and its components.

### Pin decision

Do **not** pin `dracon-utilities` on the GitHub profile.

Reason:

- The repo is useful and worth featuring in the README, but the label `dracon-utilities` is too vague for a pin.
- A pin must communicate value in one glance. "Utilities" sounds generic and risks feeling like a lie unless the pin description carries too much explanation.
- The three component jobs are clearer in README/profile copy than in a pin slot.

Better pin choice:

- Pin `obs-wayland-hotkey` instead if you need a concrete utility pin.
- Suggested pin description: "OBS hotkey daemon for Wayland/X11. 8★; concrete utility people can understand quickly."

README/profile wording remains:

> `dracon-utilities` — three installable Rust CLI utilities: `dracon-sync` for invisible Git sync, `dracon-system` for disk/process protection, and `dracon-warden` for Git secret encryption and repo hardening.

## Business-angle fit

This is a strong business-angle feature because it shows:

- shipped installable tools,
- Rust systems depth,
- developer pain points,
- repeatable utility rather than one-off employment work,
- possible future productized support or paid add-ons tied to existing assets.

It does **not** need to be the primary revenue product yet. It should be featured as proof and as a usable-tools product.

## Constraints and caveats

- Do not claim paid revenue or users.
- Do not claim the repo is polished for casual non-technical users.
- Be clear that installation expects Rust/systemd/Linux familiarity.
- Do not hide that the three components are distinct jobs.
- Do not promote it ahead of verified Chrome extension/game product destinations in product-destination sections.
- Keep `dracon-libs` as a build prerequisite if mentioned.
- If future public-readiness scans find issues, pause promotion until fixed.

## Final decision

Feature `dracon-utilities` prominently as a public usable-tools product in README/profile copy, with the three components named clearly:

1. `dracon-sync` — invisible Git sync for AI-powered development.
2. `dracon-system` — disk/process guard.
3. `dracon-warden` — Git secret encryption and repo hardening.

Do **not** pin `dracon-utilities` on the GitHub profile. Use the pin slot for a clearer concrete utility such as `obs-wayland-hotkey`, while keeping `dracon-utilities` in the README as a named component suite.
