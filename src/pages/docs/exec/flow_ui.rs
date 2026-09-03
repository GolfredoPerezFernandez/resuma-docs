use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Flow UI (resuma-flow)"</h1>
            <p class="lead">
                "Server-rendered ops widgets for graphs, events, and controls. "
                "Add " <code>"resuma-flow"</code> " as a path dependency in the monorepo, or copy components into your app."
            </p>

            {crate::site::demos::exec_flow_ui()}

            <h2>"Cargo.toml"</h2>
            {code_block(r#"[dependencies]
resuma = "1.3.1"
resuma-flow = { path = "../resuma-flow" }  # monorepo
tokio = { version = "1", features = ["full"] }"#)}

            <h2>"Ops dashboard"</h2>
            {code_block(r#"use resuma::prelude::*;
use resuma_flow::{flow_dashboard_poll, flow_styles_link};

#[component]
fn OpsPage(initial: ExecStatus) -> View {
    view! {
        <main>
            {flow_styles_link()}
            {flow_dashboard_poll(5000, Some(initial))}
        </main>
    }
}"#)}

            <h2>"Live graph + controls"</h2>
            {code_block(r#"use resuma_flow::{flow_execution_panel_auth, flow_styles_link};

// access_token from StartWorkerResponse — scopes SSE + pause/resume/cancel
// Pair flow_styles_link() in layout with panel_auth for dynamic injection.
view! {
    {flow_styles_link()}
    {flow_execution_panel_auth(&graph_id, true, Some(access_token))}
}"#)}

            <h2>"Components"</h2>
            <table class="docs-table">
                <thead><tr><th>"Component"</th><th>"Purpose"</th></tr></thead>
                <tbody>
                    <tr><td><code>"flow_dashboard_poll"</code></td><td>"Ops metrics cards (polls exec_status)"</td></tr>
                    <tr><td><code>"flow_graph_auth"</code></td><td>"Node status pills + live refresh"</td></tr>
                    <tr><td><code>"event_stream_auth"</code></td><td>"SSE event timeline"</td></tr>
                    <tr><td><code>"worker_panel_auth"</code></td><td>"Pause / Resume / Cancel / Replay"</td></tr>
                    <tr><td><code>"flow_execution_panel_auth"</code></td><td>"Panel without inline styles (dynamic HTML + flow.css)"</td></tr>
                    <tr><td><code>"flow_styles_link"</code></td><td>"Link to GET /_resuma/flow.css (CSP-safe)"</td></tr>
                    <tr><td><code>"flow_ops_page"</code></td><td>"All-in-one ops layout"</td></tr>
                </tbody>
            </table>

            <h2>"Client runtime"</h2>
            <p>
                "Widgets mount via " <code>"runtime/src/flow.ts"</code> " — included when you serve Flow apps. "
                "Graph mutations send " <code>"x-resuma-csrf"</code> " and graph tokens on same-origin requests."
            </p>

            <p><a href="/docs/exec/tools">"← Tools"</a> " · " <a href="/docs/exec/security">"Exec security →"</a></p>
        </>
    }
}
