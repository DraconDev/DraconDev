# Sales Funnel Recommendations for DraconDev — Maker/Product-Seller Revision

Date: 2026-06-11

## Positioning correction

DraconDev is not primarily an educator. The profile should not sound like a course catalog or teaching portfolio.

The stronger positioning is:

> **DraconDev makes real tools, presents them clearly, and directs the right people to buy or use them.**

The profile should treat GitHub as credibility and proof. The product destination is `dracon.uk`, Chrome Web Store pages, or verified game install/play pages.

## Top 3 patterns to copy

1. **Product/company funnel** — Rauchg → Vercel, Amir Salihefendic → Doist, Feross → Socket.
   - Why it fits DraconDev: the profile points to a real product/company destination instead of asking visitors to infer the offer from repos.
   - DraconDev adaptation: make `dracon.uk` or a specific product page the obvious destination.

2. **Shipped-tool proof** — DraconDev's own public tools (`dracon-terminal-engine`, `tiles-tui-file-manager`, `obs-wayland-hotkey`, `git-seal`, `youtube-video-uploader`).
   - Why it fits DraconDev: concrete shipped code proves the maker is real without turning the profile into a course pitch.
   - DraconDev adaptation: keep a short Code section with one-line reasons why each repo matters.

3. **Product-destination dropdowns** — Chrome extensions and Games separated from code.
   - Why it fits DraconDev: buyers/users see install/play pages, while developers can inspect source without confusing the two paths.
   - DraconDev adaptation: never link source code where a public install/play/store page is the better destination.

## Top 3 patterns to avoid

1. **Educator/course-first funnel** — Wes Bos, Kent C. Dodds, Brad Traversy, Frontend Masters teachers.
   - Why to avoid: those patterns work when the product is teaching. DraconDev's product is tools.
   - DraconDev adaptation: use them only as structural inspiration (short bio + clear CTA), not as wording.

2. **Vague personal brand with no destination** — Josh Comeau, Addy Osmani-style portfolio-first profiles.
   - Why to avoid: strong personal brands can still fail to send buyers anywhere.
   - DraconDev adaptation: every profile section should point to either a product destination or a concrete proof point.

3. **Repo dump as funnel** — profiles that list many repos without explaining what is shipped or where to buy/use it.
   - Why to avoid: it makes the visitor do the sales work.
   - DraconDev adaptation: feature fewer repos, each with a one-line "why it matters" reason.

## Concrete README changes to consider later

- Lead with a maker/product-seller hero:
  - "I make tools that run themselves. Buy/use them on dracon.uk."
- Keep `dracon.uk` as the main product destination.
- Keep YouTube in the footer as a trust/awareness channel, not as the primary funnel.
- Keep **Chrome extensions** and **Games** as product-destination dropdowns.
- Keep **Code** as a separate proof section, not as the main sales path.
- Promote `dracon-utilities` only when it is public and clean.
- Promote private games or extensions only when verified public install/play pages exist.
- Remove any wording that implies DraconDev is primarily teaching, running courses, or selling learning content.

## Best-fit DraconDev pattern

The best fit is a hybrid of:

- **Rauchg / Amir Salihefendic / Feross** for product/company destination clarity.
- **DraconDev's own shipped-tool audit** for proof.
- **Current DraconDev README structure** for separating product destinations from source code.

The profile should answer three questions in under ten seconds:

1. What does DraconDev make?
2. Where do I buy or use it?
3. Where can I inspect the code if I care?

## Evidence

- Maker positioning note: `MAKER_PROFILE_POSITIONING.md`
- Research file: `SALES_FUNNEL_PROFILE_RESEARCH.md`
- Scores: `sales_funnel_profile_scores.json`
- Evidence directory: `/tmp/sales_funnel_profiles`
