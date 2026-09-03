//! Site-wide theming — palettes live on `html[data-theme]` via Resuma [`HtmlTheme`].
//!
//! The navbar picker is a `<Popup>`; buttons use `data-r-theme` so the framework
//! boot restyles the whole document (not a layout handler chunk).

use resuma::cookie_value;
use resuma::current_request;
use resuma::prelude::*;

pub const THEME_COOKIE: &str = "resuma_docs_theme";
const THEME_POPUP: &str = "docs-theme";
const THEME_PANEL_ID: &str = "r-popup-docs-theme";

#[derive(Clone, Copy)]
pub struct ThemeSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub scheme: &'static str,
    pub primary: &'static str,
    pub background: &'static str,
    pub foreground: &'static str,
    pub swatch_a: &'static str,
    pub swatch_b: &'static str,
    pub swatch_c: &'static str,
}

pub const THEMES: &[ThemeSpec] = &[
    ThemeSpec {
        id: "paper",
        name: "Paper",
        scheme: "light",
        primary: "#1e293b",
        background: "#eceff4",
        foreground: "#0f172a",
        swatch_a: "#eceff4",
        swatch_b: "#2563eb",
        swatch_c: "#0f172a",
    },
    ThemeSpec {
        id: "slate",
        name: "Slate",
        scheme: "light",
        primary: "#44403c",
        background: "#f4efe6",
        foreground: "#1c1917",
        swatch_a: "#f4efe6",
        swatch_b: "#c2410c",
        swatch_c: "#44403c",
    },
    ThemeSpec {
        id: "midnight",
        name: "Midnight",
        scheme: "dark",
        primary: "#818cf8",
        background: "#0b1020",
        foreground: "#e6e8ee",
        swatch_a: "#0b1020",
        swatch_b: "#818cf8",
        swatch_c: "#e6e8ee",
    },
    ThemeSpec {
        id: "ember",
        name: "Ember",
        scheme: "dark",
        primary: "#e8a87c",
        background: "#1a100c",
        foreground: "#f5e6d3",
        swatch_a: "#1a100c",
        swatch_b: "#e8a87c",
        swatch_c: "#c9a962",
    },
    ThemeSpec {
        id: "aurora",
        name: "Aurora",
        scheme: "dark",
        primary: "#22d3ee",
        background: "#0a1628",
        foreground: "#e0f2fe",
        swatch_a: "#0a1628",
        swatch_b: "#22d3ee",
        swatch_c: "#a78bfa",
    },
    ThemeSpec {
        id: "forest",
        name: "Forest",
        scheme: "dark",
        primary: "#34d399",
        background: "#0c1410",
        foreground: "#e8f5e9",
        swatch_a: "#0c1410",
        swatch_b: "#34d399",
        swatch_c: "#6ee7b7",
    },
];

pub fn spec(id: &str) -> &'static ThemeSpec {
    THEMES.iter().find(|t| t.id == id).unwrap_or(&THEMES[0])
}

/// Cookie from the incoming request (SSR). The blocking head script still wins
/// for first paint via `localStorage`.
pub fn theme_id_from_request() -> &'static str {
    let raw = current_request()
        .and_then(|r| r.header("cookie").map(str::to_string))
        .unwrap_or_default();
    let Some(id) = cookie_value(&raw, THEME_COOKIE) else {
        return "paper";
    };
    if THEMES.iter().any(|t| t.id == id) {
        THEMES
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.id)
            .unwrap_or("paper")
    } else {
        "paper"
    }
}

pub fn provide_docs_theme() {
    let t = spec(theme_id_from_request());
    provide_theme(Theme {
        mode: t.scheme.into(),
        primary: t.primary.into(),
        background: t.background.into(),
        foreground: t.foreground.into(),
    });
}

/// Site chrome boot (browse sheet, search shortcut). Document theme is
/// [`HtmlTheme`] in `main.rs` — `[data-r-theme]` clicks restyle `<html>`.
pub const THEME_BOOT: &str = r##"<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<script>
(function () {
  var CHROME = ["r-popup-docs-theme", "r-popup-docs-explore", "docs-browse", "r-modal-docs-search"];
  function hideEl(el) {
    if (!el) return;
    try {
      if (el.tagName === "DIALOG" && el.open && el.close) el.close();
      else if (el.hidePopover) el.hidePopover();
    } catch (e) {}
  }
  function hideChrome() {
    CHROME.forEach(function (id) { hideEl(document.getElementById(id)); });
  }
  function syncDocsBrowseMode() {
    var el = document.getElementById("docs-browse");
    var compact = window.matchMedia("(max-width: 959px)").matches;
    if (el) {
      if (compact) {
        if (!el.hasAttribute("popover")) el.setAttribute("popover", "auto");
      } else if (el.hasAttribute("popover")) {
        try { if (el.hidePopover) el.hidePopover(); } catch (e) {}
        el.removeAttribute("popover");
      }
    }
    if (!compact) hideEl(document.getElementById("r-popup-docs-explore"));
  }
  function onReady(fn) {
    if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", fn);
    else fn();
  }
  onReady(syncDocsBrowseMode);
  document.addEventListener("keydown", function (ev) {
    if (ev.key !== "/" || ev.ctrlKey || ev.metaKey || ev.altKey) return;
    var ae = ev.target;
    var tag = (ae && ae.tagName) || "";
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT" || (ae && ae.isContentEditable)) return;
    var dlg = document.getElementById("r-modal-docs-search");
    if (!dlg || typeof dlg.showModal !== "function") return;
    ev.preventDefault();
    if (!dlg.open) dlg.showModal();
    var q = dlg.querySelector("input[type=search], input[name=q]");
    if (q && q.focus) q.focus();
  });
  try {
    window.matchMedia("(max-width: 959px)").addEventListener("change", syncDocsBrowseMode);
  } catch (e) {}
  document.addEventListener("toggle", function (ev) {
    var t = ev.target;
    if (!t || !t.hasAttribute || !t.hasAttribute("popover") || !t.id) return;
    var open = false;
    try { open = t.matches(":popover-open"); } catch (e) { open = t.classList.contains("r-popover-open"); }
    document.querySelectorAll('[popovertarget="' + t.id + '"]').forEach(function (btn) {
      btn.setAttribute("aria-expanded", open ? "true" : "false");
    });
  }, true);
  document.addEventListener("resuma:navigate", function () {
    hideChrome();
    syncDocsBrowseMode();
  });
})();
</script>"##;

