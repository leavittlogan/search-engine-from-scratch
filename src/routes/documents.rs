use axum::routing::{get, post};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::SharedState, documents::Document};

#[derive(Deserialize)]
pub struct DocumentInput {
    text: String,
}

pub async fn handle_create_document(
    State(state): State<SharedState>,
    Json(document): Json<DocumentInput>,
) -> Result<Json<Document>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let document = Document::new(id, document.text);

    state
        .write()
        .unwrap()
        .document_store
        .insert(document.clone());

    Ok(Json(document))
}

pub async fn handle_update_document(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    Json(document): Json<DocumentInput>,
) {
    let document = Document::new(key, document.text);

    state.write().unwrap().document_store.insert(document);
}

pub async fn handle_get_document(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Document>, StatusCode> {
    match state.read().unwrap().document_store.get(&key) {
        Some(document) => Ok(Json(document.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn handle_get_all_documents(
    State(state): State<SharedState>,
) -> Result<Json<Vec<Document>>, StatusCode> {
    let documents = state
        .read()
        .unwrap()
        .document_store
        .iter()
        .map(|(_, document)| document.clone())
        .collect::<Vec<Document>>();

    Ok(Json(documents))
}

pub fn document_routes() -> Router<SharedState> {
    Router::new()
        .route("/document", post(handle_create_document))
        .route(
            "/document/{key}",
            get(handle_get_document).put(handle_update_document),
        )
        .route("/documents", get(handle_get_all_documents))
}

#[cfg(test)]
mod test {
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use serde_json::json;
    use std::sync::{Arc, RwLock};
    use tower::ServiceExt;

    use super::*;
    use crate::{app::AppState, documents::DocumentStore};

    #[tokio::test]
    async fn create_document() {
        // Create test state
        let document_store = DocumentStore::default();
        let app_state = AppState { document_store };
        let shared_state = Arc::new(RwLock::new(app_state));

        // Create app with state
        let app = document_routes().with_state(shared_state.clone());

        // Build proper request
        let request = Request::builder()
            .method(Method::POST)
            .uri("/document")
            .header("content-type", "application/json")
            .body(Body::from(json!({ "text": "hello world" }).to_string()))
            .unwrap();

        // Call oneshot and await the result
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Verify document was stored
        let state = shared_state.read().unwrap();
        let documents: Vec<_> = state.document_store.iter().collect();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].1.text, "hello world");
        assert_eq!(documents[0].1.word_count, 2);
    }

    #[tokio::test]
    async fn get_document() {
        // Create test state with a document
        let mut document_store = DocumentStore::default();
        let test_doc = Document::new("test-id".to_string(), "test content".to_string());
        document_store.insert(test_doc.clone());

        let app_state = AppState { document_store };
        let shared_state = Arc::new(RwLock::new(app_state));
        let app = document_routes().with_state(shared_state);

        let request = Request::builder()
            .method(Method::GET)
            .uri("/document/test-id")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_nonexistent_document() {
        let document_store = DocumentStore::default();
        let app_state = AppState { document_store };
        let shared_state = Arc::new(RwLock::new(app_state));
        let app = document_routes().with_state(shared_state);

        let request = Request::builder()
            .method(Method::GET)
            .uri("/document/nonexistent")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn update_document() {
        // Create test state with a document
        let mut document_store = DocumentStore::default();
        let test_doc = Document::new("test-id".to_string(), "test content".to_string());
        document_store.insert(test_doc);

        let app_state = AppState { document_store };
        let shared_state = Arc::new(RwLock::new(app_state));
        let app = document_routes().with_state(shared_state.clone());

        let request = Request::builder()
            .method(Method::PUT)
            .uri("/document/test-id")
            .header("content-type", "application/json")
            .body(Body::from(json!({ "text": "updated content" }).to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Verify document was updated
        let state = shared_state.read().unwrap();
        let updated_doc = state.document_store.get("test-id").unwrap();
        assert_eq!(updated_doc.text, "updated content");
        assert_eq!(updated_doc.id, "test-id");
        assert_eq!(updated_doc.word_count, 2);
    }

    #[tokio::test]
    async fn get_all_documents() {
        // Create test state with multiple documents
        let mut document_store = DocumentStore::default();
        document_store.insert(Document::new(
            "doc1".to_string(),
            "first document".to_string(),
        ));
        document_store.insert(Document::new(
            "doc2".to_string(),
            "second document".to_string(),
        ));

        let app_state = AppState { document_store };
        let shared_state = Arc::new(RwLock::new(app_state));
        let app = document_routes().with_state(shared_state);

        let request = Request::builder()
            .method(Method::GET)
            .uri("/documents")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
