use crate::site::bundle_sizes;
use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Prefetch"</h1>
            <p class="lead">"Resuma prefetches lazy handler chunks when resumable boundaries enter the viewport, and prefetches route HTML on NavLink hover (1.0+)."</p>

            {crate::site::demos::flow_prefetch()}

            <h2>"Handler prefetch (automatic)"</h2>
            <p>
                "Every " <code>"#[component]"</code> " registers " <code>"/_resuma/handler/{Name}.js"</code> ". "
                "The "
                {bundle_sizes::LOADER_GZIP.to_string()}
                " loader uses " <code>"IntersectionObserver"</code> " to prefetch handlers before the user clicks."
            </p>

            <h2>"Route prefetch (NavLink)"</h2>
            <p>
                <code>"NavLink"</code> " prefetches the next page HTML on hover/focus (SPA navigation without a full reload). "
                "See " <a href="/docs/components/nav_link">"NavLink"</a> "."
            </p>

            <h2>"Loader cache headers"</h2>
            <p>
                "Use short " <code>"cache"</code> " on " <code>"#[load]"</code> " for CDN edge caching when data can be stale briefly:"
            </p>
            {code_block(r#"#[load(cache = "public, max-age=30")]
async fn docs_index(_req: &FlowRequest) -> DocsNav {
    DocsNav { sections: list_sections().await }
}"#)}

            <h2>"Related"</h2>
            <ul>
                <li><a href="/docs/flow/caching">"Caching"</a></li>
                <li><a href="/docs/cookbook/loader_invalidation">"Loader invalidation"</a></li>
                <li><a href="/docs/architecture">"Architecture"</a></li>
            </ul>
        </>
    }
}
