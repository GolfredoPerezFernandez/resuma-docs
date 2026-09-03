//! Full docs navigation — mapped to Resuma APIs.

use resuma::prelude::*;

use crate::site::DocsCopyNavBtn;

macro_rules! nav {
    ($($href:expr => $label:expr),+ $(,)?) => {
        view! {
            <nav class="docs-nav">
                $(
                    <NavLink
                        href={$href}
                        class="docs-nav-link"
                        activeClass="docs-nav-link--active"
                        exact=true
                    >
                        {$label}
                    </NavLink>
                )+
            </nav>
        }
    };
}

pub fn doc_sidebar(_active_path: &str) -> View {
    view! {
        <>
            {crate::site::browse_docs_chip()}
        <aside id="docs-browse" class="docs-sidebar" data-r-nav-exclusive="true">
            <form method="get" action="/docs/search" class="docs-search-form">
                <input type="search" name="q" placeholder="Search docs..." aria-label="Search docs" />
            </form>
            <DocsCopyNavBtn />
            <div class="docs-sidebar-scroll">
                <section class="docs-sidebar-section">
                    <h4>"Introduction"</h4>
                    {nav!(
                        "/docs" => "Overview",
                        "/docs/getting_started" => "Getting Started",
                        "/docs/cli" => "CLI",
                        "/docs/benchmark" => "Benchmark",
                        "/docs/examples" => "Examples",
                        "/docs/project_structure" => "Project structure",
                        "/docs/faq" => "FAQ",
                    )}
                </section>

                <section class="docs-sidebar-section">
                    <h4>"Resuma OS"</h4>
                    {nav!(
                        "/docs/exec" => "Overview",
                        "/docs/exec/workers" => "Workers",
                        "/docs/exec/queue" => "Queue",
                        "/docs/exec/scheduler" => "Scheduler",
                        "/docs/exec/webhooks" => "Webhooks",
                        "/docs/exec/tools" => "Tools",
                        "/docs/exec/flow_ui" => "Flow UI",
                        "/docs/exec/ops" => "Ops & production",
                        "/docs/exec/security" => "Exec security",
                    )}
                </section>

                <section class="docs-sidebar-section">
                    <h4>"Security"</h4>
                    {nav!(
                        "/docs/security" => "Overview",
                        "/docs/security/environment" => "Environment variables",
                        "/docs/security/configure" => "Configure server",
                        "/docs/security/server_actions" => "Server actions",
                        "/docs/security/middleware" => "Auth middleware",
                        "/docs/security/authorization" => "Authorization & RLS",
                        "/docs/security/backend_patterns" => "Backend patterns",
                        "/docs/security/todo" => "Todo example",
                    )}
                </section>

                <section class="docs-sidebar-section">
                    <h4>"Components"</h4>
                    {nav!(
                        "/docs/components" => "Overview",
                        "/docs/components/view" => "view!",
                        "/docs/components/control_flow" => "Control flow",
                        "/docs/components/signals" => "Signals",
                        "/docs/components/effects" => "Effects",
                        "/docs/components/error_boundary" => "Error boundaries",
                        "/docs/components/handlers" => "Handlers",
                        "/docs/components/islands" => "Islands",
                        "/docs/components/client" => "Client (TypeScript)",
                        "/docs/components/server" => "Server actions",
                        "/docs/components/js" => "js!",
                        "/docs/components/slots" => "Slots",
                        "/docs/components/nav_link" => "NavLink",
                        "/docs/components/form" => "Form",
                        "/docs/components/popup" => "Popup",
                        "/docs/components/modal" => "Modal",
                        "/docs/components/gesture" => "GestureView",
                        "/docs/components/virtual_for" => "Virtual For",
                        "/docs/components/desktop_ui" => "Desktop UI",
                        "/docs/components/store" => "Store",
                        "/docs/components/context" => "Context",
                        "/docs/components/tasks" => "Tasks",
                        "/docs/components/testing" => "Testing",
                    )}
                </section>

                <section class="docs-sidebar-section">
                    <h4>"Resuma Flow"</h4>
                    {nav!(
                        "/docs/flow" => "Overview",
                        "/docs/flow/routing" => "Routing",
                        "/docs/flow/query_params" => "Query params",
                        "/docs/flow/pages" => "Pages",
                        "/docs/flow/layouts" => "Layouts",
                        "/docs/flow/loaders" => "Loaders",
                        "/docs/flow/submits" => "Actions",
                        "/docs/flow/middleware" => "Middleware",
                        "/docs/flow/endpoints" => "Endpoints",
                        "/docs/flow/errors" => "Error handling",
                        "/docs/flow/caching" => "Caching",
                        "/docs/flow/streaming" => "Streaming",
                        "/docs/flow/prefetch" => "Prefetch",
                        "/docs/flow/pwa" => "PWA & public/",
                    )}
                </section>

                <section class="docs-sidebar-section">
                    <h4>"Integrations"</h4>
                    {nav!(
                        "/docs/integrations" => "Overview",
                        "/docs/integrations/ai_assistant" => "AI assistant",
                        "/docs/integrations/sqlx" => "SQLx",
                        "/docs/integrations/turso" => "Turso",
                        "/docs/integrations/supabase" => "Supabase",
                        "/docs/integrations/auth" => "Auth",
                        "/docs/integrations/validator" => "Validation",
                        "/docs/integrations/i18n" => "i18n",
                        "/docs/integrations/tailwind" => "Tailwind",
                        "/docs/integrations/og_image" => "OG Image",
                        "/docs/integrations/seo_geo" => "SEO, GEO & AEO",
                        "/docs/integrations/e2e" => "E2E testing",
                    )}
                </section>

                <section class="docs-sidebar-section">
                    <h4>"Cookbook"</h4>
                    {nav!(
                        "/docs/cookbook" => "Overview",
                        "/docs/cookbook/debouncer" => "Debouncer",
                        "/docs/cookbook/portals" => "Portals",
                        "/docs/cookbook/view_transitions" => "View transitions",
                        "/docs/cookbook/theme" => "Theme",
                        "/docs/cookbook/streaming_loaders" => "Streaming loaders",
                        "/docs/cookbook/prg" => "PRG pattern",
                        "/docs/cookbook/loader_invalidation" => "Loader invalidation",
                        "/docs/cookbook/deploy" => "Deploy (Fly, AWS, CF, …)",
                    )}
                </section>

                <section class="docs-sidebar-section">
                    <h4>"Reference"</h4>
                    {nav!(
                        "/docs/architecture" => "Architecture",
                        "/docs/reactivity" => "Reactivity internals",
                        "/docs/package" => "Package",
                        "/docs/api" => "API reference",
                    )}
                </section>

                <section class="docs-sidebar-section docs-sidebar-section--resources">
                    <h4>"Resources"</h4>
                    <nav class="docs-nav">
                        <a href="https://crates.io/crates/resuma" target="_blank" rel="noopener noreferrer" class="docs-nav-link docs-nav-link--external">"crates.io"</a>
                        <a href="https://docs.rs/resuma" target="_blank" rel="noopener noreferrer" class="docs-nav-link docs-nav-link--external">"docs.rs"</a>
                        <a href="https://github.com/GoldevLab/resuma" target="_blank" rel="noopener noreferrer" class="docs-nav-link docs-nav-link--external">"GitHub"</a>
                    </nav>
                </section>
            </div>
        </aside>
        </>
    }
}
