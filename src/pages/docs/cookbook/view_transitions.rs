use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"View Transitions"</h1>
            <p class="lead">"Wrap page content with " <code>"with_view_transition"</code> " for per-route boundaries. " <code>"NavLink"</code> " SPA navigation uses the View Transitions API automatically."</p>

            {crate::site::demos::cookbook_view_transitions()}

            <h2>"Docs site wiring"</h2>
            <p>
                "This documentation app wraps layout " <code>"Slot"</code> " content in "
                <code>"with_view_transition(path_slug, …)"</code> " and styles "
                <code>"::view-transition-old/new(root)"</code> " in site CSS — try the sidebar or the demo above."
            </p>

            <h2>"Page wrapper"</h2>
            {code_block(r#"pub fn page(_req: FlowRequest) -> View {
    with_view_transition(
        "home",
        vec![Child::View(view! {
            <article class="page">
                <h1>"Home"</h1>
                <p>"Content animates on navigation."</p>
            </article>
        })],
    )
}"#)}

            <h2>"Unique transition names"</h2>
            <p>"Use a distinct name per route (e.g. " <code>"home"</code> ", " <code>"about"</code>") so the browser can cross-fade between pages."</p>

            <h2>"CSS"</h2>
            {code_block(r#"::view-transition-old(root) {
    animation: fade-out 200ms ease;
}
::view-transition-new(root) {
    animation: fade-in 200ms ease;
}"#)}

            <h2>"Skip when a popover is open"</h2>
            <p>
                "Chromium skips the View Transition update callback if a "
                <code>":popover-open"</code>
                " or " <code>"dialog[open]"</code>
                " is on the page. Resuma therefore skips VT in that case, and also on soft "
                <code>"invalidate"</code>
                " / " <code>"loader_poll"</code>
                " (no full-page flash every tick). Do "
                <strong>"not"</strong>
                " wrap " <code>"html[data-theme]"</code>
                " swaps in " <code>"startViewTransition"</code>
                " — see " <a href="/docs/cookbook/theme">"Theme"</a> "."
            </p>

            <h2>"Fallback"</h2>
            <p>"Browsers without View Transitions support render content normally — no polyfill required."</p>
        </>
    }
}
