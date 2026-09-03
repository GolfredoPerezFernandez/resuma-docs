//! Live Resuma OS worker demo — variants per docs page.

use resuma::prelude::*;
use resuma_flow::{flow_dashboard_poll, flow_styles_link};
use serde_json::{json, Value};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExecDemoMode {
    /// `/docs/exec` — worker + ops dashboard.
    Overview,
    /// `/docs/exec/workers` — focus on `#[worker]` lifecycle (no ops dashboard).
    Workers,
}

#[server]
pub async fn start_docs_showcase(topic: String, blurb: String) -> Result<Value> {
    let started =
        resuma::exec::FlowEngine::start("docs_showcase", json!({ "topic": topic, "blurb": blurb }))
            .await?;
    Ok(json!({
        "graph_id": started.graph_id.0,
        "access_token": started.access_token.unwrap_or_default(),
    }))
}

/// Interactive worker + live execution graph (Resuma OS).
fn exec_showcase_demo_view(mode: ExecDemoMode) -> View {
    let status = resuma::exec::exec_status();
    let show_dashboard = mode == ExecDemoMode::Overview;
    let lead = match mode {
        ExecDemoMode::Overview => view! {
            <p class="exec-demo-lead">
                "A real "
                <code>"#[worker]"</code>
                " runs on this server: durable graph, SSE event stream, pause/resume/cancel. "
                "No Redis — self-hosted execution in the same binary as your app."
            </p>
        },
        ExecDemoMode::Workers => view! {
            <p class="exec-demo-lead">
                "Starts "
                <code>"docs_showcase"</code>
                " on this server — watch logs, progress, pause/resume/cancel, and the final result."
            </p>
        },
    };

    view! {
        <section class="exec-demo" aria-label="Resuma OS worker demo">
            {flow_styles_link()}
            {lead}
            <div class="exec-demo-controls">
                <label class="exec-demo-label" for="exec-topic">
                    "Topic"
                    <input id="exec-topic" type="text" name="exec_topic" value="Resuma OS" />
                </label>
                <label class="exec-demo-label" for="exec-blurb">
                    "Text to analyze"
                    <textarea id="exec-blurb" name="exec_blurb" rows="3">"Durable workers with checkpointed graphs, queue recovery, and an ops dashboard — all in Rust."</textarea>
                </label>
                <button
                    type="button"
                    class="btn btn-primary"
                    id="exec-start-btn"
                    onClick={js!(async () => {
                        const m = await import("/static/client/docs-flow-worker.js?v=1.3.1");
                        await m.runDocsFlowWorker("exec");
                    })}
                >
                    "Run worker"
                </button>
                <p id="exec-err" class="exec-demo-err" role="alert" hidden></p>
            </div>
            {crate::site::exec_guide::worker_try_it_guide("exec")}
            {crate::site::exec_guide::worker_panel_placeholder("exec-flow-placeholder")}
            <div id="exec-flow-slot" class="exec-flow-slot" data-docs-exec-panel hidden></div>
            {if show_dashboard {
                view! {
                    <div class="exec-demo-dash">
                        <h4 class="exec-demo-dash__title">"Ops snapshot"</h4>
                        <p class="demo-muted exec-demo-dash__hint">
                            "From "
                            <code>"flow_dashboard_poll"</code>
                            " — same data as "
                            <code>"GET /_resuma/status"</code>
                            "."
                        </p>
                        {flow_dashboard_poll(4000, Some(status))}
                    </div>
                }
            } else {
                view! { <span hidden aria-hidden="true"></span> }
            }}
        </section>
    }
}

fn exec_showcase_demo_mode(mode: ExecDemoMode) -> View {
    exec_showcase_demo_view(mode)
}

/// Overview — worker + ops dashboard.
pub fn exec_showcase_demo() -> View {
    exec_showcase_demo_mode(ExecDemoMode::Overview)
}

/// Workers page — worker lifecycle without ops dashboard.
pub fn exec_workers_demo() -> View {
    exec_showcase_demo_mode(ExecDemoMode::Workers)
}
