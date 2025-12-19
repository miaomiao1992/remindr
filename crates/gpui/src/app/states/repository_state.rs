use gpui::Global;

use crate::infrastructure::repositories::document_repository::DocumentRepository;

pub struct RepositoryState {
    pub documents: DocumentRepository,
}

impl Global for RepositoryState {}
