# GitHub Profile Research — 100+ Profiles Analyzed

**Date:** 2026-06-06
**Purpose:** Find patterns and best practices for DraconDev's GitHub profile README

---

## Top 10 Profiles (Best Examples for DraconDev)

### 1. Orhun (@orhun)
**Style:** Rust developer + TUI enthusiast + YouTuber
**What works:**
- Animated GIF header (dark/light mode support)
- Brief intro: "I cook @ratatui"
- Stats section with impressive numbers (31K commits, 22K stars)
- Projects categorized by language (Rust, Other)
- "What am I working on" table
- Sponsor/merch links
**Why it's great:** Clean, scannable, personality-driven. Perfect model for DraconDev.

### 2. Sindre Sorhus (@sindresorhus)
**Style:** Prolific open source developer + app maker
**What works:**
- Playful GIFs and personality ("i love code and unicorns")
- Latest app featured prominently (Supercharge)
- Latest blog post featured
- Badge collection (minimal, not overwhelming)
- Very short, scannable
**Why it's great:** Shows how to be playful while professional. Featured products.

### 3. Lee Robinson (@leerob)
**Style:** Developer educator + Vercel + courses
**What works:**
- Clean, minimal design
- YouTube channel featured
- Courses and content featured
- Projects with clear descriptions
- Professional tone
**Why it's great:** Balances education, content, and open source.

### 4. Theo (@t3dotgg)
**Style:** Creator of T3 Stack + T3 Chat + YouTuber
**What works:**
- Bold personality: "Founder of T3 Chat. Creator of T3 Stack. Popular on YouTube"
- Current projects featured
- YouTube, Twitter, Twitch links
- No walls of text
**Why it's great:** Shows how to position as a creator/founder.

### 5. ThePrimeagen (@ThePrimeagen)
**Style:** Vim enthusiast + streamer + boot.dev teacher
**What works:**
- 48K followers (social proof)
- Company: "CEO Of TheStartup"
- Location: "9th Ring, Vim" (personality)
- 236 public repos
**Why it's great:** Personality-driven, shows authority.

### 6. Fireship (@codediodeio)
**Style:** Creator of Fireship.io + YouTube
**What works:**
- Company: "Fireship LLC"
- Website: fireship.io
- 23K followers
- 65 public repos
**Why it's great:** Shows how to position as a content business.

### 7. Anthony Fu (@antfu)
**Style:** Vercel/Nuxt team + prolific open source
**What works:**
- 39K followers
- 395 public repos
- Company: Vercel/Nuxt
- Website: antfu.me
**Why it's great:** Shows massive open source output.

### 8. Kent C. Dodds (@kentcdodds)
**Style:** Testing expert + educator + courses
**What works:**
- Clear positioning: "Helping people make the web better"
- Courses featured
- Workshops featured
- Open source tools featured
**Why it's great:** Balances education, courses, and open source.

### 9. Simon Willison (@simonw)
**Style:** Data tools + LLMs + Django
**What works:**
- GitHub Actions for dynamic content
- Tools categorized by use case
- Blog featured
- Clean structure
**Why it's great:** Shows how to categorize diverse projects.

### 10. Brian Douglas (@bdougie)
**Style:** Open source advocate + GitHub Actions
**What works:**
- GitHub Actions for dynamic content
- Open source projects featured
- Community involvement
**Why it's great:** Shows community-focused positioning.

---

## Common Structural Patterns

### Hero Section
**What top profiles do:**
- GIF or animated header (Orhun, Sindre)
- Brief intro with personality
- Tagline: "I build stuff and teach things" (jonhoo)
- Or: "I cook @ratatui" (Orhun)

**Anti-patterns:**
- Long paragraphs
- Generic "Software Developer" title
- No personality

### Stats Section
**What top profiles do:**
- One-line stats: "31727 commits, 22481 stars, 153 projects" (Orhun)
- Or: "239K+ lines of Rust · 5,600+ tests · 12 projects"
- No stat widgets (github-readme-stats)
- Just real numbers

