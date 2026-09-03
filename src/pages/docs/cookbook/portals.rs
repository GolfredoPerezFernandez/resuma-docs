use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Portals"</h1>
            <p class="lead">
                "Need a dialog or anchored menu? Use "
                <a href="/docs/components/modal">"Modal"</a> " / "
                <a href="/docs/components/popup">"Popup"</a>
                " — they ship native " <code>"<dialog>"</code> " / " <code>"popover"</code>
                " with focus restore. " <code>"portal()"</code>
                " only moves DOM nodes (toasts, custom shells)."
            </p>

            {crate::site::demos::cookbook_portals()}

            <h2>"Prefer Modal / Popup"</h2>
            {code_block(r#"view! {
    <Modal id="confirm">
        <button slot="trigger" type="button">"Open"</button>
        <p>"Focus-trapped dialog"</p>
    </Modal>
}"#)}

            <h2>"Portal target"</h2>
            <p>"Add a target element in your layout shell:"</p>
            {code_block(r#"view! {
    <>
        <main><Slot /></main>
        <div id="modal-root" data-r-portal-target="modal"></div>
    </>
}"#)}

            <h2>"Portal content"</h2>
            {code_block(r#"#[component]
fn Modal(open: bool, children: Vec<Child>) -> View {
    if !open {
        return view! { <></> };
    }
    portal("modal", children)
}

view! {
    <Modal open={show_modal.get()}>
        <div class="modal-backdrop">
            <div class="modal">"Dialog content"</div>
        </div>
    </Modal>
}"#)}

            <h2>"data-r-portal"</h2>
            <p>"SSR emits a " <code>"<template data-r-portal=\"modal\">"</code> ". After resume, the runtime moves children into the matching " <code>"data-r-portal-target"</code> " element."</p>
        </>
    }
}
