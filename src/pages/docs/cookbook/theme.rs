use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Theme"</h1>
            <p class="lead">
                "The Theme menu in this header is the real API: "
                <code>"FlowApp::with_html_theme"</code>
                " + " <code>"data-r-theme"</code>
                " buttons inside a "
                <code>"&lt;Popup&gt;"</code>
                ". Clicking Slate restyles "
                <code>"html[data-theme]"</code>
                " for the whole site — cookie, " <code>"localStorage"</code>
                ", no hydration, no View Transition around the swap."
            </p>

            {crate::site::demos::cookbook_theme()}

            <h2>"Live palettes (whole app)"</h2>
            <p>
                "Do not put the switcher " <code>"onClick"</code>
                " in a layout handler — that chunk is lazy, so the first click waits. "
                <code>"with_html_theme"</code>
                " injects a blocking head script that listens for "
                <code>"[data-r-theme]"</code>
                ". Style the selected chip with "
                <code>"[aria-pressed=true]"</code>
                " or "
                <code>".r-theme-on"</code>
                " — do not bake a selected class from the SSR cookie (it stays after a live swap)."
            </p>
            {code_block(r##"FlowApp::new()
    .with_html_theme(
        HtmlTheme::new(["paper", "slate", "midnight"])
            .dark(["midnight"])
            .cookie("my_app_theme")       // default: resuma_theme
            .storage_key("my-app-theme")  // default: resuma-theme
    )

// CSS:
// html[data-theme="slate"] { --bg: #f4efe6; --text: #1c1917; }
// body { background: var(--bg); color: var(--text); }

view! {
    <Popup id="themes">
        <button slot="anchor" type="button">"Theme"</button>
        <ThemeSwitch id="slate">"Slate"</ThemeSwitch>
        <button type="button" data-r-theme="midnight">"Midnight"</button>
    </Popup>
}"##)}

            <h2>"Per-response override"</h2>
            <p>
                <code>"set_page_theme(\"slate\")"</code>
                " in a page or " <code>"#[load]"</code>
                " sets SSR " <code>"html[data-theme]"</code>
                " for that response. After the user picks a palette, the boot cookie wins on the next full load."
            </p>

            <h2>"SPA must not clobber a live pick"</h2>
            <p>
                "NavLink prefetch can be older than the palette the user just chose. The runtime copies "
                <code>"dir"</code>
                " from the fetched document (RTL survives) but "
                <strong>"does not"</strong>
                " copy " <code>"data-theme"</code>
                ". Do not wrap the theme swap in " <code>"document.startViewTransition"</code>
                " while a popover is open — Chromium skips the update callback."
            </p>

            <h2>"Snapshot tokens (SSR / Show)"</h2>
            <p>
                <code>"provide_theme"</code> " / " <code>"theme_css_vars"</code>
                " bake inline " <code>"--resuma-*"</code>
                " on a wrapper. That snapshot does "
                <strong>"not"</strong>
                " update when " <code>"html[data-theme]"</code>
                " changes. Use it for a static shell or a "
                <code>"&lt;Show&gt;"</code>
                " branch (demo above)."
            </p>
            {code_block(r##"#[layout("/")]
fn AppLayout() -> View {
    provide_theme(Theme {
        mode: "dark".into(),
        primary: "#6366f1".into(),
        background: "#0b1020".into(),
        foreground: "#e6e8ee".into(),
    });

    view! {
        <div class="app" style={theme_css_vars(&use_theme())}>
            <Slot />
        </div>
    }
}"##)}

            <h2>"Consume in components"</h2>
            {code_block(r#"#[component]
fn ThemedButton() -> View {
    let theme = use_theme();
    view! {
        <button style={format!("background: {}", theme.primary)}>
            "Click"
        </button>
    }
}"#)}

            <h2>"PWA colors from theme"</h2>
            {code_block(r##"FlowApp::new()
    .with_theme_pwa(Theme {
        primary: "#c9a962".into(),
        background: "#0a0908".into(),
        ..Default::default()
    })
    .auto_pages("src/pages", PagesRegistry)"##)}
            <p><a href="/docs/flow/pwa">"PWA & static files"</a>"."</p>

            <h2>"Toggle mode"</h2>
            <p>"Use " <code>"&lt;Show when={dark}&gt;"</code> " with two " <code>"theme_css_vars"</code> " panels — try the live demo above."</p>
            {code_block(r#"let dark = signal(true);

view! {
    <Show when={dark} fallback={light_panel}>
        <div class="app" style={theme_css_vars(&dark_theme)}>...</div>
    </Show>
}"#)}
        </>
    }
}
