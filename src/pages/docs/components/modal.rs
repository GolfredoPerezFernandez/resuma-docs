use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Modal"</h1>
            <p class="lead">
                "Stacked dialogs with focus restore. ESC, backdrop click, and "
                <code>"form method=dialog"</code>
                " all dismiss. The trigger gets focus back when the top dialog closes. "
                "Press " <kbd>"/"</kbd> " or use "
                <strong>"Search"</strong>
                " in this header — that’s a live "
                <code>"&lt;Modal&gt;"</code>
                "."
            </p>

            {crate::site::demos::components_modal()}

            <h2>"view!"</h2>
            {code_block(r#"view! {
    <Modal id="confirm" closedBy="any">
        <button slot="trigger" type="button">"Delete"</button>
        <h2>"Delete this job?"</h2>
        <form method="dialog">
            <button type="submit">"Cancel"</button>
        </form>
    </Modal>
}"#)}

            <h2>"Trigger without a slot"</h2>
            <p>
                "If you omit " <code>"slot=\"trigger\""</code>
                ", a leading " <code>"&lt;button&gt;"</code>
                " becomes the opener. A non-button first child stays in the dialog "
                "(open it with " <code>"__resuma.showModal"</code> ")."
            </p>

            <h2>"Native HTML"</h2>
            <p>
                "SSR emits " <code>"<dialog closedby=\"any\">"</code>
                " and a trigger with " <code>"command=\"show-modal\""</code>
                " / " <code>"commandfor"</code>
                ". Chrome uses Invoker Commands with no JS. Safari gets a click fallback in "
                <code>"/_resuma/ui.js"</code> " plus light-dismiss when "
                <code>"closedby"</code> " is missing."
            </p>

            <h2>"Stack"</h2>
            {code_block(r#"js! { __resuma.showModal("confirm"); }
js! { __resuma.hideModal("confirm"); }
js! { __resuma.dismissAll(); }"#)}

            <p>
                "A global live region (" <code>"#r-live"</code> ") is in the document. After a submit, call "
                <code>"__resuma.announce(\"Saved\")"</code>
                " — see " <a href="/docs/components/desktop_ui">"Desktop UI"</a> "."
            </p>
        </>
    }
}
