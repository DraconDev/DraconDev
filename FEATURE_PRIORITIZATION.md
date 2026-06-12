# Feature Prioritization

**Date:** 2026-06-12  
**Decision:** Feature DraconDev by business/product signal, not by job-seeking appeal.

## Feature rule

Feature what a buyer, user, or aligned technical partner can understand and act on quickly.

Do **not** feature what mainly says:

- “I would be a good employee.”
- “Look at all my repos.”
- “I can build anything.”
- “I am experimenting.”
- “This is impressive infrastructure” without a product/use path.

The feature order should answer:

1. What can I use, install, play, or buy?
2. What proves the maker can build serious systems?
3. What is private or unverified and should stay out of the public product path?

## Priority order

### 1. Verified public product destinations

**Feature first:** Chrome extensions and games only when there is a verified public install/play/store page.

Why:

- Highest business/product signal.
- Clearest user action.
- Separates product visitors from source-code visitors.
- Avoids pretending private work is public.

Current handling:

- `README.md` and `README_DRAFT.md` feature **Chrome extensions** and **Games** in details sections.
- `DIRECTION.md` says Chrome extensions and games are the main product destinations when verified.
- Private browser extensions and private games stay noted honestly, not promoted as public products.

Do:

- Feature Auto Fullscreen because it has a verified Chrome Web Store page.
- Feature a game only after a public play/install page is verified.
- Keep product links on install/play/store pages, not source repos.

Do not:

- Link private extension source as a product.
- Link private game source as a product.
- Claim public product status without a verified destination.

### 2. Productized browser and creator tools

**Feature next:** Browser extensions and creator tools with a clear revenue path.

Best candidates:

- SamAI — AI browser companion, if store/pricing claims are verified.
- vidpro-extension — YouTube Studio optimization.
- api-debugger — developer/browser debugging utility.
- bugkit — replayable bug evidence.
- auto-form-filler — AI form filling.

Why:

- Browser products are easy to install.
- Creator tools map naturally to YouTube distribution.
- Paid tiers, subscriptions, and one-time purchases are clearer than employment signals.
- These products can be demoed in short videos.

Feature wording:

- “Browser products with paid tiers when ready.”
- “Creator tools for faster YouTube workflows.”
- “Use or buy the finished product when it is ready.”

Do not:

- Overclaim traction or revenue.
- Say “founder of” unless the product/company structure is real.
- Treat unverified store status as public fact.

### 3. Usable tools with direct use paths

**Feature after product destinations:** Tools people can install, run, or use directly.

Current examples:

- `folder-auto-banner`
- `tiles-tui-file-manager`
- `obs-wayland-hotkey`
- `youtube-video-uploader`
- `git-seal`
- `dracon-utilities` only when public and clean

Why:

- Strong proof that DraconDev builds real systems.
- Useful to developers and technical buyers.
- Supports the product/business angle without becoming a repo dump.

Feature wording:

- “Usable tools — things that solve a job directly.”
- “Inspect public code if you want the proof.”

Do not:

- Make these the first feature ahead of verified products.
- Promote `dracon-utilities` until public and validation-clean.

### 4. Private product candidates

**Mention cautiously:** Private products should not be featured as public products until they are public, verified, and ready.

Examples:

- `dracon-platform`
- `dracon-code`
- `dracon-demons`
- `dracon-utilities`
- `pully-fully-pull-based-fleet-reconciler`
- `rust-ai-web-auto`
- `ai-auto-writer`
- `avid`

Why they matter:

- They may become the strongest business assets.
- They prove product thinking beyond public repos.
- They can become future launches.

How to feature them:

- “Private product candidate.”
- “Not promoted until public and verified.”
- “Coming when release-ready.”
- “Used as internal infrastructure/product seed.”

Do not:

- Link private source as a product.
- Claim pricing, users, revenue, or readiness without evidence.
- Put private products ahead of verified public products in public docs.

### 5. Build-with foundations

**Keep secondary:** Build-with foundations are useful, but they should not lead the profile.

Examples:

- `dracon-terminal-engine`
- `azumi-live-ssr-framework`
- `ai-gui-auto-video-editor`
- `dracon-libs`

Why secondary:

- They are impressive but less grab-and-run than products.
- They attract builders more than buyers/users.
- They support credibility, not the primary business signal.

