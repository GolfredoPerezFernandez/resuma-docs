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
  function flashThemeCopied(btn, label) {
    var prev = btn.textContent;
    btn.textContent = label || "Copied!";
    btn.disabled = true;
    setTimeout(function () {
      btn.textContent = prev;
      btn.disabled = false;
    }, 1600);
  }
  function themeCssBlock(css, id) {
    var needle = 'html[data-theme="' + id + '"]';
    var start = css.indexOf(needle);
    if (start < 0) return "";
    var from = css.slice(start);
    var next = from.indexOf("html[data-theme=", needle.length);
    return (next < 0 ? from : from.slice(0, next)).trim();
  }
  function copyThemeText(text, btn) {
    var value = (text || "").trim();
    if (!value) {
      flashThemeCopied(btn, "Copy failed");
      return;
    }
    function fallback() {
      var ta = document.createElement("textarea");
      ta.value = value;
      ta.setAttribute("readonly", "");
      ta.className = "docs-copy-fallback";
      document.body.appendChild(ta);
      ta.select();
      try {
        document.execCommand("copy");
        flashThemeCopied(btn);
      } catch (e) {
        flashThemeCopied(btn, "Copy failed");
      }
      ta.remove();
    }
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(value).then(function () {
        flashThemeCopied(btn);
      }).catch(fallback);
    } else {
      fallback();
    }
  }
  document.addEventListener("click", function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest("[data-theme-copy]");
    if (!btn || btn.tagName !== "BUTTON" || btn.disabled) return;
    var mode = btn.getAttribute("data-theme-copy");
    if (mode !== "current" && mode !== "all") return;
    var src = document.getElementById("resuma-theme-css-src");
    var all = (src && src.textContent) || "";
    if (!all.trim()) {
      flashThemeCopied(btn, "Copy failed");
      return;
    }
    if (mode === "all") {
      copyThemeText(all, btn);
      return;
    }
    var id = document.documentElement.getAttribute("data-theme") || "paper";
    var block = themeCssBlock(all, id);
    if (!block) {
      flashThemeCopied(btn, "Copy failed");
      return;
    }
    copyThemeText(block, btn);
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
                        {THEMES.iter().map(|t| theme_option(t, current)).collect::<Vec<_>>()}
                    </div>
                    <div class="theme-copy-row">
                        <button
                            type="button"
                            class="btn btn-ghost btn-sm theme-copy-btn"
                            data-theme-copy="current"
                            aria-label="Copy CSS for the active theme"
                        >
                            "Copy this CSS"
                        </button>
                        <button
                            type="button"
                            class="btn btn-ghost btn-sm theme-copy-btn"
                            data-theme-copy="all"
                            aria-label="Copy CSS for every official palette"
                        >
                            "Copy all palettes"
                        </button>
                        <a
                            class="btn btn-ghost btn-sm theme-copy-btn"
                            href="/themes.css"
                            download="resuma-themes.css"
                        >
                            "Download CSS"
                        </a>
                    </div>
                    <pre id="resuma-theme-css-src" hidden>{OFFICIAL_PALETTE_CSS.to_string()}</pre>
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

/// Official `html[data-theme]` palettes (same file as the Resuma crate).
/// Served at `/themes.css` so visitors can download or curl it.
pub const OFFICIAL_PALETTE_CSS: &str = include_str!("../../public/themes.css");

/// Palette overrides + chrome that must live beside `SITE_CSS`.
pub fn theme_sheet() -> String {
    THEME_SHEET_TEMPLATE.replace("__RESUMA_OFFICIAL_THEMES__", OFFICIAL_PALETTE_CSS)
}

const THEME_SHEET_TEMPLATE: &str = r#"<style>
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
__RESUMA_OFFICIAL_THEMES__

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
.theme-copy-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem;
  margin-top: 0.85rem;
}
.theme-copy-row .theme-copy-btn {
  flex: 1 1 auto;
  min-width: 8.5rem;
}

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
