# DraconDev

Founder at [Dracon](https://dracon.uk). I build small tools for the browser and
small libraries for Rust, and I ship all of them.

**20+ browser extensions** for Chrome and Firefox, **10+ Rust crates** on
crates.io, plus games, music and AI experiments at [dracon.uk](https://dracon.uk).

Everything below is free and open source. Start with the one you need.

<details open><summary>🤖 pi coding-agent extensions — 4 + monorepo</summary>

| Extension | Description |
|:----------|:------------|
| [pi-goal-list-loop-audit](https://github.com/DraconDev/pi-goal-list-loop-audit) | supervised long-running work — drafting, queues, loops, with a detached auditor that can't be bamboozled by the implementing agent |
| [pi-use-last-selected-thinking-level](https://github.com/DraconDev/pi-use-last-selected-thinking-level) | per-model thinking-level memory for pi — picks up where you left off, no settings.json edits |
| [pi-length-continue](https://github.com/DraconDev/pi-length-continue) | continue past output-length stops without losing the thread |
| [pi-search-skill](https://github.com/DraconDev/pi-search-skill) | unlimited, local, key-less web search via search-daemon (Rust) + SearXNG |
| [pi-plugins](https://github.com/DraconDev/pi-plugins) | the factory — plugins, skills, and extensions monorepo |

</details>

<details><summary>📦 Browser extensions — 21 Chrome · 21 Firefox</summary>

*Counts verified live against both stores on 2026-10-22. If they look stale,
they are: check the Chrome Web Store detail pages and the AMO API before
trusting them.*

| Category | Extension | Description |
|:---------|:----------|:------------|
| 🤖 AI | [SamAI](https://chromewebstore.google.com/detail/samai-summarize-chat-fill/lkdhbmlacakmgjenicbbccdkalojifpk) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/samai-smart-ai-assistant/) | summarize, chat, fill forms (BYOK) |
| 🤖 AI | [AI Auto Form Filler](https://chromewebstore.google.com/detail/ai-auto-form-filler-byok/jeaepppjlimcnobjmnlkbckebljhjjkp) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/ai-auto-form-filler-byok/) | form filling with your own API keys |
| 🤖 AI | [AI Search](https://addons.mozilla.org/en-US/firefox/addon/ai-search-ask-ai-address-bar/) | ask AI from the address bar, streaming answers with clickable results (Firefox only) |
| 🤖 AI | [YouTube Dislike](https://chromewebstore.google.com/detail/youtube-dislike-from-comm/kclenebickklgodlbedflhgebpgbdbca) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/youtube-dislike-audience-score/) | AI comment sentiment, 100% local |
| 🛒 Shopping | [Amazon Price Drop Alerts](https://chromewebstore.google.com/detail/amazon-price-drop-alerts/popfipiodhmfnbkmaegidkjbkmkhcjgc) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/amazon-price-drop-alerts/) | drop alerts, price history, verdicts and a fake-deal flag (local-first, no account) |
| 🔒 Privacy & focus | [BugKit](https://chromewebstore.google.com/detail/bugkit-visual-bug-reporti/ocglmnhgcocgcamdeallgieoeeoikafi) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/bugkit-visual-bug-reporting/) | privacy-first bug reporting dashcam |
| 🔒 Privacy & focus | [Custom History](https://chromewebstore.google.com/detail/custom-history-%E2%80%94-auto-cle/bddofaibhlkfnnkeooplfpejngbbgkpa) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/custom-history-auto-clean/) | rule-based history curation |
| 🔒 Privacy & focus | [Better Search](https://chromewebstore.google.com/detail/better-search-for-google/cpljcfobifeockaahnainecafkfgfobo) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/better-search-for-google/) | block content farms from Google |
| 🔒 Privacy & focus | [De-Addict](https://chromewebstore.google.com/detail/de-addict-the-social-medi/camilemalhpinkpchjocnigfdfdeggno) | progressive habit retraining (Chrome only) |
| 🛠️ Developer | [Live Reload Pro](https://chromewebstore.google.com/detail/live-reload-pro/joaoebgmkklomlblgldjllebjkglbmfl) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/live-reload-pro/) | configurable live reload for any web dev stack |
| 🛠️ Developer | [API Debugger](https://chromewebstore.google.com/detail/api-debugger-capture-debu/cmajbkmfgikmenkdnanbfacbhgolamkl) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/api-debugger-capture/) | inspect every HTTP request, no account needed |
| 🛠️ Developer | [Full Page Screenshot](https://chromewebstore.google.com/detail/full-page-screenshot-%E2%80%94-ca/fknopaaneooekhedmmogkciacfikkeil) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/full-page-screenshot-record/) | capture, annotate, and redact screenshots locally |
| 🎨 Appearance | [Cinematic Pages](https://chromewebstore.google.com/detail/cinematic-pages-%E2%80%94-visual/gpifhefjbccffjdmiaocolikdgcjbmoo) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/cinematic-pages-visual-effects/) | 48 visual effects: rain, snow, CRT, cyberpunk, neon, letterbox |
| 🎨 Appearance | [Dark Mode](https://chromewebstore.google.com/detail/dark-mode-custom-themes/fbamfbffccllbihhnklfpemnghnconhf) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/dark-mode-custom-themes/) | dark mode that keeps images natural, 11 themes, per-site |
| 🎨 Appearance | [Cursor Style](https://chromewebstore.google.com/detail/cursor-style-custom-curso/ihhnelanlghhflpgfmbmhdphkgjldele) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/custom-cursors-animated-trails/) | custom cursors with animated trails |
| 🎨 Appearance | [Dracon Black Red](https://chromewebstore.google.com/detail/dracon-black-red/eehkcojepeohgjnfnjgdfmocofjllbki) | almost pure dark with red tint (Chrome theme) |
| 📑 Tabs & browser | [Dracon New Tab](https://chromewebstore.google.com/detail/dracon-new-tab/bjfiegeaofpnpenfdhenngmikmbkhphp) | customizable new tab page (Chrome only) |
| 📑 Tabs & browser | [AI New Tab](https://addons.mozilla.org/en-US/firefox/addon/ai-new-tab-page-search/) | cinematic wallpapers, AI answers, quick links and a clock (Firefox only) |
| 📑 Tabs & browser | [Bookmarks New Tab](https://chromewebstore.google.com/detail/bookmarks-new-tab-page/kbmjpgekdcimbfaijegojcnhlhonmmbf) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/bookmarks-new-tab-page-dial/) | dark bookmark dashboard |
| 📑 Tabs & browser | [Auto Fullscreen](https://chromewebstore.google.com/detail/auto-fullscreen-fullscree/liddomimembhlpgehhkhefijckmhkkjn) | fullscreen any page (Chrome only) |
| 🎬 Media & creator | [VidPro](https://chromewebstore.google.com/detail/vidpro-video-setup-assist/igpkmfpooacbdidccnacoehjoinnjfoi) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/vidpro-ai-titles-descriptions/) | AI titles, descriptions and tags inside YouTube Studio |
| 🎬 Media & creator | [VidPro v1.13](https://addons.mozilla.org/en-US/firefox/addon/vidpro-video-setup-assistant/) | the previous VidPro listing, still live on Firefox |
| 🎬 Media & creator | [Volume Video Master](https://chromewebstore.google.com/detail/volume-video-master-1000%25/heahbpeacbdcoigcnmiloddggjncheml) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/volume-video-master-1000/) | advanced audio and video controls, 1000% volume |
| 🎬 Media & creator | [Startype](https://chromewebstore.google.com/detail/startype-typing-practice/ljmcpchnoanbbdanegpjbpepgffhbcno) · [Firefox](https://addons.mozilla.org/en-US/firefox/addon/startype-typing-tutor-wpm-game/) | typing practice and WPM game |
| 🦑 Novelty | [MyTentacles](https://addons.mozilla.org/en-US/firefox/addon/mytentacles/) | tentacles, for people who like tentacles (Firefox only) |

</details>

<details><summary>🦀 Crates on crates.io — 11</summary>

| Category | Crate | Description |
|:---------|:------|:------------|
| 🤖 AI & contracts | [dracon-ai-contracts](https://lib.rs/crates/dracon-ai-contracts) | AI routing contracts |
| 🤖 AI & contracts | [dracon-ai-runtime-contracts](https://lib.rs/crates/dracon-ai-runtime-contracts) | AI runtime contracts |
| 🤖 AI & contracts | [dracon-tts-runtime](https://lib.rs/crates/dracon-tts-runtime) | text-to-speech runtime |
| 🔐 Security & encryption | [dracon-security](https://lib.rs/crates/dracon-security) | secret scanning and age-based encryption primitives |
| 🔐 Security & encryption | [dracon-warden](https://lib.rs/crates/dracon-warden) | Git filter encryption and repo hardening |
| 📁 File & system | [dracon-files](https://lib.rs/crates/dracon-files) | file, media, and developer tooling libraries |
| 📁 File & system | [dracon-system-lib](https://lib.rs/crates/dracon-system-lib) | system diagnostics, SSH, and notification adapters |
| 📁 File & system | [dracon-system](https://lib.rs/crates/dracon-system) | local disk, process, storage, and service diagnostics |
| 🛠️ Git & dev tools | [dracon-git](https://lib.rs/crates/dracon-git) | reusable Git and developer tooling libraries |
| 🛠️ Git & dev tools | [dracon-sync](https://lib.rs/crates/dracon-sync) | invisible Git sync daemon |
| ⚙️ Terminal | [dracon-terminal-engine](https://lib.rs/crates/dracon-terminal-engine) | terminal compositor engine |

</details>

<details><summary>🛠️ Desktop tools — 2</summary>

- [obs-wayland-hotkey](https://github.com/DraconDev/obs-wayland-hotkey) — OBS hotkey daemon for Wayland/X11.
- [kiki-desktop-announcer](https://github.com/DraconDev/kiki-desktop-announcer) — sassy desktop announcer with local TTS and configurable personalities.

</details>

<details><summary>⚙️ Engines & demos — 4</summary>

- [dracon-terminal-engine](https://github.com/DraconDev/dracon-terminal-engine) — TUI engine for terminal apps.
- [azumi-live-ssr-framework](https://github.com/DraconDev/azumi-live-ssr-framework) — live SSR framework/product demo for Rust.
- [ai-gui-auto-video-editor](https://github.com/DraconDev/ai-gui-auto-video-editor) — AGAVE video editor demo.
- [tiles-tui-file-manager](https://github.com/DraconDev/tiles-tui-file-manager) — dual-pane TUI file manager with git, SSH, system monitor, and editor.

</details>

[🌐 dracon.uk](https://dracon.uk) · [🎥 YouTube](https://youtube.com/@DraconDev)
