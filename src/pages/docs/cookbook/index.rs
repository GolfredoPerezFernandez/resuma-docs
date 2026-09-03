use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Cookbook"</h1>
            <p class="lead">"Practical recipes for common Resuma patterns — copy, adapt, and ship."</p>

            {crate::site::demos::cookbook_overview()}

            <h2>"Recipes"</h2>
            <div class="grid-3">
                <a href="/docs/cookbook/debouncer" class="card" style="text-decoration: none;">
                    <h3>"Debouncer"</h3>
                    <p>"Rate-limit search input with use_debounce."</p>
                </a>
                <a href="/docs/cookbook/portals" class="card" style="text-decoration: none;">
                    <h3>"Portals"</h3>
                    <p>"Teleport only — prefer Modal / Popup for dialogs."</p>
                </a>
                <a href="/docs/cookbook/view_transitions" class="card" style="text-decoration: none;">
                    <h3>"View transitions"</h3>
                    <p>"Animated route changes — skipped while a popover is open."</p>
                </a>
                <a href="/docs/cookbook/theme" class="card" style="text-decoration: none;">
                    <h3>"Theme"</h3>
                    <p>"HtmlTheme + Popup — live html[data-theme], no hydration."</p>
                </a>
                <a href="/docs/cookbook/streaming_loaders" class="card" style="text-decoration: none;">
                    <h3>"Streaming loaders"</h3>
                    <p>"Deferred SSR for slow data."</p>
                </a>
                <a href="/docs/cookbook/deploy" class="card" style="text-decoration: none;">
                    <h3>"Deploy"</h3>
                    <p>"Fly.io, DigitalOcean, AWS, Cloudflare, Docker."</p>
                </a>
                <a href="/docs/cookbook/prg" class="card" style="text-decoration: none;">
                    <h3>"PRG pattern"</h3>
                    <p>"Post/Redirect/Get after form submits."</p>
                </a>
                <a href="/docs/cookbook/loader_invalidation" class="card" style="text-decoration: none;">
                    <h3>"Loader invalidation"</h3>
                    <p>"Refresh stale #[load] data after mutations."</p>
                </a>
                <a href="/docs/flow/query_params" class="card" style="text-decoration: none;">
                    <h3>"Query-driven loaders"</h3>
                    <p>"Date pickers, filters, SPA navigate."</p>
                </a>
            </div>
        </>
    }
}
