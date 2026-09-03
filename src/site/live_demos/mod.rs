//! Live demo entry points — one function per documentation topic.

mod widgets;

use crate::site::demo_actions::use_docs_delayed_load;
use crate::site::demo_actions::{DocsCachedData, DocsSearchData};
use crate::site::demo_shell::{live_demo, live_info};
use crate::site::exec_demo::{exec_showcase_demo, exec_workers_demo};
use crate::site::server_demo::server_function_demo;
use resuma::prelude::*;
use resuma_flow::{flow_dashboard_poll, flow_styles_link};
use widgets::*;

// ── Resuma OS ───────────────────────────────────────────────────────────────

pub fn exec_overview() -> View {
    live_demo("Resuma OS — live worker", exec_showcase_demo())
}

pub fn exec_workers() -> View {
    live_demo("Run docs_showcase worker", exec_workers_demo())
}

pub fn exec_queue() -> View {
    live_demo(
        "Enqueue + stats",
        crate::site::queue_demo::QueueDemoWidget::render(
            crate::site::queue_demo::QueueDemoWidgetProps::default(),
        ),
    )
}

pub fn exec_scheduler() -> View {
    live_demo(
        "Cron scheduler",
        crate::site::scheduler_demo::SchedulerDemoWidget::render(
            crate::site::scheduler_demo::SchedulerDemoWidgetProps::default(),
        ),
    )
}

pub fn exec_webhooks() -> View {
    live_demo(
        "Webhooks",
        view! {
            <>
                <p>
                    "Configure outbound hooks at "
                    <code>"GET|POST /_resuma/webhooks"</code>
                    " — fires on graph lifecycle events."
                </p>
                {crate::site::webhook_demo::WebhookDemoWidget::render(
                    crate::site::webhook_demo::WebhookDemoWidgetProps::default(),
                )}
            </>
        },
    )
}

pub fn exec_tools() -> View {
    live_demo(
        "echo tool",
        crate::site::tools_demo::ToolsDemoWidget::render(
            crate::site::tools_demo::ToolsDemoWidgetProps::default(),
        ),
    )
}

pub fn exec_flow_ui() -> View {
    live_demo(
        "resuma-flow widgets",
        crate::site::flow_ui_demo::flow_ui_demo(),
    )
}

pub fn exec_ops() -> View {
    let status = resuma::exec::exec_status();
    live_demo(
        "flow_dashboard_poll",
        view! {
            <>
                <p class="demo-muted">
                    "Live ops cards from "
                    <code>"flow_dashboard_poll"</code>
                    " — polls "
                    <code>"exec_status"</code>
                    " every 5s. HTTP mirror: "
                    <code>"GET /_resuma/status"</code>
                    " · "
                    <code>"GET /_resuma/metrics"</code>
                    " (Prometheus)."
                </p>
                {flow_styles_link()}
                {flow_dashboard_poll(5000, Some(status))}
                <p class="demo-muted exec-demo-hint">
                    "SSR snapshot above updates every 5s when JavaScript loads "
                    <code>"flow.js"</code>
                    " — cards poll "
                    <code>"exec_status"</code>
                    "."
                </p>
            </>
        },
    )
}

pub fn exec_security() -> View {
    live_info(
        "Exec security",
        view! {
            <p>"Graph access tokens, SSRF guards on worker HTTP, and rate limits on " <code>"/_resuma/worker/*"</code></p>
        },
    )
}

// ── Security ────────────────────────────────────────────────────────────────

pub fn security_overview() -> View {
    live_demo(
        "Server action round-trip",
        SecurityServerActionWidget::render(SecurityServerActionWidgetProps::default()),
    )
}

pub fn security_configure() -> View {
    live_demo(
        "SecurityConfig snapshot",
        SecurityEnvWidget::render(SecurityEnvWidgetProps::default()),
    )
}

pub fn security_environment() -> View {
    live_demo(
        "Runtime env (this server)",
        SecurityEnvWidget::render(SecurityEnvWidgetProps::default()),
    )
}

