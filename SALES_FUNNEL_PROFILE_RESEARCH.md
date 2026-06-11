# Sales Funnel Profile Research

Date: 2026-06-11

## Short answer

Yes. A number of public GitHub profiles use the profile as a sales funnel, not just as a code portfolio. The strongest examples are course creators, training platforms, and company founders who make the destination explicit in the bio or profile README.

## Maker/product-seller lens

DraconDev is not primarily an educator, so the most relevant examples are not the course creators. The best fit is the **product/company funnel** pattern:

- `rauchg` → Vercel: the profile routes visitors to a product/company destination.
- `amix` → Doist: the profile bio identifies the founder role and routes visitors to the company site.
- `feross` → Socket: the profile bio identifies the founder/CEO role and points to a security product, though the endpoint was bot-blocked in this run.

For DraconDev, the profile should not say "I teach" or copy educator/course language. It should say, in effect: **I make real tools; inspect the code if you want, buy/use the finished product if you want.**

The strongest DraconDev-specific pattern is therefore:

1. **Product/company destination clarity** — route buyers/users to `dracon.uk` or a specific product page.
2. **Shipped-tool proof** — show public, useful code as evidence that the maker ships real things.
3. **Separated product vs code paths** — Chrome extensions and games go to install/play pages; code goes to GitHub source repos.

Course-funnel examples like Wes Bos, Kent C. Dodds, Brad Traversy, and Frontend Masters teachers are useful only as structural inspiration: short bio, clear destination, single CTA. DraconDev should not borrow their educator wording unless the business actually becomes course-led.

## Classification summary

| Bucket | Count |
|---|---:|
| Strong sales funnel | 9 |
| Soft funnel | 6 |
| Portfolio-first | 2 |
| No funnel | 1 |

## Strong sales funnel examples

### wesbos

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile meta description says: "I create courses that make learning things like JavaScript easy and fun 🔥"; website is wesbos.com.
- **Funnel path:** GitHub profile → wesbos.com/courses
- **Monetization destination:** Wes Bos courses
- **Why it is a funnel:** The profile bio explicitly sells courses and the website destination is the course catalog.
- **DraconDev advice:** Copy the direct "I create courses" style only if DraconDev wants a course-led funnel. Otherwise, keep the profile README product/code split clean.
- **Endpoint:** https://wesbos.com/courses (200) — Courses - Wes Bos
- **Evidence files:** /tmp/sales_funnel_profiles/wesbos/profile.html, /tmp/sales_funnel_profiles/wesbos/github_api.json, /tmp/sales_funnel_profiles/wesbos/endpoint.html, /tmp/sales_funnel_profiles/wesbos/endpoint_status.txt, /tmp/sales_funnel_profiles/wesbos/profile_readme.md

### kentcdodds

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile meta description says: "EpicAI.pro EpicWeb.dev EpicReact.dev"; website is kentcdodds.com.
- **Funnel path:** GitHub profile → kentcdodds.com
- **Monetization destination:** EpicAI / EpicWeb / EpicReact
- **Why it is a funnel:** The profile bio names the products directly and the site is the destination for them.
- **DraconDev advice:** Copy the product-name funnel if DraconDev wants to promote dracon.uk products directly from the profile bio.
- **Endpoint:** https://kentcdodds.com/ (200) — Kent C. Dodds
- **Evidence files:** /tmp/sales_funnel_profiles/kentcdodds/profile.html, /tmp/sales_funnel_profiles/kentcdodds/github_api.json, /tmp/sales_funnel_profiles/kentcdodds/endpoint.html, /tmp/sales_funnel_profiles/kentcdodds/endpoint_status.txt, /tmp/sales_funnel_profiles/kentcdodds/profile_readme.md

### getify

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile HTML shows worksFor Getify Solutions and the website getify.me.
- **Funnel path:** GitHub profile → getify.me → Frontend Masters course page
- **Monetization destination:** Frontend Masters courses
- **Why it is a funnel:** The profile points to a course destination and the endpoint page is a course catalog.
- **DraconDev advice:** Copy only if DraconDev wants to route visitors to a course or training page; otherwise keep the profile focused on code and product destinations.
- **Endpoint:** https://frontendmasters.com/teachers/kyle-simpson/ (200) — Learn from Kyle Simpson's courses | Frontend Masters
- **Evidence files:** /tmp/sales_funnel_profiles/getify/profile.html, /tmp/sales_funnel_profiles/getify/github_api.json, /tmp/sales_funnel_profiles/getify/endpoint.html, /tmp/sales_funnel_profiles/getify/endpoint_status.txt

