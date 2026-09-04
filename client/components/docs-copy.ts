/**
 * Copy-for-AI helpers: page body, docs nav outline, and per-section buttons.
 */

const COPY_UI =
  ".docs-copy-toolbar, .docs-copy-nav, .docs-copy-section, .docs-section-head button, .docs-copy-hint, .theme-copy-row, .theme-copy-btn";

/** Mount Flow widgets outside dynamically injected exec panels (SSR dashboard only). */
export async function initDocsFlow() {
  const scope = document.querySelector<HTMLElement>(".docs-main");
  if (!scope) return;
  const hasDashboard = scope.querySelector(
    "[data-r-flow-dashboard]:not([data-docs-exec-panel] [data-r-flow-dashboard])",
  );
  const hasStaticGraph = Array.from(
    scope.querySelectorAll<HTMLElement>("[data-r-flow-graph]"),
  ).some((el) => !el.closest("[data-docs-exec-panel]"));
  if (!hasDashboard && !hasStaticGraph) return;
  try {
    if (window.__resumaCoreReady) await window.__resumaCoreReady;
    const mod = await import("/_resuma/flow.js?v=1.3.1");
    mod.initFlowWidgets(scope, {
      flush: false,
      exclude: "[data-docs-exec-panel]",
    });
  } catch (e) {
    console.warn("[docs-flow]", e);
  }
}

function flashCopied(btn: HTMLButtonElement, label = "Copied!") {
  const prev = btn.textContent;
  btn.textContent = label;
  btn.disabled = true;
  setTimeout(() => {
    btn.textContent = prev;
    btn.disabled = false;
  }, 1600);
}

async function copyText(text: string, btn: HTMLButtonElement) {
  const value = text.trim();
  if (!value) return;
  try {
    if (!navigator.clipboard?.writeText) throw new Error("clipboard");
    await navigator.clipboard.writeText(value);
    flashCopied(btn);
  } catch {
    const ta = document.createElement("textarea");
    ta.value = value;
    ta.setAttribute("readonly", "");
    ta.className = "docs-copy-fallback";
    document.body.appendChild(ta);
    ta.select();
    try {
      document.execCommand("copy");
      flashCopied(btn);
    } catch {
      flashCopied(btn, "Copy failed");
    }
    ta.remove();
  }
}

export async function copyDocsPage(btn: HTMLButtonElement) {
  const main = document.querySelector<HTMLElement>(".docs-main");
  if (!main) return;
  await copyText(pageMarkdown(main), btn);
}

export async function copyDocsNav(btn: HTMLButtonElement) {
  const sidebar = document.querySelector(".docs-sidebar");
  if (!sidebar) return;
  await copyText(navMarkdown(sidebar), btn);
}

async function themeCssSource(): Promise<string> {
  const embedded =
    document.getElementById("resuma-theme-css-src")?.textContent ?? "";
  if (embedded.trim()) return embedded;
  try {
    const res = await fetch("/themes.css", { credentials: "same-origin" });
    if (!res.ok) return "";
    return await res.text();
  } catch {
    return "";
  }
}

function themeBlock(css: string, id: string): string {
  const needle = `html[data-theme="${id}"]`;
  const start = css.indexOf(needle);
  if (start < 0) return "";
  const from = css.slice(start);
  const next = from.indexOf("html[data-theme=", needle.length);
  return (next < 0 ? from : from.slice(0, next)).trim();
}

/** Copy official `html[data-theme]` CSS — current palette or every official one. */
export async function copyThemeCss(
  btn: HTMLButtonElement,
  mode: "current" | "all" = "all",
) {
  const all = await themeCssSource();
  if (!all.trim()) {
    flashCopied(btn, "Copy failed");
    return;
  }
  if (mode === "all") {
    await copyText(all, btn);
    return;
  }
  const id =
    document.documentElement.getAttribute("data-theme") ||
    btn.getAttribute("data-theme-id") ||
    "paper";
  const block = themeBlock(all, id);
  if (!block) {
    flashCopied(btn, "Copy failed");
    return;
  }
  await copyText(block, btn);
}

function pageMarkdown(main: HTMLElement) {
  const clone = main.cloneNode(true) as HTMLElement;
  clone.querySelectorAll(COPY_UI).forEach((n) => n.remove());
  clone.querySelectorAll("resuma-boundary").forEach((n) => n.remove());
  clone.querySelectorAll(".docs-section-head").forEach((wrapper) => {
    const h2 = wrapper.querySelector("h2");
    if (h2) wrapper.replaceWith(h2);
  });
  const title = clone.querySelector("h1")?.textContent?.trim() || document.title;
  clone.querySelector("h1")?.remove();
  clone.querySelectorAll("pre").forEach((pre) => {
    const p = document.createElement("p");
    p.textContent = "```\n" + (pre.textContent ?? "").replace(/\n$/, "") + "\n```";
    pre.replaceWith(p);
  });
  const body = blocksToText(clone).replace(/\n{3,}/g, "\n\n").trim();
  return [`# ${title}`, `Source: ${location.href}`, "", body].join("\n");
}

