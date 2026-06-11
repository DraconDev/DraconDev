# Maker/Product-Seller Profile Positioning

Date: 2026-06-11

## Core positioning

**DraconDev makes real tools, presents them clearly, and directs the right people to buy or use them.**

This is not an educator-first profile. The profile should not sound like:

- a course catalog,
- a teaching portfolio,
- a newsletter-first brand,
- a generic open-source maintainer page,
- a wall of repos with no product destination.

The profile should sound like a maker/product seller:

- I build tools.
- The tools solve concrete problems.
- You can inspect the code if you want.
- You can buy/use the product if you want the finished thing.
- The strongest proof is shipped work, not promises.

## What DraconDev is selling

The profile should make the product path obvious:

1. **Chrome extensions** → Chrome Web Store install pages.
2. **Games** → public play/install pages once verified.
3. **Tools/platforms** → `dracon.uk` or a specific product page once ready.
4. **Code** → GitHub source repos, but only as proof of capability and developer usefulness.

Code is supporting evidence. It is not the funnel unless the target visitor is another developer who wants to fork/read the implementation.

## What to avoid

- Do not lead with "I teach" unless a course is genuinely the product.
- Do not copy educator/course funnels like Wes Bos, Kent C. Dodds, or Frontend Masters unless DraconDev is actually selling learning content.
- Do not make the profile a repo dump.
- Do not mix source-code links and product install pages in the same section.
- Do not promote private repos, private games, or unverified install pages as if they are public products.
- Do not use vague claims like "building the future" without a shipped tool or product destination.

## Profile patterns to copy

### Copy: product/company funnel

Best examples from the research:

- `rauchg` → Vercel
- `amix` → Doist
- `feross` → Socket

Why this fits DraconDev:

- The profile points to a product or company destination.
- The visitor understands where to go next.
- The GitHub profile acts as credibility, not as the whole pitch.

How DraconDev should adapt it:

- Keep the hero line short.
- Point to `dracon.uk` or a specific product page.
- Use GitHub to prove the work is real: shipped repos, tests, crates, stars, or technical details.

### Copy: shipped-tool proof

Examples from DraconDev's own audit:

- `dracon-terminal-engine` — TUI engine with 43 widgets.
- `tiles-tui-file-manager` — TUI app that ships the engine.
- `obs-wayland-hotkey` — Linux desktop automation tool.
- `git-seal` — focused Git encryption utility.
- `youtube-video-uploader` — Rust wrapper around YouTube Data API v3.

Why this fits DraconDev:

- It proves DraconDev makes actual stuff.
- It gives developers a reason to trust the product path.
- It keeps the profile concrete and tangible.

How DraconDev should adapt it:

- Feature only public, clean, developer-useful code.
- Give each code item a one-line "why it matters" reason.
- Keep product destinations separate from code links.

### Copy: product-destination dropdowns

Examples from the current DraconDev README form:

- **Chrome extensions** dropdown → Chrome Web Store install pages.
- **Games** dropdown → public play/install pages once verified.

Why this fits DraconDev:

- It separates "use it now" from "inspect the code."
- It prevents product visitors from landing on source repos.
- It keeps the first screen short while preserving useful details.

How DraconDev should adapt it:

- Only link install/play/store pages.
- Never link private source repos as product destinations.
- Document unverified products honestly.

## Recommended README shape

The README should use this order:

1. **Hero** — maker/product-seller positioning.
   - Example: "I make tools that run themselves. Buy/use them on dracon.uk."
2. **Proof line** — shipped tools, tests, crates, or product categories.
   - Example: "Rust terminal engines, Linux desktop tools, Git utilities, and browser extensions."
3. **Products / Use it** — Chrome extensions, games, tools on `dracon.uk`.
4. **Code / Inspect it** — strongest developer-useful open source with one-line rationale.
5. **Footer** — `dracon.uk`, YouTube, Sponsor.

The exact section labels can vary, but the principle should not: **product destinations first for buyers/users, code second for developers/inspectors.**

## Recommended tone

Use:

- "I make tools..."
- "I build infrastructure..."
- "Shipped..."
- "Install..."
- "Use..."
- "Buy..."
- "Public..."
- "Verified..."
- "Source is here if you want to inspect it."

Avoid:

- "I teach..."
- "Learn from me..."
- "Course..."
- "Master..."
- "Sign up for my class..."
- "Follow my learning journey..."
- "Check out my tutorials..."

YouTube can remain in the footer as a trust/awareness channel, but the profile should not be framed as a teaching funnel unless that becomes the actual business.

## Success criterion

The profile is working if a first-time visitor can answer these three questions in under ten seconds:

1. What does DraconDev make?
2. Where do I buy or use it?
3. Where can I inspect the code if I care?

If the answer is unclear, the profile is still too much like a repo dump or educator portfolio and not enough like a maker/product-seller page.
