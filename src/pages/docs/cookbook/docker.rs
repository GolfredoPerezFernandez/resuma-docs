use resuma::prelude::*;

/// Old `/docs/cookbook/docker` URL — permanent redirect to the deploy guide.
pub fn page(_req: FlowRequest) -> View {
    stage_response_status(301);
    stage_response_redirect("/docs/cookbook/deploy");
    View::empty()
}
