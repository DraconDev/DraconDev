# License Migration — Checklist

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

## Repos Updated (2026-05-16)

All 18 DraconDev repositories have been migrated to AGPLv3 + CLA:

```
ai-auto-writer                  → https://github.com/DraconDev/ai-auto-writer
ai-vid-editor                   → https://github.com/DraconDev/ai-vid-editor
azumi                           → https://github.com/DraconDev/azumi
browser-extensions-shared        → https://github.com/DraconDev/browser-extensions-shared
dracon-code                     → https://github.com/DraconDev/dracon-code
dracon-demons                   → https://github.com/DraconDev/dracon-demons
dracon-libs                     → https://github.com/DraconDev/dracon-libs
dracon-platform                → https://github.com/DraconDev/dracon-platform
dracon-rust-ui                 → https://github.com/DraconDev/dracon-rust-ui
dracon-spark-and-director       → https://github.com/DraconDev/dracon-spark-and-director
dracon-terminal-engine          → https://github.com/DraconDev/dracon-terminal-engine
dracon-utilities                → https://github.com/DraconDev/dracon-utilities
Junk-Runner-bevy                → https://github.com/DraconDev/Junk-Runner-bevy
opencode-auto-continue          → https://github.com/DraconDev/opencode-auto-continue
opencode-auto-review-completed-todos → https://github.com/DraconDev/opencode-auto-review-completed-todos
tiles                           → https://github.com/DraconDev/tiles
video-uploader                  → https://github.com/DraconDev/video-uploader
DraconDev (meta-repo)           → https://github.com/DraconDev/DraconDev
```

Non-git repos (skipped): sqlite-embedded-continuous-wal-backup-to-object-storage, video-factory

## Post-Migration Checklist

- [x] Verify LICENSE appears on GitHub repo page (shows as AGPL-3.0)
- [x] Verify PRs require CLA check to pass (CLA-pr job in cla.yml)
- [x] Update each project's `package.json` / `Cargo.toml` with SPDX license identifier
- [ ] Add `SPDX-License-Identifier: AGPL-3.0` to all Rust `lib.rs` / `main.rs` file headers (optional — not yet done)
- [ ] Announce migration in project docs / release notes (optional)

## One-Shot Bash (from DraconDev root)

```bash
# Clone all repos, add license files, commit, push
# Adjust paths as needed

REPOS=(
  "~/Dev/ai-auto-writer"
  "~/Dev/ai-vid-editor"
  "~/Dev/azumi"
  "~/Dev/browser-extensions-shared"
  "~/Dev/dracon-code"
  "~/Dev/dracon-demons"
  "~/Dev/dracon-libs"
  "~/Dev/dracon-platform"
  "~/Dev/dracon-rust-ui"
  "~/Dev/dracon-spark-and-director"
  "~/Dev/dracon-terminal-engine"
  "~/Dev/dracon-utilities"
  "~/Dev/Junk-Runner-bevy"
  "~/Dev/opencode-auto-continue"
  "~/Dev/opencode-auto-review-completed-todos"
  "~/Dev/tiles"
  "~/Dev/video-uploader"
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