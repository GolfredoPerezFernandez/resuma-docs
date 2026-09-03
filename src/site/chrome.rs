//! Site chrome — Explore and Theme are `<Popup>`; Search is `<Modal>`.
//!
//! These live in the root layout. Clicks that must be instant (theme) still
//! go through the blocking boot script in `theme.rs` — layout handler chunks
//! would wait on the first interaction.

use resuma::prelude::*;

pub fn explore_nav() -> View {
    view! {
        <div class="explore-wrap">
            <Popup id="docs-explore" positions="bottom left top right" class="explore-sheet">
                <button
                    slot="anchor"
                    type="button"
                    class="explore-btn"
                    aria-haspopup="dialog"
                    aria-controls="r-popup-docs-explore"
                    aria-expanded="false"
                    aria-label="Explore Resuma"
                >
                    <span class="explore-mark" aria-hidden="true"></span>
                    <span class="explore-btn-label">"Explore"</span>
                </button>
                <div class="explore-sheet-body" role="dialog" aria-label="Explore Resuma">
                    <p class="explore-kicker">"Go anywhere"</p>
                    <div class="explore-grid">
                        <NavLink href="/docs" class="explore-tile" activeClass="explore-tile--active" exact=true>
                            <strong>"Docs"</strong>
                            <span>"Guides, demos, API"</span>
                        </NavLink>
                        <NavLink href="/docs/getting_started" class="explore-tile" activeClass="explore-tile--active" exact=true>
                            <strong>"Tutorial"</strong>
                            <span>"First resumable app"</span>
                        </NavLink>
                        <NavLink href="/docs/flow" class="explore-tile" activeClass="explore-tile--active">
                            <strong>"Flow"</strong>
                            <span>"Pages, loaders, SSR"</span>
                        </NavLink>
                        <NavLink href="/docs/exec" class="explore-tile" activeClass="explore-tile--active">
                            <strong>"Resuma OS"</strong>
                            <span>"Workers, queues, ops"</span>
                        </NavLink>
                        <NavLink href="/docs/benchmark" class="explore-tile" activeClass="explore-tile--active" exact=true>
                            <strong>"Benchmark"</strong>
                            <span>"1021 B vs hydration"</span>
                        </NavLink>
                        <NavLink href="/docs/components/popup" class="explore-tile" activeClass="explore-tile--active" exact=true>
                            <strong>"Popup"</strong>
                            <span>"This menu is one"</span>
                        </NavLink>
                    </div>
                </div>
            </Popup>
        </div>
    }
}

pub fn docs_search_modal() -> View {
    view! {
        <Modal id="docs-search" class="docs-search-dialog">
            <button
                slot="trigger"
                type="button"
                class="search-btn"
                aria-label="Search documentation"
                title="Search docs (/)"
            >
                <span class="search-btn-mark" aria-hidden="true"></span>
                <span class="search-btn-label">"Search"</span>
            </button>
            <form method="get" action="/docs/search" class="docs-search-dialog-form">
                <h2 class="docs-search-dialog-title">"Search docs"</h2>
                <p class="docs-search-dialog-lead">
                    "Server index — no client search bundle. Press "
                    <kbd>"/"</kbd>
                    " from any page."
                </p>
                <input
                    type="search"
                    name="q"
                    placeholder="Popup, Modal, Flow, CSP…"
                    aria-label="Search documentation"
                    autocomplete="off"
                />
                <button type="submit" class="btn btn-primary">"Search"</button>
            </form>
        </Modal>
    }
}

pub fn browse_docs_chip() -> View {
    view! {
        <button
            type="button"
            class="browse-chip"
            popovertarget="docs-browse"
            aria-haspopup="dialog"
            aria-controls="docs-browse"
            aria-expanded="false"
            aria-label="Browse documentation"
        >
            <span class="browse-chip-mark" aria-hidden="true"></span>
            "Browse docs"
        </button>
    }
}