pub fn skip_link() -> View {
    view! {
        <a href="#main-content" class="skip-link">"Skip to content"</a>
    }
}

pub fn theme_picker() -> View {
    let current = theme_id_from_request();
    view! {
        <div class="theme-wrap">
            <Popup id={THEME_POPUP} positions="bottom left top right" class="theme-popover">
                <button
                    slot="anchor"
                    type="button"
                    class="theme-btn"
                    aria-haspopup="dialog"
                    aria-controls={THEME_PANEL_ID}
                    aria-expanded="false"
                    aria-label="Change site theme"
                    title="Themes"
                >
                    <span class="theme-btn-wheel" aria-hidden="true"></span>
                    <span class="theme-btn-label">"Theme"</span>
                </button>
                <div class="theme-popover-body" role="dialog" aria-label="Choose a theme">
                    <p class="theme-popover-kicker">"Live themes"</p>
                    <p class="theme-popover-lead">"Same pages. Instant restyle — Resuma signals stay resumed."</p>
                    <div class="theme-grid" role="group" aria-label="Site palettes">
                        {theme_option(&THEMES[0], current)}
                        {theme_option(&THEMES[1], current)}
                        {theme_option(&THEMES[2], current)}
                        {theme_option(&THEMES[3], current)}
                        {theme_option(&THEMES[4], current)}
                        {theme_option(&THEMES[5], current)}
                    </div>
                </div>
            </Popup>
        </div>
    }
}

fn theme_option(t: &ThemeSpec, current: &str) -> View {
    let swatch = format!(
        "background: conic-gradient({a} 0 120deg, {b} 120deg 240deg, {c} 240deg 360deg)",
        a = t.swatch_a,
        b = t.swatch_b,
        c = t.swatch_c
    );
    let pressed = if t.id == current { "true" } else { "false" };
    view! {
        <button
            type="button"
            class="theme-opt"
            data-r-theme={t.id.to_string()}
            aria-pressed={pressed.to_string()}
            popovertarget={THEME_PANEL_ID}
            popovertargetaction="hide"
        >
            <span class="theme-swatch" style={swatch}></span>
            <span class="theme-opt-copy">
                <strong>{t.name.to_string()}</strong>
                <span>{t.scheme.to_string()}</span>
            </span>
        </button>
    }
}

