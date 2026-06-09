# External Platform Update Snippets

**Date:** 2026-06-09
**Purpose:** Provide exact copy/templates for the remaining external platform updates identified in `CROSS_PLATFORM_ACTION_PLAN.md`. These snippets are ready to paste once the relevant platform access is available.

**Ko-fi positioning note:** Ko-fi is **not** part of the default premium YouTube funnel. It can stay on `dracon.uk` as optional low-friction support, but it should not lead YouTube channel nav or descriptions unless we are deliberately selling small-ticket products.

**README toggle note:** `@isair` uses collapsed `<details>` toggles to hide secondary project categories. This is a good pattern, but it is future-only for DraconDev; the current README is already short and concrete.

**Utilities monorepo note:** `DraconDev/dracon-utilities` can stay as the simple parent repo. Make `dracon-sync`, `dracon-system`, and `dracon-warden` distinct through component READMEs, config examples, service/hook docs, and profile subpath links. Do not split into standalone repos unless explicitly requested later.

## 1. GitHub Profile Sidebar

**Asset:** GitHub profile sidebar fields: name, bio, website, company, location.
**Recommended fields:**
- **Name:** `DraconDev`
- **Bio:** `Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io.`
- **Website:** `https://dracon.uk`
- **Company:** `DraconDev`
- **Location:** only if public and accurate; do not add just for SEO

**Why these fields:** The sidebar is the first context a visitor sees before the README. It should reinforce the README, not duplicate it.

**Verification:** Screenshot or exported profile text showing the sidebar fields.

---

## 2. GitHub Profile Bio

**Asset:** GitHub profile bio field
**Recommended copy:**
> Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io.

**Why this copy:** Name-drops the killer work and leads with scale. It follows the mitsuhiko pattern: short, specific, and anchored to the most impressive artifact.

**Verification:** Screenshot or exported profile text showing the new bio.

---

## 3. GitHub Sponsors Page Bio

**Asset:** `github.com/sponsors/DraconDev`
**Recommended copy:**
> 239K+ lines of Rust, 5,600+ tests, 24 crates on crates.io, 12 in-scope repos per the audit.

**Why this copy:** Concrete stats make the page credible. This is the dtolnay pattern: the bio does the heavy lifting once the visitor is already warm.

**Verification:** Screenshot or exported page text showing the new bio.

---

## 4. `dracon.uk` Footer Copy

