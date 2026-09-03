use crate::site::bundle_sizes;
use crate::site::code_block;
use resuma::prelude::*;

fn ext_link(href: &str, label: &str) -> View {
    view! {
        <a href={href.to_string()} target="_blank">{label.to_string()}</a>
    }
}

fn summary_row(framework: &str, initial: &str, first: &str, statik: &str, highlight: bool) -> View {
    let td_class = if highlight { "yes" } else { "" };
    view! {
        <tr>
            <td><strong>{framework.to_string()}</strong></td>
            <td class={td_class}>{initial.to_string()}</td>
            <td class={td_class}>{first.to_string()}</td>
            <td class={td_class}>{statik.to_string()}</td>
        </tr>
    }
}

fn detail_row(framework: &str, initial: &str, first: &str, notes: &str) -> View {
    view! {
        <tr>
            <td><strong>{framework.to_string()}</strong></td>
            <td><code>{initial.to_string()}</code></td>
            <td>{first.to_string()}</td>
            <td>{notes.to_string()}</td>
        </tr>
    }
}

fn runtime_row(
    bundle: &str,
    when: &str,
    raw: &str,
    gzip: &str,
    brotli: &str,
    highlight: bool,
) -> View {
    let td_class = if highlight { "yes" } else { "" };
    view! {
        <tr>
            <td><code>{bundle.to_string()}</code></td>
            <td>{when.to_string()}</td>
            <td>{raw.to_string()}</td>
            <td class={td_class}>{gzip.to_string()}</td>
            <td class={td_class}>{brotli.to_string()}</td>
        </tr>
    }
}