### bradtraversy

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile meta description says: "Full stack web developer and online instructor"; website is traversymedia.com.
- **Funnel path:** GitHub profile → traversymedia.com/store
- **Monetization destination:** Traversy Media courses/store
- **Why it is a funnel:** The profile bio and endpoint both point to paid learning content.
- **DraconDev advice:** Copy the "online instructor + store" pattern if DraconDev wants to sell courses or tutorials directly.
- **Endpoint:** https://www.traversymedia.com/store (200) — Traversy Media | Learn Web Development
- **Evidence files:** /tmp/sales_funnel_profiles/bradtraversy/profile.html, /tmp/sales_funnel_profiles/bradtraversy/github_api.json, /tmp/sales_funnel_profiles/bradtraversy/endpoint.html, /tmp/sales_funnel_profiles/bradtraversy/endpoint_status.txt

### unclebob

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile HTML shows worksFor Uncle Bob Consulting LLC and pinned repo desc mentions cleancoders.com.
- **Funnel path:** GitHub profile → cleancoders.com
- **Monetization destination:** Clean Coders training videos / consulting
- **Why it is a funnel:** The profile routes visitors to a training site, not just a code portfolio.
- **DraconDev advice:** Copy the training-site funnel if DraconDev wants to sell learning content. Keep the profile README short and destination-led.
- **Endpoint:** https://cleancoders.com/ (200) — Clean Coders : Level up your code.
- **Evidence files:** /tmp/sales_funnel_profiles/unclebob/profile.html, /tmp/sales_funnel_profiles/unclebob/github_api.json, /tmp/sales_funnel_profiles/unclebob/endpoint.html, /tmp/sales_funnel_profiles/unclebob/endpoint_status.txt

### rauchg

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile HTML shows worksFor Vercel; endpoint is vercel.com.
- **Funnel path:** GitHub profile → Vercel product site
- **Monetization destination:** Vercel AI Cloud / product platform
- **Why it is a funnel:** The profile is a company/product portal, not just a code dump.
- **DraconDev advice:** Copy only if DraconDev wants to funnel to dracon.uk as a product site. Otherwise, keep the README split between product destinations and code.
- **Endpoint:** https://vercel.com/ (200) — Vercel: Build and deploy the best web experiences with the AI Cloud
- **Evidence files:** /tmp/sales_funnel_profiles/rauchg/profile.html, /tmp/sales_funnel_profiles/rauchg/github_api.json, /tmp/sales_funnel_profiles/rauchg/endpoint.html, /tmp/sales_funnel_profiles/rauchg/endpoint_status.txt, /tmp/sales_funnel_profiles/rauchg/profile_readme.md

### 1Marc

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile HTML shows worksFor @FrontendMasters and website marcgrabanski.com.
- **Funnel path:** GitHub profile → Frontend Masters teacher page
- **Monetization destination:** Frontend Masters courses
- **Why it is a funnel:** The profile is tied to a course platform and the endpoint is a course catalog.
- **DraconDev advice:** Copy the platform-funnel pattern if DraconDev wants to route visitors to a course or learning platform.
- **Endpoint:** https://frontendmasters.com/teachers/marc-grabanski/ (200) — Learn from Marc Grabanski's courses | Frontend Masters
- **Evidence files:** /tmp/sales_funnel_profiles/1Marc/profile.html, /tmp/sales_funnel_profiles/1Marc/github_api.json, /tmp/sales_funnel_profiles/1Marc/endpoint.html, /tmp/sales_funnel_profiles/1Marc/endpoint_status.txt

