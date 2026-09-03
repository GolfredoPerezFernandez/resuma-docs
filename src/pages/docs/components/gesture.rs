use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"GestureView"</h1>
            <p class="lead">
                "Pan, pinch, long-press, double-tap, and scroll-wheel for PWAs. The loader stays small: gestures live in the lazy "
                <code>"ui.js"</code> " chunk."
            </p>

            {crate::site::demos::components_gesture()}

            <h2>"view!"</h2>
            {code_block(r#"view! {
    <GestureView
        preferredPan="horizontal"
        panThreshold={10}
        onPan={js! { state.dx.set(event.detail.dx); }}
        onLongPress={js! { state.held.set(true); }}
        onDoubletap={js! { /* … */ }}
        onPinch={js! { state.scale.set(event.detail.scale); }}
        onScrollwheel={js! { /* event.detail.dx / dy */ }}
    >
        <canvas />
    </GestureView>
}"#)}

            <p>
                <code>"preferredPan"</code> " is " <code>"horizontal"</code> " or "
                <code>"vertical"</code> " (lock the other axis until the threshold). "
                <code>"panThreshold"</code> " defaults to 10px."
            </p>
            <p>
                "Handlers compile to " <code>"data-r-on:pan"</code> " etc. They are "
                <strong>"not"</strong> " in the loader event list — "
                <code>"ui.js"</code> " calls " <code>"__resuma.runHandler"</code> " after detecting the gesture."
            </p>
        </>
    }
}