function blocksToText(root: HTMLElement) {
  const parts: string[] = [];
  const walk = (el: Element) => {
    if (el.matches(COPY_UI) || el.tagName === "SCRIPT" || el.tagName === "STYLE") return;
    const tag = el.tagName;
    if (tag === "H2" || tag === "H3" || tag === "H4") {
      const hashes = tag === "H2" ? "##" : tag === "H3" ? "###" : "####";
      const t = (el as HTMLElement).innerText.trim();
      if (t) parts.push(`${hashes} ${t}`);
      return;
    }
    if (tag === "P" || tag === "PRE" || tag === "TABLE" || tag === "UL" || tag === "OL" || tag === "BLOCKQUOTE" || tag === "DL") {
      const t = (el as HTMLElement).innerText.trim();
      if (t) parts.push(t);
      return;
    }
    if (el.children.length) {
      Array.from(el.children).forEach(walk);
      return;
    }
    const t = (el as HTMLElement).innerText?.trim();
    if (t) parts.push(t);
  };
  Array.from(root.children).forEach(walk);
  return parts.join("\n\n");
}

function navMarkdown(sidebar: Element) {
  const origin = location.origin;
  const lines = [
    "# Resuma documentation",
    `Base: ${origin}`,
    "Copy a page with the Copy page button, or open a URL below.",
    "",
  ];
  sidebar.querySelectorAll(".docs-sidebar-section").forEach((section) => {
    const heading = section.querySelector("h4")?.textContent?.trim();
    if (heading) lines.push(`## ${heading}`);
    section.querySelectorAll("a.docs-nav-link").forEach((a) => {
      const label = a.textContent?.trim();
      const href = a.getAttribute("href");
      if (!label || !href) return;
      const url = href.startsWith("http") ? href : new URL(href, origin).href;
      lines.push(`- [${label}](${url})`);
    });
    lines.push("");
  });
  return lines.join("\n").trim();
}

function sectionText(h2: HTMLHeadingElement) {
  const lines = [h2.innerText.trim()];
  const wrapper = h2.closest(".docs-section-head");
  let el: Element | null = wrapper ? wrapper.nextElementSibling : h2.nextElementSibling;
  while (el) {
    if (el.matches("h2, .docs-section-head, .docs-copy-toolbar")) break;
    if (!el.classList.contains("docs-copy-toolbar")) {
      const t = (el as HTMLElement).innerText.trim();
      if (t) lines.push(t);
    }
    el = el.nextElementSibling;
  }
  return lines.join("\n\n");
}

function teardown(main: HTMLElement) {
  main.querySelectorAll(".docs-copy-toolbar:not([data-docs-copy-ssr])").forEach((n) => n.remove());
  main.querySelectorAll(".docs-copy-live").forEach((n) => n.remove());
  main.querySelectorAll(".docs-section-head").forEach((wrapper) => {
    const h2 = wrapper.querySelector("h2");
    if (h2) wrapper.replaceWith(h2);
  });
}

function sidebarHrefMatches(href: string, current: string, exact: boolean): boolean {
  if (exact) {
    if (href === current) return true;
    const base = "http://resuma.local";
    const a = new URL(href, base);
    const b = new URL(current, base);
    if (a.search) return a.pathname + a.search === b.pathname + b.search;
    return a.pathname === b.pathname;
  }
  if (href === current) return true;
  const base = "http://resuma.local";
  const a = new URL(href, base);
  const b = new URL(current, base);
  if (a.search) return a.pathname + a.search === b.pathname + b.search;
  if (a.pathname === b.pathname) return true;
  if (a.pathname !== "/" && b.pathname.startsWith(a.pathname)) {
    const next = b.pathname.charCodeAt(a.pathname.length);
    return next === undefined || next === 47;
  }
  return false;
}

