//! Shared UI for the Resuma documentation site.

pub mod bundle_sizes;
mod chrome;
mod css;
mod demo_actions;
mod demo_shell;
mod docs_copy;
mod docs_search;
mod exec_demo;
mod exec_guide;
mod flow_panel;
mod flow_ui_demo;
mod hero_bg;
mod integration_demos;
pub mod live_demos;
mod pwa;
mod queue_demo;
mod scheduler_demo;
mod seo;
mod server_demo;
mod sidebar;
mod theme;
mod todo_demo;
mod todo_security;
pub use todo_security::install as install_security_middleware;
mod tools_demo;
mod webhook_demo;
mod workers;

pub use webhook_demo::inbox_handler;

pub use chrome::{browse_docs_chip, docs_search_modal, explore_nav};
pub use css::SITE_CSS;
pub use docs_copy::{DocsCopyNavBtn, DocsCopyPageToolbar};
pub use theme::{
    provide_docs_theme, skip_link, theme_picker, theme_sheet, THEME_BOOT, THEME_COOKIE,
};

pub fn site_head() -> String {
    format!("{THEME_BOOT}{}{SITE_CSS}", theme_sheet())
}
pub use docs_search::search;
pub use hero_bg::hero_particles_mount;
pub use live_demos as demos;
pub use pwa::config as pwa_config;
pub use seo::{
    apply_page_seo, json_ld, json_ld_faq, seo_kit, site_description, site_title, site_url,
    view_transition_name,
};
pub use sidebar::doc_sidebar;

use resuma::prelude::*;

pub fn code_block(code: &str) -> View {
    view! {
        <pre class="code"><code>{code.to_string()}</code></pre>
    }
}

pub fn playground_card(title: &str, body: &str, command: &str) -> View {
    view! {
        <article class="playground-card">
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
            <code>{command.to_string()}</code>
        </article>
    }
}

pub fn feature_card(icon: &str, title: &str, body: &str) -> View {
    view! {
        <article class="card">
            <div class="card-icon">{icon.to_string()}</div>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
        </article>
    }
}

pub fn pillar_card(icon: &str, title: &str, body: &str) -> View {
    view! {
        <article class="pillar">
            <div class="pillar-icon">{icon.to_string()}</div>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
        </article>
    }
}

pub fn metric_item(value: &str, label: &str) -> View {
    view! {
        <div class="metric-item">
            <strong>{value.to_string()}</strong>
            <span>{label.to_string()}</span>
        </div>
    }
}

pub fn compare_column(title: &str, body: &str, highlight: bool) -> View {
    let class = if highlight {
        "compare-column compare-column-highlight"
    } else {
        "compare-column"
    };
    view! {
        <article class={class}>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
        </article>
    }
}

pub fn bench_row_full(
    framework: &str,
    initial: &str,
    first_click: &str,
    statik: &str,
    highlight: bool,
) -> View {
    let class = if highlight {
        "bench-row bench-row-highlight"
    } else {
        "bench-row"
    };
    let static_class = if statik == "0 B" { "yes" } else { "" };
    view! {
        <tr class={class}>
            <td><strong>{framework.to_string()}</strong></td>
            <td>{initial.to_string()}</td>
            <td>{first_click.to_string()}</td>
            <td class={static_class}>{statik.to_string()}</td>
        </tr>
    }
}

pub fn doc_link_card(href: &str, title: &str, body: &str, tag: &str) -> View {
    if tag.is_empty() {
        return view! {
            <a href={href.to_string()} class="doc-link-card">
                <h3>{title.to_string()}</h3>
                <p>{body.to_string()}</p>
                <span class="doc-card-arrow">"→"</span>
            </a>
        };
    }
    view! {
        <a href={href.to_string()} class="doc-link-card">
            <span class="doc-card-tag">{tag.to_string()}</span>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
            <span class="doc-card-arrow">"→"</span>
        </a>
    }
}

/// Horizontal bar for landing-page framework size comparisons (width = relative %).
pub fn speed_bar(framework: &str, size: &str, width_pct: u8, highlight: bool) -> View {
    let class = if highlight {
        "speed-bar speed-bar-highlight"
    } else {
        "speed-bar"
    };
    let w = width_pct.min(100);
    view! {
        <div class={class}>
            <div class="speed-bar-head">
                <span class="speed-bar-name">{framework.to_string()}</span>
                <span class="speed-bar-size">{size.to_string()}</span>
            </div>
            <div class="speed-bar-track" aria-hidden="true">
                <div class="speed-bar-fill" style={format!("--speed:{w}")}></div>
            </div>
        </div>
    }
}

/// Layer in the “what ships to the browser” stack on the landing page.
pub fn payload_layer(label: &str, size: &str, detail: &str, accent: bool) -> View {
    let class = if accent {
        "payload-layer payload-layer-accent"
    } else {
        "payload-layer"
    };
    view! {
        <article class={class}>
            <div class="payload-layer-top">
                <strong>{label.to_string()}</strong>
                <span class="payload-layer-size">{size.to_string()}</span>
            </div>
            <p>{detail.to_string()}</p>
        </article>
    }
}

pub fn learn_path_card(step: &str, title: &str, body: &str, href: &str, label: &str) -> View {
    view! {
        <article class="learn-path-card">
            <span class="learn-path-step">{step.to_string()}</span>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
            <a href={href.to_string()} class="learn-path-link">{label.to_string()}</a>
        </article>
    }
}
