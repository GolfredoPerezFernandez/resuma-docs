//! Live webhook demo — on-site inbox + sample `graph.done` delivery.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use resuma::exec::types::{
    ExecutionPlan, GraphId, GraphNodeSnapshot, GraphSnapshot, GraphStatus, NodeId, NodeKind,
    NodeStatus,
};
use resuma::exec::webhooks::{self, RegisterWebhookBody};
use resuma::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

const INBOX_PATH: &str = "/_resuma/demo/webhook-inbox";
const MAX_INBOX: usize = 20;

#[derive(Clone, Serialize)]
pub struct InboxEntry {
    pub received_ms: u64,
    pub signature: Option<String>,
    pub payload: Value,
}

static INBOX: OnceLock<Mutex<Vec<InboxEntry>>> = OnceLock::new();

fn inbox_store() -> &'static Mutex<Vec<InboxEntry>> {
    INBOX.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn clear_inbox() {
    inbox_store().lock().unwrap().clear();
}

pub fn last_inbox() -> Option<InboxEntry> {
    inbox_store().lock().unwrap().last().cloned()
}

pub async fn inbox_handler(request: Request) -> impl IntoResponse {
    let signature = request
        .headers()
        .get("x-resuma-signature")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);

    let body = match axum::body::to_bytes(request.into_body(), 64 * 1024).await {
        Ok(b) => b,
        Err(_) => return StatusCode::PAYLOAD_TOO_LARGE,
    };

    let payload: Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({ "raw": String::from_utf8_lossy(&body) }));

    let mut inbox = inbox_store().lock().unwrap();
    inbox.push(InboxEntry {
        received_ms: unix_ms(),
        signature,
        payload,
    });
    if inbox.len() > MAX_INBOX {
        let drop = inbox.len() - MAX_INBOX;
        inbox.drain(0..drop);
    }

    StatusCode::NO_CONTENT
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookDemoResult {
    pub ok: bool,
    pub message: String,
    pub webhook_url: String,
    pub signature: Option<String>,
    pub payload: Option<Value>,
}

#[server]
pub async fn docs_webhook_test() -> Result<WebhookDemoResult> {
    let base = crate::site::site_url();
    let webhook_url = format!("{base}{INBOX_PATH}");

    webhooks::register(RegisterWebhookBody {
        url: webhook_url.clone(),
        events: vec!["graph.done".into()],
        enabled: true,
    })?;

    clear_inbox();

    let snap = demo_graph_snapshot();
    webhooks::notify_done(
        &snap,
        1_234,
        Some(json!({
            "demo": true,
            "message": "Webhook test from Resuma docs",
            "topic": "Resuma OS"
        })),
    );

    // Delivery runs on a spawned task — brief wait for the inbox POST.
    tokio::time::sleep(Duration::from_millis(1_200)).await;

    if let Some(entry) = last_inbox() {
        return Ok(WebhookDemoResult {
            ok: true,
            message: "Webhook delivered to the on-site inbox.".into(),
            webhook_url,
            signature: entry.signature,
            payload: Some(entry.payload),
        });
    }

    Ok(WebhookDemoResult {
        ok: false,
        message: "No payload received yet. On localhost, outbound webhooks to loopback are blocked by SSRF — try on resuma-docs.fly.dev.".into(),
        webhook_url,
        signature: None,
        payload: None,
    })
}

fn demo_graph_snapshot() -> GraphSnapshot {
    GraphSnapshot {
        id: GraphId(format!("g_demo_{}", unix_ms())),
        worker: "docs_showcase".into(),
        intent: "webhook demo".into(),
        plan: ExecutionPlan::default(),
        nodes: vec![GraphNodeSnapshot {
            id: NodeId("worker".into()),
            kind: NodeKind::Worker,
            label: "worker".into(),
            status: NodeStatus::Done,
            duration_ms: Some(1_234),
        }],
        edges: vec![],
        status: GraphStatus::Done,
        progress: 100,
    }
}

fn unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Interactive panel for the webhooks docs page.
#[component]
pub fn WebhookDemoWidget() -> View {
    view! {
        <div class="webhook-demo">
            <p class="demo-muted">
                "Registers an on-site inbox at "
                <code>{INBOX_PATH.to_string()}</code>
                ", fires a sample "
                <code>"graph.done"</code>
                " payload (with "
                <code>"X-Resuma-Signature"</code>
                " when "
                <code>"RESUMA_WEBHOOK_SECRET"</code>
                " is set), and shows the JSON received."
            </p>
            <div class="demo-row">
                <button
                    type="button"
                    class="btn btn-primary btn-sm"
                    id="webhook-demo-btn"
                    onClick={js!(async (_event, _state, __resuma) => {
                        const statusEl = document.getElementById("webhook-demo-status");
                        const outEl = document.getElementById("webhook-demo-out");
                        const btn = document.getElementById("webhook-demo-btn");
                        statusEl.textContent = "Sending test webhook…";
                        outEl.textContent = "";
                        btn.disabled = true;
                        const res = await __resuma.safeAction("docs_webhook_test", []);
                        btn.disabled = false;
                        if (!res.ok) {
                            statusEl.textContent = res.error;
                            return;
                        }
                        const v = res.value;
                        statusEl.textContent = v.message + " → " + v.webhook_url;
                        const lines = [];
                        if (v.signature) lines.push("X-Resuma-Signature: " + v.signature);
                        if (v.payload) lines.push(JSON.stringify(v.payload, null, 2));
                        outEl.textContent = lines.join("\n\n") || "(no body)";
                    })}
                >
                    "Send test webhook"
                </button>
            </div>
            <p id="webhook-demo-status" class="webhook-demo-status" aria-live="polite"></p>
            <pre id="webhook-demo-out" class="demo-output webhook-demo-output"></pre>
        </div>
    }
}
