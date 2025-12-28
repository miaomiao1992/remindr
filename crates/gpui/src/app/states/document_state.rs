use gpui::{App, AppContext, Entity, Global, Window};
use serde_json::Value;

use crate::app::components::node_renderer::NodeRenderer;

#[derive(Clone)]
pub struct Document {
    pub uid: String,
    pub title: String,
    pub renderer: Entity<NodeRenderer>,
}

pub struct DocumentState {
    pub documents: Vec<Document>,
    pub current_document: Option<Document>,
}

impl DocumentState {
    pub fn get_current_document_index(&self) -> Option<usize> {
        self.documents.iter().position(|doc| {
            doc.uid
                == self
                    .current_document
                    .as_ref()
                    .map(|doc| doc.uid.clone())
                    .unwrap_or_default()
        })
    }

    pub fn get_previous_document(&self, uid: String) -> Option<Document> {
        let current_index = self.documents.iter().position(|doc| doc.uid == uid);
        if let Some(index) = current_index {
            if index > 0 {
                Some(self.documents[index - 1].clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn add_document(
        &mut self,
        uid: String,
        title: String,
        nodes: Vec<Value>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let already_has_document = self.documents.iter().any(|element| element.uid == uid);
        if !already_has_document {
            let renderer = NodeRenderer::new(nodes, window, cx);
            self.documents.push(Document {
                uid,
                title,
                renderer: cx.new(|_| renderer),
            });
        }
    }

    pub fn add_document_and_focus(
        &mut self,
        uid: String,
        title: String,
        nodes: Vec<Value>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.add_document(uid.clone(), title.clone(), nodes, window, cx);
        self.current_document = self
            .documents
            .clone()
            .into_iter()
            .find(|element| element.uid == uid)
    }

    pub fn remove_document(&mut self, uid: String) {
        self.documents.retain(|element| element.uid != uid);
    }
}

impl Global for DocumentState {}
