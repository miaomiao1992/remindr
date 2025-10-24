use std::{cell::RefCell, rc::Rc, sync::Arc};

use gpui::{
    AppContext, BorrowAppContext, Context, Entity, IntoElement, ParentElement, Pixels, Render,
    Styled, Window, div,
};
use gpui_component::ActiveTheme;

use crate::{
    controllers::drag_controller::{DragController, DragElement},
    entities::{document_parser::DocumentParser, ui::elements::ElementNode},
    screens::parts::{document::Document, sidebar::MenuSidebar},
    states::document_state::{DocumentState, ViewState},
};

pub struct MainScreen {
    document: Entity<Document>,
}

impl MainScreen {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let parser = DocumentParser;
        let entries = parser.load_file("artifacts/demo.json");

        let mut nodes = Vec::new();
        cx.new(|cx| {
            for (id, element) in parser.parse_nodes(&entries, window, cx) {
                let drag_element = cx.new(|_| DragElement::new(id, element));
                let element_node = ElementNode::with_id(id, drag_element);

                nodes.push(element_node);
            }

            parser
        });

        cx.update_global::<ViewState, _>(|this, _| {
            this.current = Some(DocumentState {
                drag_controller: DragController::default(),
                elements: nodes,
            });
        });

        let document = cx.new(|_| Document);

        Self { document }
    }
}

impl Render for MainScreen {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .flex()
            .child(div().bg(cx.theme().accent).child(cx.new(|_| MenuSidebar)))
            .child(self.document.clone())
    }
}
