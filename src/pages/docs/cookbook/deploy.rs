use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Deploy"</h1>
            <p class="lead">
                "Resuma is a long-running Rust process (Axum), not a JavaScript serverless function. "
                "Any host that can run a Docker image — Fly.io, DigitalOcean, AWS App Runner, "
                "a VM behind Cloudflare, Railway, Render — works. "
                "Not Lambda, not Cloudflare Workers. This documentation site runs on Fly."
            </p>

            {crate::site::demos::cookbook_docker()}

            <h2>"What you are shipping"</h2>
            <ul>
                <li>"One binary that listens on " <code>"0.0.0.0"</code> " and the platform " <code>"PORT"</code> "."</li>
                <li>"Built-in probes: " <code>"GET /health"</code> " (liveness) and " <code>"GET /ready"</code> " (readiness)."</li>
                <li><code>"RESUMA_ENV=production"</code> " — sanitized errors, disk rate limits, stricter origin checks."</li>
                <li>
                    "Behind Fly / App Platform / nginx / Caddy: "
                    <code>"RESUMA_TRUST_PROXY=1"</code> " "
                    <strong>"and"</strong> " "
                    <code>"RESUMA_TRUSTED_PROXY_CIDRS"</code>
                    " — without the CIDR list the process "
                    <strong>"refuses to start"</strong> "."
                </li>
            </ul>
            <p>
                "Fastest scaffold: "
                <code>"resuma new my-app --template production"</code>
                " (Dockerfile + " <code>"fly.toml"</code> "). Then pick a host below."
            </p>

            <h2>"Why there is no Lambda / Workers adapter"</h2>
            <p>
                "Qwik City (including the v2 beta) can ship "
                <code>"qwik add aws-lambda"</code> " because the app "
                <strong>"is"</strong> " a JavaScript module. Vite emits "
                <code>"entry_aws-lambda.tsx"</code> " that exports a "
                <code>"handler(event, context)"</code>
                ". The same source becomes a Worker " <code>"fetch"</code>
                " handler, a Node listener, or a Netlify/Vercel edge function. "
                "Fifteen adapter pages is cheap when each one is a 50-line Vite plugin."
            </p>
            <p>
                "Resuma's entry is " <code>"FlowApp::serve()"</code>
                ": bind a TCP socket, run Tokio + Axum until SIGTERM, write rate-limits and "
                "Resuma OS queues to disk. There is no "
                <code>"fetch(request)"</code>
                " export to wrap. A Lambda/Workers adapter would be a lie — cold start a whole "
                "native binary per request, no durable process, "
                <code>"/tmp"</code> " wiped, cron/workers dead. "
                "Do not use the AWS Lambda Web Adapter for that reason."
            </p>
            <table class="docs-table">
                <thead>
                    <tr>
                        <th>"Qwik City target"</th>
                        <th>"Resuma equivalent"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"AWS Lambda adapter"</td>
                        <td>"App Runner or ECS Fargate + ALB (same Docker image)"</td>
                    </tr>
                    <tr>
                        <td>"Cloudflare Workers / Pages"</td>
                        <td>
                            "Workers/Pages: no. Origin on Fly/VPS + Cloudflare DNS, or Cloudflare Containers. "
                            "Static brochure: " <code>"resuma build --static-export"</code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Vercel / Netlify Edge"</td>
                        <td>"No. Those run JS isolates. Use Fly, Cloud Run, or a VM."</td>
                    </tr>
                    <tr>
                        <td>"Google Cloud Run"</td>
                        <td>"Yes — same Dockerfile, honor " <code>"PORT"</code> ", health " <code>"/health"</code></td>
                    </tr>
                    <tr>
                        <td>"Node / Deno / Bun / Firebase / Azure SWA"</td>
                        <td>"JS runtimes. Skip. Self-host the binary or the image instead."</td>
                    </tr>
                    <tr>
                        <td>"Static / GitHub Pages"</td>
                        <td><code>"resuma build --static-export"</code> " — no " <code>"#[server]"</code> " / " <code>"#[submit]"</code></td>
                    </tr>
                    <tr>
                        <td>"Self-hosting"</td>
                        <td>"Docker + Caddy/nginx, or " <code>"fly.toml"</code></td>
                    </tr>
                </tbody>
            </table>

            <h2>"Bind address — pick one style"</h2>
            <p>
                "Flow reads " <code>"RESUMA_ADDR"</code> " first. If that is unset, it uses "
                <code>"HOST"</code> " + " <code>"PORT"</code> " (default " <code>"127.0.0.1:3000"</code> "). "
                "When " <code>"PORT"</code> " is set, Resuma binds that port exactly — platforms require it."
            </p>
            <table class="docs-table">
                <thead>
                    <tr>
                        <th>"Style"</th>
                        <th>"When"</th>
                        <th>"Trap"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>"HOST=0.0.0.0"</code> " + platform " <code>"PORT"</code></td>
                        <td>"Fly, DigitalOcean, Railway, Render (they inject " <code>"PORT"</code> ")"</td>
                        <td>"Do not also set " <code>"RESUMA_ADDR"</code> " to a " <em>"different"</em> " port"</td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_ADDR=0.0.0.0:8080"</code></td>
                        <td>"Fixed port in Docker / Fly " <code>"internal_port"</code></td>
                        <td>"Must match the platform HTTP port (DO default is 8080)"</td>
                    </tr>
                </tbody>
            </table>

            <h2>"Environment checklist"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"Variable"</th><th>"Production"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"RESUMA_ENV"</code></td><td><code>"production"</code></td></tr>
                    <tr>
                        <td><code>"HOST"</code> " / " <code>"PORT"</code> " or " <code>"RESUMA_ADDR"</code></td>
                        <td>"Listen on all interfaces; port = platform HTTP port"</td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_TRUST_PROXY"</code></td>
                        <td><code>"1"</code> " behind a load balancer"</td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_TRUSTED_PROXY_CIDRS"</code></td>
                        <td>
                            "Required with trust-proxy. Fly: " <code>"fdaa::/16"</code>
                            ". Many private fabrics: " <code>"10.0.0.0/8"</code>
                            ". Local nginx on the same machine: " <code>"127.0.0.1/32"</code>
                        </td>
                    </tr>
                    <tr>
                        <td><code>"SITE_URL"</code></td>
                        <td>"Public origin, no trailing slash (sitemap, OG tags)"</td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_DATA_DIR"</code></td>
                        <td>
                            "Writable dir for the container user (uid 65532). "
                            "Use " <code>"/data"</code> " — " <code>".resuma"</code> " under " <code>"/app"</code>
                            " is not writable after " <code>"USER 65532"</code> "."
                        </td>
                    </tr>
                    <tr>
                        <td><code>"CARGO_MANIFEST_DIR"</code></td>
                        <td><code>"/app"</code> " so " <code>"public/"</code> " resolves in the image"</td>
                    </tr>
                    <tr>
                        <td><code>"RESUMA_EXEC_API_KEY"</code></td>
                        <td>"Only if the app uses " <code>".workers()"</code> " — never commit it"</td>
                    </tr>
                </tbody>
            </table>
            <p>
                "Full matrix: "
                <a href="/docs/security/environment">"Environment variables"</a>
                ". Check locally with " <code>"resuma doctor"</code> "."
            </p>

            <h2>"Dockerfile (all of the hosts below)"</h2>
            <p>
                "Apps from crates.io do " <strong>"not"</strong> " need Node in the image — loader/core JS is embedded in the "
                <code>"resuma"</code>
                " crate. Add a Node stage only if you ship "
                <code>"ClientComponent"</code>
                " TypeScript (this docs site does)."
            </p>
            {code_block(r#"FROM rust:1.91-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/my-app /app/server
COPY --from=builder /app/public /app/public
# Non-root cannot write /app/.resuma — #[server] actions then 500.
RUN mkdir -p /data && chown 65532:65532 /data
ENV RESUMA_ENV=production
ENV RESUMA_ADDR=0.0.0.0:8080
ENV RESUMA_DATA_DIR=/data
ENV CARGO_MANIFEST_DIR=/app
EXPOSE 8080
USER 65532:65532
CMD ["/app/server"]"#)}
            <p>
                "Replace " <code>"my-app"</code> " with the binary name in " <code>"Cargo.toml"</code>
                ". Commit " <code>"Cargo.lock"</code>
                ". Depend on crates.io (or a git tag), not a sibling " <code>"path = \"../resuma\""</code>
                " — remote builders will not see that folder."
            </p>
            <ul>
                <li>
                    <code>"COPY …/public"</code> " — Flow serves "
                    <code>"{CARGO_MANIFEST_DIR}/public"</code>
                    " at runtime (icons, CSS, PWA). Without this copy, those URLs 404. "
                    "Set " <code>"CARGO_MANIFEST_DIR=/app"</code> " so the path matches "
                    <code>"WORKDIR"</code> "."
                </li>
                <li>
                    <code>"RESUMA_DATA_DIR=/data"</code> " must be writable by uid 65532. "
                    "A Fly volume is optional; the directory itself is not. Production rate limits "
                    "write here. If it is missing, every " <code>"#[server]"</code> " action returns 500."
                </li>
                <li>
                    "Put " <code>"RESUMA_TRUST_PROXY"</code> " and CIDRs on the "
                    <strong>"platform"</strong> " (" <code>"fly.toml"</code> "), not only in the image — "
                    "otherwise a local " <code>"docker run"</code> " exits."
                </li>
                <li>
                    "Native extras belong in the image when the app needs them (e.g. ffmpeg, cmake). "
                    "Keep the default image slim."
                </li>
            </ul>
            {code_block(r#"# .dockerignore
/target
.git
.github"#)}
            {code_block(r#"docker build -t my-app .
    docker run --rm -p 8080:8080 \
    -e RESUMA_TRUSTED_PROXY_CIDRS=127.0.0.1/32 \
    -e RESUMA_TRUST_PROXY=1 \
  my-app
curl -fsS http://127.0.0.1:8080/health"#)}

            <h2>"Fly.io"</h2>
            <p>
                "Working Resuma apps on Fly use a fixed " <code>"RESUMA_ADDR=0.0.0.0:8080"</code>
                " (not " <code>"HOST"</code> "+" <code>"PORT"</code> "), "
                <code>"fdaa::/16"</code> ", a writable " <code>"/data"</code>
                ", and " <code>"flyctl deploy --remote-only --ha=false"</code>
                ". Examples: "
                <a href="https://youtubetotext.fly.dev" target="_blank" rel="noopener">"youtubetotext"</a>
                ", "
                <a href="https://placaqr.fly.dev" target="_blank" rel="noopener">"placaqr"</a>
                ", "
                <a href="https://underkb.fly.dev" target="_blank" rel="noopener">"underkb"</a>
                ". This docs site: "
                <a href="https://resuma-docs.fly.dev" target="_blank" rel="noopener">"resuma-docs.fly.dev"</a> "."
            </p>
            {code_block(r#"resuma new my-app --template production
cd my-app
fly launch --no-deploy --ha=false
# Edit fly.toml (snippet below), then:
fly deploy --ha=false
fly open"#)}
            {code_block(r#"# fly.toml
app = "my-app"
primary_region = "iad"

[build]
  dockerfile = "Dockerfile"

[env]
  RESUMA_ENV = "production"
  RESUMA_TRUST_PROXY = "1"
  RESUMA_TRUSTED_PROXY_CIDRS = "fdaa::/16"
  RESUMA_ADDR = "0.0.0.0:8080"
  RESUMA_DATA_DIR = "/data"
  CARGO_MANIFEST_DIR = "/app"
  SITE_URL = "https://my-app.fly.dev"

[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 0
  processes = ["app"]

  [[http_service.checks]]
    grace_period = "20s"
    path = "/health"
    interval = "15s"
    timeout = "2s"

[[vm]]
  size = "shared-cpu-1x"
  memory = "512mb""#)}
            <ul>
                <li>
                    <code>"--ha=false"</code> " — Fly's default is two machines. One is enough and cheaper."
                </li>
                <li>
                    <code>"grace_period = \"20s\""</code>
                    " — the first request after a Rust cold start is slow; without grace the check kills the machine."
                </li>
                <li>
                    <code>"min_machines_running = 0"</code> " + " <code>"auto_stop_machines"</code>
                    " — scale to zero (typical SSR app). Workers / cron / SQLite that must keep running: "
                    <code>"min_machines_running = 1"</code> " and " <code>"auto_stop_machines = false"</code>
                    ", plus a volume (see below)."
                </li>
                <li>
                    "Optional: " <code>"RESUMA_CSP=0"</code> " if you embed YouTube or other third-party frames; "
                    <code>"RESUMA_BODY_LIMIT"</code> " (bytes) for uploads."
                </li>
                <li>
                    "Volume (survives restarts): "
                    <code>"fly volumes create resuma_data --size 1"</code>
                    " and in " <code>"fly.toml"</code> ":"
                </li>
            </ul>
            {code_block(r#"[mounts]
  source = "resuma_data"
  destination = "/data""#)}
            <p>
                "Without a volume, " <code>"/data"</code> " is still required so rate limits can write. "
                "It is just wiped on each new machine."
            </p>

            <h3>"GitHub Actions (push to main)"</h3>
            <p>
                "Create a deploy token once, store it as the repo secret " <code>"FLY_API_TOKEN"</code> ":"
            </p>
            {code_block(r#"fly tokens create deploy -x 999999h -a my-app"#)}
            {code_block(r#"# .github/workflows/fly.yml
name: Fly Deploy
on:
  push:
    branches: [main]
  workflow_dispatch:
jobs:
  deploy:
    name: Deploy app
    runs-on: ubuntu-latest
    concurrency: deploy-group
    steps:
      - uses: actions/checkout@v4
      - uses: superfly/flyctl-actions/setup-flyctl@master
      - run: flyctl deploy --remote-only --ha=false
        env:
          FLY_API_TOKEN: YOUR_FLY_API_TOKEN"#)}
            <p>
                "In the real file use the GitHub secret expression "
                <code>"secrets.FLY_API_TOKEN"</code>
                " (the usual Actions " <code>"secrets.*"</code> " form). "
                <code>"--remote-only"</code> " builds on Fly. "
                <code>"concurrency: deploy-group"</code> " prevents overlapping deploys. "
                "App secrets (" <code>"RESUMA_EXEC_API_KEY"</code> ", API keys) stay in "
                <code>"fly secrets"</code> ", not in GitHub env."
            </p>

            <h2>"DigitalOcean App Platform"</h2>
            <p>
                "App Platform's default HTTP port is " <strong>"8080"</strong>
                ". It injects " <code>"PORT"</code>
                ". Bind " <code>"0.0.0.0"</code>
                ", not localhost. Use a Dockerfile (Rust buildpacks are slower and easier to misconfigure)."
            </p>
            {code_block(r#"# .do/app.yaml — create the app from this spec or paste it in the control panel
name: my-resuma-app
services:
  - name: web
    dockerfile_path: Dockerfile
    http_port: 8080
    health_check:
      http_path: /health
      initial_delay_seconds: 30
      period_seconds: 10
    envs:
      - key: RESUMA_ENV
        value: production
      - key: HOST
        value: "0.0.0.0"
      - key: RESUMA_TRUST_PROXY
        value: "1"
      - key: RESUMA_TRUSTED_PROXY_CIDRS
        value: "10.0.0.0/8"
      - key: SITE_URL
        value: "https://my-resuma-app.ondigitalocean.app"
    # Encrypted secrets (API keys) belong in the control panel, not this file."#)}
            <p>
                "Do " <strong>"not"</strong> " set " <code>"RESUMA_ADDR=0.0.0.0:3000"</code>
                " in the Dockerfile if App Platform expects 8080 — "
                <code>"RESUMA_ADDR"</code> " wins over " <code>"PORT"</code>
                " and health checks fail."
            </p>
            <p>
                "App Platform disks are ephemeral. Rate-limit files and exec queues reset on each deploy. "
                "That is fine for a brochure site. For workers, SQLite, or durable graphs, use a "
                <strong>"Droplet + volume"</strong> " (or Fly volumes) instead."
            </p>

            <h3>"DigitalOcean Droplet (Docker + Caddy)"</h3>
            {code_block(r#"# On the droplet, after docker build:
    docker run -d --name my-app --restart unless-stopped \
    -p 127.0.0.1:8080:8080 \
    -v /var/lib/resuma:/data/resuma \
    -e RESUMA_ENV=production \
    -e HOST=0.0.0.0 \
    -e PORT=8080 \
    -e RESUMA_TRUST_PROXY=1 \
    -e RESUMA_TRUSTED_PROXY_CIDRS=127.0.0.1/32 \
    -e RESUMA_DATA_DIR=/data/resuma \
    -e SITE_URL=https://example.com \
  my-app

# Caddyfile
example.com {
  reverse_proxy 127.0.0.1:8080
}"#)}

            <h2>"Railway, Render, and similar"</h2>
            <p>
                "Same image. They inject " <code>"PORT"</code>
                " (Render often uses 10000). Set " <code>"HOST=0.0.0.0"</code>
                ", " <code>"RESUMA_ENV"</code> ", trust-proxy + a private CIDR "
                "(" <code>"10.0.0.0/8"</code> " is the usual starting point), "
                <code>"SITE_URL"</code>
                ", and an HTTP health path of " <code>"/health"</code> "."
            </p>
            {code_block(r#"# Railway / Render-style env (PORT comes from the platform)
HOST=0.0.0.0
RESUMA_ENV=production
RESUMA_TRUST_PROXY=1
RESUMA_TRUSTED_PROXY_CIDRS=10.0.0.0/8
SITE_URL=https://your-app.up.railway.app"#)}

            <h2>"AWS"</h2>
            <p>
                "Same Docker image. Resuma is a long-running process — "
                <strong>"not"</strong> " Lambda, Amplify Hosting, or API Gateway+Node. "
                "Pick a container or a VM."
            </p>
            <table class="docs-table">
                <thead>
                    <tr><th>"Service"</th><th>"Fit"</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"App Runner"</td>
                        <td>"Closest to Fly. Push the image to ECR, port 8080, HTTP health " <code>"/health"</code> "."</td>
                    </tr>
                    <tr>
                        <td>"ECS Fargate + ALB"</td>
                        <td>"Production default. Mount EFS at " <code>"/data"</code> " if queues/SQLite must survive tasks."</td>
                    </tr>
                    <tr>
                        <td>"Lightsail or EC2"</td>
                        <td>"Same as a Droplet: Docker + Caddy/nginx on " <code>"127.0.0.1"</code> "."</td>
                    </tr>
                    <tr>
                        <td>"Lambda"</td>
                        <td>"Skip. Cold starts and no durable process. Do not wrap Resuma in the Lambda Web Adapter."</td>
                    </tr>
                </tbody>
            </table>
            <p>"App Runner (console or CLI) — after " <code>"docker push"</code> " to ECR:"</p>
            {code_block(r#"# Service
Port: 8080
Health check: HTTP  /health  (not /)

# Runtime env
RESUMA_ENV=production
RESUMA_ADDR=0.0.0.0:8080
RESUMA_TRUST_PROXY=1
RESUMA_TRUSTED_PROXY_CIDRS=10.0.0.0/8
RESUMA_DATA_DIR=/data
CARGO_MANIFEST_DIR=/app
SITE_URL=https://xxxxx.awsapprunner.com"#)}
            <p>
                "App Runner injects " <code>"PORT"</code> " (default 8080). Keep "
                <code>"RESUMA_ADDR"</code> " on that same port. Give the health check 20s+ to allow a Rust boot. "
                "Disk is ephemeral unless you move to ECS+EFS."
            </p>
            <p>
                "ALB / App Runner hop is a private address — " <code>"10.0.0.0/8"</code>
                " is the usual " <code>"RESUMA_TRUSTED_PROXY_CIDRS"</code>
                ". If rate-limit IPs look wrong, tighten to your VPC CIDR."
            </p>

            <h2>"Cloudflare"</h2>
            <p>
                "Resuma does " <strong>"not"</strong> " run on Cloudflare Workers or Pages. "
                "Workers are V8 isolates (JS/WASM); Pages is static files. "
                <code>"resuma build --static-export"</code> " can feed Pages for a brochure site with no "
                <code>"#[server]"</code> " / " <code>"#[submit]"</code>
                ". A real app needs a Linux container or a VM, then Cloudflare in front if you want."
            </p>

            <h3>"DNS / proxy in front of Fly or a VPS (usual path)"</h3>
            <p>
                "Keep the origin on Fly, App Runner, or a Droplet. Point the domain at Cloudflare. "
                "Orange-cloud (proxied) means Cloudflare overwrites " <code>"X-Forwarded-For"</code>
                " — set " <code>"RESUMA_TRUST_PROXY=1"</code> " and put "
                <a href="https://www.cloudflare.com/ips/" target="_blank" rel="noopener">"Cloudflare's published IP ranges"</a>
                " into " <code>"RESUMA_TRUSTED_PROXY_CIDRS"</code>
                " (comma-separated). Grey-cloud (DNS only) leaves TLS to the origin; then you do not need those ranges."
            </p>
            <p>
                "Simpler: " <strong>"Cloudflare Tunnel"</strong> " (" <code>"cloudflared"</code> ") on the same machine as the binary, "
                "proxy to " <code>"http://127.0.0.1:8080"</code> ", and set "
                <code>"RESUMA_TRUSTED_PROXY_CIDRS=127.0.0.1/32"</code> "."
            </p>

            <h3>"Cloudflare Containers"</h3>
            <p>
                "Paid Workers plan. The same Dockerfile runs as a container; a small Worker "
                <code>"fetch"</code> "s it. " <code>"defaultPort"</code> " must be "
                <strong>"8080"</strong> " to match " <code>"RESUMA_ADDR"</code>
                ". Instances sleep when idle (" <code>"sleepAfter"</code> ") — "
                <code>"/data"</code> " is wiped, like Fly with no volume. "
                "Docs: "
                <a href="https://developers.cloudflare.com/containers/" target="_blank" rel="noopener">"developers.cloudflare.com/containers"</a> "."
            </p>
            {code_block(r#"// src/index.js — Worker that forwards to the Resuma container
import { Container, getContainer } from "@cloudflare/containers";

export class Resuma extends Container {
  defaultPort = 8080;
  sleepAfter = "10m";
}

export default {
  async fetch(request, env) {
    return getContainer(env.RESUMA).fetch(request);
  },
};"#)}
            {code_block(r#"{
  "name": "my-resuma-app",
  "main": "src/index.js",
  "compatibility_date": "2026-09-02",
  "containers": [
    { "class_name": "Resuma", "image": "./Dockerfile", "max_instances": 2 }
  ],
  "durable_objects": {
    "bindings": [{ "class_name": "Resuma", "name": "RESUMA" }]
  },
  "migrations": [{ "tag": "v1", "new_sqlite_classes": ["Resuma"] }]
}"#)}
            <p>
                "Set production env on the container (Wrangler secrets / " <code>"envVars"</code>
                " on the class): " <code>"RESUMA_ENV"</code> ", "
                <code>"RESUMA_ADDR=0.0.0.0:8080"</code> ", "
                <code>"RESUMA_DATA_DIR=/data"</code> ", "
                <code>"CARGO_MANIFEST_DIR=/app"</code> ", "
                <code>"SITE_URL"</code> ", and trust-proxy CIDRs for the Cloudflare hop. "
                "Then " <code>"npx wrangler deploy"</code> "."
            </p>

            <h2>"GCP, Azure, and the rest"</h2>
            <p>
                "Cloud Run, Azure Container Apps, Google Compute, Azure VM: same image, "
                "listen on " <code>"0.0.0.0"</code> ", honor " <code>"PORT"</code>
                " when the platform injects it, health " <code>"/health"</code>
                ", writable " <code>"/data"</code> ", "
                <code>"RESUMA_TRUST_PROXY=1"</code> " + the load-balancer CIDR. "
                "If the host can run the Fly Dockerfile, it can run Resuma."
            </p>

            <h2>"If it does not boot"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"Symptom"</th><th>"Cause"</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Process exits mentioning " <code>"RESUMA_TRUSTED_PROXY_CIDRS"</code></td>
                        <td><code>"RESUMA_TRUST_PROXY=1"</code> " without CIDRs"</td>
                    </tr>
                    <tr>
                        <td>"Health check timeout / 502"</td>
                        <td>"Bound to " <code>"127.0.0.1"</code> ", or " <code>"RESUMA_ADDR"</code> " port ≠ platform HTTP port"</td>
                    </tr>
                    <tr>
                        <td>"Docker build cannot find " <code>"resuma"</code></td>
                        <td>"Path dependency; pin crates.io or a git tag"</td>
                    </tr>
                    <tr>
                        <td><code>"#[server]"</code> " / actions return 500"</td>
                        <td>
                            <code>"RESUMA_DATA_DIR"</code> " missing or not writable by the non-root user"
                        </td>
                    </tr>
                    <tr>
                        <td>"Static files / PWA icons 404"</td>
                        <td>
                            "Forgot " <code>"COPY public"</code> " or " <code>"CARGO_MANIFEST_DIR=/app"</code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Two machines / surprise bill"</td>
                        <td>"Omitted " <code>"--ha=false"</code> " on " <code>"fly launch"</code> " / " <code>"fly deploy"</code></td>
                    </tr>
                    <tr>
                        <td>"Health check kills a booting app"</td>
                        <td>"Missing " <code>"grace_period"</code> " (~20s for a Rust binary)"</td>
                    </tr>
                </tbody>
            </table>

            <p>
                <a href="/docs/getting_started">"← Getting started"</a>
                " · "
                <a href="/docs/security/environment">"Environment variables"</a>
                " · "
                <a href="/docs/exec/ops">"Ops & production →"</a>
            </p>
        </>
    }
}
