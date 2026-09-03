use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"SEO, GEO & AEO"</h1>
            <p class="lead">
                "Resuma is a long-running SSR process: titles, descriptions, canonicals, and answers "
                "are in the first HTML response. Search engines and answer engines can read them without executing your app JS. "
                "This page is the production pattern this docs site uses — including "
                <code>"SITE_URL"</code>
                " so you can ship on a "
                <code>".fly.dev"</code>
                " host today and point a custom domain at the same binary tomorrow."
            </p>

            <h2>"What each acronym means"</h2>
            <ul>
                <li>
                    <strong>"SEO"</strong>
                    " — crawl, index, and snippets in Google/Bing: canonical URLs, unique titles/descriptions, sitemap, status codes, Open Graph."
                </li>
                <li>
                    <strong>"GEO"</strong>
                    " (generative engines) — help assistants find a trustworthy source: "
                    <code>"llms.txt"</code>
                    ", crawler-specific " <code>"robots.txt"</code> " rules, copy-as-markdown, no JS-only docs."
                </li>
                <li>
                    <strong>"AEO"</strong>
                    " (answer engines) — a visible H1 plus a first paragraph that answers the query, FAQ as H2+paragraph, "
                    "and FAQ JSON-LD only where the Q&A is on the page."
                </li>
            </ul>
            <p>
                "Google documents that AI Overviews/AI Mode use the same foundations as Search. "
                "As of 2026-08-03, Google also documents that " <code>"llms.txt"</code>
                " neither helps nor harms Google ranking. Keep " <code>"llms.txt"</code>
                " for other agents (ChatGPT, Perplexity, editor skills). Do not promise citations or rich results."
            </p>

            <h2>"Minimal FlowApp"</h2>
            {code_block(r#"use resuma::prelude::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let origin = std::env::var("SITE_URL")
        .unwrap_or_else(|_| "https://my-app.fly.dev".into());

    let kit = SeoKit::new("My App", &origin)
        .with_locale("en_US")
        .with_llms_summary("My App does X for Y. Not a résumé builder.")
        .with_llms_section("Docs", format!("{origin}/docs — how to use the product."));

    FlowApp::new()
        .with_title("My App")
        .with_description("One or two sentences that match the homepage.")
        .with_site_url(origin)
        .with_og_image("/og.png") // PNG/JPEG 1200×630 — avoid SVG for Facebook/LinkedIn
        .with_seo_kit(kit)
        .with_sitemap_exclude(["/search"]) // noindex / thin URLs
        .auto_pages(pages_root, PagesRegistry)
        .serve(FlowServeOptions::default())
        .await
}"#)}
            <p>
                <code>"SeoKit"</code>
                " is in " <code>"resuma::prelude"</code>
                ". Call " <code>"with_seo_kit"</code>
                " once — kit routes live on the app router; Flow does not remount "
                <code>"/robots.txt"</code> "."
            </p>

            <h2>"SITE_URL — fly.dev today, your domain tomorrow"</h2>
            <p>
                "Canonical, Open Graph, JSON-LD, sitemap " <code>"&lt;loc&gt;"</code>
                ", and " <code>"llms.txt"</code> " links all come from "
                <code>"SITE_URL"</code>
                " (no trailing slash). This docs site currently uses "
                <code>"https://resuma-docs.fly.dev"</code>
                ". When you attach a custom domain, change one value and redeploy — do not hardcode the host in page bodies."
            </p>
            {code_block(r#"# fly.toml [env]
SITE_URL = "https://resuma-docs.fly.dev"

# After DNS + fly certs add docs.example.com:
SITE_URL = "https://docs.example.com""#)}
            <p>"Cutover checklist (do this when the domain is live, not before):"</p>
            <ol>
                <li>"Add the hostname in Fly (" <code>"fly certs add"</code> ") and wait for HTTPS."</li>
                <li>"Set " <code>"SITE_URL"</code> " to the new origin and deploy."</li>
                <li>"301 the old " <code>".fly.dev"</code> " host to the new origin (Fly rewrite or a tiny redirect app)."</li>
                <li>"New Search Console / Bing property on the custom domain; submit " <code>"/sitemap.xml"</code> "."</li>
                <li>"Keep internal links relative (" <code>"/docs/…"</code> ") so HTML does not bake the old host."</li>
            </ol>
            <p>
                "This docs site is already wired that way. Do not add the 301 until the custom host serves HTTPS."
            </p>

            <h2>"Per-page title and description"</h2>
            <p>
                "Call these during render (layout or page). Streaming SSR still sees them: the view is built before the document head is sent."
            </p>
            {code_block(r#"pub fn page(_req: FlowRequest) -> View {
    set_page_title("Workers | Resuma Docs");
    set_page_description(
        "Background workers in Resuma OS: register_worker, events, and jobs next to SSR.",
    );
    view! { <h1>"Workers"</h1> /* … */ }
}"#)}
            <p>
                "Search UIs and parameterized pages should not compete with real docs. This site sets "
                <code>"noindex, follow"</code> " on " <code>"/docs/search"</code>
                " and omits it from the sitemap."
            </p>
            {code_block(r#"set_page_robots("noindex, follow");
FlowApp::new().with_sitemap_exclude(["/docs/search", "/old-alias"])"#)}

            <h2>"Redirect aliases, don't duplicate"</h2>
            <p>
                "If two URLs render the same guide, pick one canonical and 301 the other. "
                "This site maps " <code>"/docs/cookbook/docker"</code> " → " <code>"/docs/cookbook/deploy"</code> "."
            </p>
            {code_block(r#"pub fn page(_req: FlowRequest) -> View {
    stage_response_status(301);
    stage_response_redirect("/docs/cookbook/deploy");
    View::empty()
}"#)}

            <h2>"What SeoKit serves"</h2>
            <ul>
                <li><code>"/robots.txt"</code> " — " <code>"Allow: /"</code> " plus GPTBot, OAI-SearchBot, ChatGPT-User, Google-Extended, Claude-Web, PerplexityBot, and a Sitemap line"</li>
                <li><code>"/llms.txt"</code> " — short product summary + section URLs (from " <code>"SITE_URL"</code> ")"</li>
                <li><code>"/sitemap.xml"</code> " — static Flow routes, minus " <code>"with_sitemap_exclude"</code></li>
                <li>"Head extras: " <code>"llms.txt"</code> " alternate link, optional theme-color (no duplicate robots meta — pages own " <code>"noindex"</code> ")"</li>
            </ul>
            <p>
                "On this site: "
                <a href="/robots.txt">"/robots.txt"</a>
                " · "
                <a href="/llms.txt">"/llms.txt"</a>
                " · "
                <a href="/sitemap.xml">"/sitemap.xml"</a>
                " · "
                <a href="/docs/integrations/og_image">"OG Image"</a>
                "."
            </p>
            <p>
                <strong>"Crawler split (OpenAI):"</strong>
                " " <code>"OAI-SearchBot"</code> " is ChatGPT Search discovery; "
                <code>"GPTBot"</code> " may be used for model training; "
                <code>"ChatGPT-User"</code> " is a user-triggered fetch and may not follow robots the same way. "
                "Do not treat one directive as a switch for all three."
            </p>

            <h2>"AEO on a Resuma page"</h2>
            <ol>
                <li>"One H1 that names the topic (same idea as the title)."</li>
                <li>"A lead paragraph that answers in plain language before the first code sample."</li>
                <li>"Section headings as real questions when the page is an FAQ."</li>
                <li>"Answers in HTML from SSR — not only after a client fetch."</li>
                <li>
                    "FAQ JSON-LD only on pages that show the same Q&A. Valid Schema.org is not a Google FAQ rich-result guarantee "
                    "(Google limited that feature). This site adds FAQPage on "
                    <a href="/docs/faq">"/docs/faq"</a> " only."
                </li>
            </ol>
            {code_block(r#"// Visible Q&A + matching JSON-LD (same questions/answers)
set_page_json_ld(faq_graph(&origin, &[
    ("What is resumability vs hydration?", "Hydration replays the tree; Resuma resumes from SSR HTML."),
]));"#)}

            <h2>"GEO extras that actually help people"</h2>
            <ul>
                <li>
                    <strong>"Copy page / Copy nav"</strong>
                    " — agents and humans can paste canonical markdown with URLs (this docs chrome)."
                </li>
                <li>
                    <strong>"Disambiguation"</strong>
                    " — say what the product is not (Resuma is not a résumé builder) in the homepage title, description, and "
                    <code>"llms.txt"</code> "."
                </li>
                <li>
                    <strong>"Honest JSON-LD"</strong>
                    " — Organization + WebSite on every page; SoftwareApplication on the homepage; no fake ratings/reviews."
                </li>
            </ul>

            <h2>"Open Graph"</h2>
            <p>
                "Use a " <strong>"PNG or JPEG"</strong> " 1200×630. Many social crawlers still mishandle SVG. "
                "Resuma still serves " <code>"/og.svg"</code> " as a fallback icon; this site sets "
                <code>"with_og_image(\"/og.png\")"</code> "."
            </p>
            {code_block(r#"FlowApp::new()
    .with_og_image("/og.png")
    .with_public_dir("public"); // public/og.png → GET /og.png"#)}

            <h2>"What not to do"</h2>
            <ul>
                <li>"Do not put the same meta description on every URL."</li>
                <li>"Do not list redirect aliases and search-result URLs in the sitemap."</li>
                <li>"Do not emit Open Graph tags as " <code>"meta name=og:…"</code> " — use " <code>"property"</code> " (Resuma 1.3.1+)."</li>
                <li>"Do not stuff " <code>"meta name=\"keywords\""</code> " — Google ignores it."</li>
                <li>"Do not block duplicates in robots.txt as a canonicalization method; 301 or noindex instead."</li>
                <li>"Do not claim " <code>"llms.txt"</code> " improves Google rank or that Schema.org guarantees rich results or AI citations."</li>
            </ul>

            <h2>"Analytics (optional)"</h2>
            <p>
                "SeoKit can attach a Meta Pixel or GTM snippet with SPA " <code>"PageView"</code>
                " on " <code>"resuma:navigate"</code>
                ". That is analytics, not ranking. This docs site does not load third-party tags."
            </p>
            {code_block(r#"// Optional — skip unless you actually use the pixel
SeoKit::new("My App", &origin).with_meta_pixel("1234567890");"#)}
        </>
    }
}
