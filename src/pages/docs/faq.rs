use crate::site::bundle_sizes;
use resuma::prelude::*;

const FAQ_LD: &[(&str, &str)] = &[
    (
        "What is resumability vs hydration?",
        "Hydration re-executes your entire component tree on the client to attach event listeners. Resumability serializes signals and handler references into HTML during SSR; the client resumes only what the user interacts with — no full-tree replay.",
    ),
    (
        "Does Resuma run Rust in the browser?",
        "No. Components always execute on the server. Client-side code is a fixed runtime (loader plus lazy core on first interaction) plus small handler chunks. Business logic stays in Rust.",
    ),
    (
        "How big is the client bundle?",
        "Pages with no signals or handlers ship zero Resuma runtime. Interactive pages parse HTML immediately with a ~1021 B gzip loader, then fetch core.js plus the handler chunk on the first click. See /docs/benchmark.",
    ),
    (
        "Do I need Node.js?",
        "Only if you rebuild the JS runtime from source. Prebuilt assets ship inside the resuma crate. For app development, Rust and cargo are enough.",
    ),
    (
        "Can I use Resuma without Flow?",
        "Yes. ResumaApp supports single-page apps with manual route registration. Flow adds multi-page routing, loaders, submits, and middleware when you need a full site.",
    ),
    (
        "Does Flow include a PWA?",
        "Yes — manifest and service worker are enabled by default on FlowApp. Use .without_pwa() or RESUMA_PWA=0 to disable. See /docs/flow/pwa.",
    ),
    (
        "How do forms work without JavaScript?",
        "The Form component renders a real HTML form with POST /_resuma/submit/:name. Progressive enhancement: the runtime intercepts submit when loaded, but forms work as plain POST without JS.",
    ),
    (
        "How do I paste docs into an AI?",
        "Every docs page has Copy page (article as markdown, with URL). The sidebar has Copy nav (full outline with links). Each section heading also has Copy.",
    ),
    (
        "How do I add SEO, GEO, and AEO?",
        "Use FlowApp::with_seo_kit, set SITE_URL to your public origin, and unique titles/descriptions per page. Resuma serves /robots.txt, /sitemap.xml, and /llms.txt. Guide: /docs/integrations/seo_geo.",
    ),
    (
        "How do I deploy?",
        "Resuma is a Docker image (or a Linux binary) plus env vars — not Vercel-style serverless. Guides: /docs/cookbook/deploy. Or resuma new my-app --template production.",
    ),
    (
        "How do I add a live theme switcher or popup menu?",
        "Use FlowApp::with_html_theme with HtmlTheme and ThemeSwitch or data-r-theme buttons inside a Popup. Do not put the click handler in a layout chunk. Document palettes live on html[data-theme]. Guides: /docs/cookbook/theme and /docs/components/popup.",
    ),
    (
        "How do I teach Cursor or Codex Resuma patterns?",
        "Run resuma install skill (use --force after a CLI upgrade). That copies SKILL.md with view!, Flow, HtmlTheme, overlays, SeoKit, and deploy rules. Guide: /docs/integrations/ai_assistant.",
    ),
    (
        "Is Resuma production-ready?",
        "Yes — 1.0 follows semver for public APIs. Security defaults (CSRF, CSP, rate limits) ship enabled. See /docs/security and STABILITY.md in the framework repo.",
    ),
    (
        "Where is the backend security reference?",
        "examples/todo — guards, DTO validation, service layer, authorization. Docs: /docs/security/todo.",
    ),
    (
        "Do I need Redis, Postgres, or Cloudflare Workers?",
        "No for core Resuma features. SSR, signals, server actions, CSRF, CSP, and rate limiting work with no external services. Resuma OS adds disk-backed queues without Redis. Optional: add Postgres via SQLx for your app data.",
    ),
];

