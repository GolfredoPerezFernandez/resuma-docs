use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"i18n"</h1>
            <p class="lead">"Internationalization in Resuma Flow — load locale strings server-side in " <code>"#[load]"</code>"."</p>

            {crate::site::demos::integrations_i18n()}

            <h2>"Recommended crates"</h2>
            <ul>
                <li><code>"fluent"</code> " / " <code>"fluent-bundle"</code> " — Mozilla Fluent (.ftl files)"</li>
                <li><code>"rust-i18n"</code> " — compile-time JSON/YAML catalogs"</li>
            </ul>

            <h2>"Locale loader"</h2>
            {code_block(r#"#[load]
async fn i18n(req: &FlowRequest) -> Messages {
    let lang = req.query_param("lang")
        .or_else(|| req.header("accept-language").map(|s| s.split(',').next().unwrap_or("en")))
        .unwrap_or("en");
    Messages::load(lang).await
}

pub fn page(_req: FlowRequest) -> View {
    let t = use_i18n_load();
    view! {
        <h1>{t.get("home.title")}</h1>
        <p>{t.get("home.lead")}</p>
    }
}"#)}

            <h2>"RTL layout"</h2>
            <p>
                "Strings from Fluent are not enough — flex and margins must flip. Set "
                <code>"<html dir>"</code> " and use logical CSS:"
            </p>
            {code_block(r#"FlowApp::new().with_dir("rtl")

#[load]
async fn i18n(req: &FlowRequest) -> Messages {
    let lang = req.query_param("lang").unwrap_or("en");
    if lang.starts_with("ar") || lang.starts_with("he") {
        set_page_dir("rtl");
    }
    Messages::load(lang).await
}

/* CSS */
.card { margin-inline-start: 1rem; padding-inline: 1rem; }"#)}

            <h2>"URL strategy"</h2>
            <p><code>"/en/docs"</code> ", " <code>"/es/docs"</code> " via Flow file routes or " <code>"?lang=es"</code> " query param with " <code>"#[load]"</code> " cache keys per locale."</p>
        </>
    }
}
