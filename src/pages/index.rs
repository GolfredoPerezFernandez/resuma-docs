use resuma::prelude::*;

use crate::site::{
    bench_row_full, bundle_sizes, code_block, compare_column, doc_link_card, feature_card,
    hero_particles_mount, metric_item, payload_layer, pillar_card, speed_bar,
};

pub fn page(_req: FlowRequest) -> View {
    let dash = "—";

    view! {
        <main id="main-content" class="landing">
            <div class="hero-wrap">
                {hero_particles_mount()}
                <section class="hero">
                    <div>
                        <span class="hero-badge">
                            <span class="hero-badge-dot"></span>
                            "v1.3.1 · Full Rust · UI + API + jobs"
                        </span>
                        <h1>
                            "The "
                            <span class="accent">"lightest"</span>
                            " path to instant interactivity in Rust"
                        </h1>
                        <p class="hero-tagline">
                            {format!("{} to resume. Zero Resuma JS on static pages.", bundle_sizes::LOADER_GZIP)}
                        </p>
                        <p class="hero-lead">
                            "Resuma renders UI once on the server, serialises signals into HTML, and lazy-loads "
                            "only the handlers you touch. No WASM hydration. Pages, APIs, auth, and jobs — one crate."
                        </p>
                        <div class="hero-actions">
                            <a href="/docs/getting_started" class="btn btn-primary">"Get Started"</a>
                            <a href="/docs" class="btn btn-ghost">"Docs with live demos"</a>
                        </div>
                        <p class="hero-note">
                            <code>"cargo install resuma"</code>
                            " · "
                            <code>"resuma new my-app --template todo"</code>
                            " · no Node.js for app development"
                        </p>
                        <p class="hero-note hero-note-quiet">
                            "This site’s Theme menu is a native "
                            <code>"<Popup>"</code>
                            "; Search is a "
                            <code>"<Modal>"</code>
                            "."
                        </p>
                    </div>
                    <div class="hero-panel">
                        <div class="hero-panel-top">
                            <div class="hero-panel-dots">
                                <span></span><span></span><span></span>
                            </div>
                            <span class="hero-panel-label">"what ships (gzip)"</span>
                        </div>
                        <div class="hero-panel-body hero-payload-preview">
                            <div class="hero-payload-row hero-payload-row-zero">
                                <span>"Static page"</span>
                                <strong>"0 B JS"</strong>
                            </div>
                            <div class="hero-payload-row hero-payload-row-accent">
                                <span>"Interactive page — loader"</span>
                                <strong>{bundle_sizes::RESUMA_INITIAL.to_string()}</strong>
                            </div>
                            <div class="hero-payload-row">
                                <span>"First click — loader + core + handler"</span>
                                <strong>{bundle_sizes::RESUMA_FIRST.to_string()}</strong>
                            </div>
                            <div class="hero-payload-row hero-payload-row-muted">
                                <span>"Next.js counter (default scaffold)"</span>
                                <strong>{bundle_sizes::NEXT_GZIP.to_string()}</strong>
                            </div>
                            <p class="hero-panel-caption">
                                <strong>{format!("{} smaller", bundle_sizes::SMALLER_THAN_NEXT)}</strong>
                                " initial payload than a default Next.js app — same counter UX. "
                                <a href="/docs/benchmark">"Methodology →"</a>
                            </p>
                        </div>
                    </div>
                </section>

                <div class="metrics-bar">
                    {metric_item(bundle_sizes::LOADER_GZIP, "loader (gzip)")}
                    {metric_item(bundle_sizes::RESUMA_STATIC, "static pages")}
                    {metric_item(bundle_sizes::RESUMA_FIRST, "first interaction")}
                    {metric_item(bundle_sizes::CORE_GZIP, "runtime core")}
                    {metric_item("1", "cargo dependency")}
                </div>
            </div>

            <section class="section zero-strip">
                <div class="zero-strip-inner">
                    <p class="zero-strip-eyebrow">"Zero-cost static"</p>
                    <h2 class="zero-strip-title">"Marketing pages ship no JavaScript"</h2>
                    <p class="zero-strip-body">
                        "Docs, blogs, and landing sections without signals or handlers compile to pure HTML. "
                        "No runtime. No hydration tax. Deploy to the edge and forget about bundle budgets."
                    </p>
                    <a href="/docs/flow/pages" class="btn btn-ghost">"Static vs interactive pages →"</a>
                </div>
            </section>

            <section class="section section-alt">
                <p class="section-eyebrow">"Payload anatomy"</p>
                <h2 class="section-title">"Every byte has a job"</h2>
                <p class="section-sub">
                    "Hydration frameworks ship the whole app up front. Resuma ships HTML plus a resumability "
                    "payload — handlers and core load only when needed."
                </p>
                <div class="payload-stack">
                    {payload_layer(
                        "SSR HTML + state",
                        "Server",
                        "View tree, data-r-on hooks, and resumability state — ready before any JS runs.",
                        false,
                    )}
                    {payload_layer(
                        "Loader",
                        bundle_sizes::LOADER_GZIP,
                        "Bootstraps signals from SSR. Enough for the page to feel alive.",
                        true,
                    )}
                    {payload_layer(
                        "Core runtime",
                        bundle_sizes::CORE_GZIP,
                        "DOM updates, Show/For, effects — fetched on first interaction or prefetch.",
                        false,
                    )}
                    {payload_layer(
                        "Handler chunks",
                        "Per component",
                        "onClick lives in a handler chunk — you pay only for what users touch.",
                        false,
                    )}
                </div>
            </section>

            <section class="section section-cv">
                <p class="section-eyebrow">"Measured"</p>
                <h2 class="section-title">"Counter page — initial load (gzip)"</h2>
                <p class="section-sub">
                    "Same UX everywhere: SSR heading + one increment button. "
                    "Bar width is relative to Next.js (" {bundle_sizes::NEXT_GZIP.to_string()} ")."
                </p>
                <div class="speed-chart">
                    {speed_bar("Resuma", bundle_sizes::RESUMA_INITIAL, 1, true)}
                    {speed_bar("Qwik", "1.96 KiB", 1, false)}
                    {speed_bar("SolidStart", "16.75 KiB", 12, false)}
                    {speed_bar("SvelteKit", "27.71 KiB", 19, false)}
                    {speed_bar("React (Vite)", "57.99 KiB", 41, false)}
                    {speed_bar("Leptos", "79.02 KiB", 56, false)}
                    {speed_bar("Next.js", bundle_sizes::NEXT_GZIP, 100, false)}
                </div>
                <div class="bench-wrap">
                    <p class="bench-caption">"Initial load, first interaction, and static-page JS — gzip"</p>
                    <table class="bench">
                        <thead>
                            <tr>
                                <th>"Framework"</th>
                                <th>"Initial load"</th>
                                <th>"First interaction"</th>
                                <th>"Static page"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {bench_row_full("Resuma", bundle_sizes::RESUMA_INITIAL, bundle_sizes::RESUMA_FIRST, bundle_sizes::RESUMA_STATIC, true)}
                            {bench_row_full("Leptos", "79.02 KiB", "79.02 KiB", dash, false)}
                            {bench_row_full("Next.js", bundle_sizes::NEXT_GZIP, bundle_sizes::NEXT_GZIP, dash, false)}
                            {bench_row_full("React (Vite)", "57.99 KiB", "57.99 KiB", dash, false)}
                            {bench_row_full("Astro", "57.76 KiB", "57.76 KiB", dash, false)}
                            {bench_row_full("SvelteKit", "27.71 KiB", "27.71 KiB", dash, false)}
                            {bench_row_full("Qwik", "1.96 KiB", "22.32 KiB", dash, false)}
                            {bench_row_full("SolidStart", "16.75 KiB", "16.75 KiB", dash, false)}
                            {bench_row_full("templ + HTMX", "16.21 KiB", "16.21 KiB", dash, false)}
                        </tbody>
                    </table>
                </div>
                <p class="bench-note">
                    "Hydration frameworks load the same JS on page load — initial and first click match. "
                    "Resuma static pages ship " <strong>"0 B"</strong> " client JS. "
                    <code>"node benchmark/run.mjs"</code>
                </p>
            </section>

            <section class="section section-alt section-cv">
                <p class="section-eyebrow">"Try it in the docs"</p>
                <h2 class="section-title">"Live demos — inside the documentation"</h2>
                <p class="section-sub">
                    "Interactive examples run on every docs page: workers, signals, forms, and server functions. "
                    "The homepage stays lean — open a guide and click."
                </p>
                <div class="docs-try-grid">
                    {doc_link_card(
                        "/docs/exec/workers",
                        "Resuma OS workers",
                        "Run a real #[worker], watch the execution graph, pause and cancel.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/signals",
                        "Signals & reactivity",
                        "Increment a counter — fine-grained updates, no full re-render.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/server",
                        "Server functions",
                        "Call Rust from the browser without a page reload.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/form",
                        "Forms & #[submit]",
                        "Progressive enhancement — works as HTML POST before JS loads.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/control_flow",
                        "Control flow",
                        "Show, For, Match — conditional UI with lazy boundaries.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/flow/loaders",
                        "Loaders & streaming",
                        "#[load] data fetching with cache headers and streaming SSR.",
                        "LIVE",
                    )}
                </div>
            </section>

            <section class="section section-cv">
                <p class="section-eyebrow">"One binary"</p>
                <h2 class="section-title">"Frontend speed. Backend included."</h2>
                <p class="section-sub">
                    "Qwik, React, Angular, and Next.js still need a second stack for APIs, jobs, and auth. "
                    "Leptos hydrates with WASM. Resuma resumes a few kilobytes of JS — and the rest is Rust: "
                    "typed actions, file routes, SQL, sessions, queues, cron, and WebSockets."
                </p>
                <div class="compare-3">
                    {compare_column(
                        "Hydration stacks",
                        "Ship a client runtime (React, Angular, Next, Leptos WASM). Interact later. Your API lives in another service.",
                        false,
                    )}
                    {compare_column(
                        "Qwik",
                        "Resumable JS — closest mental model. Still a JavaScript framework; workers and APIs are a separate deploy.",
                        false,
                    )}
                    {compare_column(
                        "Resuma",
                        "Resumable Rust SSR + lazy JS handlers. Pages, RPC, jobs, and realtime in one cargo install. No WASM by default.",
                        true,
                    )}
                </div>
            </section>

            <section class="section section-alt section-cv">
                <p class="section-eyebrow">"Performance model"</p>
                <h2 class="section-title">"Interactive from the first click"</h2>
                <p class="section-sub">"The client never re-runs your component tree. State and handlers are already in the HTML."</p>
                <div class="pillars">
                    {pillar_card("⚡", "HTML first", "Static pages ship no Resuma JS. Interactive pages resume a loader; core and handlers arrive on the click that needs them.")}
                    {pillar_card("🦀", "Full Rust stack", "#[server] RPC, #[submit] forms, #[load] data, workers, queues — axum-native. One process, no adapter maze.")}
                    {pillar_card("📋", "Progressive enhancement", "<Form submit> works as plain HTML POST before JS loads; runtime enhances in place.")}
                    {pillar_card("🧩", "Resumable by default", "Every #[component] is a lazy boundary. Handlers live in /_resuma/handler/{Component}.js.")}
                </div>
            </section>

            <section class="section section-cv">
                <div class="showcase">
                    <div class="showcase-copy">
                        <p class="section-eyebrow">"Components"</p>
                        <h2>"Write UI once — on the server"</h2>
                        <p>"view! with JSX-like syntax, fine-grained signals, and onClick handlers that compile to lazy JavaScript."</p>
                        <ul class="showcase-list">
                            <li>"signal for reactive state"</li>
                            <li>"computed! / effect! for client replay"</li>
                            <li>"#[component] props builder generated for you"</li>
                        </ul>
                        <a href="/docs/components/view" class="btn btn-ghost">"Component guide →"</a>
                    </div>
                    <div class="showcase-code">
                        <div class="code-window">
                            {code_block(r#"#[component]
fn Counter() {
    let count = signal(0);
    view! {
        <button onClick={count.update(|c| *c += 1)}>
            "Count: " {count}
        </button>
    }
}
// Handler lazy-loads from /_resuma/handler/Counter.js"#)}
                        </div>
                    </div>
                </div>
            </section>

            <section class="section section-alt section-cv">
                <div class="showcase showcase-reverse">
                    <div class="showcase-copy">
                        <p class="section-eyebrow">"Resuma OS"</p>
                        <h2>"Durable workers — self-hosted"</h2>
                        <p>"#[worker] functions, disk-backed queues, cron scheduler, and an ops dashboard. No Redis, no external orchestrator — same binary as your app."</p>
                        <ul class="showcase-list">
                            <li>"Execution graphs with SSE event streams"</li>
                            <li>"Pause, resume, cancel from the Flow UI"</li>
                            <li>"Queue recovery and checkpointed state"</li>
                        </ul>
                        <a href="/docs/exec/workers" class="btn btn-ghost">"Run the live worker demo →"</a>
                    </div>
                    <div class="showcase-code">
                        <div class="code-window">
                            {code_block(r#"#[worker(intent = "enrich page")]
async fn enrich(input: Input, ctx: WorkerContext) -> Result<Value> {
    ctx.log("fetching");
    let page = ctx.tool("fetch", json!({ "url": input.url })).await?;
    ctx.progress(50);
    let summary = ctx.tool("ai", json!({
        "prompt": "Extract key facts",
        "data": page
    })).await?;
    Ok(summary)
}"#)}
                        </div>
                    </div>
                </div>
            </section>

            <section class="section section-alt section-cv">
                <p class="section-eyebrow">"Why Resuma?"</p>
                <h2 class="section-title">"Everything you need for modern SSR"</h2>
                <p class="section-sub">"Resumable SSR in Rust — one install, progressive enhancement, full-stack Flow when you need it."</p>
                <div class="grid-3">
                    {feature_card("🌊", "Resuma Flow", "File-based pages, #[load], #[submit], layouts, middleware — built into the same crate.")}
                    <a href="/docs/cookbook/deploy" class="card">
                        <div class="card-icon">"🚀"</div>
                        <h3>"Deploy"</h3>
                        <p>"Same Docker image on Fly, DigitalOcean, AWS App Runner, or a VM. Not Lambda, not Workers."</p>
                    </a>
                    {feature_card("🔧", "Dev experience", "resuma dev with HMR WebSocket, resuma new templates (basic, todo, flow, production).")}
                    {feature_card("🔗", "JS bridge", "view! translates Rust closures via rs2js. js!{} for escape hatches when you need raw client code.")}
                    {feature_card("🏝️", "Islands (optional)", "#[island(load = \"visible\")] for heavy widgets — most UI only needs #[component].")}
                    {feature_card("🛡️", "Security built in", "Crypto CSRF, security headers, rate limits — see examples/todo for production patterns.")}
                </div>
            </section>

            <section class="section section-cv">
                <p class="section-eyebrow">"One package"</p>
                <h2 class="section-title">"Resuma¹ + Flow²"</h2>
                <p class="section-sub">"Two layers, one dependency. Core stays stable; Flow adds routing, data loading, and forms."</p>
                <div class="package-diagram">
                    <article class="package-box">
                        <p class="tag">"RESUMA¹ — CORE"</p>
                        <h3>"Components & resumability"</h3>
                        <ul>
                            <li>"view!, #[component], signal"</li>
                            <li>"computed! / effect! / debounce!"</li>
                            <li>{format!("#[server], ResumaApp, {} runtime", bundle_sizes::CORE_GZIP)}</li>
                        </ul>
                    </article>
                    <div class="package-plus">"+"</div>
                    <article class="package-box">
                        <p class="tag">"FLOW² — FULL-STACK"</p>
                        <h3>"Pages, loads & submits"</h3>
                        <ul>
                            <li>"FlowApp, src/pages/, #[layout]"</li>
                            <li>"#[load], #[submit], #[middleware]"</li>
                            <li>"Streaming SSR, cache headers"</li>
                        </ul>
                    </article>
                </div>
            </section>

            <section class="section section-alt section-cv">
                <p class="section-eyebrow">"AI assistants"</p>
                <h2 class="section-title">"Build faster with Cursor, Codex, or Gemini"</h2>
                <p class="section-sub">"Install the Resuma agent skill in one command — view!, HtmlTheme, Popup/Modal, Flow, SeoKit, and the debugging checklist. After a CLI upgrade: resuma install skill --force."</p>
                <div class="cta-install cta-install-block">"resuma install skill"</div>
                <p class="section-center">
                    <a href="/docs/integrations/ai_assistant" class="btn btn-ghost">"AI assistant guide →"</a>
                </p>
            </section>

            <section class="section section-cv">
                <p class="section-eyebrow">"Integrations"</p>
                <h2 class="section-title">"Database, auth, and tooling"</h2>
                <p class="section-sub">"Integration guides for SQLx, Turso, auth, validation, i18n, and E2E testing."</p>
                <div class="grid-3">
                    <a href="/docs/integrations/sqlx" class="card">
                        <h3>"SQLx"</h3>
                        <p>"Type-safe SQL in #[load] and #[submit]."</p>
                    </a>
                    <a href="/docs/integrations/turso" class="card">
                        <h3>"Turso"</h3>
                        <p>"Edge libSQL — file in dev, remote in prod."</p>
                    </a>
                    <a href="/docs/integrations/auth" class="card">
                        <h3>"Auth"</h3>
                        <p>"Sessions and middleware for protected routes."</p>
                    </a>
                </div>
                <p class="section-links">
                    <a href="/docs/integrations">"All integrations"</a>
                    " · "
                    <a href="/docs/search">"Search docs"</a>
                </p>
            </section>

            <section class="cta-section">
                <div class="cta-banner">
                    <h2>"Start building in 60 seconds"</h2>
                    <p>"Install the CLI, scaffold a project, and ship instantly-interactive Rust UI — ultralight by default."</p>
                    <a href="/docs/getting_started" class="btn btn-primary">"Read the tutorial"</a>
                    <div class="cta-install">"cargo install resuma && resuma new my-app --template todo"</div>
                </div>
            </section>
        </main>
    }
}
