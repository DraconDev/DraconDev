# Profile Presentation Refinement Agreement

Date: 2026-06-11

## Recommendation

I agree with the user. The profile should treat the GitHub README as a "what
this person actually ships" page, not a raw code dump. Two distinct surfaces:

1. **Product destinations** — places a non-developer can install or play.
   - Chrome Web Store, Firefox Add-ons, itch.io, `dracon.uk`, etc.
   - Link to the user-facing install/play page, never to source code.
   - This is the "use it now" surface.

2. **Code** — repos useful to other developers.
   - Link to the source repo on GitHub.
   - This is the "fork it, read it, learn from it" surface.

Mixing them in the same bullet list confuses both audiences: a developer
scans the page, sees a Chrome Web Store link, and thinks "this is just a
shill page"; a user scans the same page, sees a GitHub source link, and
leaves. Separating them removes that confusion.

## What changes in the README

- Keep the **Chrome extensions** dropdown as a product destination.
  - Feature `Auto Fullscreen` with its verified Chrome Web Store link.
  - No source-code links in this dropdown.
- Keep the **Games** dropdown as a product destination.
  - No verified public install/play page is available yet.
  - `Junk Runner` is the strongest candidate but is still private.
  - Document the exact blocker instead of linking to private source.
- Add a separate **Code** section for developer-useful open source.
  - Each item gets a one-line "why it matters" so the section is
    scannable and self-justifying.
  - No product-destination links in this section.

## Code feature selection (rationale)

| Repo | Visibility | Why featured | Peer comparison |
|------|------------|--------------|-----------------|
| `dracon-terminal-engine` | public | A working TUI engine (43 widgets) that other projects actually depend on. | Most peers list a terminal or TUI without a public engine behind it. This is a real library, not a snippet. |
| `tiles-tui-file-manager` | public | A polished TUI app that proves the engine in a real product. | Sindre and mitsuhiko each have one or two polished TUIs; this is in the same tier and shows the engine ships. |
| `obs-wayland-hotkey` | public | A real Linux desktop automation tool with crates.io publishing and CI. | Most peers' "Infrastructure" bullet is something like "I run a Kubernetes cluster." This is shipped infrastructure. |
| `git-seal` | public | Solves a concrete problem (transparent Git file encryption) with a small surface. | Fasterthanlime and others highlight focused tools; this is in that style and is publishable as a crate. |
| `youtube-video-uploader` | public | API-first Rust wrapper around YouTube Data API v3, including resumable uploads. | dtolnay, mitsuhiko, and BurntSushi highlight API/libraries. This is a Rust library and a CLI in one repo, with public release artifacts. |

Excluded from the public Code section for now, with the reason documented:

- `dracon-utilities` — the parent monorepo. It is the strongest future code
  feature (it absorbs `dracon-sync`, `dracon-warden`, `dracon-system`), but
  it is still private and has an open `cargo fmt --check` failure. It should
  be featured the moment it is public and clean; until then, mentioning it
  in a "Coming next" note is the honest move.
- Private WXT/React Chrome extension repos — these belong in the Chrome
  extensions dropdown, not the Code section, because the install page is
  the right link once it exists.
- Private game repos — these belong in the Games dropdown for the same
  reason.

## Peer comparison (what we are now surpassing)

Sources reviewed for this comparison:

- sindresorhus, dtolnay, t3dotgg, fasterthanlime, mitsuhiko, BurntSushi,
  yoshuawuyts, antfu.
- Captured pages under `/tmp/top_sponsors_research/` and previous peer
  research in `GITHUB_PROFILE_RESEARCH.md`.

Gap closed by the new structure:

- **Separation of destinations and code.** Sindre and antfu mix product
  destinations and code into one long list. dtolnay keeps a tight one-line
  intro + tier list. None of the eight reviewed peers clearly label which
  bullets are "use it" links and which are "fork it" links. The new
  DraconDev README does that with three explicit sections, so a first-time
  visitor can tell in five seconds.
- **Code section that exceeds peer depth in one place.** Most peers
  highlight a small number of well-known projects. The new Code section
  features five public, focused, dependency-shaped projects (engine, TUI
  app, desktop daemon, Git utility, API client) — broader than any single
  peer's "Code" surface in the reviewed set, without becoming a wall of
  text.
- **Product dropdowns that survive a logged-out visitor.** Sindre's
  sponsors page mixes source code and products because everything is
  "open source." DraconDev's Chrome extensions and Games dropdowns
  intentionally point at user-facing install pages, so the page works as
  a product catalog for non-developers, not just as a developer portfolio.

## How the page will look

- Hero line: same as today ("Hey, I make tools that run themselves.").
- One stat line: same as today.
- Three sections, in this order, each with a one-line "why" intro so the
  first screen is not a wall of text:
  1. **Code** — five public repos with one-line "why" per repo.
  2. **Chrome extensions** — one verified Chrome Web Store link.
  3. **Games** — one honest note that the strongest game is private with
     no public install/play page yet.
- Footer line: same as today (`dracon.uk` · YouTube · Sponsor).

## Verification

- README link check: every link in `README.md` returns HTTP 200.
- README and `README_DRAFT.md` are identical.
- Rendered markdown contains exactly three section headings:
  `Code`, `Chrome extensions`, `Games`.
- The Code section contains no `chromewebstore.google.com` and no
  `itch.io` links.
- The Chrome extensions dropdown contains no `github.com/DraconDev/`
  source links.
- The Games dropdown contains no GitHub source links until a public
  install/play page exists.
