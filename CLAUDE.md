# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based search engine implementation from scratch using Axum for the web framework. The project is structured as a document storage and retrieval system with plans to implement BM25 scoring for search functionality.

## Architecture

The application follows a modular architecture:

- **main.rs**: Entry point that initializes tracing and starts the HTTP server on port 8080
- **app.rs**: Defines the application state (SharedState) using Arc<RwLock<AppState>> for thread-safe access to the document store and sets up routing
- **documents.rs**: Contains the Document model and DocumentStore (in-memory HashMap). Includes detailed BM25 scoring algorithm comments for future implementation
- **apis.rs**: HTTP handlers for CRUD operations on documents

The application uses shared state pattern with Arc<RwLock<AppState>> to manage concurrent access to the document store across multiple HTTP requests.

## Key Commands

**Build and run:**
```bash
cargo build
cargo run
```

**Development:**
```bash
cargo check        # Fast compilation check
cargo clippy       # Linting
cargo fmt          # Code formatting
```

## API Endpoints

- `POST /document` - Create a new document (generates UUID)
- `GET /document/{id}` - Retrieve a specific document
- `PUT /document/{id}` - Update a document
- `GET /documents` - Get all documents

Search functionality (`/search`) is planned but not yet implemented.

## Future Implementation Notes

The BM25 scoring algorithm is documented in documents.rs with specific parameters (k1=1.5, b=0.75) and required calculations for term frequency, inverse document frequency, and document length statistics.