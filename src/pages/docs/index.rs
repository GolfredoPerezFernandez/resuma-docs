use resuma::prelude::*;

use crate::site::{bundle_sizes, doc_link_card, learn_path_card, metric_item};

pub fn page(_req: FlowRequest) -> View {
    view! {
        <div class="docs-hub">
            <header class="docs-hero">
                <h1>"Documentation"</h1>
                <p class="docs-hero-lead">
                    "Resumable SSR in Rust — components, server actions, full-stack Flow, jobs, and security. "
                    "One " <code>"cargo install resuma"</code> " for UI + APIs + CLI. "
                    <strong>"Theme"</strong> " is a live " <code>"&lt;Popup&gt;"</code>
                    ", " <strong>"Search"</strong> " is a " <code>"&lt;Modal&gt;"</code>
                    " (" <kbd>"/"</kbd> "). Instant restyle, no hydration."
                </p>
                <form method="get" action="/docs/search" class="docs-search-hero">
                    <input type="search" name="q" placeholder="Search docs…" aria-label="Search documentation" />
                    <button type="submit">"Search"</button>
                </form>
                <div class="docs-quick-links">
                    <a href="/docs/getting_started">"Getting started"</a>
                    <a href="/docs/cookbook/theme">"Theme"</a>
                    <a href="/docs/integrations/ai_assistant">"AI skill"</a>
                    <a href="/docs/cookbook/deploy">"Deploy"</a>
                    <a href="/docs/benchmark">"Benchmark"</a>
                    <a href="/docs/examples">"Examples"</a>
                    <a href="https://docs.rs/resuma" target="_blank">"API (docs.rs)"</a>
                    <a href="https://github.com/GoldevLab/resuma" target="_blank">"GitHub"</a>
                </div>
            </header>

            <div class="docs-stat-strip">
                {metric_item(bundle_sizes::RESUMA_INITIAL, "initial JS (gzip)")}
                {metric_item(bundle_sizes::RESUMA_FIRST, "first interaction")}
                {metric_item("0 B", "static pages")}
                {metric_item("1 crate", "core + Flow + CLI")}
            </div>

            <h2 class="docs-section-title">"Choose your path"</h2>
            <div class="learn-paths">
                {learn_path_card(
                    "1",
                    "New to Resuma",
                    "Install the CLI, pick a template, and ship your first resumable page in minutes.",
                    "/docs/getting_started",
                    "Start tutorial →",
                )}
                {learn_path_card(
                    "2",
                    "Full-stack with Flow",
                    "File-based routing, loaders, form submits, layouts, and middleware in one crate.",
                    "/docs/flow",
                    "Flow guide →",
                )}
                {learn_path_card(
                    "3",
                    "Production ready",
                    "CSRF, rate limits, auth middleware, validation — walk through the todo showcase.",
                    "/docs/security/todo",
                    "Security walkthrough →",
                )}
            </div>

            <h2 class="docs-section-title">"Start here"</h2>
            <div class="grid-3">
                {doc_link_card(
                    "/docs/cli",
                    "CLI",
                    "Install, scaffold, dev, build — run resuma update to align deps with your CLI.",
                    "resuma update",
                )}
                {doc_link_card(
                    "/docs/getting_started",
                    "Getting Started",
                    "CLI install, templates (basic / todo / flow), first app.",
                    "Recommended",
                )}
                {doc_link_card(
                    "/docs/benchmark",
                    "Bundle benchmark",
                    "Measured comparison vs Qwik, Leptos, Next.js, React, Astro, and more.",
                    "Measured",
                )}
                {doc_link_card(
                    "/docs/examples",
                    "Examples",
                    "Runnable crates: counter, todo, flow-demo, flow-pages.",
                    "",
                )}
            </div>

            <h2 class="docs-section-title">"Learn by topic"</h2>
            <div class="grid-3">
                {doc_link_card(
                    "/docs/components",
                    "Components",
                    "view!, Popup, Modal, HtmlTheme, signals, islands.",
                    "",
                )}
                {doc_link_card(
                    "/docs/flow",
                    "Resuma Flow",
                    "Pages, loads, submits, routing, streaming, caching.",
                    "",
                )}
                {doc_link_card(
                    "/docs/exec",
                    "Resuma OS",
                    "Workers, disk queue, cron scheduler, ops dashboard.",
                    "New",
                )}
                {doc_link_card(
                    "/docs/security",
                    "Security",
                    "CSRF, headers, rate limits, auth, authorization.",
                    "",
                )}
                {doc_link_card(
                    "/docs/cookbook",
                    "Cookbook",
                    "Theme, portals, streaming loaders, Fly and DigitalOcean deploy.",
                    "",
                )}
                {doc_link_card(
                    "/docs/integrations",
                    "Integrations",
                    "AI skill, SQLx, Turso, auth, Tailwind, SEO/GEO.",
                    "",
                )}
                {doc_link_card(
                    "/docs/architecture",
                    "Architecture",
                    "Resumability vs hydration — SSR payload and runtime.",
                    "",
                )}
                {doc_link_card(
                    "/docs/project_structure",
                    "Project structure",
                    "ResumaApp vs FlowApp layouts and conventions.",
                    "",
                )}
                {doc_link_card(
                    "/docs/cli",
                    "CLI",
                    "new, dev, build, routes --generate, doctor.",
                    "",
                )}
                {doc_link_card(
                    "/docs/api",
                    "API reference",
                    "Link to docs.rs for the full Rust API surface.",
                    "",
                )}
            </div>

            <h2 class="docs-section-title">"What is Resuma?"</h2>
            <p>
                "Components run once on the server. SSR embeds signals and handler references in HTML; "
                "a "
                <strong>{bundle_sizes::LOADER_GZIP.to_string()}</strong>
                " gzip loader resumes interactivity on first click — no hydration, no WASM bundle by default."
            </p>
            <p>
                <strong>"Resuma Flow"</strong>
                " adds file-based pages, "
                <code>"#[load]"</code>
                ", "
                <code>"#[submit]"</code>
                ", and middleware — still one "
                <code>"resuma"</code>
                " crate."
            </p>
            <p>
                "Published on "
                <a href="https://crates.io/crates/resuma" target="_blank">"crates.io"</a>
                " · "
                <a href="https://docs.rs/resuma" target="_blank">"docs.rs"</a>
                " · benchmark source in the "
                <a href="https://github.com/GoldevLab/resuma/tree/main/benchmark" target="_blank">"GitHub repo"</a>"."
            </p>
        </div>
    }
}
