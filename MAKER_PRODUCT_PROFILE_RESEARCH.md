# Maker/Product-Seller Profile Research

Date: 2026-06-11

## Purpose

Research public GitHub/profile examples that match DraconDev's maker/product-seller angle better than the current awkward README form. The target pattern is:

> Shipped tools, product destinations, short proof points, clear CTAs, and code as supporting evidence — without sounding like a forced sales funnel.

Evidence was captured under `/tmp/maker_profile_research_goal/examples/` with GitHub user JSON, optional profile README, endpoint status/title, and top public repos.

## Summary

The strongest external pattern is not a long README. It is a compact identity + destination + proof model:

1. **Identity** — founder/maker/tool builder.
2. **Destination** — product/company/personal site in bio, company field, or README.
3. **Proof** — shipped artifact names, user counts, stars, packages, tests, or concrete tool descriptions.
4. **Code path** — source repos only when they are useful to developers.
5. **No forced funnel language** — the CTA is obvious, but the profile does not beg people to buy.

For DraconDev, the current README already has the right separation: product destinations first, code second. The research supports one refinement: remove defensive wording like "finished tools, not source code" and use more natural labels such as "Use / buy finished tools" and "Inspect public code".

## Examples

### 1. `rauchg` → Vercel

- **Evidence:** GitHub user JSON: company `https://vercel.com`; endpoint `https://vercel.com/` returned `200` with title "Vercel: Build and deploy the best web experiences with the AI Cloud".
- **Profile shape:** No profile README captured; the GitHub profile routes through company field/company destination.
- **Pattern:** Product/company destination clarity.
- **Fit for DraconDev:** High. Shows that a profile does not need a long pitch when the product destination is obvious.
- **Adaptation:** Keep `dracon.uk` as the main product destination; do not over-explain the funnel.

### 2. `amix` → Doist

- **Evidence:** Bio "Founder/CEO of @Doist"; blog `https://doist.com/`; endpoint returned `200` with title "Doist: The remote company behind Todoist & Twist".
- **Profile shape:** Short founder/company identity + company site.
- **Pattern:** Founder/company funnel without README clutter.
- **Fit for DraconDev:** High. Founder/product-seller identity can sit in the profile/sidebar while the README proves shipped work.
- **Adaptation:** Do not claim founder status unless true; use "I make tools" and route to `dracon.uk`.

### 3. `feross` → Socket / open-source product credibility

- **Evidence:** Bio says "Founder + CEO of Socket", "Started @webtorrent and @standard", "100+ open source packages on npm"; profile README is a minimal package README; `feross.org` returned `200`.
- **Profile shape:** Bio carries product/company identity; README is not a sales page.
- **Pattern:** Product identity + open-source credibility.
- **Fit for DraconDev:** Medium-high. The bio style is relevant, but DraconDev should not rely only on bio because the GitHub profile README is the main editable presentation surface.
- **Adaptation:** Keep proof line and public code section; do not copy Socket wording.

### 4. `yyx990803` → VoidZero / Vue / Vite

- **Evidence:** Bio "Founder @voidzero-dev, author of @vuejs and @vitejs"; endpoint `evanyou.me` returned `200`; no profile README captured.
- **Profile shape:** Strong artifact name-drops in bio; no README needed.
- **Pattern:** Name-drop shipped artifacts.
- **Fit for DraconDev:** High. Specific artifacts are stronger than generic "developer" language.
- **Adaptation:** "Rust terminal engine, Git utilities, video workflows" is better than broad "software projects".

### 5. `fabpot` → Symfony / Upsun

- **Evidence:** Bio "Founder and project lead at Symfony; CTO at Upsun"; endpoint returned `200`; no profile README captured.
- **Profile shape:** Founder/project-lead identity + company/product context.
- **Pattern:** Product/company context in bio, not README.
- **Fit for DraconDev:** Medium. Useful for sidebar/bio positioning, but less directly applicable because DraconDev's main products are not yet represented by a single named company destination.
- **Adaptation:** Use `dracon.uk` as the destination until product pages are ready.

### 6. `mcollina` → Platformatic / Fastify

- **Evidence:** Bio "@platformatic Co-Founder & CTO, TSC member @nodejs, Lead Maintainer @fastify"; endpoint `nodeland.dev` returned `200`; no profile README captured.
- **Profile shape:** Maintainer/founder credibility with product/company context.
- **Pattern:** Credibility through ecosystem roles.
- **Fit for DraconDev:** Medium-high. DraconDev has credible Rust/tooling work, but should avoid overstating ecosystem authority.
- **Adaptation:** Use objective proof: Rust lines, tests, crates, shipped repos.

### 7. `alyssaxuu` → Screenity / many shipped products

