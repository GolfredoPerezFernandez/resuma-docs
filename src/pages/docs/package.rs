use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Resuma product map"</h1>
            <p class="lead">"One crate to install — layered products like Qwik + Qwik City or Solid + SolidStart."</p>

            {crate::site::demos::reference_package()}

            <h2>"Layers"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"#"</th><th>"Product"</th><th>"Rust"</th><th>"Purpose"</th></tr>
                </thead>
                <tbody>
                    <tr><td>"1"</td><td><strong>"Resuma"</strong></td><td><code>"resuma"</code></td><td>"Signals, view!, resumability, ResumaApp"</td></tr>
                    <tr><td>"2"</td><td><strong>"Resuma Flow"</strong></td><td><code>"resuma::flow"</code></td><td>"FlowApp, pages, loaders, actions"</td></tr>
                    <tr><td>"3"</td><td><strong>"Resuma OS"</strong></td><td><code>"resuma::exec"</code></td><td>"Workers, queue, scheduler, graphs"</td></tr>
                    <tr><td>"4"</td><td><strong>"Resuma Flow UI"</strong></td><td><code>"resuma-flow"</code></td><td>"Ops dashboard + graph widgets"</td></tr>
                    <tr><td>"5"</td><td><strong>"Resuma Macros"</strong></td><td><code>"resuma-macros"</code></td><td>"view!, #[component], #[worker], rs2js"</td></tr>
                    <tr><td>"6"</td><td><strong>"Resuma Runtime"</strong></td><td><code>"runtime/"</code></td><td>"Browser loader + core (/_resuma/*.js)"</td></tr>
                    <tr><td>"7"</td><td><strong>"Resuma Client"</strong></td><td><code>"client/resuma-client.ts"</code></td><td>"TypeScript widgets via ClientComponent"</td></tr>
                    <tr><td>"8"</td><td><strong>"Resuma CLI"</strong></td><td><code>"resuma"</code>" feature "</td><td>"new, dev, build, update"</td></tr>
                </tbody>
            </table>

            <h2>"When to use what"</h2>
            <ul>
                <li><strong>"Resuma"</strong>" — ResumaApp for single-page apps, widgets, resumable UI."</li>
                <li><strong>"Resuma Flow"</strong>" — FlowApp + src/pages/ for multi-page apps."</li>
                <li><strong>"Resuma OS"</strong>" — " <code>"#[worker]"</code> ", queues, scheduler for background work (" <a href="/docs/exec">"docs →"</a> ")."</li>
                <li><strong>"Resuma Flow UI"</strong>" — optional " <code>"resuma-flow"</code> " crate for ops dashboards."</li>
                <li><strong>"Resuma Client"</strong>" — ClientComponent for TypeScript bundles (Three.js, charts)."</li>
            </ul>
            <p>"Users depend on a single crate:"</p>
            {code_block(r#"[dependencies]
resuma = "1.3.1"
tokio  = { version = "1", features = ["full"] }"#)}

            <p>"Everything re-exports through " <code>"resuma::prelude"</code>":"</p>
            {code_block(r#"use resuma::prelude::*;
// ResumaApp, view!, #[component], #[server]
// FlowApp, #[load], #[submit], ClientComponent, client_asset"#)}

            <h2>"Project structure (Resuma Flow)"</h2>
            {code_block(r#"my-app/
  src/
    main.rs           # FlowApp bootstrap
    pages/
      index.rs        # GET /
      about.rs        # GET /about
  client/             # optional — Resuma Client (TypeScript)
    resuma-client.ts
    components/
  Cargo.toml          # resuma + tokio only"#)}

            <h2>"CLI commands"</h2>
            {code_block(r#"cargo install resuma
resuma new my-app                    # static SSR (default)
resuma new my-app --template todo    # full showcase
resuma new my-app --template flow-fullstack  # Flow + SQLx + users CRUD
resuma add sqlx                      # add SQLx to existing project
resuma add turso                     # add Turso/libSQL client
resuma dev
resuma dev --open                    # open browser
resuma build
resuma routes --generate --path src/pages   # Flow apps only"#)}

            <h2>"Published crates"</h2>
            <p>"Only two crates ship on crates.io: " <code>"resuma"</code> " (runtime) and " <code>"resuma-macros"</code> " (proc-macros — required by Rust)."</p>
            <table class="docs-table">
                <thead>
                    <tr><th>"Crate"</th><th>"crates.io"</th><th>"docs.rs"</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>"resuma"</code></td>
                        <td><a href="https://crates.io/crates/resuma" target="_blank">"crates.io/crates/resuma"</a></td>
                        <td><a href="https://docs.rs/resuma" target="_blank">"docs.rs/resuma"</a></td>
                    </tr>
                    <tr>
                        <td><code>"resuma-macros"</code></td>
                        <td><a href="https://crates.io/crates/resuma-macros" target="_blank">"crates.io/crates/resuma-macros"</a></td>
                        <td><a href="https://docs.rs/resuma-macros" target="_blank">"docs.rs/resuma-macros"</a></td>
                    </tr>
                </tbody>
            </table>

            <h2>"API map"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"Concept"</th><th>"Product"</th><th>"API"</th></tr>
                </thead>
                <tbody>
                    <tr><td>"Component"</td><td>"Resuma"</td><td>"#[component] + view!"</td></tr>
                    <tr><td>"Server RPC"</td><td>"Resuma"</td><td>"#[server]"</td></tr>
                    <tr><td>"Server data loader"</td><td>"Resuma Flow"</td><td>"#[load]"</td></tr>
                    <tr><td>"Form mutation"</td><td>"Resuma Flow"</td><td>"#[submit]"</td></tr>
                    <tr><td>"Request middleware"</td><td>"Resuma Flow"</td><td>"#[middleware]"</td></tr>
                    <tr><td>"File-based pages"</td><td>"Resuma Flow"</td><td>"src/pages/"</td></tr>
                    <tr><td>"Background worker"</td><td>"Resuma OS"</td><td>"#[worker]"</td></tr>
                    <tr><td>"Job queue / cron"</td><td>"Resuma OS"</td><td>"enqueue, scheduler HTTP"</td></tr>
                    <tr><td>"TS client widget"</td><td>"Resuma Client"</td><td>"ClientComponent, client_asset()"</td></tr>
                </tbody>
            </table>
        </>
    }
}
