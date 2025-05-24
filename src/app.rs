use std::sync::{Arc, RwLock};

use axum::Router;

use crate::documents::DocumentStore;
use crate::routes::document_routes;

#[derive(Default)]
pub struct AppState {
    pub document_store: DocumentStore,
}

pub type SharedState = Arc<RwLock<AppState>>;

pub fn build_app() -> Router {
    let shared_state = SharedState::default();

    Router::new()
        // .route("/search", get(handle_search))
        .merge(document_routes())
        .with_state(shared_state)
}