### amix

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile meta description says: "Founder/CEO of @Doist"; endpoint is doist.com.
- **Funnel path:** GitHub profile → Doist company site
- **Monetization destination:** Doist / Todoist / Twist
- **Why it is a funnel:** The profile bio identifies the founder role and routes visitors to the company site.
- **DraconDev advice:** Copy the founder/company funnel if DraconDev wants to route visitors to dracon.uk as the product/company destination.
- **Endpoint:** https://doist.com/ (200) — Doist: The remote company behind Todoist & Twist
- **Evidence files:** /tmp/sales_funnel_profiles/amix/profile.html, /tmp/sales_funnel_profiles/amix/github_api.json, /tmp/sales_funnel_profiles/amix/endpoint.html, /tmp/sales_funnel_profiles/amix/endpoint_status.txt, /tmp/sales_funnel_profiles/amix/profile_readme.md

### btholt

- **Classification:** Strong sales funnel
- **Profile evidence:** Profile meta description says: "teacher on @FrontendMasters"; endpoint is a Frontend Masters teacher page.
- **Funnel path:** GitHub profile → Frontend Masters teacher page
- **Monetization destination:** Frontend Masters courses
- **Why it is a funnel:** The profile bio and endpoint both point to a course platform.
- **DraconDev advice:** Copy the teacher/platform funnel if DraconDev wants to sell learning content through a platform.
- **Endpoint:** https://frontendmasters.com/teachers/brian-holt/ (200) — Learn from Brian Holt's courses | Frontend Masters
- **Evidence files:** /tmp/sales_funnel_profiles/btholt/profile.html, /tmp/sales_funnel_profiles/btholt/github_api.json, /tmp/sales_funnel_profiles/btholt/endpoint.html, /tmp/sales_funnel_profiles/btholt/endpoint_status.txt, /tmp/sales_funnel_profiles/btholt/profile_readme.md

## Soft funnel / portfolio-first / no funnel examples

### cassidoo

- **Classification:** Soft funnel
- **Profile evidence:** Profile README says: "I have a weekly newsletter ... Patreon ... livestream ... blog".
- **Funnel path:** GitHub profile → cassidoo.co/newsletter → Patreon / blog / apps
- **Monetization destination:** Newsletter, Patreon, blog, apps
- **Why it is or is not a funnel:** The profile promotes a newsletter and community, but the funnel is softer than a direct sales page.
- **DraconDev advice:** Adapt the newsletter/community pattern if DraconDev wants a low-friction funnel into YouTube or dracon.uk.
- **Endpoint:** https://cassidoo.co/ (200) — Cassidy Williams
- **Evidence files:** /tmp/sales_funnel_profiles/cassidoo/profile.html, /tmp/sales_funnel_profiles/cassidoo/github_api.json, /tmp/sales_funnel_profiles/cassidoo/endpoint.html, /tmp/sales_funnel_profiles/cassidoo/endpoint_status.txt, /tmp/sales_funnel_profiles/cassidoo/profile_readme.md

### colbyfayock

- **Classification:** Soft funnel
- **Profile evidence:** Profile README says: "Astrocoder, Developer Experience Engineer ... educational content ... newsletter".
- **Funnel path:** GitHub profile → colbyfayock.com/newsletter → egghead.io courses
- **Monetization destination:** Newsletter / egghead courses
- **Why it is or is not a funnel:** The profile is content-led, with a newsletter and course destination, but it is not a hard sell on the profile page.
- **DraconDev advice:** Adapt the educational-content funnel if DraconDev wants to route visitors from GitHub to YouTube or a newsletter.
- **Endpoint:** https://egghead.io/q/resources-by-colby-fayock (200) — Courses from Colby Fayock | egghead.io
- **Evidence files:** /tmp/sales_funnel_profiles/colbyfayock/profile.html, /tmp/sales_funnel_profiles/colbyfayock/github_api.json, /tmp/sales_funnel_profiles/colbyfayock/endpoint.html, /tmp/sales_funnel_profiles/colbyfayock/endpoint_status.txt, /tmp/sales_funnel_profiles/colbyfayock/profile_readme.md

### mbeaudru