/// Palette overrides + chrome that must live beside `SITE_CSS`.
pub const THEME_SHEET: &str = r#"<style>
:root {
  accent-color: var(--accent);
  scrollbar-color: var(--muted) var(--bg);
  scrollbar-width: thin;
  --text-muted: var(--muted);
  --on-primary: #fff;
  --primary-btn-hover: rgba(15, 23, 42, 0.72);
  --panel: rgba(255, 255, 255, 0.55);
  --panel-soft: rgba(255, 255, 255, 0.36);
  --panel-strong: rgba(255, 255, 255, 0.78);
  --panel-input: rgba(255, 255, 255, 0.62);
  --edge: rgba(255, 255, 255, 0.65);
  --edge-strong: rgba(255, 255, 255, 0.92);
  --spec: rgba(255, 255, 255, 0.95);
  --hairline: rgba(15, 23, 42, 0.07);
  --ink-from: #1e293b;
  --ink-to: #64748b;
  --ink-grad: linear-gradient(145deg, var(--ink-from), var(--ink-to));
  --accent-word: linear-gradient(135deg, var(--ink-from) 0%, var(--ink-to) 100%);
  --track: rgba(255, 255, 255, 0.45);
  --focus-ring: rgba(37, 99, 235, 0.16);
  --placeholder: #94a3b8;
  --blob-hi: rgba(255, 255, 255, 0.95);
  --blob-mid: rgba(251, 207, 232, 0.25);
  --blob-lo: rgba(186, 230, 253, 0.2);
  --blob-fill: linear-gradient(155deg, rgba(255, 255, 255, 0.48), rgba(255, 255, 255, 0.08));
  --blob-border: rgba(255, 255, 255, 0.65);
  --cta-veil: linear-gradient(135deg, rgba(255,255,255,0.9) 0%, rgba(241,245,249,0.4) 50%, rgba(255,255,255,0.7) 100%);
  --modal-scrim: rgba(248, 250, 252, 0.55);
  --ok-bg: rgba(236, 253, 245, 0.75);
  --ok-border: rgba(167, 243, 208, 0.8);
  --err-bg: rgba(254, 242, 242, 0.75);
  --err-border: rgba(254, 202, 202, 0.8);
  --success-on: #047857;
  --glass-fill: rgba(255, 255, 255, 0.18);
  --glass-bloom: rgba(255, 255, 255, 0.2);
  --glass-edge-hi: rgba(255, 255, 255, 0.82);
  --glass-inset: rgba(255, 255, 255, 0.5);
  --glass-lift: 0 8px 32px rgba(15, 23, 42, 0.1);
  --glass-blur: 42px;
  --glass-saturate: 180%;
  --surface: rgba(255, 255, 255, 0.52);
  --surface-strong: rgba(255, 255, 255, 0.86);
  --header-bg: rgba(255, 255, 255, 0.38);
  --code-bg: rgba(255, 255, 255, 0.55);
  --nav-hover: rgba(255, 255, 255, 0.55);
  --canvas-veil: rgba(255, 255, 255, 0.72);
  --canvas-hi: #f4f6fa;
  --canvas-mid: #eceff4;
  --canvas-lo: #e4e9f0;
  --orb: rgba(255, 255, 255, 0.85);
  --link-hover: #1d4ed8;
  --btn-fill: rgba(255, 255, 255, 0.18);
  --header-border: rgba(255, 255, 255, 0.5);
  --primary-btn: rgba(15, 23, 42, 0.58);
}
@media (prefers-contrast: more) {
  :root { scrollbar-color: var(--text) var(--bg); }
}
@media (prefers-color-scheme: dark) {
  :root:not([data-theme]) {
    color-scheme: dark;
    --bg: #0b1020;
    --bg-subtle: rgba(255, 255, 255, 0.04);
    --bg-card: rgba(255, 255, 255, 0.06);
    --border: rgba(255, 255, 255, 0.08);
    --border-glass: rgba(255, 255, 255, 0.12);
    --text: #e6e8ee;
    --muted: #94a3b8;
    --text-muted: #94a3b8;
    --primary: #a5b4fc;
    --primary-hover: #c7d2fe;
    --primary-soft: rgba(129, 140, 248, 0.14);
    --accent: #818cf8;
    --accent-soft: rgba(129, 140, 248, 0.14);
    --success: #34d399;
    --grid-line: rgba(255, 255, 255, 0.06);
    --glass-shadow: 0 10px 40px rgba(0, 0, 0, 0.35), 0 2px 10px rgba(0, 0, 0, 0.2), inset 0 1px 1px rgba(255,255,255,0.08);
    --glass-shadow-lg: 0 28px 80px rgba(0, 0, 0, 0.45), inset 0 1px 1px rgba(255,255,255,0.1);
    --glass-highlight: linear-gradient(145deg, rgba(255,255,255,0.12), rgba(255,255,255,0.02));
    --iridescent: linear-gradient(135deg, rgba(129,140,248,0.35), rgba(56,189,248,0.15), rgba(255,255,255,0.08));
    --surface: rgba(15, 23, 42, 0.55);
    --surface-strong: rgba(8, 12, 24, 0.92);
    --header-bg: rgba(8, 12, 24, 0.62);
    --code-bg: rgba(255, 255, 255, 0.06);
    --nav-hover: rgba(255, 255, 255, 0.08);
    --canvas-veil: rgba(8, 12, 24, 0.78);
    --canvas-hi: #12182c;
    --canvas-mid: #0b1020;
    --canvas-lo: #070a14;
    --orb: rgba(129, 140, 248, 0.28);
    --link-hover: #a5b4fc;
    --btn-fill: rgba(255, 255, 255, 0.06);
    --header-border: rgba(255, 255, 255, 0.08);
    --primary-btn: rgba(99, 102, 241, 0.72);
    --on-primary: #eef2ff;
    --primary-btn-hover: rgba(99, 102, 241, 0.9);
    --panel: rgba(255, 255, 255, 0.09);
    --panel-soft: rgba(255, 255, 255, 0.05);
    --panel-strong: rgba(15, 23, 42, 0.88);
    --panel-input: rgba(255, 255, 255, 0.1);
    --edge: rgba(255, 255, 255, 0.12);
    --edge-strong: rgba(255, 255, 255, 0.2);
    --spec: rgba(255, 255, 255, 0.14);
    --hairline: rgba(255, 255, 255, 0.08);
    --ink-from: #a5b4fc;
    --ink-to: #818cf8;
    --track: rgba(255, 255, 255, 0.08);
    --focus-ring: rgba(129, 140, 248, 0.28);
    --placeholder: #64748b;
    --blob-hi: rgba(165, 180, 252, 0.22);
    --blob-mid: rgba(56, 189, 248, 0.12);
    --blob-lo: rgba(129, 140, 248, 0.08);
    --blob-fill: linear-gradient(155deg, rgba(129,140,248,0.16), rgba(15,23,42,0.08));
    --blob-border: rgba(255, 255, 255, 0.1);
    --cta-veil: linear-gradient(135deg, rgba(129,140,248,0.18), rgba(8,12,24,0.4), transparent);
    --modal-scrim: rgba(8, 12, 24, 0.72);
    --ok-bg: rgba(16, 185, 129, 0.12);
    --ok-border: rgba(52, 211, 153, 0.28);
    --err-bg: rgba(239, 68, 68, 0.12);
    --err-border: rgba(248, 113, 113, 0.28);
    --success-on: #6ee7b7;
    --glass-fill: rgba(15, 23, 42, 0.48);
    --glass-bloom: rgba(129, 140, 248, 0.08);
    --glass-edge-hi: rgba(255, 255, 255, 0.32);
    --glass-inset: rgba(255, 255, 255, 0.16);
    --glass-lift: 0 10px 36px rgba(0, 0, 0, 0.45);
    --glass-bright: 1;
  }
}
html[data-theme="paper"] {
  color-scheme: light;
  --bg: #eceff4; --bg-subtle: rgba(255,255,255,0.42); --bg-card: rgba(255,255,255,0.52);
  --border: rgba(15,23,42,0.06); --border-glass: rgba(255,255,255,0.72);
  --text: #0f172a; --muted: #64748b; --text-muted: #64748b; --primary: #1e293b; --primary-hover: #0f172a;
  --primary-soft: rgba(255,255,255,0.65); --accent: #2563eb; --accent-soft: rgba(37,99,235,0.08);
  --success: #059669; --grid-line: rgba(15,23,42,0.07);
  --surface: rgba(255,255,255,0.52); --surface-strong: rgba(255,255,255,0.9); --header-bg: rgba(255,255,255,0.38); --code-bg: rgba(255,255,255,0.55);
  --nav-hover: rgba(255,255,255,0.55); --canvas-veil: rgba(255,255,255,0.72);
  --canvas-hi: #f4f6fa; --canvas-mid: #eceff4; --canvas-lo: #e4e9f0; --orb: rgba(255,255,255,0.85);
  --link-hover: #1d4ed8; --btn-fill: rgba(255,255,255,0.18); --header-border: rgba(255,255,255,0.5);
  --primary-btn: rgba(15,23,42,0.58); --on-primary: #fff; --primary-btn-hover: rgba(15,23,42,0.74);
  --panel: rgba(255,255,255,0.55); --panel-soft: rgba(255,255,255,0.36); --panel-strong: rgba(255,255,255,0.82); --panel-input: rgba(255,255,255,0.7);
  --edge: rgba(255,255,255,0.65); --edge-strong: rgba(255,255,255,0.95); --spec: rgba(255,255,255,0.95); --hairline: rgba(15,23,42,0.07);
  --ink-from: #1e293b; --ink-to: #64748b; --track: rgba(255,255,255,0.45); --focus-ring: rgba(37,99,235,0.16); --placeholder: #94a3b8;
  --blob-hi: rgba(255,255,255,0.95); --blob-mid: rgba(251,207,232,0.25); --blob-lo: rgba(186,230,253,0.2);
  --blob-fill: linear-gradient(155deg, rgba(255,255,255,0.48), rgba(255,255,255,0.08)); --blob-border: rgba(255,255,255,0.65);
  --cta-veil: linear-gradient(135deg, rgba(255,255,255,0.9) 0%, rgba(241,245,249,0.4) 50%, rgba(255,255,255,0.7) 100%);
  --modal-scrim: rgba(248,250,252,0.55); --ok-bg: rgba(236,253,245,0.75); --ok-border: rgba(167,243,208,0.8);
  --err-bg: rgba(254,242,242,0.75); --err-border: rgba(254,202,202,0.8); --success-on: #047857;
  --glass-fill: rgba(255,255,255,0.2); --glass-bloom: rgba(255,255,255,0.22); --glass-edge-hi: rgba(255,255,255,0.85); --glass-inset: rgba(255,255,255,0.55);
  --glass-lift: 0 8px 32px rgba(15,23,42,0.1);
  --glass-shadow: 0 10px 40px rgba(15,23,42,0.06), 0 2px 10px rgba(15,23,42,0.03), inset 0 1px 1px rgba(255,255,255,0.95);
  --glass-shadow-lg: 0 28px 80px rgba(15,23,42,0.08), inset 0 1px 1px rgba(255,255,255,1);
  --glass-highlight: linear-gradient(145deg, rgba(255,255,255,0.92), rgba(255,255,255,0.2) 42%, rgba(255,255,255,0.55));
  --iridescent: linear-gradient(135deg, rgba(255,255,255,0.95), rgba(186,230,253,0.35), rgba(251,207,232,0.28), rgba(167,243,208,0.22), rgba(255,255,255,0.88));
}
html[data-theme="slate"] {
  color-scheme: light;
  --bg: #f4efe6; --bg-subtle: rgba(255,250,240,0.5); --bg-card: rgba(255,252,245,0.62);
  --border: rgba(68,64,60,0.1); --border-glass: rgba(255,248,235,0.8);
  --text: #1c1917; --muted: #78716c; --text-muted: #78716c; --primary: #44403c; --primary-hover: #1c1917;
  --primary-soft: rgba(255,248,235,0.7); --accent: #c2410c; --accent-soft: rgba(194,65,12,0.1);
  --success: #047857; --grid-line: rgba(68,64,60,0.08);
  --surface: rgba(255,252,245,0.58); --surface-strong: rgba(255,250,240,0.92); --header-bg: rgba(255,250,240,0.5); --code-bg: rgba(255,248,235,0.7);
  --nav-hover: rgba(255,237,213,0.7); --canvas-veil: rgba(255,248,235,0.7);
  --canvas-hi: #fffaf3; --canvas-mid: #f4efe6; --canvas-lo: #e7ddd0; --orb: rgba(251,191,36,0.25);
  --link-hover: #9a3412; --btn-fill: rgba(255,247,237,0.35); --header-border: rgba(231,221,208,0.8);
  --primary-btn: rgba(194,65,12,0.82); --on-primary: #fff7ed; --primary-btn-hover: rgba(154,52,18,0.88);
  --panel: rgba(255,248,235,0.62); --panel-soft: rgba(255,247,237,0.4); --panel-strong: rgba(255,252,245,0.88); --panel-input: rgba(255,250,240,0.78);
  --edge: rgba(255,237,213,0.85); --edge-strong: rgba(255,247,237,0.98); --spec: rgba(255,255,255,0.9); --hairline: rgba(68,64,60,0.1);
  --ink-from: #44403c; --ink-to: #c2410c; --track: rgba(255,237,213,0.55); --focus-ring: rgba(194,65,12,0.2); --placeholder: #a8a29e;
  --blob-hi: rgba(255,247,237,0.95); --blob-mid: rgba(254,215,170,0.35); --blob-lo: rgba(253,186,116,0.2);
  --blob-fill: linear-gradient(155deg, rgba(255,247,237,0.5), rgba(253,186,116,0.08)); --blob-border: rgba(255,237,213,0.8);
  --cta-veil: linear-gradient(135deg, rgba(255,247,237,0.92), rgba(254,215,170,0.28), rgba(255,255,255,0.7));
  --modal-scrim: rgba(250,245,235,0.6); --ok-bg: rgba(236,253,245,0.75); --ok-border: rgba(167,243,208,0.8);
  --err-bg: rgba(254,242,242,0.75); --err-border: rgba(254,202,202,0.8); --success-on: #047857;
  --glass-fill: rgba(255,247,237,0.22); --glass-bloom: rgba(254,215,170,0.18); --glass-edge-hi: rgba(255,255,255,0.8); --glass-inset: rgba(255,255,255,0.5);
  --glass-lift: 0 8px 32px rgba(68,64,60,0.12);
  --glass-shadow: 0 10px 40px rgba(68,64,60,0.08), inset 0 1px 1px rgba(255,255,255,0.9);
  --glass-shadow-lg: 0 28px 80px rgba(68,64,60,0.1), inset 0 1px 1px rgba(255,255,255,0.95);
  --glass-highlight: linear-gradient(145deg, rgba(255,247,237,0.92), rgba(254,215,170,0.2));
  --iridescent: linear-gradient(135deg, rgba(255,247,237,0.95), rgba(254,215,170,0.35), rgba(253,186,116,0.2), rgba(255,255,255,0.8));
}
html[data-theme="midnight"] {
  color-scheme: dark;
  --bg: #0b1020; --bg-subtle: rgba(255,255,255,0.04); --bg-card: rgba(255,255,255,0.06);
  --border: rgba(255,255,255,0.08); --border-glass: rgba(255,255,255,0.12);
  --text: #e6e8ee; --muted: #94a3b8; --text-muted: #94a3b8; --primary: #a5b4fc; --primary-hover: #c7d2fe;
  --primary-soft: rgba(129,140,248,0.14); --accent: #818cf8; --accent-soft: rgba(129,140,248,0.16);
  --success: #34d399; --grid-line: rgba(255,255,255,0.055);
  --surface: rgba(15,23,42,0.58); --surface-strong: rgba(8,12,24,0.92); --header-bg: rgba(8,12,24,0.66); --code-bg: rgba(255,255,255,0.06);
  --nav-hover: rgba(129,140,248,0.14); --canvas-veil: rgba(8,12,24,0.78);
  --canvas-hi: #141a30; --canvas-mid: #0b1020; --canvas-lo: #06080f; --orb: rgba(129,140,248,0.3);
  --link-hover: #c7d2fe; --btn-fill: rgba(255,255,255,0.06); --header-border: rgba(255,255,255,0.08);
  --primary-btn: rgba(99,102,241,0.78); --on-primary: #eef2ff; --primary-btn-hover: rgba(99,102,241,0.92);
  --panel: rgba(255,255,255,0.09); --panel-soft: rgba(255,255,255,0.05); --panel-strong: rgba(15,23,42,0.88); --panel-input: rgba(255,255,255,0.1);
  --edge: rgba(255,255,255,0.12); --edge-strong: rgba(255,255,255,0.2); --spec: rgba(255,255,255,0.14); --hairline: rgba(255,255,255,0.08);
  --ink-from: #a5b4fc; --ink-to: #818cf8; --track: rgba(255,255,255,0.08); --focus-ring: rgba(129,140,248,0.28); --placeholder: #64748b;
  --blob-hi: rgba(165,180,252,0.22); --blob-mid: rgba(56,189,248,0.12); --blob-lo: rgba(129,140,248,0.08);
  --blob-fill: linear-gradient(155deg, rgba(129,140,248,0.16), rgba(15,23,42,0.08)); --blob-border: rgba(255,255,255,0.1);
  --cta-veil: linear-gradient(135deg, rgba(129,140,248,0.18), rgba(8,12,24,0.4), transparent);
  --modal-scrim: rgba(8,12,24,0.72); --ok-bg: rgba(16,185,129,0.12); --ok-border: rgba(52,211,153,0.28);
  --err-bg: rgba(239,68,68,0.12); --err-border: rgba(248,113,113,0.28); --success-on: #6ee7b7;
  --glass-fill: rgba(15,23,42,0.5); --glass-bloom: rgba(129,140,248,0.09); --glass-edge-hi: rgba(255,255,255,0.32); --glass-inset: rgba(255,255,255,0.16);
  --glass-lift: 0 10px 36px rgba(0,0,0,0.45);
  --glass-bright: 1;
  --glass-shadow: 0 12px 40px rgba(0,0,0,0.4), inset 0 1px 1px rgba(255,255,255,0.08);
  --glass-shadow-lg: 0 28px 80px rgba(0,0,0,0.5), inset 0 1px 1px rgba(255,255,255,0.1);
  --glass-highlight: linear-gradient(145deg, rgba(255,255,255,0.12), rgba(255,255,255,0.02));
  --iridescent: linear-gradient(135deg, rgba(129,140,248,0.4), rgba(56,189,248,0.15), rgba(255,255,255,0.06));
}
html[data-theme="ember"] {
  color-scheme: dark;
  --bg: #1a100c; --bg-subtle: rgba(232,168,124,0.06); --bg-card: rgba(255,237,213,0.06);
  --border: rgba(232,168,124,0.14); --border-glass: rgba(232,168,124,0.18);
  --text: #f5e6d3; --muted: #c4b5a0; --text-muted: #c4b5a0; --primary: #e8a87c; --primary-hover: #f0c4a0;
  --primary-soft: rgba(232,168,124,0.14); --accent: #f59e0b; --accent-soft: rgba(245,158,11,0.14);
  --success: #86efac; --grid-line: rgba(232,168,124,0.08);
  --surface: rgba(40,24,16,0.62); --surface-strong: rgba(20,12,8,0.92); --header-bg: rgba(20,12,8,0.7); --code-bg: rgba(232,168,124,0.08);
  --nav-hover: rgba(232,168,124,0.14); --canvas-veil: rgba(20,10,6,0.75);
  --canvas-hi: #2a1810; --canvas-mid: #1a100c; --canvas-lo: #0e0806; --orb: rgba(245,158,11,0.28);
  --link-hover: #fbbf24; --btn-fill: rgba(232,168,124,0.1); --header-border: rgba(232,168,124,0.16);
  --primary-btn: rgba(194,65,12,0.75); --on-primary: #fff7ed; --primary-btn-hover: rgba(194,65,12,0.9);
  --panel: rgba(255,237,213,0.08); --panel-soft: rgba(232,168,124,0.06); --panel-strong: rgba(40,24,16,0.88); --panel-input: rgba(232,168,124,0.12);
  --edge: rgba(232,168,124,0.18); --edge-strong: rgba(232,168,124,0.28); --spec: rgba(245,158,11,0.16); --hairline: rgba(232,168,124,0.14);
  --ink-from: #e8a87c; --ink-to: #c9a962; --track: rgba(232,168,124,0.1); --focus-ring: rgba(245,158,11,0.28); --placeholder: #a8a29e;
  --blob-hi: rgba(245,158,11,0.22); --blob-mid: rgba(232,168,124,0.14); --blob-lo: rgba(194,65,12,0.1);
  --blob-fill: linear-gradient(155deg, rgba(245,158,11,0.16), rgba(26,16,12,0.08)); --blob-border: rgba(232,168,124,0.16);
  --cta-veil: linear-gradient(135deg, rgba(245,158,11,0.16), rgba(20,10,6,0.4), transparent);
  --modal-scrim: rgba(20,10,6,0.72); --ok-bg: rgba(16,185,129,0.12); --ok-border: rgba(134,239,172,0.28);
  --err-bg: rgba(239,68,68,0.12); --err-border: rgba(248,113,113,0.28); --success-on: #86efac;
  --glass-fill: rgba(40,24,16,0.55); --glass-bloom: rgba(245,158,11,0.1); --glass-edge-hi: rgba(255,237,213,0.35); --glass-inset: rgba(232,168,124,0.22);
  --glass-lift: 0 10px 36px rgba(20,10,6,0.5);
  --glass-bright: 1;
  --glass-shadow: 0 12px 40px rgba(0,0,0,0.45), inset 0 1px 1px rgba(245,158,11,0.12);
  --glass-shadow-lg: 0 28px 80px rgba(0,0,0,0.5), inset 0 1px 1px rgba(245,158,11,0.14);
  --glass-highlight: linear-gradient(145deg, rgba(245,158,11,0.14), rgba(255,255,255,0.02));
  --iridescent: linear-gradient(135deg, rgba(245,158,11,0.35), rgba(232,168,124,0.18), rgba(255,255,255,0.05));
}
html[data-theme="aurora"] {
  color-scheme: dark;
  --bg: #0a1628; --bg-subtle: rgba(34,211,238,0.05); --bg-card: rgba(167,139,250,0.07);
  --border: rgba(34,211,238,0.12); --border-glass: rgba(167,139,250,0.18);
  --text: #e0f2fe; --muted: #8fb4c9; --text-muted: #8fb4c9; --primary: #22d3ee; --primary-hover: #67e8f9;
  --primary-soft: rgba(34,211,238,0.12); --accent: #a78bfa; --accent-soft: rgba(167,139,250,0.16);
  --success: #5eead4; --grid-line: rgba(34,211,238,0.07);
  --surface: rgba(10,30,50,0.6); --surface-strong: rgba(6,16,32,0.92); --header-bg: rgba(6,16,32,0.7); --code-bg: rgba(34,211,238,0.07);
  --nav-hover: rgba(167,139,250,0.16); --canvas-veil: rgba(6,14,28,0.78);
  --canvas-hi: #12243c; --canvas-mid: #0a1628; --canvas-lo: #050d18; --orb: rgba(34,211,238,0.28);
  --link-hover: #c4b5fd; --btn-fill: rgba(34,211,238,0.08); --header-border: rgba(34,211,238,0.14);
  --primary-btn: rgba(14,165,233,0.72); --on-primary: #042f2e; --primary-btn-hover: rgba(14,165,233,0.88);
  --panel: rgba(167,139,250,0.09); --panel-soft: rgba(34,211,238,0.06); --panel-strong: rgba(10,30,50,0.88); --panel-input: rgba(34,211,238,0.1);
  --edge: rgba(34,211,238,0.16); --edge-strong: rgba(167,139,250,0.28); --spec: rgba(34,211,238,0.16); --hairline: rgba(34,211,238,0.12);
  --ink-from: #22d3ee; --ink-to: #a78bfa; --track: rgba(34,211,238,0.1); --focus-ring: rgba(167,139,250,0.3); --placeholder: #64748b;
  --blob-hi: rgba(34,211,238,0.2); --blob-mid: rgba(167,139,250,0.16); --blob-lo: rgba(56,189,248,0.08);
  --blob-fill: linear-gradient(155deg, rgba(34,211,238,0.14), rgba(10,22,40,0.08)); --blob-border: rgba(34,211,238,0.14);
  --cta-veil: linear-gradient(135deg, rgba(34,211,238,0.16), rgba(6,14,28,0.4), transparent);
  --modal-scrim: rgba(6,14,28,0.72); --ok-bg: rgba(45,212,191,0.12); --ok-border: rgba(94,234,212,0.28);
  --err-bg: rgba(239,68,68,0.12); --err-border: rgba(248,113,113,0.28); --success-on: #5eead4;
  --glass-fill: rgba(10,30,50,0.52); --glass-bloom: rgba(34,211,238,0.1); --glass-edge-hi: rgba(186,230,253,0.38); --glass-inset: rgba(34,211,238,0.18);
  --glass-lift: 0 10px 36px rgba(4,12,24,0.5);
  --glass-bright: 1;
  --glass-shadow: 0 12px 40px rgba(0,0,0,0.4), inset 0 1px 1px rgba(34,211,238,0.12);
  --glass-shadow-lg: 0 28px 80px rgba(0,0,0,0.5), inset 0 1px 1px rgba(34,211,238,0.14);
  --glass-highlight: linear-gradient(145deg, rgba(34,211,238,0.14), rgba(255,255,255,0.02));
  --iridescent: linear-gradient(135deg, rgba(34,211,238,0.35), rgba(167,139,250,0.25), rgba(255,255,255,0.05));
}
html[data-theme="forest"] {
  color-scheme: dark;
  --bg: #0c1410; --bg-subtle: rgba(52,211,153,0.05); --bg-card: rgba(110,231,183,0.06);
  --border: rgba(52,211,153,0.12); --border-glass: rgba(110,231,183,0.16);
  --text: #e8f5e9; --muted: #8fb9a0; --text-muted: #8fb9a0; --primary: #34d399; --primary-hover: #6ee7b7;
  --primary-soft: rgba(52,211,153,0.12); --accent: #4ade80; --accent-soft: rgba(74,222,128,0.14);
  --success: #86efac; --grid-line: rgba(52,211,153,0.07);
  --surface: rgba(12,28,20,0.62); --surface-strong: rgba(6,16,12,0.92); --header-bg: rgba(6,16,12,0.7); --code-bg: rgba(52,211,153,0.07);
  --nav-hover: rgba(52,211,153,0.14); --canvas-veil: rgba(6,14,10,0.78);
  --canvas-hi: #14241c; --canvas-mid: #0c1410; --canvas-lo: #060a08; --orb: rgba(52,211,153,0.26);
  --link-hover: #6ee7b7; --btn-fill: rgba(52,211,153,0.08); --header-border: rgba(52,211,153,0.14);
  --primary-btn: rgba(5,150,105,0.78); --on-primary: #ecfdf5; --primary-btn-hover: rgba(5,150,105,0.92);
  --panel: rgba(110,231,183,0.08); --panel-soft: rgba(52,211,153,0.05); --panel-strong: rgba(12,28,20,0.88); --panel-input: rgba(52,211,153,0.1);
  --edge: rgba(52,211,153,0.16); --edge-strong: rgba(110,231,183,0.26); --spec: rgba(52,211,153,0.16); --hairline: rgba(52,211,153,0.12);
  --ink-from: #34d399; --ink-to: #10b981; --track: rgba(52,211,153,0.1); --focus-ring: rgba(52,211,153,0.28); --placeholder: #6b7c72;
  --blob-hi: rgba(52,211,153,0.2); --blob-mid: rgba(16,185,129,0.12); --blob-lo: rgba(110,231,183,0.08);
  --blob-fill: linear-gradient(155deg, rgba(52,211,153,0.14), rgba(12,20,16,0.08)); --blob-border: rgba(52,211,153,0.14);
  --cta-veil: linear-gradient(135deg, rgba(52,211,153,0.16), rgba(6,14,10,0.4), transparent);
  --modal-scrim: rgba(6,14,10,0.72); --ok-bg: rgba(16,185,129,0.12); --ok-border: rgba(134,239,172,0.28);
  --err-bg: rgba(239,68,68,0.12); --err-border: rgba(248,113,113,0.28); --success-on: #86efac;
  --glass-fill: rgba(12,28,20,0.55); --glass-bloom: rgba(52,211,153,0.1); --glass-edge-hi: rgba(167,243,208,0.32); --glass-inset: rgba(52,211,153,0.18);
  --glass-lift: 0 10px 36px rgba(4,12,8,0.5);
  --glass-bright: 1;
  --glass-shadow: 0 12px 40px rgba(0,0,0,0.4), inset 0 1px 1px rgba(52,211,153,0.1);
  --glass-shadow-lg: 0 28px 80px rgba(0,0,0,0.5), inset 0 1px 1px rgba(52,211,153,0.12);
  --glass-highlight: linear-gradient(145deg, rgba(52,211,153,0.14), rgba(255,255,255,0.02));
  --iridescent: linear-gradient(135deg, rgba(52,211,153,0.32), rgba(16,185,129,0.16), rgba(255,255,255,0.05));
}

