use crate::site::code_block;
use resuma::prelude::*;

pub fn page(req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Query parameters"</h1>
            <p class="lead">"Read URL search params from " <code>"FlowRequest"</code> " in pages, loaders, and submits."</p>

            {crate::site::demos::flow_query_params(&req)}

            <h2>"Single parameter"</h2>
            {code_block(r#"pub fn page(req: FlowRequest) -> View {
    let q = req.query_param("q").unwrap_or("");
    view! {
        <>
            <h1>"Search"</h1>
            <p>"Query: " {q.to_string()}</p>
        </>
    }
}"#)}

            <h2>"In loaders"</h2>
            {code_block(r#"#[load]
async fn search_results(req: &FlowRequest) -> Result<Vec<Item>, LoaderError> {
    let q = req.query_param("q").unwrap_or("");
    if q.is_empty() {
        return Ok(vec![]);
    }
    db::search(q).await.map_err(|_| LoaderError::new(500, "search failed"))
}"#)}

            <h2>"Forms that preserve query state"</h2>
            {code_block(r#"<form method="get" action="/docs/search">
    <input type="search" name="q" placeholder="Search docs…" />
</form>"#)}

            <h2>"Reload loaders without full page refresh (SPA)"</h2>
            <p>
                "Changing only client state does " <strong>"not"</strong> " re-run "
                <code>"#[load]"</code> ". For booking flows (pick a date, refresh time slots), "
                "navigate to the same path with new query params so the server renders again:"
            </p>
            {code_block(r#"on:change={js! {
    const input = event.target;
    if (!(input instanceof HTMLInputElement) || !input.value) return;
    await __resuma.navigate(__resuma.buildUrl("/reservar", {
        fecha: input.value,
        servicio: new URLSearchParams(location.search).get("servicio"),
    }));
}}"#)}
            <p>
                "Read the control from " <code>"event.target"</code> " (the input that fired "
                <code>"change"</code> "). " <code>"event.currentTarget"</code> " is the element "
                "that declared the handler — the same node when the handler sits on the input."
            </p>

            <h3>"Rust helpers"</h3>
            {code_block(r#"loader_refresh_input(
    "/reservar",
    "fecha",
    &date,
    &["servicio"],   // query keys to preserve
    "date",
    vec![("required".into(), AttrValue::Bool(true))],
)

query_nav_link("/servicios", &[("cat", "corte")], "active", "", vec![...]);

build_query_href("/book", &[("fecha", "2026-06-02")]);"#)}
            <p>
                "See template " <code>"flow-booking"</code> " ("
                <code>"resuma new my-app --template flow-booking"</code>") and "
                <a href="/docs/flow/pwa">"PWA and public/"</a> " for a full sample."
            </p>

            <h2>"Flash after redirect"</h2>
            <p>
                "Combine " <a href="/docs/cookbook/prg">"PRG redirects"</a> " with a query flag:"
            </p>
            {code_block(r#"#[submit]
async fn create(form: ItemForm, _req: &FlowRequest) -> Result<Redirect, SubmitError> {
    db::insert(&form).await.map_err(|_| SubmitError::new("Failed"))?;
    Ok(redirect("/items?created=1"))
}

pub fn page(req: FlowRequest) -> View {
    let created = req.query_param("created") == Some("1");
    view! {
        <>
            {if created {
                view! { <p class="toast">"Item created!"</p> }
            } else {
                View::empty()
            }}
        </>
    }
}"#)}

            <h2>"Parsing notes"</h2>
            <ul>
                <li>"Values are percent-decoded by the Flow request layer."</li>
                <li>"Duplicate keys: last value wins (same as typical query parsers)."</li>
                <li>"Use path params (" <code>"req.param('id')"</code> ") for " <code>"/users/:id"</code> " segments."</li>
            </ul>
        </>
    }
}
