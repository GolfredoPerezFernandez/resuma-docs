use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Loader invalidation"</h1>
            <p class="lead">"Refresh stale " <code>"#[load]"</code> " data after mutations by re-running loaders or invalidating cached responses."</p>

            {crate::site::demos::cookbook_loader_invalidation()}

            <h2>"Short cache TTL"</h2>
            {code_block(r#"#[load(cache = "public, max-age=10")]
async fn product_list(_req: &FlowRequest) -> Vec<Product> {
    db::products().await
}"#)}

            <h2>"Private per-user data"</h2>
            {code_block(r#"#[load(cache = "private, no-store")]
async fn cart(req: &FlowRequest) -> Cart {
    cart_for(req.user_id()).await
}"#)}

            <h2>"Same page, new query (SPA)"</h2>
            <p>
                "Filters and date pickers on one route (e.g. " <code>"/reservar?fecha="</code> ") should call "
                <code>"__resuma.navigate"</code> " or use "
                <code>"loader_refresh_input"</code> " — see "
                <a href="/docs/flow/query_params">"Query params"</a> "."
            </p>

            <h2>"After submit: full page navigation"</h2>
            <p>
                "The simplest invalidation — redirect to GET (see "
                <a href="/docs/cookbook/prg">"PRG pattern"</a>
                "). The next SSR run re-executes all loaders."
            </p>

            <h2>"set_load_cache (runtime)"</h2>
            {code_block(r#"// After successful mutation in #[server] or enhanced submit client path:
    set_load_cache("product_list", "public, max-age=0");"#)}

            <h2>"SPA invalidation (1.0+)"</h2>
            <p>"Re-fetch the current page or a linked route without a full reload:"</p>
            {code_block(r#"// Rust (Flow helpers):
invalidate_href("/docs");
invalidate_href_now("/docs");
invalidate_link(nav_anchor_element);

// Client (after mutation in js!):
await __resuma.invalidate();"#)}

            <h2>"loader_poll"</h2>
            <p>
                "Live dashboards: " <code>"loader_poll(\"/ops\", 8_000)"</code>
                " re-runs " <code>"#[load]"</code> " on an interval. Ticks skip while the tab is hidden "
                <strong>"or idle"</strong> " (" <code>"__resuma.presence()"</code> "). See "
                <a href="/docs/components/desktop_ui">"Desktop UI"</a> "."
            </p>

            <h2>"FlowExtensions for DB"</h2>
            <p>
                "Use " <code>"FlowApp::with_extension(\"db\", \"ready\")"</code> " so loaders know the pool is initialized. "
                "See " <a href="/docs/integrations">"Integrations"</a>"."
            </p>
        </>
    }
}
