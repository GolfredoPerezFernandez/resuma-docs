use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"PWA & static files"</h1>
            <p class="lead">
                "Flow apps ship an installable PWA (manifest + service worker) by default. "
                "Static assets can live in " <code>"public/"</code> " instead of "
                <code>"include_bytes!"</code> "."
            </p>

            <h2>"PWA (on by default)"</h2>
            <p>
                <code>"FlowApp::into_router"</code> " enables PWA from page title/description and precaches static routes. "
                "Opt out:"
            </p>
            {code_block(r#"FlowApp::new()
    .without_pwa()   // or RESUMA_PWA=0 in the environment
    .auto_pages("src/pages", PagesRegistry)
    .serve(FlowServeOptions::default())
    .await"#)}

            <h2>"Customize manifest & precache"</h2>
            {code_block(r##"FlowApp::new()
    .with_pwa(
        FlowPwaConfig::from_page_options("My App", "Tagline", "es", &[])
            .theme("#4f46e5", "#0f172a")
            .shortcut("Book", "Book", "/book")
            .precache_path("/images/hero.jpg"),
    )
    .with_theme_pwa(Theme {
        primary: "#4f46e5".into(),
        background: "#0f172a".into(),
        ..Default::default()
    })"##)}
            <p>
                <code>"with_theme_pwa"</code> " copies " <code>"Theme::primary"</code> " / "
                <code>"background"</code> " into manifest colors when you do not call "
                <code>".theme(...)"</code> " explicitly."
            </p>

            <h2>"Icons from " <code>"public/"</code></h2>
            <p>"Drop PNGs at standard paths — they override generated SVG icons in the manifest:"</p>
            <ul>
                <li><code>"public/icons/icon-192.png"</code> " → " <code>"/icons/icon-192.png"</code></li>
                <li><code>"public/icons/icon-512.png"</code></li>
                <li><code>"public/icons/icon-maskable.png"</code></li>
                <li><code>"public/icons/apple-touch-icon.png"</code></li>
            </ul>

            <h2>"The " <code>"public/"</code> " directory"</h2>
            <p>
                "Files under " <code>"{CARGO_MANIFEST_DIR}/public"</code> " are served at the same URL path "
                "(e.g. " <code>"public/images/logo.png"</code> " → " <code>"/images/logo.png"</code> "). "
                "No " <code>"static_asset"</code> " boilerplate required. Paths are added to the PWA precache list. "
                "In Docker, " <code>"COPY"</code> " that folder into the image and set "
                <code>"CARGO_MANIFEST_DIR=/app"</code> " — see "
                <a href="/docs/cookbook/deploy">"Deploy"</a> "."
            </p>
            {code_block(r#"FlowApp::new()
    .with_public_dir("public")   // optional — this is the default root
    .auto_pages("src/pages", PagesRegistry)"#)}
            <p>
                "Prefer " <code>"public/"</code> " for JPEG/PNG/fonts; keep "
                <code>"static_asset"</code> " for bytes compiled into the binary. "
                "See " <a href="/docs/security/configure">"CSP configure"</a> " for "
                <code>"img-src 'self'"</code> "."
            </p>

            <h2>"Scaffold: booking app"</h2>
            {code_block("resuma new my-salon --template flow-booking")}
            <p>
                "Demonstrates query-driven " <code>"#[load]"</code> " with "
                <a href="/docs/flow/query_params">"loader_refresh_input"</a> "."
            </p>
            <p>
                "This docs site: "
                <a href="/manifest.webmanifest" target="_blank">"/manifest.webmanifest"</a>
            </p>

            <h2>"Online + storage on the client"</h2>
            <p>
                "SSR cannot read " <code>"localStorage"</code> " or "
                <code>"navigator.onLine"</code>
                ". Use " <code>"__resuma.online()"</code> " and "
                <code>"__resuma.storage"</code> " from " <code>"js!"</code>
                " — they no-op when storage is blocked. See "
                <a href="/docs/components/desktop_ui">"Desktop UI"</a> "."
            </p>
        </>
    }
}