- **Evidence:** Bio says founder of Screenity and Product Hunt maker recognition; profile README lists products with proof such as "over 180K users", "over 1000 copies sold", Product Hunt placements.
- **Profile shape:** Product list with proof points.
- **Pattern:** Shipped product proof.
- **Fit for DraconDev:** Medium. The proof style is useful, but the full README is too long and product-list-heavy for DraconDev's first screen.
- **Adaptation:** Keep product proof short; use one proof line and collapsible product details.

### 8. `zenorocha` → Resend

- **Evidence:** Bio "Founder & CEO @ Resend"; endpoint `zenorocha.com` returned `200`; no profile README captured.
- **Profile shape:** Clean founder/product identity.
- **Pattern:** Founder/product destination clarity.
- **Fit for DraconDev:** High. Short identity plus product destination is exactly the desired profile direction.
- **Adaptation:** Keep the README short; let `dracon.uk` carry product depth.

### 9. `steipete` → OpenClaw / PSPDFKit

- **Evidence:** Bio mentions OpenAI, OpenClaw, and "Ex-PSPDFKit Founder"; profile README has a "Start Here" list with product/tool links and star counts.
- **Profile shape:** Product/tool list with proof.
- **Pattern:** Start-here list for warm visitors.
- **Fit for DraconDev:** Medium. The "Start Here" pattern is useful, but the README is too dense for DraconDev's first screen.
- **Adaptation:** Use a short "Inspect public code" list, not a giant tool catalog.

### 10. `geerlingguy` → Jeff Geerling

- **Evidence:** Profile README says "I'm an open source developer with a YouTube channel... my Internet home is my website"; includes GitHub Sponsors and Patreon CTAs.
- **Profile shape:** Short personal site/support CTA.
- **Pattern:** Low-friction destination + support path.
- **Fit for DraconDev:** Medium. Not a product-seller example, but the short README and support CTA are useful.
- **Adaptation:** Keep `dracon.uk`, YouTube, Sponsor footer; do not add Patreon unless it becomes real.

### 11. `tomnomnom` → tool maker

- **Evidence:** Bio "Open-source tool maker, trainer, talker, fixer, eater"; endpoint `tomnomnom.com` returned `200`; no profile README captured.
- **Profile shape:** Maker identity without hard selling.
- **Pattern:** Natural maker tone.
- **Fit for DraconDev:** High. "Tool maker" language fits DraconDev better than educator/course language.
- **Adaptation:** Use "I make tools" / "I build..." rather than "sales funnel" wording.

### 12. `stormzhang` → AI builder

- **Evidence:** Bio "Full-stack developer, Productor, AI Builder"; profile README says "AI Builder" and links `stormzhang.ai`; top repos include `token-tracker`, `ai-translate`, and `ipcheck`.
- **Profile shape:** Maker identity + home page + tool repos.
- **Pattern:** Builder identity + home page.
- **Fit for DraconDev:** Medium. Relevant as a maker/AI-builder profile, but less useful structurally because it is content-heavy and partially non-English.
- **Adaptation:** Keep DraconDev's English, product-first README; use concrete tool descriptions.

## What to avoid

1. **Overlong product catalogs.** `alyssaxuu` and `steipete` show that proof can work, but too many bullets push DraconDev back into "trying too hard" territory.
2. **Course-funnel language.** The course examples from earlier research are structurally useful only. Do not use "I teach", "learn from me", or "course" language unless that becomes the actual business.
3. **Defensive source-code disclaimers.** "Finished tools, not source code" is accurate, but it sounds defensive. Better: separate product and code sections with natural labels.
4. **Unverified product claims.** Do not add product pages, install pages, or "public product" status unless verified.
5. **Repo dumps.** Profiles with many repos but no product destination make visitors do the sales work.

## Recommended DraconDev direction

Keep the current product-first / code-second structure, but make the language more natural:

- Replace "Use or buy the finished tools on dracon.uk" with a less defensive product-destination sentence.
- Replace "**Use it / buy it** — finished tools, not source code." with "**Use / buy finished tools**".
- Replace "**Inspect the code** — public Rust/tooling that proves the products are real." with "**Inspect public code** — developer-useful Rust/tooling that backs the products."

This keeps the proven structure while reducing the awkward funnel feel.

## Verification artifacts

- `/tmp/maker_profile_research_goal/summary.json` — concise endpoint/readme/bio summary for 12 examples.
- `/tmp/maker_profile_research_goal/examples/<login>/github_api.json` — GitHub user JSON for each example.
- `/tmp/maker_profile_research_goal/examples/<login>/top_public_repos.json` — public repos for each example.
- `/tmp/maker_profile_research_goal/examples/<login>/endpoint_status.txt` — endpoint status/title when available.
- `/tmp/maker_profile_research_goal/examples/<login>/profile_readme.md` — profile README when available.
