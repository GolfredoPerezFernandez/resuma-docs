use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Virtual For"</h1>
            <p class="lead">
                "Keyed " <code>"<For>"</code> " diffs on the client, but 10k rows still mean 10k DOM nodes. "
                <code>"virtual"</code> " paints a window plus spacers."
            </p>

            {crate::site::demos::components_virtual_for()}

            <h2>"view!"</h2>
            {code_block(r#"view! {
    <div data-r-virtual-scroller="true" style="height: 24rem; overflow: auto">
        <For each={jobs} key="id" virtual itemHeight={48} overscan={6} let:job>
            <div>{job.title.clone()}</div>
        </For>
    </div>
}"#)}

            <ul>
                <li><code>"itemHeight"</code> " — row height in CSS pixels (default 48)."</li>
                <li><code>"overscan"</code> " — extra rows above/below the viewport (default 6)."</li>
                <li><code>"data-r-virtual-scroller"</code> " — scroll container; otherwise the list parent is used."</li>
            </ul>
            <p>
                "SSR renders only the first window so HTML stays small. The client window lives in "
                <code>"/_resuma/ui.js"</code> " — the one runtime path that actually recycles rows."
            </p>
        </>
    }
}
