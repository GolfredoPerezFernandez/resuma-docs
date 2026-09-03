//! SSR copy-for-AI buttons (page + docs nav). Click handlers live in `docs-copy.ts`.

use resuma::prelude::*;

#[component]
pub fn DocsCopyNavBtn() {
    view! {
        <div class="docs-copy-nav">
            <button
                type="button"
                class="btn btn-ghost btn-sm docs-copy-nav-btn"
                aria-label="Copy the documentation outline for an AI"
                title="Copy nav — paste into an AI"
                onClick={js!(async (event) => {
                    const btn = event.currentTarget;
                    if (!(btn instanceof HTMLButtonElement)) return;
                    const mod = await import("/static/client/docs-copy.js?v=1.3.1");
                    await mod.copyDocsNav(btn);
                })}
            >
                "Copy nav"
            </button>
        </div>
    }
}

#[component]
pub fn DocsCopyPageToolbar() {
    view! {
        <div class="docs-copy-toolbar" data-docs-copy-ssr="1">
            <p class="docs-copy-hint">"For an AI"</p>
            <button
                type="button"
                class="btn btn-ghost btn-sm docs-copy-page"
                aria-label="Copy this page for an AI"
                title="Copy page — paste into an AI"
                onClick={js!(async (event) => {
                    const btn = event.currentTarget;
                    if (!(btn instanceof HTMLButtonElement)) return;
                    const mod = await import("/static/client/docs-copy.js?v=1.3.1");
                    await mod.copyDocsPage(btn);
                })}
            >
                "Copy page"
            </button>
        </div>
    }
}