- **Classification:** Soft funnel
- **Profile evidence:** Profile meta description says: "Github Modern JS Cheatsheet author"; endpoint is the cheatsheet site.
- **Funnel path:** GitHub profile → modern-js-cheatsheet
- **Monetization destination:** Free cheatsheet / community
- **Why it is or is not a funnel:** The profile routes visitors to a free resource, which can become a lead magnet but is not a hard sale.
- **DraconDev advice:** Adapt the free-resource funnel if DraconDev wants to attract developers before pitching dracon.uk products.
- **Endpoint:** https://mbeaudru.github.io/modern-js-cheatsheet/ (200) — Modern JavaScript Cheatsheet | Modern JS Cheatsheet
- **Evidence files:** /tmp/sales_funnel_profiles/mbeaudru/profile.html, /tmp/sales_funnel_profiles/mbeaudru/github_api.json, /tmp/sales_funnel_profiles/mbeaudru/endpoint.html, /tmp/sales_funnel_profiles/mbeaudru/endpoint_status.txt

### troyhunt

- **Classification:** Soft funnel
- **Profile evidence:** Profile meta description is minimal; endpoint is troyhunt.com.
- **Funnel path:** GitHub profile → troyhunt.com
- **Monetization destination:** Security blog / personal brand
- **Why it is or is not a funnel:** The profile is more of a personal-brand funnel than a direct sales funnel.
- **DraconDev advice:** Use only as a contrast case: personal-brand funnels are softer and less conversion-focused than DraconDev needs.
- **Endpoint:** https://www.troyhunt.com/ (200) — Troy Hunt: Troy Hunt
- **Evidence files:** /tmp/sales_funnel_profiles/troyhunt/profile.html, /tmp/sales_funnel_profiles/troyhunt/github_api.json, /tmp/sales_funnel_profiles/troyhunt/endpoint.html, /tmp/sales_funnel_profiles/troyhunt/endpoint_status.txt

### benawad

- **Classification:** Soft funnel
- **Profile evidence:** Profile meta description says: "Voidpet cofounder • ex-youtuber"; endpoint is benawad.com.
- **Funnel path:** GitHub profile → benawad.com
- **Monetization destination:** Personal site / Voidpet
- **Why it is or is not a funnel:** The profile hints at a business and a creator brand, but the funnel is not explicit.
- **DraconDev advice:** Use as a soft-funnel contrast case, not a template for DraconDev.
- **Endpoint:** https://benawad.com/ (200) — Ben Awad
- **Evidence files:** /tmp/sales_funnel_profiles/benawad/profile.html, /tmp/sales_funnel_profiles/benawad/github_api.json, /tmp/sales_funnel_profiles/benawad/endpoint.html, /tmp/sales_funnel_profiles/benawad/endpoint_status.txt

### feross

- **Classification:** Soft funnel
- **Profile evidence:** Profile meta description says: "Founder + CEO of Socket (@SocketDev)"; endpoint is socket.dev but bot-blocked.
- **Funnel path:** GitHub profile → socket.dev
- **Monetization destination:** Socket security product
- **Why it is or is not a funnel:** The profile points to a company/product, but the endpoint is bot-blocked in this run, so the funnel is only partially verifiable.
- **DraconDev advice:** Do not copy unless DraconDev can keep the destination page accessible and clearly product-led.
- **Endpoint:** https://www.socket.dev/ (403) — Just a moment...
- **Evidence files:** /tmp/sales_funnel_profiles/feross/profile.html, /tmp/sales_funnel_profiles/feross/github_api.json, /tmp/sales_funnel_profiles/feross/endpoint.html, /tmp/sales_funnel_profiles/feross/endpoint_status.txt, /tmp/sales_funnel_profiles/feross/profile_readme.md

### addyosmani

- **Classification:** Portfolio-first
- **Profile evidence:** Profile README says: "I’m currently working on AI at Google (Gemini, Agents, Vertex)" and endpoint is addyosmani.com.
- **Funnel path:** GitHub profile → personal site
- **Monetization destination:** Personal site / Google AI
- **Why it is or is not a funnel:** The profile is more of a professional portfolio than a sales funnel.
- **DraconDev advice:** Use as a contrast case: strong personal brand, weak sales funnel.
- **Endpoint:** https://addyosmani.com/ (200) — AddyOsmani.com
- **Evidence files:** /tmp/sales_funnel_profiles/addyosmani/profile.html, /tmp/sales_funnel_profiles/addyosmani/github_api.json, /tmp/sales_funnel_profiles/addyosmani/endpoint.html, /tmp/sales_funnel_profiles/addyosmani/endpoint_status.txt, /tmp/sales_funnel_profiles/addyosmani/profile_readme.md

