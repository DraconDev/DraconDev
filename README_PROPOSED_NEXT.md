# Proposal-only README draft

This is a direction draft, not the live profile README. It previews where the profile is heading while keeping private/unpublished work clearly labeled.

---

# Hey, I make usable things people can run or play.

I build Chrome extensions, games, Rust infrastructure, terminal UX, Git utilities, creator workflows, and private control-plane tools. Finished products and install/play pages go in the details below; code is shown when it proves the work.

[Dracon crates on lib.rs](https://lib.rs/search?q=dracon)

When in doubt, let the stuff do the talking: shipped tools, install pages, public code, and honest notes for private work.

---

## Featured first: usable Chrome extensions and games

DraconDev mainly features usable things people can install, open, or play. Build-with foundations are secondary and stay below the product-facing items. Open only if you want install/play links. I do not link private source repos as public products.

<details><summary>Chrome extensions</summary>

- [**Auto Fullscreen**](https://chromewebstore.google.com/detail/auto-fullscreen-fullscree/liddomimembhlpgehhkhefijckmhkkjn) — fullscreen any page on load, navigation, or click-hold.
- Private browser extensions: no public install pages verified yet.

</details>

<details><summary>Games</summary>

- **Junk Runner** — private survival/trading roguelike; no public play page verified yet.

</details>

---

## Usable tools now

Things that solve a job directly.

- [**folder-auto-banner**](https://github.com/DraconDev/folder-auto-banner) — contextual directory dashboard; `ls` with git, TODO, ports, and build context.
- [**tiles-tui-file-manager**](https://github.com/DraconDev/tiles-tui-file-manager) — TUI file manager; ships the terminal engine.
- [**obs-wayland-hotkey**](https://github.com/DraconDev/obs-wayland-hotkey) — OBS hotkey daemon; public repo, local validation still to restore.
- [**youtube-video-uploader**](https://github.com/DraconDev/youtube-video-uploader) — Rust YouTube API client + CLI; resumable uploads.
- [**git-seal**](https://github.com/DraconDev/git-seal) — Git filter for transparent file encryption; `dracon-warden` is the stronger private successor.

---

## Private / on the way

Strong work that should become public only when it is clean, verifiable, and safe to link.

### Higher-priority private/on-the-way

- `pully-fully-pull-based-fleet-reconciler` — private fleet/control-plane work; stronger than it first appears because local validation passed `cargo fmt --check` and `cargo test --workspace`.
- `dracon-utilities` — private monorepo; not linked until public and `cargo fmt --check` is clean.
  - `dracon-warden` — private successor to `git-seal`; stronger Git encryption/security story once public.
  - `dracon-sync` — private Git sync / multi-mirror workflow tooling.
  - `dracon-system` — private system monitoring and maintenance tooling.

### Destination-gated or still private

- `rust-ai-web-auto` — private AI browser automation work; fmt passed, but no public destination yet.
- `dracon-code` — private autonomous engineering runtime work; fmt passed, but no public destination yet.
- `browser-extensions-shared` — private extension monorepo; large local codebase, but only verified public install pages should be featured.
- `avid` — private video automation product; no public destination verified; fmt still needs cleanup.
- `Junk Runner` — private game/product; web build passed, but no public play page verified.
- `one-mil-girls` — private game/product; no public play page verified.
- `ai-auto-writer` — private AI writing product; no public destination verified.

---

## Build-with foundations (secondary)

Libraries and frameworks for people building similar tools; useful, but not the main feature.

- [**dracon-terminal-engine**](https://github.com/DraconDev/dracon-terminal-engine) — TUI engine; 43 widgets for building terminal UX.
- [**dracon-libs**](https://github.com/DraconDev/dracon-libs) — reusable Rust libraries and runtimes; fmt passed, workspace tests blocked in this environment by missing `sqlite3` linker library.
- [**azumi-live-ssr-framework**](https://github.com/DraconDev/azumi-live-ssr-framework) — live SSR framework for Rust on Axum; ~10KB gzipped.
- [**ai-gui-auto-video-editor**](https://github.com/DraconDev/ai-gui-auto-video-editor) — AGAVE: AI-assisted GUI video editor; product demo, not first-screen library.

---

## What this is heading toward

- Usable Chrome extensions and games first.
- Usable tools next.
- Product destinations only when verified: Chrome Web Store pages, play pages, or install pages.
- Build-with foundations second: terminal engine, Rust libs, SSR framework, AI GUI demo.
- Destination-gated work stays out of product lists until public install/play pages exist.

[🌐 dracon.uk](https://dracon.uk) · [🎥 YouTube](https://youtube.com/@DraconDev) · [💰 Sponsor](https://github.com/sponsors/DraconDev)