::selection { background: color-mix(in srgb, var(--accent) 36%, transparent); color: var(--text); }

.skip-link {
  position: absolute;
  left: 0.75rem;
  top: -3rem;
  z-index: 80;
  padding: 0.45rem 0.85rem;
  border-radius: 999px;
  background: var(--primary-btn);
  color: var(--on-primary);
  font-weight: 600;
  font-size: 0.85rem;
}
.skip-link:focus { top: 0.75rem; }

.theme-wrap { position: relative; }
.theme-btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.45rem;
  padding: 0.42rem 0.85rem 0.42rem 0.45rem;
  border-radius: 999px;
  border: 1px solid var(--header-border);
  background: var(--btn-fill);
  color: var(--text);
  font: inherit;
  font-weight: 600;
  font-size: 0.82rem;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  box-shadow: var(--glass-shadow);
}
.theme-btn:hover { background: var(--nav-hover); }
.theme-btn:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
.theme-btn-wheel {
  width: 1.45rem;
  height: 1.45rem;
  border-radius: 50%;
  background: conic-gradient(var(--accent) 0 90deg, var(--primary) 90deg 180deg, var(--bg) 180deg 270deg, var(--text) 270deg 360deg);
  box-shadow: inset 0 0 0 2px color-mix(in srgb, var(--text) 12%, transparent);
}
.r-popup.theme-popover,
.theme-popover {
  margin: 0;
  inset: auto;
  overflow: visible;
  width: min(22rem, calc(100vw - 1.5rem));
  padding: 1rem 1rem 1.05rem;
  border: 1px solid var(--border-glass);
  border-radius: 22px;
  background: var(--glass-fill);
  color: var(--text);
  box-shadow:
    var(--glass-lift),
    inset 0 1px 0 var(--glass-inset),
    inset 0 -1px 0 color-mix(in srgb, var(--spec) 12%, transparent),
    inset 0 0 48px 18px var(--glass-bloom);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
}
.theme-popover-body { position: relative; z-index: 1; }
.theme-popover-kicker {
  margin: 0;
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--muted);
}
.theme-popover-lead {
  margin: 0.25rem 0 0.85rem;
  font-size: 0.82rem;
  color: var(--muted);
  line-height: 1.45;
}
.theme-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.45rem;
}
.theme-opt {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: center;
  gap: 0.6rem;
  padding: 0.55rem 0.65rem;
  border-radius: 14px;
  border: 1px solid var(--border);
  background: var(--bg-subtle);
  color: var(--text);
  font: inherit;
  text-align: left;
  cursor: pointer;
  pointer-events: auto;
  transition: border-color 0.15s ease, background 0.15s ease;
}
.theme-opt:hover { border-color: var(--accent); background: var(--accent-soft); }
.theme-opt:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
.theme-opt[aria-pressed="true"],
.theme-opt.r-theme-on {
  border-color: var(--accent);
  box-shadow: 0 0 0 1px var(--accent);
}
.theme-swatch {
  width: 1.7rem;
  height: 1.7rem;
  border-radius: 50%;
  flex-shrink: 0;
  box-shadow: inset 0 0 0 1px rgba(255,255,255,0.35), 0 2px 6px rgba(0,0,0,0.18);
}
.theme-opt-copy { display: flex; flex-direction: column; gap: 0.05rem; min-width: 0; }
.theme-opt-copy strong { font-size: 0.86rem; line-height: 1.2; }
.theme-opt-copy span { font-size: 0.7rem; color: var(--muted); text-transform: capitalize; }

@media (prefers-reduced-motion: reduce) {
  ::view-transition-group(*),
  ::view-transition-old(*),
  ::view-transition-new(*) { animation: none !important; }
}
::view-transition-old(root),
::view-transition-new(root) { animation-duration: 0.32s; }

@media (max-width: 920px) {
  .theme-btn-label,
  .search-btn-label {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    clip-path: inset(50%);
    white-space: nowrap;
    border: 0;
  }
  .theme-btn,
  .search-btn {
    width: 2.75rem;
    height: 2.75rem;
    min-width: 2.75rem;
    padding: 0;
    gap: 0;
  }
  .theme-btn-wheel { width: 1.2rem; height: 1.2rem; }
  .header-actions .docs-rs-link,
  .header-actions .crates-link { display: none; }
}
</style>"#;
