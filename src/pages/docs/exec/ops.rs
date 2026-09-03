use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Ops dashboard & production"</h1>
            <p class="lead">
                "Monitor graphs, queues, and scheduler health. Lock down exec routes and the "
                <code>"/ops"</code> " page before exposing to the internet."
            </p>

            {crate::site::demos::exec_ops()}

            <h2>"Environment variables"</h2>
            <p>
                "The production template needs secrets for exec routes and the "
                <code>"/ops"</code> " dashboard. "
                <code>"RESUMA_ENV"</code> " and " <code>"RESUMA_TRUST_PROXY"</code> " are already in the scaffold "
                <code>"fly.toml"</code> " — set secrets manually:"
            </p>
            <p><a href="/docs/security/environment">"Full environment guide →"</a></p>
            {code_block(r#"fly secrets set RESUMA_EXEC_API_KEY=$(openssl rand -hex 32)
fly secrets set RESUMA_OPS_SESSION=$(openssl rand -hex 32)
# Browser cookie: resuma_session=<RESUMA_OPS_SESSION>

# Mount a persistent volume at /data/resuma (rate-limit, queue, durable graphs)
RESUMA_DATA_DIR=/data/resuma
RESUMA_RATE_BACKEND=disk
RESUMA_SCHEDULER_TICK_SECS=30
RESUMA_METRICS_PUBLIC=0"#)}

            <h2>"Persistent volume"</h2>
            <p>
                "Production apps should mount a volume at " <code>"RESUMA_DATA_DIR"</code> ". "
                "Resuma stores rate-limit counters, exec queues, scheduler jobs, and graph checkpoints on disk — "
                "no Redis. On Fly: " <code>"fly volumes create resuma_data --size 1"</code> " and mount at "
                <code>"/data/resuma"</code> " in " <code>"fly.toml"</code>". "
                "See " <a href="/docs/cookbook/deploy">"Deploy"</a> "."
            </p>

            <h2>"Dashboard"</h2>
            <p>"The production template serves " <code>"/ops"</code> " with "
                <code>"flow_dashboard_poll"</code> " — polls the " <code>"exec_status"</code> " server action "
                "(session + CSRF) or falls back to " <code>"GET /_resuma/status"</code> " with API key."</p>

            <h2>"Auth model"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"Surface"</th><th>"Auth"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"GET /_resuma/status"</code></td><td>"Bearer / X-Resuma-Exec-Key"</td></tr>
                    <tr><td><code>"exec_status" action</code></td><td>"Session admin or API key"</td></tr>
                    <tr><td><code>"/ops" page</code></td><td>"Authenticated session"</td></tr>
                    <tr><td><code>"Graph SSE / controls"</code></td><td>"Graph token + CSRF + same-origin"</td></tr>
                </tbody>
            </table>

            <h2>"Prometheus"</h2>
            {code_block(r#"GET /_resuma/metrics
# HELP resuma_graphs_started_total ...
# Set RESUMA_METRICS_PUBLIC=1 only behind VPC / internal network"#)}

            <h2>"Production middleware"</h2>
            <p>
                "The production template validates " <code>"resuma_session"</code> " against "
                <code>"RESUMA_OPS_SESSION"</code> " — replace the stub in "
                <code>"security.rs"</code> " with your real auth provider before deploy."
            </p>

            <h2>"Deploy"</h2>
            <p>"See " <a href="/docs/cookbook/deploy">"Cookbook → Deploy"</a> " and the "
                <code>"templates/production"</code> " scaffold (" <code>"resuma new --template production"</code> ")."</p>

            <p><a href="/docs/exec">"← Resuma OS overview"</a></p>
        </>
    }
}
