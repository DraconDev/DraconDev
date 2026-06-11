# Maker/Product-Seller README Recommendation

Date: 2026-06-11

## Decision

Apply a small live README refinement.

The research supports the current product-first / code-second structure, but not the current defensive wording. The README should remain short, concrete, and tangible; the change should remove awkward funnel language without adding unverified claims.

## Evidence supporting the change

- `rauchg`, `zenorocha`, and `amix` show that product/company destination clarity can be short and natural.
- `yyx990803` and `feross` show that artifact name-drops and proof beat generic positioning.
- `alyssaxuu` and `steipete` show that product proof can become too dense if copied directly.
- `tomnomnom` shows that "tool maker" language feels natural and does not need sales-copy phrasing.
- The current README already separates product destinations from source code, so the structure should stay.

## Files to change

- `README.md`
- `README_DRAFT.md`

## Exact changes

1. Change:

```markdown
I build Rust infrastructure, terminal UX, Git utilities, and video workflows. Use or buy the finished tools on [dracon.uk](https://dracon.uk).
```

To:

```markdown
I build Rust infrastructure, terminal UX, Git utilities, and video workflows. Finished products and install pages are on [dracon.uk](https://dracon.uk).
```

2. Change:

```markdown
**Use it / buy it** — finished tools, not source code.
```

To:

```markdown
**Use / buy finished tools**
```

3. Change:

```markdown
**Inspect the code** — public Rust/tooling that proves the products are real.
```

To:

```markdown
**Inspect public code** — developer-useful Rust/tooling that backs the products.
```

## What not to change yet

- Do not add a "Tools on dracon.uk" dropdown until specific product pages are verified.
- Do not link private browser extensions or games as public products.
- Do not promote `dracon-utilities` until it is public and `cargo fmt --check` is clean.
- Do not add educator/course language.
- Do not add unverified product claims.

## Validation required after edit

- `README.md` and `README_DRAFT.md` must be identical.
- All public links must return HTTP 200.
- Rendered markdown must still contain Chrome extensions, Games, Auto Fullscreen, Junk Runner, and the code sections.
- No product-source mixing: Chrome extensions/games must not link to GitHub source.
- No secret/token/cookie leakage.
