use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Client components (TypeScript)"</h1>
            <p class="lead">"Ship prebuilt TypeScript widgets alongside resumable Rust UI — Three.js, charts, editors — without forcing them through rs2js or " <code>"#[island]"</code> "."</p>

            {crate::site::demos::components_client()}

            <h2>"When to use"</h2>
            <ul>
                <li>"Heavy third-party JS (Three.js, WebGPU, D3, Monaco)"</li>
                <li>"Canvas/WebGL widgets with their own render loop"</li>
                <li>"Code you prefer to author in TypeScript rather than " <code>"js!"</code></li>
            </ul>
            <p>"For counters, forms, filters, and most UI — stick with " <code>"#[component]"</code> " + resumable handlers. Client components complement resumability; they do not replace it."</p>

            <h2>"Product map"</h2>
            <div class="table-wrap">
                <table>
                    <thead>
                        <tr>
                            <th>"Layer"</th>
                            <th>"Brand name"</th>
                            <th>"Rust / TS"</th>
                            <th>"Role"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Core"</td>
                            <td><strong>"Resuma"</strong></td>
                            <td><code>"resuma"</code></td>
                            <td>"Signals, view!, resumability, runtime"</td>
                        </tr>
                        <tr>
                            <td>"Full-stack"</td>
                            <td><strong>"Resuma Flow"</strong></td>
                            <td><code>"resuma::flow"</code></td>
                            <td>"Pages, routing, loaders, actions"</td>
                        </tr>
                        <tr>
                            <td>"Compile-time"</td>
                            <td><strong>"Resuma Macros"</strong></td>
                            <td><code>"resuma-macros"</code></td>
                            <td>"view!, #[component], rs2js"</td>
                        </tr>
                        <tr>
                            <td>"Browser runtime"</td>
                            <td><strong>"Resuma Runtime"</strong></td>
                            <td><code>"runtime/"</code></td>
                            <td>"loader.js + core.js resumability"</td>
                        </tr>
                        <tr>
                            <td>"TS widgets"</td>
                            <td><strong>"Resuma Client"</strong></td>
                            <td><code>"client/resuma-client.ts"</code></td>
                            <td>"bootClientComponent mount contract"</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <h2>"1. TypeScript entry"</h2>
            {code_block(r#"// client/components/hero-particles.ts
import { bootClientComponent } from '../resuma-client';

function initHeroParticles(root: HTMLElement) {
  // mount canvas, WebGPU/WebGL, return cleanup
  return () => { /* dispose */ };
}

bootClientComponent('hero-particles', initHeroParticles);"#)}

            <h2>"2. Rust mount point"</h2>
            {code_block(r#"use resuma::prelude::*;

FlowApp::new()
    .client_asset("hero-particles", include_bytes!("../static/client/hero-particles.js"))
    .page("/", || view! {
        {client_component(
            ClientComponent::new("hero-particles")
                .class("hero-particles")
                .lazy(true)
        )}
    })"#)}

            <h2>"3. Build client bundles"</h2>
            {code_block(r#"npm install
npm run build:client   # esbuild: client/components/*.ts → static/client/*.js
cargo run"#)}

            <h2>"Mount contract"</h2>
            <p>"Rust emits a mount root + CSP-nonce module script (production CSP requires the nonce on external scripts, 1.0.2+):"</p>
            {code_block(r#"<div data-r-client="hero-particles" id="r-client-hero-particles" class="hero-particles"></div>
    <script type="module" src="/static/client/hero-particles.js" defer nonce="…"></script>"#)}
            <p>"Optional props via " <code>"ClientComponent::props(...)"</code> " → " <code>"data-r-client-props"</code> " JSON on the root element."</p>

            <h2>"Live example"</h2>
            <p>"This docs site uses " <code>"hero-particles"</code> " on the " <a href="/">"home page"</a> " with " <code>".lazy(true)"</code> " — the Three.js bundle waits for idle so it does not compete with LCP. WebGPU with WebGL fallback, from " <code>"client/components/hero-particles.ts"</code> "."</p>

            <h2>"See also"</h2>
            <ul>
                <li><a href="/docs/components/islands">"Islands"</a> " — lazy resumable boundaries for Rust UI"</li>
                <li><a href="/docs/components/js">"js!"</a> " — inline client JS from Rust handlers"</li>
                <li><a href="/docs/package">"Package overview"</a> " — Resuma vs Resuma Flow vs Macros"</li>
            </ul>
        </>
    }
}