/** Keep docs sidebar active state in sync after SPA navigation. */
export function updateDocsSidebarNav(path = location.pathname + location.search): void {
  const sidebar = document.querySelector(".docs-sidebar");
  if (!sidebar) return;

  let best: HTMLAnchorElement | null = null;
  let bestLen = -1;

  sidebar.querySelectorAll<HTMLAnchorElement>("a[data-r-nav]").forEach((a) => {
    const href = a.getAttribute("href");
    if (!href) return;
    const exact = a.hasAttribute("data-r-nav-exact");
    if (!sidebarHrefMatches(href, path, exact)) return;
    if (href.length > bestLen) {
      best = a;
      bestLen = href.length;
    }
  });

  sidebar.querySelectorAll<HTMLAnchorElement>("a[data-r-nav]").forEach((a) => {
    const activeClass = a.getAttribute("data-r-active-class") ?? "docs-nav-link--active";
    const isBest = a === best;
    a.className = isBest ? `docs-nav-link ${activeClass}` : "docs-nav-link";
    if (isBest) a.setAttribute("aria-current", "page");
    else a.removeAttribute("aria-current");
  });
}

function scrollActiveSidebarLink() {
  const sidebar = document.querySelector<HTMLElement>(".docs-sidebar-scroll");
  const active =
    sidebar?.querySelector<HTMLElement>('.docs-nav-link--active[aria-current="page"]') ??
    sidebar?.querySelector<HTMLElement>(".docs-nav-link--active");
  if (!sidebar || !active) return;
  const top = active.offsetTop - sidebar.clientHeight / 2 + active.offsetHeight / 2;
  sidebar.scrollTo({ top: Math.max(0, top), behavior: "smooth" });
}

/** Refresh the Caching page live demo (headers + server stamp). */
export async function refreshCacheDemo(): Promise<void> {
  const root = document.querySelector<HTMLElement>("[data-docs-cache-demo]");
  if (!root) return;
  const headerEl = root.querySelector<HTMLElement>("[data-cache-header]");
  const stampEl = root.querySelector<HTMLElement>("[data-cache-stamp]");
  const statusEl = root.querySelector<HTMLElement>("[data-cache-status]");
  const r = window.__resuma;
  if (!r) return;
  try {
    const res = await fetch(location.pathname + location.search, {
      credentials: "same-origin",
      cache: "no-store",
    });
    if (headerEl) {
      headerEl.textContent = res.headers.get("cache-control") ?? "(none)";
    }
    const info = await r.safeAction("docs_cache_info", []);
    if (info.ok && stampEl) {
      stampEl.textContent = String(info.value.stamp ?? "—");
    }
    if (statusEl) {
      statusEl.textContent = `Headers refreshed (${res.status})`;
    }
  } catch {
    if (statusEl) statusEl.textContent = "Refresh failed";
  }
}

export function initCacheDemo(): void {
  void refreshCacheDemo();
}

export function initDocsSidebar() {
  const sync = () => {
    updateDocsSidebarNav();
    scrollActiveSidebarLink();
  };
  sync();
  document.addEventListener("resuma:navigate", sync);
}

export function initDocsCopy() {
  const main = document.querySelector<HTMLElement>(".docs-main");
  if (!main) return;

  teardown(main);

  if (!main.querySelector(".docs-copy-page")) {
    const toolbar = document.createElement("div");
    toolbar.className = "docs-copy-toolbar";
    const pageBtn = document.createElement("button");
    pageBtn.type = "button";
    pageBtn.className = "btn btn-ghost btn-sm docs-copy-page";
    pageBtn.textContent = "Copy page";
    pageBtn.setAttribute("aria-label", "Copy this page for an AI");
    pageBtn.addEventListener("click", () => {
      void copyDocsPage(pageBtn);
    });
    toolbar.appendChild(pageBtn);
    main.insertBefore(toolbar, main.firstChild);
  }

  main.querySelectorAll(".live-demo").forEach((section) => {
    const header = section.querySelector(".live-demo-header");
    if (!header || header.querySelector(".docs-copy-section")) return;
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "btn btn-ghost btn-sm docs-copy-section docs-copy-live";
    btn.textContent = "Copy";
    const title = section.querySelector(".live-demo-title")?.textContent?.trim();
    if (title) btn.setAttribute("aria-label", `Copy demo: ${title}`);
    btn.addEventListener("click", () => {
      void copyText((section as HTMLElement).innerText.trim(), btn);
    });
    header.appendChild(btn);
  });

  main.querySelectorAll("h2").forEach((node) => {
    const h2 = node as HTMLHeadingElement;
    if (h2.closest(".docs-section-head, .docs-copy-toolbar, .live-demo-header")) return;
    const wrapper = document.createElement("div");
    wrapper.className = "docs-section-head";
    h2.parentNode?.insertBefore(wrapper, h2);
    wrapper.appendChild(h2);

    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "btn btn-ghost btn-sm docs-copy-section";
    btn.textContent = "Copy";
    btn.setAttribute("aria-label", `Copy section: ${h2.textContent}`);
    btn.addEventListener("click", () => {
      void copyText(sectionText(h2), btn);
    });
    wrapper.appendChild(btn);
  });
}
