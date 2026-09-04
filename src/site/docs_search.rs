//! Static docs search index (server-side filter, zero client JS).

#[derive(Clone, Copy)]
pub struct DocEntry {
    pub title: &'static str,
    pub href: &'static str,
    pub keywords: &'static str,
}

pub const ENTRIES: &[DocEntry] = &[
    DocEntry {
        title: "Overview",
        href: "/docs",
        keywords: "introduction start documentation hub",
    },
    DocEntry {
        title: "Getting Started",
        href: "/docs/getting_started",
        keywords: "install cli scaffold tutorial template production deploy fly",
    },
    DocEntry {
        title: "Benchmark",
        href: "/docs/benchmark",
        keywords: "bundle size gzip loader core qwik leptos next react",
    },
    DocEntry {
        title: "Examples",
        href: "/docs/examples",
        keywords: "runnable counter todo flow-demo smoke fullstack",
    },
    DocEntry {
        title: "FAQ",
        href: "/docs/faq",
        keywords: "hydration resumability wasm production ready bundle deploy fly digitalocean copy page nav ai clipboard HtmlTheme Popup skill",
    },
    DocEntry {
        title: "Project structure",
        href: "/docs/project_structure",
        keywords: "layout src pages ResumaApp FlowApp cargo",
    },
    DocEntry {
        title: "Architecture",
        href: "/docs/architecture",
        keywords: "resumability hydration ssr payload runtime loader",
    },
    DocEntry {
        title: "Reactivity internals",
        href: "/docs/reactivity",
        keywords: "signals batching scheduler resuma-dyn resuma-show",
    },
    DocEntry {
        title: "Package",
        href: "/docs/package",
        keywords: "crates install dependencies resuma-macros version",
    },
    DocEntry {
        title: "CLI",
        href: "/docs/cli",
        keywords: "resuma new dev build routes doctor static-export update install skill",
    },
    DocEntry {
        title: "API reference",
        href: "/docs/api",
        keywords: "docs.rs rust types traits prelude",
    },
    DocEntry {
        title: "Search docs",
        href: "/docs/search",
        keywords: "find query index",
    },
    DocEntry {
        title: "Components overview",
        href: "/docs/components",
        keywords: "view signals handlers islands form",
    },
    DocEntry {
        title: "view!",
        href: "/docs/components/view",
        keywords: "macro template jsx html attributes",
    },
    DocEntry {
        title: "Control flow",
        href: "/docs/components/control_flow",
        keywords: "For Match When Show If keyed list reactive",
    },
    DocEntry {
        title: "Signals",
        href: "/docs/components/signals",
        keywords: "use_signal state reactive update",
    },
    DocEntry {
        title: "Effects",
        href: "/docs/components/effects",
        keywords: "use_effect side effect lifecycle",
    },
    DocEntry {
        title: "Error boundaries",
        href: "/docs/components/error_boundary",
        keywords: "safeAction Result catch fallback ui",
    },
    DocEntry {
        title: "Handlers",
        href: "/docs/components/handlers",
        keywords: "onclick event closure rs2js chunk",
    },
    DocEntry {
        title: "Islands",
        href: "/docs/components/islands",
        keywords: "partial hydration lazy client boundary",
    },
    DocEntry {
        title: "Client components",
        href: "/docs/components/client",
        keywords: "typescript vite module nonce csp",
    },
    DocEntry {
        title: "Server actions",
        href: "/docs/components/server",
        keywords: "server macro rpc action post",
    },
    DocEntry {
        title: "js!",
        href: "/docs/components/js",
        keywords: "escape hatch async safeAction __resuma state",
    },
    DocEntry {
        title: "Slots",
        href: "/docs/components/slots",
        keywords: "children composition layout",
    },
    DocEntry {
        title: "NavLink",
        href: "/docs/components/nav_link",
        keywords: "spa navigation prefetch active class",
    },
    DocEntry {
        title: "Form",
        href: "/docs/components/form",
        keywords: "submit prg progressive enhancement csrf",
    },
    DocEntry {
        title: "Popup",
        href: "/docs/components/popup",
        keywords: "popover anchor css position-try menu tooltip HtmlTheme theme switch",
    },
    DocEntry {
        title: "Modal",
        href: "/docs/components/modal",
        keywords: "dialog focus trap stack closedby showModal",
    },
    DocEntry {
        title: "GestureView",
        href: "/docs/components/gesture",
        keywords: "pan pinch long-press double-tap pwa touch",
    },
    DocEntry {
        title: "Virtual For",
        href: "/docs/components/virtual_for",
        keywords: "virtual list window recycle for 10k",
    },
    DocEntry {
        title: "Desktop UI",
        href: "/docs/components/desktop_ui",
        keywords: "announce measure presence storage online rtl focus HtmlTheme set_page_dir set_page_theme",
    },
    DocEntry {
        title: "Store",
        href: "/docs/components/store",
        keywords: "derive Store typed fields global state",
    },
    DocEntry {
        title: "Context",
        href: "/docs/components/context",
        keywords: "provide use_context dependency injection",
    },
    DocEntry {
        title: "Tasks",
        href: "/docs/components/tasks",
        keywords: "visible task async background work",
    },
    DocEntry {
        title: "Testing",
        href: "/docs/components/testing",
        keywords: "unit test render assert view",
    },
    DocEntry {
        title: "Resuma OS",
        href: "/docs/exec",
        keywords: "workers queue scheduler durable execution ops resuma os",
    },
    DocEntry {
        title: "Workers",
        href: "/docs/exec/workers",
        keywords: "worker macro graph pause resume cancel queue WorkerContext",
    },
    DocEntry {
        title: "Disk queue",
        href: "/docs/exec/queue",
        keywords: "enqueue pending processing multi-process claim",
    },
    DocEntry {
        title: "Cron scheduler",
        href: "/docs/exec/scheduler",
        keywords: "cron schedule tick jobs fire",
    },
    DocEntry {
        title: "Webhooks",
        href: "/docs/exec/webhooks",
        keywords: "graph.done failed paused hmac signature outbound",
    },
    DocEntry {
        title: "Exec tools",
        href: "/docs/exec/tools",
        keywords: "fetch ai scrape map-reduce planner SSRF",
    },
    DocEntry {
        title: "Flow UI",
        href: "/docs/exec/flow_ui",
        keywords: "resuma-flow dashboard graph execution worker panel",
    },
    DocEntry {
        title: "Ops dashboard",
        href: "/docs/exec/ops",
        keywords: "production RESUMA_EXEC_API_KEY metrics prometheus ops",
    },
    DocEntry {
        title: "Exec security",
        href: "/docs/exec/security",
        keywords: "api key graph token rate limit exec public",
    },
    DocEntry {
        title: "Flow overview",
        href: "/docs/flow",
        keywords: "pages routing fullstack file-based",
    },
    DocEntry {
        title: "Routing",
        href: "/docs/flow/routing",
        keywords: "dynamic params catch-all registry",
    },
    DocEntry {
        title: "Query params",
        href: "/docs/flow/query_params",
        keywords: "search filter loader_refresh navigate buildUrl",
    },
    DocEntry {
        title: "Pages",
        href: "/docs/flow/pages",
        keywords: "page function FlowRequest registry generate",
    },
    DocEntry {
        title: "Layouts",
        href: "/docs/flow/layouts",
        keywords: "nested layout shell sidebar",
    },
    DocEntry {
        title: "Loaders",
        href: "/docs/flow/loaders",
        keywords: "load data fetch server use_load Path Query",
    },
    DocEntry {
        title: "Actions (submits)",
        href: "/docs/flow/submits",
        keywords: "submit form post redirect get prg",
    },
    DocEntry {
        title: "Middleware",
        href: "/docs/flow/middleware",
        keywords: "auth guard session request pipeline",
    },
    DocEntry {
        title: "Endpoints",
        href: "/docs/flow/endpoints",
        keywords: "custom route api json handler",
    },
    DocEntry {
        title: "Error handling",
        href: "/docs/flow/errors",
        keywords: "ResumaError status load_boundary not found",
    },
    DocEntry {
        title: "Caching",
        href: "/docs/flow/caching",
        keywords: "cache-control max-age stale loader",
    },
    DocEntry {
        title: "Streaming",
        href: "/docs/flow/streaming",
        keywords: "chunked html deferred suspense",
    },
    DocEntry {
        title: "Prefetch",
        href: "/docs/flow/prefetch",
        keywords: "viewport lazy handler navlink hover",
    },
    DocEntry {
        title: "PWA & public",
        href: "/docs/flow/pwa",
        keywords: "manifest service worker installable icons precache static",
    },
    DocEntry {
        title: "Security overview",
        href: "/docs/security",
        keywords: "csrf rate limit headers production disk memory no redis RESUMA_DATA_DIR",
    },
    DocEntry {
        title: "Configure server",
        href: "/docs/security/configure",
        keywords: "csp nonce auto_pages security SecurityConfig",
    },
    DocEntry {
        title: "Environment variables",
        href: "/docs/security/environment",
        keywords: "RESUMA_ENV RESUMA_TRUST_PROXY RESUMA_EXEC_API_KEY RESUMA_RATE_BACKEND RESUMA_DATA_DIR fly secrets deploy production local dev no redis",
    },
    DocEntry {
        title: "Secure server actions",
        href: "/docs/security/server_actions",
        keywords: "validation Result middleware audit safeAction",
    },
    DocEntry {
        title: "Auth middleware",
        href: "/docs/security/middleware",
        keywords: "session cookie guard FlowRequest user_id",
    },
    DocEntry {
        title: "Authorization & RLS",
        href: "/docs/security/authorization",
        keywords: "owner row level policy permission",
    },
    DocEntry {
        title: "Backend patterns",
        href: "/docs/security/backend_patterns",
        keywords: "service layer dto repository guard",
    },
    DocEntry {
        title: "Todo security example",
        href: "/docs/security/todo",
        keywords: "reference showcase guards validation",
    },
    DocEntry {
        title: "Cookbook overview",
        href: "/docs/cookbook",
        keywords: "recipes patterns deploy theme",
    },
    DocEntry {
        title: "Debouncer",
        href: "/docs/cookbook/debouncer",
        keywords: "input delay search throttle",
    },
    DocEntry {
        title: "Portals",
        href: "/docs/cookbook/portals",
        keywords: "modal overlay teleport dialog popup",
    },
    DocEntry {
        title: "View transitions",
        href: "/docs/cookbook/view_transitions",
        keywords: "animation page transition css popover skip view-transition",
    },
    DocEntry {
        title: "Theme",
        href: "/docs/cookbook/theme",
        keywords: "dark mode css variables toggle HtmlTheme ThemeSwitch data-r-theme data-theme palette copy theme CSS official palettes resuma theme",
    },
    DocEntry {
        title: "Streaming loaders",
        href: "/docs/cookbook/streaming_loaders",
        keywords: "deferred suspense progressive",
    },
    DocEntry {
        title: "PRG pattern",
        href: "/docs/cookbook/prg",
        keywords: "post redirect get submit form",
    },
    DocEntry {
        title: "Loader invalidation",
        href: "/docs/cookbook/loader_invalidation",
        keywords: "revalidate stale invalidate_href __resuma.invalidate",
    },
    DocEntry {
        title: "Deploy (Fly, AWS, Cloudflare)",
        href: "/docs/cookbook/deploy",
        keywords: "fly.io digitalocean droplet railway render docker production dockerfile hosting paas PORT health trust proxy github actions ha=false cargo_manifest_dir public aws app runner ecs fargate lambda cloudflare workers containers tunnel qwik adapter vercel edge",
    },
    DocEntry {
        title: "Integrations overview",
        href: "/docs/integrations",
        keywords: "database auth styling testing",
    },
    DocEntry {
        title: "SQLx",
        href: "/docs/integrations/sqlx",
        keywords: "postgres sqlite orm query migrate pool",
    },
    DocEntry {
        title: "Turso",
        href: "/docs/integrations/turso",
        keywords: "libsql edge sqlite remote",
    },
    DocEntry {
        title: "Supabase",
        href: "/docs/integrations/supabase",
        keywords: "postgres hosted backend auth",
    },
    DocEntry {
        title: "Auth integration",
        href: "/docs/integrations/auth",
        keywords: "session login jwt middleware cookie",
    },
    DocEntry {
        title: "Validation",
        href: "/docs/integrations/validator",
        keywords: "validator zod submit form dto",
    },
    DocEntry {
        title: "i18n",
        href: "/docs/integrations/i18n",
        keywords: "translation locale fluent gettext rtl dir",
    },
    DocEntry {
        title: "Tailwind CSS",
        href: "/docs/integrations/tailwind",
        keywords: "css styling utility class",
    },
    DocEntry {
        title: "OG Image",
        href: "/docs/integrations/og_image",
        keywords: "open graph social preview meta",
    },
    DocEntry {
        title: "SEO, GEO & AEO",
        href: "/docs/integrations/seo_geo",
        keywords: "seo geo aeo site_url canonical sitemap robots llms gptbot oai-searchbot json-ld faq SeoKit prelude",
    },
    DocEntry {
        title: "AI assistant",
        href: "/docs/integrations/ai_assistant",
        keywords: "cursor skill mcp codex gemini agent install HtmlTheme Popup SeoKit --force",
    },
    DocEntry {
        title: "E2E testing",
        href: "/docs/integrations/e2e",
        keywords: "playwright test integration browser",
    },
];

pub fn search(query: &str) -> Vec<DocEntry> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return ENTRIES.to_vec();
    }
    ENTRIES
        .iter()
        .copied()
        .filter(|e| {
            e.title.to_lowercase().contains(&q)
                || e.href.to_lowercase().contains(&q)
                || e.keywords.to_lowercase().contains(&q)
        })
        .collect()
}
