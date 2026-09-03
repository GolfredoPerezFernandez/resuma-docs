use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Desktop UI APIs"</h1>
            <p class="lead">
                "Small helpers on " <code>"__resuma"</code>
                ": announce, measure, keyboard vs pointer, text zoom, presence, storage, online, and a focus queue."
            </p>

            {crate::site::demos::components_desktop_ui()}

            <h2>"Accessibility.announce"</h2>
            {code_block(r#"js! {
    await __resuma.action("save", []);
    __resuma.announce("Saved");
}"#)}
            <p>
                "Every interactive page gets a polite " <code>"#r-live"</code>
                " region (" <code>"role=\"status\""</code> "). Prefer this over per-widget "
                <code>"aria-live"</code> "."
            </p>

            <h2>"measure / keyboard / text zoom"</h2>
            {code_block(r#"js! {
    const box = __resuma.measure(event.currentTarget); // { x, y, width, height }
    const kb = __resuma.isNavigatingWithKeyboard();     // html[data-r-nav=keyboard]
    const zoom = __resuma.contentSizeMultiplier();      // html font-size / 16
}"#)}

            <h2>"Presence (idle vs active)"</h2>
            <p>
                <code>"__resuma.presence()"</code> " is " <code>"{ idle, hidden }"</code>
                ". Idle after 60s without pointer/key/scroll, or when the tab is hidden. "
                <code>"loader_poll"</code> " skips ticks while idle so live dashboards do not burn the network. Listen for "
                <code>"resuma:presence"</code> " on " <code>"document"</code> "."
            </p>

            <h2>"Online + Storage (SSR-safe)"</h2>
            {code_block(r#"js! {
    if (!__resuma.online()) { /* offline PWA */ }
    __resuma.storage.set("theme", "dark"); // no-op if localStorage throws
    const t = __resuma.storage.get("theme");
}"#)}
            <p>
                "There is no Rust " <code>"localStorage"</code> " during SSR — keep reads/writes in "
                <code>"js!"</code> " / visible tasks. Pair with "
                <a href="/docs/flow/pwa">"PWA"</a> ". "
                "Document palettes (" <code>"html[data-theme]"</code>
                ") use " <a href="/docs/cookbook/theme">"HtmlTheme"</a>
                ", not " <code>"storage.set(\"theme\")"</code> "."
            </p>

            <h2>"RTL layout"</h2>
            {code_block(r#"FlowApp::new().with_dir("rtl")

// or per request in #[load] / page:
set_page_dir("rtl");
set_page_theme("slate"); // SSR html[data-theme] for this response"#)}
            <p>
                "This sets " <code>"<html dir>"</code>
                ". Use logical CSS (" <code>"margin-inline-start"</code> ", "
                <code>"inset-inline-end"</code> ") instead of left/right. Fluent strings in "
                <code>"#[load]"</code> " do not flip flex by themselves — see "
                <a href="/docs/integrations/i18n">"i18n"</a> "."
            </p>

            <h2>"Focus arbitrator"</h2>
            <p>
                <code>"__resuma.focus(el)"</code>
                " queues " <code>"focus()"</code> " for the next microtask so SPA swaps, popups, and dialogs do not steal focus from each other. "
                <code>"focusMain()"</code> " after navigation uses the same queue."
            </p>
        </>
    }
}
