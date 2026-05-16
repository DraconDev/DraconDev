# Dracon License Migration — Checklist

**Goal:** Standardize AGPLv3 + CLA across all Dracon projects.

## Files to Add to Every Repo

Each repo needs these 4 files copied from `DraconDev` root:

| File | Purpose |
|------|---------|
| `LICENSE` | AGPLv3 full text |
| `CLA.md` | Contributor License Agreement (grant-back model) |
| `CONTRIBUTING.md` | Contribution guidelines + CLA notice |
| `COMMERCIAL-LICENSE.md` | Commercial licensing tier info |

## Per-Repo Migration

For **each** Dracon repo, run:

```bash
# 1. Copy the 4 template files
cp /path/to/DraconDev/LICENSE /path/to/repo/
cp /path/to/DraconDev/CLA.md /path/to/repo/
cp /path/to/DraconDev/CONTRIBUTING.md /path/to/repo/
cp /path/to/DraconDev/COMMERCIAL-LICENSE.md /path/to/repo/

# 2. Stage and commit
cd /path/to/repo
git add LICENSE CLA.md CONTRIBUTING.md COMMERCIAL-LICENSE.md
git commit -m "docs: adopt AGPLv3 + CLA license stack

- Add AGPLv3 LICENSE (GNU Affero General Public License v3)
- Add Contributor License Agreement (grant-back model)
- Add CONTRIBUTING.md with CLA notice
- Add COMMERCIAL-LICENSE.md for proprietary use tier
- Standardizes license across Dracon ecosystem
"

# 3. Push to all remotes
git push origin main && git push codeberg main && git push gitlab main
```

## Repos to Update

```
azumi                          → https://github.com/DraconDev/azumi
dracon-terminal-engine         → https://github.com/DraconDev/dracon-terminal-engine
tiles                          → https://github.com/DraconDev/tiles
dracon-libs                    → https://github.com/DraconDev/dracon-libs
ai-vid-editor                 → https://github.com/DraconDev/ai-vid-editor
dracon-utilities               → https://github.com/DraconDev/dracon-utilities
dracon-platform                → https://github.com/DraconDev/dracon-platform
Junk-Runner-bevy               → https://github.com/DraconDev/Junk-Runner-bevy
opencode-auto-continue         → https://github.com/DraconDev/opencode-auto-continue
opencode-auto-review-completed-todos → https://github.com/DraconDev/opencode-auto-review-completed-todos
kittentts-showcase             → https://github.com/DraconDev/kittentts-showcase
browser-extensions-shared       → https://github.com/DraconDev/browser-extensions-shared
dracon-code                    → https://github.com/DraconDev/dracon-code
SamAI                          → https://github.com/DraconDev/SamAI
```

## GitHub: Enable CLA Assistant Bot

For GitHub repos, enable CLA enforcement:

1. Go to **Settings → Apps** for each repo
2. Search for **CLA Assistant** (or use `assistor` bot)
3. Install and configure with DraconDev/DraconDev as the org-level app
4. Add a `.claire` file to each repo root:

```yaml
# .claire — CLA configuration
cla:
  github:
    label: "cla"
    bot: "cla-assistant[bot]"
  CONTRIBUTOR:
    name: "individual"
    min-contributions: 1
  SIGN:
    individual:
      by-email: false
      by-github: true
```

Alternatively, use the GitHub Action version (see `.github/workflows/cla.yml` in DraconDev).

## After Migration

- [ ] Verify LICENSE appears on GitHub repo page (shows as AGPL-3.0)
- [ ] Verify PRs require CLA check to pass
- [ ] Update each project's `package.json` / `Cargo.toml` with SPDX license identifier
- [ ] Add `SPDX-License-Identifier: AGPL-3.0` to all Rust `lib.rs` / `main.rs` file headers
- [ ] Announce migration in project docs / release notes

## One-Shot Bash (from DraconDev root)

```bash
# Clone all repos, add license files, commit, push
# Adjust paths as needed

REPOS=(
  "~/Dev/azumi"
  "~/Dev/dracon-terminal-engine"
  "~/Dev/tiles"
  "~/Dev/dracon-libs"
  "~/Dev/ai-vid-editor"
  "~/Dev/dracon-utilities"
  "~/Dev/dracon-platform"
  "~/Dev/Junk-Runner-bevy"
  "~/Dev/opencode-auto-continue"
  "~/Dev/opencode-auto-review-completed-todos"
  "~/Dev/kittentts-showcase"
  "~/Dev/browser-extensions-shared"
  "~/Dev/dracon-code"
  "~/Dev/SamAI"
)

for repo in "${REPOS[@]}"; do
  if [ -d "$repo" ]; then
    echo "Processing: $repo"
    cp LICENSE CLA.md CONTRIBUTING.md COMMERCIAL-LICENSE.md "$repo/"
    cd "$repo"
    git add LICENSE CLA.md CONTRIBUTING.md COMMERCIAL-LICENSE.md
    git commit -m "docs: adopt AGPLv3 + CLA license stack" 2>/dev/null
    git push origin main 2>/dev/null && echo "  ✓ pushed" || echo "  ✗ push failed"
  else
    echo "Not found: $repo"
  fi
done
```

---

*Last updated: 2026-05-16*