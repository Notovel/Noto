use crate::editor::document::Document;

pub mod action;
pub mod cursor;
pub mod document;
pub mod history;

pub struct Editor {
    documents: Vec<Document>,
    current_document: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            current_document: 0,
        }
    }
}