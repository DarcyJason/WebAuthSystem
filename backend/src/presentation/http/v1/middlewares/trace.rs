use tower_http::trace::{HttpMakeClassifier, TraceLayer};

pub fn trace_middleware() -> TraceLayer<HttpMakeClassifier> {
    TraceLayer::new_for_http()
}
