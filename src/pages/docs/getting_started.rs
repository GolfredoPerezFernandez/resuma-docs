use resuma::prelude::*;

use crate::site::{code_block, playground_card};

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Getting Started"</h1>
            <p class="lead">
                "Resuma is a resumable Rust web framework — no hydration, no eager JS execution. "
                "Components run on the server; a tiny loader resumes interactivity on demand. "
                "Resuma Flow adds file-based pages, loads, and submits in one crate — core and full-stack unified."
            </p>

            <h2>"Examples in this repo"</h2>
            <p>"See " <a href="/docs/examples">"Examples"</a> " for all runnable crates and when to use each."</p>
            <p>
                "Rust apps run on the server — clone the repo and launch a live example in one command:"
            </p>
            <div class="playground-grid">
                {playground_card(
                    "Todo (full showcase)",
                    "Signals, #[server], #[island], js!, theme — every Resuma feature in one app.",
                    "cargo run -p example-todo",
                )}
                {playground_card(
                    "Flow demo (full-stack)",
                    "Loads, submits, streaming SSR, and deferred chunks.",
                    "cargo run -p example-flow-demo",
                )}
            </div>
            <p>
                "Docs are live at "
                <a href="https://resuma-docs.fly.dev/" target="_blank">"resuma-docs.fly.dev"</a>
                ". To hack on the docs site locally: "
                <code>"cargo run"</code> " (from "
                <code>"apps/resuma-docs"</code>", sibling to "
                <code>"apps/resuma"</code>"; not under "
                <code>"examples/"</code>"). "
                "The header Theme control is a "
                <code>"&lt;Popup&gt;"</code>
                "; Search is a "
                <code>"&lt;Modal&gt;"</code>
                "."
            </p>

            <h2>"Prerequisites"</h2>
            <p>"To build Resuma apps locally, you need:"</p>
            <ul>
                <li><a href="https://rustup.rs">"Rust 1.91+"</a>" (stable channel via rustup)"</li>
                <li><a href="https://nodejs.org">"Node.js 18+"</a>" (optional — only to rebuild the JS runtime)"</li>
                <li>"Your favorite editor ("<a href="https://code.visualstudio.com/">"VS Code"</a>" + rust-analyzer recommended)"</li>
            </ul>
            <p>
                "Core Resuma needs " <strong>"no Redis, no database, and no external services"</strong> " — "
                "CSRF, CSP, and rate limiting are built in. Optional SQLx/Postgres is for your app data only. "
                "See " <a href="/docs/faq">"FAQ"</a> " and " <a href="/docs/security">"Security"</a>"."
            </p>
            <p>
                "Optionally, read "
                <a href="/docs/architecture">"How resumability works"</a>
                " before scaffolding."
            </p>

            <h2>"Install the CLI"</h2>
            <p>
                "From "
                <a href="https://crates.io/crates/resuma" target="_blank">"crates.io"</a>
                " (recommended):"
            </p>
            {code_block("cargo install resuma")}
            <p>
                "API reference: "
                <a href="https://docs.rs/resuma" target="_blank">"docs.rs/resuma"</a>
                " · "
                <a href="https://docs.rs/resuma-macros" target="_blank">"docs.rs/resuma-macros"</a>
            </p>
            <p>"From source while developing the monorepo:"</p>
            {code_block(r#"git clone https://github.com/GoldevLab/resuma
cd resuma
cargo install --path crates/resuma --features cli

resuma --help"#)}

            <h2>"Install the AI skill (Cursor / Codex)"</h2>
            <p>
                "Optional but recommended — teaches your editor reactive " <code>"view!"</code>
                ", Flow, " <code>"HtmlTheme"</code>
                ", " <code>"Popup"</code>
                " / " <code>"Modal"</code>
                ", " <code>"SeoKit"</code>
                ", and deploy (not Lambda/Workers):"
            </p>
            {code_block("resuma install skill")}
            <p>
                "After a CLI upgrade, run " <code>"resuma install skill --force"</code>
                " so the skill on disk picks up " <code>"HtmlTheme"</code>
                ", " <code>"Popup"</code>
                " / " <code>"Modal"</code>
                ", and SEO traps. "
                <a href="/docs/integrations/ai_assistant">"Full guide: skill vs MCP, Gemini, team setup →"</a>
            </p>

            <h2>"Create an app using the CLI"</h2>
            <p>
                "Use " <code>"resuma new"</code> " or " <code>"resuma create"</code> " to scaffold a starter. "
                "Pick a template:"
            </p>
            <div class="template-grid">
                <div class="template-pill">
                    <strong>"basic"</strong>
                    <span>"Static SSR · zero client JS"</span>
                </div>
                <div class="template-pill">
                    <strong>"todo"</strong>
                    <span>"Signals · #[server] · #[island] · js!"</span>
                </div>
                <div class="template-pill">
                    <strong>"flow"</strong>
                    <span>"Multi-page · src/pages/ · layouts"</span>
                </div>
                <div class="template-pill">
                    <strong>"flow-booking"</strong>
                    <span>"Appointments · query loaders · SPA date picker"</span>
                </div>
                <div class="template-pill">
                    <strong>"flow-fullstack"</strong>
                    <span>"Flow + SQLx SQLite sample"</span>
                </div>
                <div class="template-pill">
                    <strong>"production"</strong>
                    <span>"Flow + security stub + Dockerfile + fly.toml"</span>
                </div>
            </div>
            {code_block(r#"resuma new my-app                    # interactive menu
resuma new my-app --template basic
resuma new my-app --template todo
resuma new my-app --template flow
resuma new my-app --template flow-booking
resuma new my-app --template flow-fullstack
resuma new my-app --template production

cd my-app"#)}

            <p>"The CLI generates " <code>"Cargo.toml"</code> " and " <code>"src/main.rs"</code> "."</p>

            <h2>"Start the development server"</h2>
            <p>
                "Inside your project directory. "
                <code>"resuma dev"</code> " installs " <code>"cargo-watch"</code> " if needed, rebuilds on save, and refreshes the browser automatically."
            </p>
            {code_block(r#"resuma dev
resuma dev --open          # open browser
resuma dev --kill-stale    # free port if something is stuck (Linux)
# cargo-watch also watches public/ for static file changes"#)}
            <p>"Without the CLI, plain Cargo works too:"</p>
            {code_block("cargo run")}

            <h2>"Hello, Resuma"</h2>
            <p>"A minimal component with resumable state:"</p>
            {code_block(r#"use resuma::prelude::*;

#[component]
fn Hello() {
    let excited = signal(false);
    view! {
        <main>
            <h1>"Hello Resuma"</h1>
            <button onClick={excited.set(true)}>
                "Click me"
            </button>
        </main>
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    ResumaApp::new()
        .component("/", Hello)
        .serve(ServeOptions::default())
        .await
}"#)}

            <h2>"Add a server action"</h2>
            {code_block(r#"#[server]
async fn greet(name: String) -> String {
    format!("Hello, {name}!")
}"#)}
            <p>"From a handler, call " <code>"__resuma.action('greet', [name])"</code> " — RPC at " <code>"POST /_resuma/action/:name"</code>"."</p>

            <h2>"Project structure"</h2>
            <p><strong>"basic / todo"</strong>" — single " <code>"main.rs"</code> " (+ security modules for todo). "<strong>"Flow"</strong>" — add " <code>"src/pages/"</code> " (see " <a href="/docs/project_structure">"Project structure"</a> ")."</p>
            {code_block(r##"my-app/                  # resuma new --template todo
├── Cargo.toml
└── src/
    ├── main.rs
    ├── security.rs
    └── todo_store.rs"##)}

            <h2>"Deploy to production"</h2>
            <p>
                "Local dev needs "
                <strong>"zero env vars"</strong> " — "
                <code>"cargo run"</code> " and " <code>"resuma dev"</code> " work out of the box. "
                "Resuma is a Docker-friendly Rust process: bind " <code>"0.0.0.0"</code>
                ", honor " <code>"PORT"</code> ", set " <code>"RESUMA_ENV=production"</code>
                ". Walkthroughs for Fly.io, DigitalOcean App Platform, Droplets, Railway, and Render: "
                <a href="/docs/cookbook/deploy">"Deploy"</a> "."
            </p>
            <p>
                "Or scaffold " <code>"resuma new my-app --template production"</code>
                " (" <code>"Dockerfile"</code> " + " <code>"fly.toml"</code> "). "
                "Only add " <code>"RESUMA_EXEC_API_KEY"</code> " if the app uses workers. "
                <a href="/docs/security/environment">"Environment variables →"</a>
            </p>

            <h2>"Next steps"</h2>
            <ul>
                <li><a href="/docs/cookbook/theme">"Theme — HtmlTheme + Popup palettes"</a></li>
                <li><a href="/docs/components/popup">"Popup / Modal / Desktop UI"</a></li>
                <li><a href="/docs/integrations/ai_assistant">"AI skill — resuma install skill"</a></li>
                <li><a href="/docs/security/todo">"Todo example — full backend reference"</a></li>
                <li><a href="/docs/security/environment">"Environment variables — deploy checklist"</a></li>
                <li><a href="/docs/flow">"Resuma Flow — multi-page apps"</a></li>
                <li><a href="/docs/exec">"Resuma OS — workers, queue, scheduler"</a></li>
                <li><a href="/docs/package">"Package map"</a></li>
                <li><a href="/docs/architecture">"Architecture"</a></li>
                <li><a href="/docs/cookbook/deploy">"Deploy — Fly, DigitalOcean, AWS, Cloudflare"</a></li>
            </ul>
        </>
    }
}