**Asset:** Site footer or global nav
**Recommended copy:**
> Support what I build: [GitHub Sponsors](https://github.com/sponsors/DraconDev) · [Ko-fi](https://ko-fi.com/adamdracon)

**Why this copy:** Ko-fi belongs on the site, not the README. The site is the right place for product/tip traffic.

**Verification:** Rendered site screenshot or deployed URL showing the footer link.

---

## 5. YouTube Channel Nav Links

**Asset:** YouTube Studio → Customization → Basic info → Links
**Recommended links:**
1. `https://dracon.uk`
2. `https://github.com/DraconDev`
3. `https://github.com/sponsors/DraconDev`
4. Discord, if active

Ko-fi is intentionally **not** included by default because the current YouTube positioning is premium subscriptions, not small tips.

**Why this copy:** This is the Theo pattern: a compact ecosystem map focused on premium products and support.

**Verification:** Channel page screenshot or YouTube Studio export showing the links.

---

## 6. YouTube Description Template

**Asset:** Video description template
**Recommended structure:**
```text
[1-2 line hook about the specific problem solved in this video]

Thank you [Sponsor] for sponsoring! Check them out at: [sponsor link]

[2-3 paragraphs of context, source links, or notes]

🔖 Topics Covered
- [Topic 1]
- [Topic 2]

Find more of my stuff on: https://dracon.uk
Support the OSS work: https://github.com/sponsors/DraconDev
```

**Why this structure:** Sponsor at the top, social links at the bottom, recurring CTA at the end. This is the Fireship/Theo pattern without making Ko-fi lead the premium funnel.

**Verification:** Published video description screenshot or exported description text.

---

## 7. `DraconDev/dracon-platform` README (Future Only)

**Asset:** Platform repo README, only if/when `dracon-platform` becomes public-facing.
**Status:** WIP; do not add yet.
**Recommended decision:** Do **not** add Ko-fi to the profile README while the platform is WIP. If the platform repo later has a real landing page, public demo, pricing, or install path, add a low-friction support/product link there.
**Why:** The profile README is for current, tangible proof. A WIP platform repo should not dilute the 4-repo pin list or create a broken funnel.
**Verification needed:** Public repo URL, rendered README screenshot, or deployed landing page showing the platform has enough substance to support a Ko-fi/product CTA.

---

## 8. `DraconDev/dracon-utilities` Monorepo Release Snippet

**Asset:** Parent monorepo release and profile README links.
**Status:** Release-readiness gate; do not add to profile README until public, clean, and verified.
**Recommended structure:** Keep `dracon-utilities` as the simple parent repo; make these components distinct:
- `dracon-sync` — Git auto-commit, multi-mirror daemon
- `dracon-system` — disk/process monitoring and cleanup
- `dracon-warden` — git encryption, secret scanning, repo hardening

**Candidate profile README snippet after public release:**
```markdown
**Infrastructure**
• [dracon-sync](https://github.com/DraconDev/dracon-utilities/tree/main/dracon-sync) — Git auto-commit, multi-mirror
• [dracon-system](https://github.com/DraconDev/dracon-utilities/tree/main/dracon-system) — system monitor, SSH, notifications
• [dracon-warden](https://github.com/DraconDev/dracon-utilities/tree/main/dracon-warden) — encryption, team keys, secret scanning
```

**Release gate:** public repo, clean secret scan, subcomponent READMEs, passing component/full workspace tests, and verified links before adding to the profile README. Current local gate: `dracon-system`, `dracon-warden`, and `dracon-sync` integration tests pass; full workspace tests are blocked by `dracon-sync` unit-test failures.

---

## 9. Collapsed `<details>` README Toggles (Future Only)

**Asset:** Profile README secondary-category toggles.
**Status:** Future-only; do not add now.
**Pattern source:** `@isair` uses `<details><summary>...</summary>` toggles to hide secondary project categories while keeping the first screen short.
**Candidate future toggle:**
```markdown
<details>
  <summary>Products / AI tools</summary>

  - [SamAI](https://dracon.uk) — AI browser companion, BYOK.
  - [rust-ai-web-auto](https://dracon.uk) — AI-driven browser automation.

</details>
```
**Why not now:** The current README is already short and concrete. The trending-developer audit found many profiles write too much, so we should not turn the README into a second portfolio.
**Verification needed:** Only add a toggle after the linked products have public, tangible surfaces strong enough to support the profile funnel.

---

## 10. Content Cadence / Format

**Asset:** Publishing workflow
**Recommended cadence:** Weekly shorts or weekly live streams
**Recommended format:** Short-form hooks + long-form deep-dives

**Why this works:** Consistency plus retention. Short-form hooks attract; long-form deep-dives retain.

**Verification:** Published video schedule or content calendar.

---

## 11. Execution Order

1. GitHub profile sidebar
2. GitHub profile bio
3. GitHub Sponsors page bio
4. `dracon.uk` footer
5. `DraconDev/dracon-platform` README, only after public release
6. `DraconDev/dracon-utilities` monorepo release, only after public/clean/verified
7. Collapsed `<details>` README toggles, only if future secondary categories need hiding
8. YouTube channel nav
9. YouTube description template
10. Content cadence / format

**Reason:** Start with the assets that convert warm visitors first (GitHub profile + Sponsors page), then expand the funnel outward (site footer, YouTube nav, descriptions, cadence).
