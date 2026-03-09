use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::TraceLayer;

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
}
