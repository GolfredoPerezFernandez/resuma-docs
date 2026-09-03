//! PWA configuration for the docs site.

use resuma::FlowPwaConfig;

pub fn config() -> FlowPwaConfig {
    FlowPwaConfig {
        name: "Resuma — SSR + Resumability for Rust".into(),
        short_name: "Resuma".into(),
        description: super::seo::site_description().into(),
        theme_color: "#eceff4".into(),
        background_color: "#eceff4".into(),
        start_url: "/".into(),
        scope: "/".into(),
        cache_version: "10".into(),
        display: "standalone".into(),
        orientation: "any".into(),
        lang: "en".into(),
        icon_char: Some("R".into()),
        precache_paths: vec![
            "/docs".into(),
            "/docs/getting_started".into(),
            "/docs/flow".into(),
        ],
        shortcuts: vec![
            resuma::flow::pwa::PwaShortcut {
                name: "Documentation".into(),
                short_name: "Docs".into(),
                url: "/docs".into(),
            },
            resuma::flow::pwa::PwaShortcut {
                name: "Getting Started".into(),
                short_name: "Start".into(),
                url: "/docs/getting_started".into(),
            },
        ],
        offline_title: "You are offline".into(),
        offline_message:
            "Resuma docs need a network connection for the latest pages. Cached content may still be available.".into(),
        manifest_icons: Vec::new(),
    }
}
