use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Popup"</h1>
            <p class="lead">
                "Anchored overlay: the panel follows its trigger and flips "
                <code>"bottom → right → top → left"</code>
                " so it stays on screen. Positioning is CSS, not a JS layout loop. "
                "This site’s "
                <strong>"Theme"</strong>
                " and "
                <strong>"Explore"</strong>
                " controls in the header are the same "
                <code>"&lt;Popup&gt;"</code>
                " — native " <code>"popover"</code> ", no overlay runtime on Chromium."
            </p>

            {crate::site::demos::components_popup()}

            <h2>"view!"</h2>
            {code_block(r#"view! {
    <Popup id="menu" positions="bottom right top left" dismissIfShown>
        <button slot="anchor" type="button">"Menu"</button>
        <div>
            <a href="/docs">"Docs"</a>
        </div>
    </Popup>
}"#)}

            <h2>"Trigger without a slot"</h2>
            <p>
                "If you omit " <code>"slot=\"anchor\""</code>
                ", the first child is the opener. The framework forces "
                <code>"type=\"button\""</code>
                " so a trigger inside a form does not submit. Host "
                <code>"style"</code>
                " is concatenated with " <code>"anchor-name"</code>
                ", not replaced. Native " <code>"toggle"</code>
                " keeps " <code>"aria-expanded"</code>
                " in sync."
            </p>

            <h2>"CSS that breaks the panel"</h2>
            <p>
                "Do not put " <code>"position: relative"</code>
                " or " <code>"overflow: hidden"</code>
                " on the popup " <strong>"panel"</strong>
                " (glass utilities often do). The top layer then fails hit-testing. "
                <code>".r-popup"</code>
                " already sets " <code>"position: absolute"</code>
                " and " <code>"overflow: visible"</code> "."
            </p>

            <h2>"What SSR emits"</h2>
            <p>
                <code>"popovertarget"</code> " + " <code>"[popover=auto]"</code>
                " + " <code>"anchor-name"</code> " / " <code>"position-anchor"</code>
                " / " <code>"position-try-fallbacks"</code>
                ". Chromium needs no overlay JS. Safari without CSS anchor loads "
                <code>"/_resuma/ui.js"</code> " (lazy) to flip with "
                <code>"getBoundingClientRect"</code> "."
            </p>

            <h2>"Optional open signal"</h2>
            {code_block(r#"let open = signal(false);
view! {
    <Popup id="tip" open={open}>
        <button slot="anchor" type="button">"Tip"</button>
        <p>"Hello"</p>
    </Popup>
}"#)}

            <h2>"Programmatic"</h2>
            {code_block(r#"js! { __resuma.showPopup("menu"); }
    js! { __resuma.hidePopup("menu"); }"#)}

            <p>
                "See also " <a href="/docs/components/modal">"Modal"</a>
                " (focus trap + stack) and "
                <a href="/docs/cookbook/portals">"portals"</a>
                " (teleport only — not a dialog API)."
            </p>
        </>
    }
}