Feature wording:

- “Build-with foundations (secondary).”
- “Useful if you are building similar tools, but not the main feature.”

Do not:

- Put foundations ahead of usable products.
- Treat libraries as the main sales path.
- Let impressive Rust work crowd out product destinations.

### 6. Games

**Feature only when public:** Games can be featured, but only with verified public play/install destinations.

Current state:

- Junk Runner is private; no public play page verified.
- one-mil-girls is private/local; no public play page verified.

Why this matters:

- Games fit “usable things people can play.”
- They are not the strongest near-term business wedge unless a public destination exists.
- Private games should be noted honestly, not promoted as public products.

Feature wording:

- “Games — public play/install pages when verified.”
- “Private game/product; no public play page verified yet.”

Do not:

- Link private game source as a product.
- Put private games ahead of verified browser products.

### 7. YouTube and support

**Feature as distribution/support:** YouTube and sponsors should support the product/business angle.

Why:

- YouTube can demo products and drive installs.
- GitHub Sponsors and Ko-fi can support development.
- They are not the main product path.

Feature wording:

- “YouTube — product demos and build proof.”
- “Support what I build: GitHub Sponsors / Ko-fi / dracon.uk.”

Do not:

- Make YouTube look like a course funnel.
- Make sponsors the main business signal.
- Add sponsor links where they dilute product destinations.

## What should be featured where

### README / public profile

Feature order:

1. Hero: usable things people can run or play.
2. Chrome extensions and games details.
3. Usable tools.
4. Build-with foundations (secondary).
5. Footer: `dracon.uk`, YouTube, Sponsor.

Do not feature:

- private products as public products,
- job-seeking language,
- freelance/service language,
- course language,
- repo dumps,
- unverified install/play pages.

### `dracon.uk`

Feature order:

1. Product pages.
2. Pricing or paid-tier paths.
3. Install/play/download links.
4. License pages.
5. Support/sponsor links.
6. Code/proof pages only where useful.

### GitHub profile

Feature order:

1. Product/business bio.
2. Website/product destination.
3. Pinned repos as proof, not as the main offer.
4. Sponsor/support as secondary.

Pinned repos should prove serious systems work. They should not replace product destinations.

### YouTube

Feature order:

1. Product demos.
2. Problem/solution stories.
3. Links to product pages.
4. GitHub proof when relevant.
5. Sponsor/support only when appropriate.

## Current active-doc alignment

| Doc | Alignment |
|:----|:----------|
| `README.md` | Aligned: usable Chrome extensions/games first, usable tools next, foundations secondary. |
| `README_DRAFT.md` | Aligned: mirrors `README.md`. |
| `DIRECTION.md` | Aligned: Chrome extensions/games as main product destinations when verified; foundations secondary. |
| `PROFILE_STRATEGY.md` | Aligned after updates: product/license/support funnel; SamAI treated as product work to verify, not overclaimed. |
| `PRODUCT_INVENTORY.md` | Aligned after updates: consulting removed; SamAI store status marked as claim requiring verification. |
| `STRATEGIC_ANGLE_DECISION.md` | Aligned: product/business angle chosen over job-seeking. |
| `CHASING_TWO_BIRDS.md` | Aligned: job-vs-business tension documented; public profile stays business-first. |
| `BUSINESS_POSITIONING_STRATEGY.md` | Aligned: non-employment revenue models and productized offers. |
| `SALES_FUNNEL_RECOMMENDATIONS.md` | Aligned after update: hero uses usable-product language, not old job-style tool-maker wording. |

## Hard constraints

- Verified product destinations beat impressive private code.
- Usable tools beat generic foundations.
- Productized revenue paths beat job-seeking appeal.
- Private/unverified work stays private until verified.
- Build-with foundations remain secondary.
- YouTube supports products; it is not the main product.
- GitHub proves capability; it is not the whole business.
- Do not feature anything only because it might impress an employer.

## Success criteria

This prioritization is complete when:

- The feature order is explicit.
- The order is justified by business/product signal, not job appeal.
- Chrome extensions and games are featured only when verified.
- Usable tools remain visible but do not crowd out products.
- Build-with foundations remain secondary.
- Private products are not promoted as public products.
- Active docs either already match this order or have been updated to match it.