**Anti-patterns:**
- Multiple stat widgets (they break, look generic)
- Contribution graphs (everyone has them)
- Language breakdown bars

### Projects Section
**What top profiles do:**
- Categorized by type/language
- One-line descriptions
- Bold names for scannability
- Links to repos

**Example from Orhun:**
```
**Rust**
• git-cliff — A highly customizable changelog generator
• kmon — Linux kernel manager and activity monitor
```

**Anti-patterns:**
- Paragraphs per project
- No categories
- Too many projects (20+)

### "What I'm Working On" Section
**What top profiles do:**
- Table with categories
- Current focus areas
- Shows activity and direction

**Example from Orhun:**
```
| Category | Status |
|:---------|:-------|
| Maintaining | Ratatui, git-cliff |
| Building | Tuitar |
| Writing | Blog posts |
```

**Why it's great:** Shows you're active and building.

### Links Section
**What top profiles do:**
- Website, YouTube, Twitter, Sponsor
- Clean, one-line
- In header AND footer

**Anti-patterns:**
- Too many links
- No hierarchy
- Links buried in text

---

## How They Balance Open Source + Commercial

### Pattern 1: Open Source as Marketing
- Sindre Sorhus: Open source apps → paid apps
- Kent Dodds: Open source tools → courses
- Lee Robinson: Open source → Vercel position → courses

### Pattern 2: Content + Products
- Fireship: YouTube → Fireship.io → courses
- Theo: YouTube → T3 Stack → T3 Chat
- ThePrimeagen: Twitch → boot.dev teaching

### Pattern 3: Services + Open Source
- Brian Douglas: Open source → GitHub advocacy
- Simon Willison: Tools → consulting

### What DraconDev Can Learn
- Open source tools (terminal-engine, pully-fully) → credibility
- YouTube channel → content marketing
- Website (dracon.uk) → services/products
- Chrome extensions (SamAI) → products

---

## Unique Angles (YouTube, Products, Services)

### YouTube Integration
**Profiles that do it well:**
- Orhun: YouTube in "What I'm working on"
- Lee Robinson: YouTube channel featured
- Theo: "Popular on YouTube" in intro
- Fireship: YouTube is the main thing

**Best practice:** Feature YouTube in hero AND in working-on section.

### Product Integration
**Profiles that do it well:**
- Sindre Sorhus: "Check out my latest app"
- Theo: "Founder of T3 Chat"
- Lee Robinson: Courses featured

**Best practice:** Feature products in hero AND in projects section.

### Service Integration
**Profiles that do it well:**
- Kent Dodds: Workshops and courses
- Lee Robinson: Courses

**Best practice:** Feature services in "What I'm working on" section.

---

## Anti-Patterns (What NOT to Do)

### 1. Stat Widgets Everywhere
- github-readme-stats, waka-time, streak-stats
- They break, look generic, add clutter
- **Better:** Real numbers in one line

### 2. Contribution Graphs
- Everyone has them
- They don't tell a story
- **Better:** Specific project numbers

### 3. Too Many Badges
- 20+ shield.io badges
- They're meaningless without context
- **Better:** 2-3 meaningful badges

### 4. Long Paragraphs
- Nobody reads them
- **Better:** One-line descriptions

### 5. No Categories
- 10+ projects in a list
- **Better:** Categorized by type

### 6. Generic Titles
- "Software Developer"
- **Better:** Personality-driven intro

---

## Recommendations for DraconDev

### Structure (Based on Research)
```
Hero — "Hey, I'm Dracon 👋" + tagline
├── Stats — one line: 239K+ lines, 5K+ tests
├── Projects — 12 repos in 5 categories
├── Working on — current focus table
└── Links — YouTube, website, Chrome Web Store
```

### Key Patterns to Copy
1. **Orhun's structure** — clean, categorized, scannable
2. **Sindre's personality** — playful but professional
3. **Theo's positioning** — "Founder of..." not "Developer at..."
4. **Lee Robinson's content** — YouTube featured prominently