pub fn page(_req: FlowRequest) -> View {
    set_page_json_ld(crate::site::json_ld_faq(&crate::site::site_url(), FAQ_LD));
    view! {
        <>
            <h1>"FAQ"</h1>
            <p class="lead">"Common questions about resumability, bundle size, and how Resuma compares to hydration-based frameworks."</p>

            <h2>"What is resumability vs hydration?"</h2>
            <p>"Hydration re-executes your entire component tree on the client to attach event listeners. Resumability serializes signals and handler references into HTML during SSR; the client resumes only what the user interacts with — no full-tree replay."</p>

            <h2>"Does Resuma run Rust in the browser?"</h2>
            <p>"No. Components always execute on the server. Client-side code is a fixed runtime (loader + lazy core on first interaction) plus small handler chunks translated from closures at compile time (rs2js in resuma-macros). Business logic stays in Rust."</p>

            <h2>"How big is the client bundle?"</h2>
            <p>
                "Pages with no signals or handlers ship zero Resuma runtime. Interactive pages parse HTML immediately with "
                <strong>{bundle_sizes::LOADER_GZIP.to_string()}</strong>
                " gzip loader.js, then fetch core.js ("
                {bundle_sizes::CORE_GZIP.to_string()}
                " gzip) plus the handler chunk on the first click — "
                <strong>{bundle_sizes::RESUMA_FIRST.to_string()}</strong>
                " total. Island chunks load on demand. See the "
                <a href="/docs/benchmark">"benchmark page"</a>
                " and "
                <a href="/docs/architecture">"architecture"</a>
                " for measured numbers."
            </p>

            <h2>"Do I need Node.js?"</h2>
            <p>"Only if you rebuild the JS runtime from source. Prebuilt assets ship inside the " <code>"resuma"</code> " crate (" <code>"assets/"</code> "). For app development, Rust + cargo (or " <code>"cargo install resuma"</code> ") is enough."</p>

            <h2>"Can I use Resuma without Flow?"</h2>
            <p>"Yes. ResumaApp supports single-page apps with manual route registration — ideal for counters, widgets, and embedded UI. Flow adds multi-page routing, loaders, submits, and middleware when you need a full site."</p>

            <h2>"Does Flow include a PWA?"</h2>
            <p>
                "Yes — manifest and service worker are enabled by default on " <code>"FlowApp"</code> ". "
                "Use " <code>".without_pwa()"</code> " or " <code>"RESUMA_PWA=0"</code> " to disable. "
                <a href="/docs/flow/pwa">"Details →"</a>
            </p>

            <h2>"How do forms work without JavaScript?"</h2>
            <p>"The " <code>"Form"</code> " component renders a real HTML form with " <code>"POST /_resuma/submit/:name"</code> ". Progressive enhancement: the runtime intercepts submit when loaded, but forms work as plain POST without JS."</p>

            <h2>"How do I paste docs into an AI?"</h2>
            <p>
                "Every docs page has " <strong>"Copy page"</strong>
                " (article as markdown, with URL). The sidebar has "
                <strong>"Copy nav"</strong>
                " (full outline with links). Each section heading also has "
                <strong>"Copy"</strong> "."
            </p>

            <h2>"How do I add SEO, GEO, and AEO?"</h2>
            <p>
                "Set " <code>"SITE_URL"</code> " to your public origin, call "
                <code>"FlowApp::with_seo_kit"</code> ", and give each page a unique title and description. "
                "Resuma serves " <code>"/robots.txt"</code> ", " <code>"/sitemap.xml"</code> ", and "
                <code>"/llms.txt"</code> ". Full guide: "
                <a href="/docs/integrations/seo_geo">"SEO, GEO & AEO"</a> "."
            </p>

            <h2>"How do I deploy?"</h2>
            <p>
                "Resuma is a Docker image (or a Linux binary) plus env vars — not Vercel-style serverless. "
                "Guides for Fly.io, DigitalOcean, AWS, Cloudflare, Railway, and Render: "
                <a href="/docs/cookbook/deploy">"Deploy"</a> ". "
                "Or " <code>"resuma new my-app --template production"</code> "."
            </p>

            <h2>"How do I add a live theme switcher or popup menu?"</h2>
            <p>
                "Use " <code>"FlowApp::with_html_theme"</code>
                " with " <code>"HtmlTheme"</code>
                " and " <code>"ThemeSwitch"</code>
                " or " <code>"data-r-theme"</code>
                " buttons inside a " <code>"&lt;Popup&gt;"</code>
                ". Do not put the click handler in a layout chunk — that JS is lazy. "
                "Document palettes live on " <code>"html[data-theme]"</code>
                ". Guides: "
                <a href="/docs/cookbook/theme">"Theme"</a>
                " and "
                <a href="/docs/components/popup">"Popup"</a> "."
            </p>

            <h2>"How do I teach Cursor or Codex Resuma patterns?"</h2>
            <p>
                "Run " <code>"resuma install skill"</code>
                " (use " <code>"--force"</code>
                " after a CLI upgrade). That copies " <code>"SKILL.md"</code>
                " with " <code>"view!"</code>
                ", Flow, " <code>"HtmlTheme"</code>
                ", overlays, " <code>"SeoKit"</code>
                ", and deploy rules. Guide: "
                <a href="/docs/integrations/ai_assistant">"AI assistant"</a> "."
            </p>

            <h2>"Is Resuma production-ready?"</h2>
            <p>
                "Yes — " <strong>"1.0"</strong> " follows semver for public APIs. Security defaults (CSRF, CSP, rate limits) ship enabled. "
                "Resuma OS adds self-hosted workers and ops. "
                "See " <a href="/docs/security">"Security"</a> ", "
                <a href="/docs/exec/ops">"Ops & production"</a> ", and "
                <a href="/docs/security/todo">"todo reference"</a> ", and "
                <a href="https://github.com/GoldevLab/resuma/blob/main/docs/STABILITY.md" target="_blank" rel="noopener">"STABILITY.md"</a> " in the framework repo."
            </p>

            <h2>"Where is the backend security reference?"</h2>
            <p><code>"examples/todo"</code> " — guards, DTO validation, service layer, authorization. Docs: " <a href="/docs/security/todo">"/docs/security/todo"</a>"."</p>

            <h2>"Do I need Redis, Postgres, or Cloudflare Workers?"</h2>
            <p>
                <strong>"No — for core Resuma features."</strong> " SSR, signals, server actions, CSRF, CSP, and "
                "rate limiting work out of the box with no external services. Rate limits use "
                <strong>"memory"</strong> " in dev and a "
                <strong>"disk backend"</strong> " in production (" <code>"{RESUMA_DATA_DIR}/rate-limit/"</code> ")."
            </p>
            <p>
                <strong>"Resuma OS"</strong> " (" <code>"resuma::exec"</code> ") adds disk-backed queues, cron scheduler, "
                "durable graphs, and webhooks — also without Redis. See "
                <a href="/docs/exec">"/docs/exec"</a> "."
            </p>
            <p>
                "Optional: add Postgres/SQLite via SQLx for your app's data — Resuma does not require a database "
                "for the framework itself. See "
                <a href="/docs/integrations/sqlx">"SQLx integration"</a> "."
            </p>
        </>
    }
}