### joshwcomeau

- **Classification:** Portfolio-first
- **Profile evidence:** Profile meta description says: "Software developer. Makes stuff."; endpoint is joshwcomeau.com.
- **Funnel path:** GitHub profile → personal site
- **Monetization destination:** Personal site / courses
- **Why it is or is not a funnel:** The profile itself does not sell; the funnel is implicit rather than explicit.
- **DraconDev advice:** Use as a contrast case: portfolio-first profiles are less effective for direct conversion.
- **Endpoint:** https://www.joshwcomeau.com/ (200) — Josh W. Comeau
- **Evidence files:** /tmp/sales_funnel_profiles/joshwcomeau/profile.html, /tmp/sales_funnel_profiles/joshwcomeau/github_api.json, /tmp/sales_funnel_profiles/joshwcomeau/endpoint.html, /tmp/sales_funnel_profiles/joshwcomeau/endpoint_status.txt

### jason

- **Classification:** No funnel
- **Profile evidence:** Profile has no bio and no clear destination; endpoint is a CodeTV series page.
- **Funnel path:** GitHub profile → CodeTV series page
- **Monetization destination:** None visible
- **Why it is or is not a funnel:** The profile does not clearly route visitors anywhere; it is effectively a no-funnel case.
- **DraconDev advice:** Avoid this pattern for DraconDev; the profile should do more than sit there.
- **Endpoint:** https://codetv.dev/series/learn-with-jason/s8 (200) — Learn With Jason Season 8
- **Evidence files:** /tmp/sales_funnel_profiles/jason/profile.html, /tmp/sales_funnel_profiles/jason/github_api.json, /tmp/sales_funnel_profiles/jason/endpoint.html, /tmp/sales_funnel_profiles/jason/endpoint_status.txt

## Top patterns to copy

1. **Product/company funnel.** Rauchg, Amir Salihefendic, and Feross route visitors from GitHub to a product/company destination. This is the best fit for DraconDev because the goal is to get people to buy/use tools, not to enroll in courses.
2. **Shipped-tool proof.** DraconDev's own public tools (`dracon-terminal-engine`, `tiles-tui-file-manager`, `obs-wayland-hotkey`, `git-seal`, `youtube-video-uploader`) prove that the maker ships real things.
3. **Product-destination dropdowns.** Chrome extensions and games should go to install/play pages, not source repos.

## Top patterns to avoid

1. **Educator/course-first funnel.** Wes Bos, Kent C. Dodds, Brad Traversy, and Frontend Masters teachers are useful only as structural inspiration. Do not copy their educator wording unless DraconDev actually sells courses.
2. **A vague personal brand with no destination.** Josh Comeau and Addy Osmani are strong personal brands, but the profile itself does not sell anything clearly.
3. **Repo dump as funnel.** A long list of repos without a buy/use destination makes the visitor do the sales work.

## DraconDev implications

- DraconDev should lead as a maker/product seller, not as an educator: **make tools, present them clearly, direct buyers/users to `dracon.uk` or a specific product page.**
- The cleanest sales funnel is a short hero line + one obvious destination: `dracon.uk` or a specific product page.
- The profile should not mix code and product links in the same bullet list; keep the Code section for developer-useful repos and the product dropdowns for install/play pages.
- A newsletter or community funnel is a softer option. That pattern fits a low-friction path into YouTube or a mailing list, but it is less direct than a product/tool funnel.
- The strongest pattern for DraconDev is likely a hybrid of Rauchg / Amir Salihefendic / Feross for product destination clarity plus DraconDev's own shipped-tool proof.

## Evidence directory

- Raw profile pages, endpoint pages, and API dumps are under `/tmp/sales_funnel_profiles`.
- Scoring sheet: `/home/dracon/Dev/DraconDev/sales_funnel_profile_scores.json`.
- Final recommendations: `/home/dracon/Dev/DraconDev/SALES_FUNNEL_RECOMMENDATIONS.md`.