pub fn page(_req: FlowRequest) -> View {
    let title = "Bundle benchmark";
    let lead = "Measured JavaScript/WASM for a counter page (SSR heading + increment button) across Resuma, Qwik, Leptos, Astro, Next.js, SvelteKit, SolidStart, React (Vite), and templ + HTMX. Static Resuma pages ship zero client JS.";

    let h_summary = "Summary (gzip)";
    let h_method = "Methodology";
    let h_validation = "External validation";
    let h_ships = "What each framework ships";
    let h_runtime = "Resuma (split runtime)";
    let h_reproduce = "Reproduce locally";
    let h_takeaways = "Takeaways";

    let col_framework = "Framework";
    let col_initial = "Initial load";
    let col_first = "First interaction";
    let col_static = "Static page";
    let col_notes = "Notes";
    let col_bundle = "Bundle";
    let col_when = "When loaded";
    let col_raw = "Raw";
    let col_gzip = "Gzip";
    let col_brotli = "Brotli";

    let readme_href = "https://github.com/GoldevLab/resuma/blob/main/benchmark/README.md";
    let readme_label = "benchmark/README.md";

    let dash = "—";
    let zero = bundle_sizes::RESUMA_STATIC;
    let resuma_init = bundle_sizes::RESUMA_INITIAL;
    let resuma_first = bundle_sizes::RESUMA_FIRST;
    let qwik_init = "1.96 KiB";
    let qwik_first = "22.32 KiB";
    let htmx_init = "16.21 KiB";
    let solid_init = "16.75 KiB";
    let svelte_init = "27.71 KiB";
    let astro_init = "57.76 KiB";
    let react_init = "57.99 KiB";
    let leptos_init = "79.02 KiB";
    let next_init = "142.43 KiB";

    let loader_js = "loader.js";
    let core_js = "core.js";
    let loader_plus_core = "loader.js + core.js + handler chunk";
    let htmx_js = "htmx.min.js";
    let wasm_glue = ".wasm + glue";
    let next_name = "Next.js";
    let next_scaffold = "React SSR + hydration; default create-next-app scaffold";

    let m1 = "Same UX: SSR heading + one interactive counter button.";
    let m2 = "Production builds in benchmark/*-counter plus runtime/ for Resuma.";
    let m3 = "Report minified raw + gzip + brotli from build artifacts (simulated compression in run.mjs).";
    let m4 = "Initial load = JS required before the page can resume/hydrate interactivity.";
    let m5 = "First interaction = total JS transferred when the user clicks + (loader + core + handler chunk for Resuma).";
    let m6 = "Regenerate anytime: node benchmark/run.mjs -> benchmark/results.json.";

    let why_title = "Why initial = first click for most rows";
    let why_body = "Classic hydration (SvelteKit, SolidStart, Astro client:load, React SPA, Leptos, Next counter) ships all client JS referenced by the HTML on first paint. There is no lazy split on click, so both columns show the same total. Only Resuma and Qwik defer meaningful runtime work until first interaction.";

    let caveat_title = "Scaffold caveat";
    let chrome_title = "This docs site is not the 0 B row";
    let chrome_body = "The 0 B column is a Resuma page with no signals, handlers, NavLink, or PWA. This documentation site always ships loader.js because the shell uses NavLink, a theme picker, view transitions, and PWA registration — plus a visible_task on /docs layouts. That chrome is separate from the counter benchmark.";
    let caveat_body = "Our Next counter uses the default create-next-app scaffold (App Router, Tailwind, Geist fonts, Turbopack chunks). That is an honest out-of-the-box number, not a lean production baseline. Optimized App Router setups often report 67-78 KiB first-load JS when Server Components eliminate most client bundles. Treat 142 KiB as upper-bound for default tooling, not a tuned app.";

    let validation_body = "Independent published measurements agree with our numbers (same ranking, same order of magnitude). Qwik preloader ~2 KiB, HTMX ~16 KiB, Astro React client ~59 KiB, React 19 Vite ~59 KiB, SvelteKit ~32 KiB in the SendOT portfolio series - all line up with our counter benchmark.";
    let validation_link = "Full reference table with links in the repo README. Next.js 142 KiB reflects the default create-next-app scaffold; optimized App Router apps often land near 67-78 KiB first-load JS.";

    let t1 = "Resuma: 1021 B gzip initial, 9.90 KiB gzip to full interactivity (loader + core + counter handler) - no WASM, lazy core on first click.";
    let t2 = "Qwik: smallest resumable JS preloader (~2 KiB gzip), core chunk adds ~20 KiB gzip on first click.";
    let t3 = "templ + HTMX: ~16 KiB gzip for HTMX alone; interactivity is a server round-trip, not client hydration.";
    let t4 = "SolidStart / SvelteKit: mid-tier hydration bundles (~17-28 KiB gzip) for a minimal counter.";
    let t5 = "Astro / React: ~58 KiB gzip - React runtime dominates whether island or SPA.";
    let t6 = "Leptos: WASM hydration bundle ~79 KiB gzip even for a minimal counter.";
    let t7 = "Next.js: ~142 KiB gzip first-load JS on default App Router scaffold (includes framework + React runtime).";
    let t8 = "Static-first: only Resuma skips all client JS on non-interactive pages.";

    let reproduce = r#"node benchmark/run.mjs

# Or individually:
cd runtime && npm run build && npm run size
cd benchmark/qwik-counter && npm run build
cd benchmark/leptos-counter && wasm-pack build --target web --release
cd benchmark/astro-counter && npm run build
cd benchmark/next-counter && npm run build
cd benchmark/sveltekit-counter && npm run build
cd benchmark/solidstart-counter && npm run build
cd benchmark/react-counter && npm run build

curl -H "Accept-Encoding: gzip" http://127.0.0.1:3000/_resuma/benchmark.json
cargo run -p example-counter"#;

    view! {
        <>
            <h1>{title.to_string()}</h1>
            <p class="lead">{lead.to_string()}</p>

            <h2>{h_summary.to_string()}</h2>
            <table class="compare">
                <thead>
                    <tr>
                        <th>{col_framework.to_string()}</th>
                        <th>{col_initial.to_string()}</th>
                        <th>{col_first.to_string()}</th>
                        <th>{col_static.to_string()}</th>
                    </tr>
                </thead>
                <tbody>
                    {summary_row("Resuma", resuma_init, resuma_first, zero, true)}
                    {summary_row("Leptos 0.7", leptos_init, leptos_init, dash, false)}
                    {summary_row("Next.js 16 (App Router)", next_init, next_init, dash, false)}
                    {summary_row("React 19 (Vite SPA)", react_init, react_init, dash, false)}
                    {summary_row("Astro 5.7 (React island)", astro_init, astro_init, dash, false)}
                    {summary_row("SvelteKit 2.57", svelte_init, svelte_init, dash, false)}
                    {summary_row("Qwik 1.20", qwik_init, qwik_first, dash, false)}
                    {summary_row("SolidStart 1.2", solid_init, solid_init, dash, false)}
                    {summary_row("templ + HTMX 2", htmx_init, htmx_init, dash, false)}
                </tbody>
            </table>

            <h2>{h_method.to_string()}</h2>
            <ol>
                <li>{m1.to_string()}</li>
                <li>{m2.to_string()}</li>
                <li>{m3.to_string()}</li>
                <li>{m4.to_string()}</li>
                <li>{m5.to_string()}</li>
                <li>{m6.to_string()}</li>
            </ol>

            <h3>{why_title.to_string()}</h3>
            <p>{why_body.to_string()}</p>

            <h3>{caveat_title.to_string()}</h3>
            <p>{caveat_body.to_string()}</p>

            <h3>{chrome_title.to_string()}</h3>
            <p>{chrome_body.to_string()}</p>

            <h2>{h_validation.to_string()}</h2>
            <p>{validation_body.to_string()}</p>
            <p>
                {validation_link.to_string()}
                {" "}
                {ext_link(readme_href, readme_label)}
            </p>

            <h2>{h_ships.to_string()}</h2>
            <table class="compare">
                <thead>
                    <tr>
                        <th>{col_framework.to_string()}</th>
                        <th>{col_initial.to_string()}</th>
                        <th>{col_first.to_string()}</th>
                        <th>{col_notes.to_string()}</th>
                    </tr>
                </thead>
                <tbody>
                    {detail_row("Resuma", loader_js, loader_plus_core, "Rust SSR + resumability; static pages = 0 B")}
                    {detail_row("Leptos", wasm_glue, "same", "Rust SSR + WASM hydrate")}
                    {detail_row(next_name, "firstLoadChunkPaths (App Router)", "same", next_scaffold)}
                    {detail_row("React (Vite)", "single SPA bundle", "same", "Client-rendered baseline")}
                    {detail_row("Astro", "React island + client runtime", "same", "client:load React island")}
                    {detail_row("SvelteKit", "entry + app + runtime chunks", "same", "SSR + client hydration (adapter-static)")}
                    {detail_row("Qwik", "preloader", "preloader + core + route + onClick chunk", "Resumability (JS)")}
                    {detail_row("SolidStart", "client + web + index chunks", "same (full hydration on load)", "Solid SSR + hydration")}
                    {detail_row("templ + HTMX", htmx_js, "same (server round-trip on click)", "Go SSR + HTMX; no client app bundle")}
                </tbody>
            </table>

            <h2>{h_runtime.to_string()}</h2>
            <table class="compare">
                <thead>
                    <tr>
                        <th>{col_bundle.to_string()}</th>
                        <th>{col_when.to_string()}</th>
                        <th>{col_raw.to_string()}</th>
                        <th>{col_gzip.to_string()}</th>
                        <th>{col_brotli.to_string()}</th>
                    </tr>
                </thead>
                <tbody>
                    {runtime_row(loader_js, "Interactive pages only", bundle_sizes::LOADER_RAW, resuma_init, bundle_sizes::LOADER_BROTLI, true)}
                    {runtime_row(core_js, "First interaction", bundle_sizes::CORE_RAW, bundle_sizes::CORE_GZIP, bundle_sizes::CORE_BROTLI, true)}
                    {runtime_row("Counter handler", "First click (with core)", "133 B", "129 B", "109 B", false)}
                    {runtime_row("Static Resuma page", "Never (no handlers/signals)", zero, zero, zero, true)}
                </tbody>
            </table>

            <h2>{h_reproduce.to_string()}</h2>
            {code_block(reproduce)}

            <h2>{h_takeaways.to_string()}</h2>
            <ul>
                <li>{t1.to_string()}</li>
                <li>{t2.to_string()}</li>
                <li>{t3.to_string()}</li>
                <li>{t4.to_string()}</li>
                <li>{t5.to_string()}</li>
                <li>{t6.to_string()}</li>
                <li>{t7.to_string()}</li>
                <li>{t8.to_string()}</li>
            </ul>
        </>
    }
}
