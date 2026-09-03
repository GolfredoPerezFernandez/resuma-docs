//! Resuma official documentation site — landing + docs (Resuma¹ + Flow²).

mod pages;
mod site;

use axum::routing::post;
use pages::PagesRegistry;
use resuma::current_request;
use resuma::prelude::*;
use site::DocsCopyPageToolbar;

#[layout("/")]
fn SiteLayout() -> View {
    site::apply_page_seo();
    site::provide_docs_theme();
    let vt = site::view_transition_name(
        &current_request()
            .map(|r| r.path)
            .unwrap_or_else(|| "/".into()),
    );
    view! {
        {site::skip_link()}
        <header class="site-header">
            <div class="header-inner">
                <a href="/" class="logo">
                    <span class="logo-mark">"🌊"</span>
                    "Resuma"
                </a>
                <nav class="site-nav">
                    <NavLink href="/docs" activeClass="active" exact=true>"Docs"</NavLink>
                    <NavLink href="/docs/getting_started" activeClass="active" exact=true>"Tutorial"</NavLink>
                    <NavLink href="/docs/flow" activeClass="active">"Flow"</NavLink>
                    <NavLink href="/docs/exec" activeClass="active">"Resuma OS"</NavLink>
                    <NavLink href="/docs/benchmark" activeClass="active" exact=true>"Benchmark"</NavLink>
                </nav>
                <div class="header-actions">
                    {site::explore_nav()}
                    {site::docs_search_modal()}
                    {site::theme_picker()}
                    <a href="/docs/getting_started" class="btn btn-ghost">"Get Started"</a>
                    <a href="https://docs.rs/resuma" class="btn btn-ghost docs-rs-link" target="_blank">"docs.rs"</a>
                    <a href="https://crates.io/crates/resuma" class="btn btn-ghost crates-link" target="_blank">"crates.io"</a>
                    <a href="https://github.com/GoldevLab/resuma" class="btn btn-primary">"GitHub"</a>
                </div>
            </div>
        </header>
        {with_view_transition(vt, vec![Child::View(view! { <Slot /> })])}
        <footer class="site-footer">
            <p>"Made with Rust · resumable SSR · MIT License"</p>
            <div class="site-footer-links">
                <a href="https://crates.io/crates/resuma" target="_blank">"crates.io"</a>
                <a href="https://docs.rs/resuma" target="_blank">"docs.rs"</a>
                <a href="/docs/package">"Install guide"</a>
                <a href="/docs/architecture">"Architecture"</a>
                <a href="/docs/benchmark">"Benchmarks"</a>
                <a href="/docs/cookbook/theme">"Theming"</a>
                <a href="https://github.com/GoldevLab/resuma" target="_blank">"GitHub"</a>
            </div>
        </footer>
    }
}

#[layout("/docs")]
fn DocsLayout() -> View {
    let path = current_request()
        .map(|r| r.path)
        .unwrap_or_else(|| "/docs".into());
    let vt = site::view_transition_name(&path);

    visible_task!(
        r#"
        async (_state, __resuma) => {
            const load = async () => {
                try {
                    const mod = await import('/static/client/docs-copy.js?v=1.3.1');
                    mod.initDocsCopy?.();
                    mod.initDocsSidebar?.();
                    mod.initCacheDemo?.();
                    mod.initDocsFlow?.();
                } catch (e) {
                    console.warn('[docs-copy]', e);
                }
            };
            await load();
            const onNav = () => { void load(); };
            document.addEventListener('resuma:navigate', onNav);
            return () => document.removeEventListener('resuma:navigate', onNav);
        }
    "#
    );

    view! {
        <div class="docs-shell">
            <div class="liquid-orbs liquid-orbs-docs" aria-hidden="true">
                <div class="liquid-blob liquid-blob-c"></div>
            </div>
            {site::doc_sidebar(&path)}
            <main id="main-content" class="docs-main">
                <DocsCopyPageToolbar />
                {with_view_transition(vt, vec![Child::View(view! { <Slot /> })])}
            </main>
            <div id="modals"></div>
        </div>
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    site::install_security_middleware();

    let pages_root = std::env::var("RESUMA_PAGES_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"));

    let site_url = site::site_url();
    let json_ld = site::json_ld(&site_url);
    let public_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public");

    FlowApp::new()
        .with_title(site::site_title())
        .with_description(site::site_description())
        .with_site_url(site_url.clone())
        .with_og_image("/og.png")
        .with_json_ld(json_ld)
        .with_seo_kit(site::seo_kit(&site_url))
        .with_sitemap_exclude(["/docs/cookbook/docker", "/docs/search"])
        .with_public_dir(public_dir)
        .with_html_theme(
            HtmlTheme::new(["paper", "slate", "midnight", "ember", "aurora", "forest"])
                .dark(["midnight", "ember", "aurora", "forest"])
                .cookie(site::THEME_COOKIE)
                .storage_key("resuma-docs-theme"),
        )
        .with_pwa(site::pwa_config())
        .with_head(site::site_head())
        .client_asset(
            "hero-particles",
            include_bytes!("../static/client/hero-particles.js"),
        )
        .client_asset("docs-copy", include_bytes!("../static/client/docs-copy.js"))
        .client_asset(
            "docs-flow-worker",
            include_bytes!("../static/client/docs-flow-worker.js"),
        )
        .route("/_resuma/demo/webhook-inbox", post(site::inbox_handler))
        .streaming(true)
        .not_found(not_found_page)
        .auto_pages(pages_root, PagesRegistry)
        .serve(FlowServeOptions::default())
        .await
}
