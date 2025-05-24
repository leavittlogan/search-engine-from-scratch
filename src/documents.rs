use std::collections::{hash_map::Iter, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Document {
    pub id: String,
    pub text: String,
}

#[derive(Default)]
pub struct DocumentStore {
    documents: HashMap<String, Document>,
}

impl DocumentStore {
    pub fn insert(&mut self, document: Document) {
        self.documents.insert(document.id.clone(), document);
    }

    pub fn get(&self, id: &str) -> Option<&Document> {
        self.documents.get(id)
    }

    pub fn iter(&self) -> Iter<String, Document> {
        self.documents.iter()
    }
}

// to calculate BM25 score:
// score(document, word) = term_frequency(document, word) * inverse_document_frequency(word)
//
// term_frequency_score(document, word) = frequency(word, document) * (k1 + 1) / (frequency(word, document) + k1 * (1 - b + b * document_length / avg_document_length))
//
// k1 = 1.5
// b = 0.75
//
// inverse_document_frequency(word) = log((total_documents - document_with_word + 0.5) / (document_with_word + 0.5))
//
// we need to calculate the following:
// - frequency(word, document)
// - document_length
// - avg_document_length
// - total_documents
// - document_with_word
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document {
            id: "test-id".to_string(),
            text: "This is a test document".to_string(),
        };

        assert_eq!(doc.id, "test-id");
        assert_eq!(doc.text, "This is a test document");
    }

    #[test]
    fn test_document_store_insert_and_get() {
        let mut store = DocumentStore::default();
        let doc = Document {
            id: "doc1".to_string(),
            text: "First document".to_string(),
        };

        store.insert(doc.clone());
        
        let retrieved = store.get("doc1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().text, "First document");
    }

    #[test]
    fn test_document_store_get_nonexistent() {
        let store = DocumentStore::default();
        let result = store.get("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_document_store_update() {
        let mut store = DocumentStore::default();
        
        let doc1 = Document {
            id: "doc1".to_string(),
            text: "Original text".to_string(),
        };
        store.insert(doc1);

        let doc2 = Document {
            id: "doc1".to_string(),
            text: "Updated text".to_string(),
        };
        store.insert(doc2);

        let retrieved = store.get("doc1").unwrap();
        assert_eq!(retrieved.text, "Updated text");
    }

    #[test]
    fn test_document_store_iter() {
        let mut store = DocumentStore::default();
        
        let doc1 = Document {
            id: "doc1".to_string(),
            text: "First document".to_string(),
        };
        let doc2 = Document {
            id: "doc2".to_string(),
            text: "Second document".to_string(),
        };

        store.insert(doc1);
        store.insert(doc2);

        let documents: Vec<_> = store.iter().collect();
        assert_eq!(documents.len(), 2);
        
        let ids: Vec<&String> = documents.iter().map(|(id, _)| *id).collect();
        assert!(ids.contains(&&"doc1".to_string()));
        assert!(ids.contains(&&"doc2".to_string()));
    }

    #[test]
    fn test_document_clone() {
        let doc = Document {
            id: "test-id".to_string(),
            text: "Test content".to_string(),
        };

        let cloned = doc.clone();
        assert_eq!(doc.id, cloned.id);
        assert_eq!(doc.text, cloned.text);
    }
}
