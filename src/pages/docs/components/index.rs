use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Components"</h1>
            <p class="lead">"Author UI with the view! macro, signals, and slots — components only execute on the server. Popup, Modal, and HtmlTheme are first-class Rust tags, not a JS overlay kit."</p>

            {crate::site::demos::components_overview()}

            <h2>"Topics"</h2>
            <div class="grid-3">
                <a href="/docs/components/view" class="card" style="text-decoration: none;">
                    <h3>"view!"</h3>
                    <p>"JSX-like templates, attributes, dynamic bindings."</p>
                </a>
                <a href="/docs/components/control_flow" class="card" style="text-decoration: none;">
                    <h3>"Control flow"</h3>
                    <p>"Show, Match, For, if/match in view!."</p>
                </a>
                <a href="/docs/components/signals" class="card" style="text-decoration: none;">
                    <h3>"Signals"</h3>
                    <p>"signal, ReadSignal, WriteSignal — fine-grained reactivity."</p>
                </a>
                <a href="/docs/components/effects" class="card" style="text-decoration: none;">
                    <h3>"Effects"</h3>
                    <p>"use_effect and use_computed for derived state."</p>
                </a>
                <a href="/docs/components/handlers" class="card" style="text-decoration: none;">
                    <h3>"Handlers"</h3>
                    <p>"onClick closures translated to lazy JS chunks."</p>
                </a>
                <a href="/docs/components/islands" class="card" style="text-decoration: none;">
                    <h3>"Islands"</h3>
                    <p>"computed! / effect! for client replay; #[island] optional for heavy widgets."</p>
                </a>
                <a href="/docs/components/client" class="card" style="text-decoration: none;">
                    <h3>"Client (TypeScript)"</h3>
                    <p>"Prebuilt TS widgets — Three.js, charts — via ClientComponent."</p>
                </a>
                <a href="/docs/components/server" class="card" style="text-decoration: none;">
                    <h3>"Server actions"</h3>
                    <p>"#[server] RPC — call Rust from the client."</p>
                </a>
                <a href="/docs/components/js" class="card" style="text-decoration: none;">
                    <h3>"js!"</h3>
                    <p>"Escape hatch for raw client-side JavaScript."</p>
                </a>
                <a href="/docs/components/slots" class="card" style="text-decoration: none;">
                    <h3>"Slots"</h3>
                    <p>"Content projection with named slots."</p>
                </a>
                <a href="/docs/components/nav_link" class="card" style="text-decoration: none;">
                    <h3>"NavLink"</h3>
                    <p>"Active-state navigation links."</p>
                </a>
                <a href="/docs/components/form" class="card" style="text-decoration: none;">
                    <h3>"Form"</h3>
                    <p>"Progressive-enhancement form submits."</p>
                </a>
                <a href="/docs/components/popup" class="card" style="text-decoration: none;">
                    <h3>"Popup"</h3>
                    <p>"Anchored popover — CSS flip, no JS layout loop."</p>
                </a>
                <a href="/docs/components/modal" class="card" style="text-decoration: none;">
                    <h3>"Modal"</h3>
                    <p>"Stacked dialog, focus trap, announce."</p>
                </a>
                <a href="/docs/components/gesture" class="card" style="text-decoration: none;">
                    <h3>"GestureView"</h3>
                    <p>"Pan, pinch, long-press for PWAs."</p>
                </a>
                <a href="/docs/components/virtual_for" class="card" style="text-decoration: none;">
                    <h3>"Virtual For"</h3>
                    <p>"Windowed lists for 10k-row tables."</p>
                </a>
                <a href="/docs/components/desktop_ui" class="card" style="text-decoration: none;">
                    <h3>"Desktop UI"</h3>
                    <p>"announce, measure, presence, storage, RTL."</p>
                </a>
                <a href="/docs/components/store" class="card" style="text-decoration: none;">
                    <h3>"Store"</h3>
                    <p>"Structured reactive state with use_store."</p>
                </a>
                <a href="/docs/components/context" class="card" style="text-decoration: none;">
                    <h3>"Context"</h3>
                    <p>"provide_context and use_context for descendants."</p>
                </a>
                <a href="/docs/components/error_boundary" class="card" style="text-decoration: none;">
                    <h3>"Error boundaries"</h3>
                    <p>"load_boundary and error_boundary for failed data."</p>
                </a>
                <a href="/docs/components/testing" class="card" style="text-decoration: none;">
                    <h3>"Testing"</h3>
                    <p>"Integration tests, render snapshots, E2E."</p>
                </a>
                <a href="/docs/components/tasks" class="card" style="text-decoration: none;">
                    <h3>"Tasks"</h3>
                    <p>"use_task, use_visible_task, use_debounce."</p>
                </a>
            </div>

            <h2>"Quick example"</h2>
            <p>"A minimal component with resumable state:"</p>
            {code_block(r#"#[component]
fn Counter() {
    let count = signal(0);
    view! {
        <button onClick={count.update(|c| *c += 1)}>
            {count}
        </button>
    }
}"#)}
        </>
    }
}