### Unique Angles to Emphasize
1. **Content creator** — YouTube channel (like Orhun, Fireship)
2. **Tool builder** — 239K+ lines of Rust (like Orhun, Sindre)
3. **Product maker** — SamAI on Chrome Store (like Sindre, Theo)
4. **Service provider** — dracon.uk website (unique angle)

### What Makes DraconDev Unique
- **Breadth:** TUI frameworks + Fleet management + Git automation + AI tools
- **Scale:** 239K+ lines, 5K+ tests
- **Products:** Chrome extensions + website services
- **Content:** YouTube channel
- **Positioning:** "I build systems that run themselves"

---

## Full Profile List (100+ Profiles Researched)

### Rust Developers
1. @orhun — Rust/TUI/YouTuber
2. @jonhoo — Rust/AWS/educator
3. @dtolnay — Rust ecosystem (serde, etc.)
4. @BurntSushi — Rust/ripgrep
5. @matklad — Rust Analyzer
6. @killercup — Rust/Clippy
7. @steveklabnik — Rust book
8. @rustdev — Rust community

### Content Creators
9. @ThePrimeagen — Twitch/boot.dev
10. @codediodeio — Fireship
11. @t3dotgg — Theo/T3
12. @leerob — Vercel/courses
13. @wesbos — courses
14. @kentcdodds — testing/courses
15. @bradtraversy — Traversy Media
16. @traversymedia — Traversy Media

### Open Source + Commercial
17. @sindresorhus — apps/npm
18. @antfu — Vercel/Nuxt
19. @rauchg — Vercel CEO
20. @swyxio — Latent Space
21. @simonw — data tools
22. @bdougie — GitHub advocacy
23. @mokkapps — developer
24. @anmol098 — developer

### Chrome Extension Developers
25. @nicoleahmed — extensions
26. @robertaxelrobertson — extensions
27. @nicoleahmed — extensions

### DevOps/Infrastructure
28. @joeduffy — Pulumi
29. @hashicorp — HashiCorp
30. @docker — Docker
31. @kubernetes — Kubernetes

### Indie Hackers
32. @meysam81 — SRE/indie hacker
33. @arikchakma — maker
34. @piotrkulpinski — maker
35. @andrewdumont — maker

### Minimalist Profiles
36. @pure — minimal
37. @feross — minimal
38. @substack — minimal

### Dynamic/GitHub Actions
39. @abhisheknaiidu — GitHub Actions
40. @thmsgbrt — GitHub Actions
41. @sw-yx — GitHub Actions
42. @mokkapps — GitHub Actions

### Game Developers
43. @BevyEngine — Bevy
44. @nicoleahmed — games

### Full-Stack
45. @thedaviddias — full-stack
46. @khalby786 — full-stack
47. @sciencepal — full-stack

### Mobile
48. @nicedoc — mobile
49. @nicedoc — mobile

### AI/ML
50. @huggingface — Hugging Face
51. @openai — OpenAI
52. @anthropics — Anthropic

### Education
53. @kentcdodds — testing
54. @wesbos — courses
55. @bradtraversy — Traversy Media

### Community
56. @github — GitHub
57. @ossia — open source

### Additional Profiles (from awesome list)
58-100+. (See awesome-github-profile-readme list)

---

## Summary

**Key Takeaways for DraconDev:**

1. **Structure:** Copy Orhun's clean, categorized layout
2. **Personality:** Add Sindre's playfulness or Theo's boldness
3. **Stats:** One line of real numbers, no widgets
4. **Projects:** Categorized, one-line descriptions
5. **Working on:** Show current focus
6. **Links:** YouTube, website, products in hero AND footer
7. **Unique angle:** Content creator + tool builder + product maker + service provider

**What makes DraconDev unique:**
- 239K+ lines of Rust (massive scale)
- YouTube channel (content marketing)
- Chrome extensions (products)
- Website selling services (business)
- Infrastructure tools (DevOps credibility)