pub fn security_server_actions() -> View {
    live_demo(
        "#[server] RPC",
        SecurityServerActionWidget::render(SecurityServerActionWidgetProps::default()),
    )
}

pub fn security_middleware() -> View {
    live_demo(
        "Action middleware session",
        WhoAmIWidget::render(WhoAmIWidgetProps::default()),
    )
}

pub fn security_authorization() -> View {
    live_demo(
        "Row-level checks (403)",
        crate::site::todo_demo::TodoDemoWidget::render(
            crate::site::todo_demo::TodoDemoWidgetProps::default(),
        ),
    )
}

pub fn security_backend() -> View {
    live_info(
        "Interactive demo",
        view! {
            <p>
                "Row-level checks and validation live on "
                <a href="/docs/security/authorization">"Authorization & RLS"</a>
                " and "
                <a href="/docs/security/todo">"Todo example"</a>
                " — this page maps patterns to files."
            </p>
        },
    )
}

pub fn security_todo() -> View {
    live_demo(
        "Server-backed todos",
        crate::site::todo_demo::TodoDemoWidget::render(
            crate::site::todo_demo::TodoDemoWidgetProps::default(),
        ),
    )
}

// ── Components ──────────────────────────────────────────────────────────────

pub fn components_overview() -> View {
    live_demo(
        "Signals + handlers",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

pub fn components_view() -> View {
    live_demo(
        "view! counter",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

pub fn components_control_flow() -> View {
    live_demo(
        "<Show> toggle",
        ShowWidget::render(ShowWidgetProps::default()),
    )
}

pub fn components_popup() -> View {
    live_demo(
        "<Popup> anchored",
        PopupWidget::render(PopupWidgetProps::default()),
    )
}

pub fn components_modal() -> View {
    live_demo(
        "<Modal> focus trap",
        ModalWidget::render(ModalWidgetProps::default()),
    )
}

pub fn components_gesture() -> View {
    live_demo(
        "<GestureView>",
        GestureWidget::render(GestureWidgetProps::default()),
    )
}

pub fn components_virtual_for() -> View {
    live_demo(
        "<For virtual>",
        VirtualForWidget::render(VirtualForWidgetProps::default()),
    )
}

pub fn components_desktop_ui() -> View {
    live_demo(
        "__resuma.announce / measure",
        DesktopUiWidget::render(DesktopUiWidgetProps::default()),
    )
}

pub fn components_signals() -> View {
    live_demo(
        "signal()",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

pub fn components_effects() -> View {
    live_demo(
        "effect! / computed",
        EffectsWidget::render(EffectsWidgetProps::default()),
    )
}

pub fn components_error_boundary() -> View {
    live_demo(
        "error_boundary()",
        ErrorBoundaryWidget::render(ErrorBoundaryWidgetProps::default()),
    )
}

pub fn components_handlers() -> View {
    live_demo(
        "onClick handler",
        HandlersWidget::render(HandlersWidgetProps::default()),
    )
}

pub fn components_islands() -> View {
    live_demo("#[island]", island_demo())
}

pub fn components_client() -> View {
    live_info(
        "TypeScript client",
        view! {
            <p>"Client components mount via " <code>"ClientComponent"</code> " — see homepage hero particles."</p>
            {JsWidget::render(JsWidgetProps::default())}
        },
    )
}

pub fn components_server() -> View {
    live_demo(
        "#[server]",
        ServerActionWidget::render(ServerActionWidgetProps::default()),
    )
}

pub fn components_js() -> View {
    live_demo("js! handlers", JsWidget::render(JsWidgetProps::default()))
}

pub fn components_slots() -> View {
    live_demo("Slots", slots_widget())
}

pub fn components_nav_link() -> View {
    live_demo("NavLink SPA", nav_link_widget())
}

pub fn components_form() -> View {
    live_demo(
        "#[submit] Form",
        GreetFormWidget::render(GreetFormWidgetProps::default()),
    )
}

pub fn flow_submits() -> View {
    live_demo(
        "#[submit] handler",
        view! {
            <>
                <p class="demo-muted">"Same " <code>"docs_greet"</code> " handler — focus here is the submit endpoint and JSON response."</p>
                {GreetFormWidget::render(GreetFormWidgetProps::default())}
            </>
        },
    )
}

pub fn components_store() -> View {
    live_demo(
        "use_store",
        StoreWidget::render(StoreWidgetProps::default()),
    )
}

pub fn components_context() -> View {
    live_demo("provide_context", context_widget())
}

pub fn components_tasks() -> View {
    live_demo(
        "visible_task!",
        VisibleTaskWidget::render(VisibleTaskWidgetProps::default()),
    )
}

pub fn components_testing() -> View {
    live_info(
        "Testing strategy",
        view! {
            <>
                <p>"Unit-test signals and stores in Rust. Integration-test " <code>"#[server]"</code> " and " <code>"#[load]"</code> " via axum " <code>"TestServer"</code> ". Snapshot SSR HTML for regressions."</p>
                <p><a href="/docs/integrations/e2e">"E2E testing →"</a> " Playwright specs for full browser flows."</p>
            </>
        },
    )
}

// ── Flow ────────────────────────────────────────────────────────────────────

pub fn flow_overview() -> View {
    live_demo("Flow + server fn", server_function_demo())
}

pub fn flow_routing() -> View {
    live_info(
        "File-based routes",
        view! {
            <>
                <p>"This page is " <code>"src/pages/docs/flow/routing.rs"</code> " → " <code>"/docs/flow/routing"</code></p>
                <div class="demo-row">
                    <NavLink href="/docs/flow/pages" activeClass="active">"Pages"</NavLink>
                    <NavLink href="/docs/flow/loaders" activeClass="active">"Loaders"</NavLink>
                </div>
            </>
        },
    )
}

pub fn flow_query_params(req: &FlowRequest) -> View {
    let q = req.query_param("q").unwrap_or("");
    let data = match try_use_load::<DocsSearchData>("docs_search") {
        Ok(d) => d,
        Err(_) => DocsSearchData {
            query: q.to_string(),
            results: vec![],
        },
    };
    live_demo(
        "#[load] + query string",
        view! {
            <>
                <form method="get" action="/docs/flow/query_params" class="demo-row">
                    <input type="search" name="q" value={q.to_string()} placeholder="Search (min 2 chars)" />
                    <button type="submit" class="btn btn-sm">"Search"</button>
                </form>
                <p class="demo-output">"Query: " {data.query.clone()}</p>
                <ul>
                    {data.results.iter().map(|r| view! { <li>{r.clone()}</li> }).collect::<Vec<_>>()}
                </ul>
            </>
        },
    )
}

pub fn flow_pages() -> View {
    live_info(
        "Pages registry",
        view! {
            <p><code>"resuma routes --generate --path src/pages"</code> " wires every " <code>"pub fn page"</code></p>
        },
    )
}

pub fn flow_layouts() -> View {
    live_info(
        "Layouts",
        view! {
            <p>"This page uses the " <code>"/docs"</code> " layout with sidebar — defined in " <code>"main.rs"</code></p>
        },
    )
}

pub fn flow_loaders() -> View {
    let cached = match try_use_load::<DocsCachedData>("docs_cached") {
        Ok(d) => d,
        Err(_) => DocsCachedData {
            value: "Loader unavailable outside Flow scope".into(),
            timestamp: String::new(),
        },
    };
    live_demo(
        "#[load] with cache",
        view! {
            <>
                <p class="demo-output">{cached.value.clone()}</p>
                <p class="demo-muted">{"Loaded at: "}{cached.timestamp.clone()}</p>
            </>
        },
    )
}

pub fn flow_middleware() -> View {
    live_info(
        "Middleware",
        view! {
            <p>"Request logged by Flow middleware — check server stdout."</p>
        },
    )
}

pub fn flow_endpoints() -> View {
    live_demo(
        "#[server] endpoint",
        ServerActionWidget::render(ServerActionWidgetProps::default()),
    )
}

pub fn flow_errors() -> View {
    live_demo(
        "Error boundary",
        ErrorBoundaryWidget::render(ErrorBoundaryWidgetProps::default()),
    )
}

pub fn flow_caching() -> View {
    let cached = match try_use_load::<DocsCachedData>("docs_cached") {
        Ok(d) => d,
        Err(_) => DocsCachedData {
            value: "Loader unavailable outside Flow scope".into(),
            timestamp: String::new(),
        },
    };
    live_demo(
        "Cache-Control on #[load]",
        view! {
            <>
                <p class="demo-output">{"SSR loader value: "}{cached.value.clone()}</p>
                <p class="demo-muted">{"SSR loaded at: "}{cached.timestamp.clone()}</p>
                {CacheControlWidget::render(CacheControlWidgetProps::default())}
            </>
        },
    )
}

pub fn flow_prefetch() -> View {
    live_info(
        "NavLink prefetch",
        view! {
            <>
                <p>"Hover a NavLink — route HTML prefetches before click."</p>
                <NavLink href="/docs/flow/routing" activeClass="active">"Hover me →"</NavLink>
            </>
        },
    )
}

// ── Integrations ────────────────────────────────────────────────────────────

pub fn integrations_overview() -> View {
    live_demo(
        "CLI scaffold preview",
        crate::site::integration_demos::ScaffoldDemoWidget::render(
            crate::site::integration_demos::ScaffoldDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_sqlx() -> View {
    live_demo(
        "Mock SQLx query",
        crate::site::integration_demos::SqlxDemoWidget::render(
            crate::site::integration_demos::SqlxDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_turso() -> View {
    live_demo(
        "Mock Turso / libSQL",
        crate::site::integration_demos::TursoDemoWidget::render(
            crate::site::integration_demos::TursoDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_supabase() -> View {
    live_demo(
        "Mock Supabase RLS",
        crate::site::integration_demos::SupabaseDemoWidget::render(
            crate::site::integration_demos::SupabaseDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_auth() -> View {
    live_demo(
        "Session middleware",
        WhoAmIWidget::render(WhoAmIWidgetProps::default()),
    )
}

pub fn integrations_validator() -> View {
    live_demo(
        "Submit validation",
        crate::site::integration_demos::ValidationDemoWidget::render(
            crate::site::integration_demos::ValidationDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_i18n() -> View {
    live_demo(
        "Locale strings",
        crate::site::integration_demos::I18nDemoWidget::render(
            crate::site::integration_demos::I18nDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_tailwind() -> View {
    live_demo(
        "Utility classes",
        crate::site::integration_demos::TailwindDemoWidget::render(
            crate::site::integration_demos::TailwindDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_og_image() -> View {
    live_demo(
        "OG meta tags",
        crate::site::integration_demos::OgImageDemoWidget::render(
            crate::site::integration_demos::OgImageDemoWidgetProps::default(),
        ),
    )
}

pub fn integrations_e2e() -> View {
    live_demo(
        "E2E ping",
        crate::site::integration_demos::E2eDemoWidget::render(
            crate::site::integration_demos::E2eDemoWidgetProps::default(),
        ),
    )
}

// ── Cookbook ────────────────────────────────────────────────────────────────

pub fn cookbook_overview() -> View {
    live_demo(
        "Debounced input",
        DebounceWidget::render(DebounceWidgetProps::default()),
    )
}

pub fn cookbook_debouncer() -> View {
    live_demo(
        "Debouncer",
        DebounceWidget::render(DebounceWidgetProps::default()),
    )
}

pub fn cookbook_portals() -> View {
    live_demo(
        "<Modal> (prefer over portal())",
        ModalWidget::render(ModalWidgetProps::default()),
    )
}

pub fn cookbook_view_transitions() -> View {
    live_demo(
        "View transitions",
        ViewTransitionsWidget::render(ViewTransitionsWidgetProps::default()),
    )
}

pub fn cookbook_theme() -> View {
    live_demo(
        "provide_theme",
        ThemeWidget::render(ThemeWidgetProps::default()),
    )
}

pub fn cookbook_streaming_loaders() -> View {
    streaming_loader_panel()
}

/// Live `#[load(stream)]` panel — used on the cookbook streaming page.
pub fn streaming_loader_panel() -> View {
    match use_docs_delayed_load() {
        LoadValue::Pending => view! {
            <section class="live-demo" aria-label="Live demo">
                <div class="live-demo-header">
                    <span class="live-demo-badge">"LIVE"</span>
                    <h2 class="live-demo-title">"Streaming loader"</h2>
                </div>
                <div class="live-demo-body">
                    <p class="demo-muted">"Shell rendered immediately — deferred chunk below (~600ms)."</p>
                    <div class="stream-skeleton" aria-busy="true">"Loading streamed data…"</div>
                    {stream_slot("docs_delayed")}
                </div>
            </section>
        },
        LoadValue::Ok(msg) => view! {
            <section class="live-demo" aria-label="Live demo">
                <div class="live-demo-header">
                    <span class="live-demo-badge">"LIVE"</span>
                    <h2 class="live-demo-title">"Streaming loader"</h2>
                </div>
                <div class="live-demo-body">
                    <p class="demo-output">{msg.clone()}</p>
                </div>
            </section>
        },
        LoadValue::Err(e) => view! {
            <section class="live-demo live-demo-info" aria-label="Live demo">
                <div class="live-demo-body">
                    <p class="demo-error">{e.message.clone()}</p>
                </div>
            </section>
        },
    }
}

pub fn cookbook_prg(req: &FlowRequest) -> View {
    let flash = flash_message(req);
    live_demo(
        "PRG submit",
        view! {
            <>
                {if let Some(msg) = flash {
                    view! {
                        <p class="demo-alert demo-alert--success" role="status">"✓ " {msg}</p>
                    }
                } else {
                    View::empty()
                }}
                <Form submit={crate::site::demo_actions::docs_prg}>
                    <label>"Item" <input name="item" type="text" required=true placeholder="e.g. Widget" /></label>
                    <button type="submit" class="btn btn-sm">"Submit → redirect"</button>
                </Form>
            </>
        },
    )
}

pub fn cookbook_loader_invalidation() -> View {
    live_demo(
        "invalidate()",
        LoaderInvalidationWidget::render(LoaderInvalidationWidgetProps::default()),
    )
}

pub fn cookbook_docker() -> View {
    live_demo(
        "Deploy runtime",
        DeployInfoWidget::render(DeployInfoWidgetProps::default()),
    )
}

// ── Reference ───────────────────────────────────────────────────────────────

pub fn reference_architecture() -> View {
    live_demo(
        "Resumability pipeline",
        PipelineWidget::render(PipelineWidgetProps::default()),
    )
}

pub fn reference_reactivity() -> View {
    live_demo(
        "Signal → effect chain",
        ReactivityWidget::render(ReactivityWidgetProps::default()),
    )
}

pub fn reference_package() -> View {
    live_info(
        "Install",
        view! {
            <p><code>"cargo add resuma@1.3.1"</code> " · " <a href="https://crates.io/crates/resuma" target="_blank">"crates.io"</a></p>
        },
    )
}

pub fn reference_cli() -> View {
    live_info(
        "CLI commands",
        view! {
            <ul>
                <li><code>"resuma new"</code></li>
                <li><code>"resuma dev"</code></li>
                <li><code>"resuma routes --generate"</code></li>
            </ul>
        },
    )
}

pub fn reference_api() -> View {
    live_info(
        "docs.rs",
        view! {
            <p><a href="https://docs.rs/resuma" target="_blank">"docs.rs/resuma"</a> " — full Rust API reference."</p>
        },
    )
}
