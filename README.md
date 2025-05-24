# Overview

`search-engine-from-scratch` is a Rust-based search engine implementation using Axum for the web framework. This is mainly a for-fun project with two goals:

- Get familiar with Rust
- Learn what makes search engines tick

If you're looking for a fully fleshed-out search engine powered by Rust, check out [tantivy](https://github.com/quickwit-oss/tantivy) and [quickwit](https://github.com/quickwit-oss/quickwit)!


### API Endpoints

- `POST /document` - Create a new document (generates UUID)
- `GET /document/{id}` - Retrieve a specific document
- `PUT /document/{id}` - Update a document
- `GET /documents` - Get all documents

Search functionality (`/search`) is planned but not yet implemented.

### Future Implementation Notes

The BM25 scoring algorithm is documented in documents.rs with specific parameters (k1=1.5, b=0.75) and required calculations for term frequency, inverse document frequency, and document length statistics.
