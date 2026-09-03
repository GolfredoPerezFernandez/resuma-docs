use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Environment variables"</h1>
            <p class="lead">
                "Resuma ships secure defaults for local development — "
                <strong>"you do not need any env vars to run "</strong>
                <code>"cargo run"</code> " or " <code>"resuma dev"</code>". "
                "Production settings are mostly automatic when you scaffold with "
                <code>"resuma new --template production"</code> " (Fly " <code>"fly.toml"</code> " included)."
            </p>

            {crate::site::demos::security_environment()}

            <h2>"What you need, by app type"</h2>
            <table class="docs-table">
                <thead>
                    <tr>
                        <th>"App type"</th>
                        <th>"Local dev"</th>
                        <th>"PaaS / Docker prod"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><strong>"SSR / Flow only"</strong><br /><span class="muted">"No "</span><code>".workers()"</code></td>
                        <td>"Nothing required"</td>
                        <td><code>"RESUMA_ENV"</code> " + " <code>"RESUMA_TRUST_PROXY"</code> " + " <code>"RESUMA_TRUSTED_PROXY_CIDRS"</code><br /><span class="muted">"(see "</span><a href="/docs/cookbook/deploy">"Deploy"</a><span class="muted">")"</span></td>
                    </tr>
                    <tr>
                        <td><strong>"+ Resuma OS workers"</strong><br /><span class="muted">"Queue, scheduler, graphs"</span></td>
                        <td><code>"RESUMA_EXEC_PUBLIC=1"</code><br /><span class="muted">"or set "</span><code>"RESUMA_EXEC_API_KEY"</code></td>
                        <td>"Above + " <code>"RESUMA_EXEC_API_KEY"</code><br /><span class="muted">"(use "</span><code>"fly secrets set"</code><span class="muted">")"</span></td>
                    </tr>
                    <tr>
                        <td><strong>"Production template"</strong><br /><span class="muted">"/ops dashboard"</span></td>
                        <td>"—"</td>
                        <td>"Also " <code>"RESUMA_OPS_SESSION"</code></td>
                    </tr>
                </tbody>
            </table>

            <h2>"The three variables everyone asks about"</h2>

            <h3><code>"RESUMA_RATE_BACKEND"</code> " and " <code>"RESUMA_DATA_DIR"</code></h3>
            <p>
                "Rate limiting is built in — " <strong>"no Redis required"</strong>". "
                "Dev uses an in-memory sliding window; production automatically switches to a "
                <strong>"disk backend"</strong> " under "
                <code>"{RESUMA_DATA_DIR}/rate-limit/"</code> " (exclusive file locks, safe across "
                "multiple processes on the same volume)."
            </p>
            {code_block(r#"RESUMA_RATE_BACKEND=memory   # dev (default)
RESUMA_RATE_BACKEND=disk     # prod (auto when RESUMA_ENV=production)
RESUMA_DATA_DIR=/data        # must be writable by the process user (not /app/.resuma)"#)}
            <p>
                "In Docker, " <code>"mkdir /data && chown 65532:65532 /data"</code>
                " before " <code>"USER 65532"</code>
                ". If this directory is not writable, production rate limits fail and "
                <code>"#[server]"</code> " actions return 500. A Fly volume is only needed if the data must survive a new machine."
            </p>
            <p>
                "Tune limits: " <code>"RESUMA_RATE_ACTIONS=120"</code> ", "
                <code>"RESUMA_RATE_SUBMITS=60"</code>". "
                "Legacy " <code>"RESUMA_RATE_BACKEND=redis"</code> " is ignored — use "
                <code>"disk"</code> " or edge rate limiting for multi-machine deploys."
            </p>

            <h3><code>"RESUMA_ENV=production"</code></h3>
            <p>"Turns on production mode:"</p>
            <ul>
                <li>"Generic error messages to clients (no internal details leaked)"</li>
                <li>"Disk-backed rate limiting (survives restarts, shared across processes on the same volume)"</li>
                <li>"Hides " <code>"/_resuma/benchmark.json"</code></li>
                <li>"Stricter origin validation on form submits and actions"</li>
                <li>"CSP enforced (unless disabled)"</li>
            </ul>
            <p>
                <strong>"When:"</strong> " real deploys only. "
                <strong>"Local:"</strong> " omit it — dev defaults are intentional."
            </p>

            <h3><code>"RESUMA_TRUST_PROXY=1"</code></h3>
            <p>
                "Trust " <code>"X-Forwarded-For"</code> " and " <code>"X-Forwarded-Proto"</code> " from your reverse proxy "
                "(Fly.io, DigitalOcean, nginx, Caddy, Cloudflare). Needed for:"
            </p>
            <ul>
                <li>"Correct HTTPS detection → HSTS, Secure cookies"</li>
                <li>"Real client IP for rate limiting"</li>
            </ul>
            <p>
                "You " <strong>"must"</strong> " also set " <code>"RESUMA_TRUSTED_PROXY_CIDRS"</code>
                " (comma-separated). Without it, " <code>"serve()"</code> " exits. "
                "Fly: " <code>"fdaa::/16"</code> ". "
                "App Platform / Railway-style private fabric: start with " <code>"10.0.0.0/8"</code> ". "
                "nginx on localhost: " <code>"127.0.0.1/32"</code> "."
            </p>
            <p>
                <strong>"When:"</strong> " only behind a reverse proxy. "
                <strong>"Never"</strong> " on bare " <code>"cargo run"</code> " without a proxy in front."
            </p>

            <h3><code>"RESUMA_EXEC_API_KEY"</code></h3>
            <p>
                "Secret for admin exec routes: " <code>"/_resuma/worker"</code> ", " <code>"/_resuma/queue"</code> ", "
                "scheduler, webhooks, metrics, status. Resuma is "
                <strong>"fail-closed"</strong> " — without a key, these routes return 401."
            </p>
            <p>
                <strong>"When:"</strong> " only if your app registers workers via "
                <code>".workers(registry)"</code> ". Pure SSR/Flow apps "
                <strong>"never mount exec routes"</strong> " and do not need this key."
            </p>
            {code_block(r#"openssl rand -hex 32
# Fly:
fly secrets set RESUMA_EXEC_API_KEY=$(openssl rand -hex 32)"#)}
            <p>
                "Send on requests: " <code>"Authorization: Bearer &lt;key&gt;"</code> " or header "
                <code>"X-Resuma-Exec-Key"</code>"."
            </p>

            <h2>"Automatic vs manual"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"Variable"</th><th>"Automatic?"</th><th>"Notes"</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>"RESUMA_ENV"</code></td>
                        <td>"Partial"</td>
                        <td><code>"resuma new --template production"</code> " writes it in " <code>"fly.toml"</code></td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_TRUST_PROXY"</code></td>
                        <td>"Partial"</td>
                        <td>"Same — included in production scaffold"</td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_TRUSTED_PROXY_CIDRS"</code></td>
                        <td>"No"</td>
                        <td>"Required when trust-proxy is on. Fly " <code>"fdaa::/16"</code> " — see " <a href="/docs/cookbook/deploy">"Deploy"</a></td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_EXEC_API_KEY"</code></td>
                        <td>"No"</td>
                        <td>"Must be a unique secret per app. Set via " <code>"fly secrets"</code> " or your platform's secret store"</td>
                    </tr>
                    <tr>
                        <td><code>"CSRF, origin check, CSP"</code></td>
                        <td>"Yes"</td>
                        <td>"On by default — no env vars needed"</td>
                    </tr>
                    <tr>
                        <td><code>"Exec routes "</code><code>"/_resuma/*"</code></td>
                        <td>"Yes"</td>
                        <td>"Only mounted when " <code>".workers()"</code> " is used or " <code>"RESUMA_EXEC_ENABLED=1"</code></td>
                    </tr>
                </tbody>
            </table>

            <h2>"Local development with workers"</h2>
            <p>"For exec routes without managing an API key locally:"</p>
            {code_block(r#"RESUMA_EXEC_PUBLIC=1 cargo run"#)}
            <p>
                "Ignored when " <code>"RESUMA_ENV=production"</code> " — production always requires a real key."
            </p>

            <h2>"Production scaffold"</h2>
            <p>
                <code>"resuma new my-app --template production"</code> " generates a Dockerfile and "
                <code>"fly.toml"</code> ". "
                "Full walkthroughs (Fly, DigitalOcean App Platform, Droplet, Railway, Render): "
                <a href="/docs/cookbook/deploy">"Deploy"</a> "."
            </p>
            {code_block(r#"# fly.toml — already included
[env]
  RESUMA_ENV = "production"
  RESUMA_ADDR = "0.0.0.0:8080"
  RESUMA_TRUST_PROXY = "1"
  RESUMA_TRUSTED_PROXY_CIDRS = "fdaa::/16"
  RESUMA_DATA_DIR = "/data"
  CARGO_MANIFEST_DIR = "/app"

[http_service]
  internal_port = 8080
  force_https = true"#)}
            <p>"Then deploy (" <code>"--ha=false"</code> " = one machine):"</p>
            {code_block(r#"fly launch --no-deploy --ha=false
fly secrets set RESUMA_EXEC_API_KEY=$(openssl rand -hex 32)   # only if using .workers()
fly deploy --ha=false"#)}

            <h2>"Full reference"</h2>
            <p>"Common server variables (CSRF, rate limits, CSP):"</p>
            {code_block(r#"RESUMA_CSRF=1                  # default on
RESUMA_ORIGIN_CHECK=1          # default on
RESUMA_BODY_LIMIT=1048576
RESUMA_RATE_ACTIONS=120
RESUMA_RATE_SUBMITS=60
RESUMA_RATE_BACKEND=memory|disk  # disk auto in production — no Redis
RESUMA_DATA_DIR=/data            # writable; rate-limit/, queue/, durable/
CARGO_MANIFEST_DIR=/app          # Docker: public/ lives next to the binary
RESUMA_CSP=1                   # off in RESUMA_DEV unless RESUMA_CSP_DEV=1; set 0 to embed YouTube/frames
SITE_URL=https://your-app.fly.dev   # public origin (canonical, OG, sitemap, llms.txt); change when you attach a domain"#)}

            <p>"Resuma OS (exec layer):"</p>
            {code_block(r#"RESUMA_DATA_DIR=/data/resuma
RESUMA_EXEC_ENABLED=1          # mount exec routes without .workers()
RESUMA_EXEC_PUBLIC=1           # dev only — open exec routes
RESUMA_METRICS_PUBLIC=0        # set 1 only inside VPC
RESUMA_SCHEDULER_TICK_SECS=30
RESUMA_WEBHOOK_SECRET=...      # HMAC for outbound webhooks"#)}

            <h2>"Check your setup"</h2>
            {code_block("resuma doctor")}
            <p>
                "Reports " <code>"RESUMA_ENV"</code> " status and project health. See also "
                <a href="/docs/security/configure">"Configure security"</a> " (Rust " <code>"SecurityConfig"</code> "), "
                <a href="/docs/exec/security">"Exec security"</a> ", and "
                <a href="/docs/cookbook/deploy">"Deploy"</a>"."
            </p>

            <p>
                <a href="/docs/security">"← Security overview"</a>
                " · "
                <a href="/docs/security/configure">"Configure in Rust →"</a>
            </p>
        </>
    }
}
