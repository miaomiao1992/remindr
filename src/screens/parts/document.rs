use std::cell::RefCell;
use std::rc::Rc;

use gpui::{Context, DragMoveEvent, Entity, Window, div, prelude::*, px};
use gpui_component::ActiveTheme;

use crate::{
    Utils,
    controllers::drag_controller::{DragController, DragElement},
    entities::elements::{Element, ElementNode, text_element::TextElement},
};

pub struct DocumentState {
    pub elements: Rc<RefCell<Vec<ElementNode>>>,
    pub drag_controller: Rc<RefCell<DragController>>,
}

pub struct Document {
    state: Entity<DocumentState>,
}

impl Document {
    pub fn new(window: &mut Window, ctx: &mut Context<Document>) -> Self {
        let state = ctx.new(|_| DocumentState {
            elements: Rc::new(RefCell::new(Vec::new())),
            drag_controller: Rc::new(RefCell::new(DragController::default())),
        });

        let elements = state.read(ctx).elements.clone();

        let id = Utils::generate_uuid();

        let drag_info =
            Element::Text(ctx.new(|ctx| TextElement::new(id, window, ctx, state.clone())));
        let drag_element = ctx.new(|_| DragElement::new(id, state.clone(), drag_info));
        let element_node = ElementNode::with_id(id, drag_element);

        elements.borrow_mut().push(element_node);

        Self { state }
    }
}

impl Render for Document {
    fn render(&mut self, _: &mut Window, ctx: &mut Context<Self>) -> impl IntoElement {
        let elements = self.state.read(ctx).elements.clone();
        let controller = self.state.read(ctx).drag_controller.clone();

        div()
            .flex()
            .flex_1()
            .justify_center()
            .bg(ctx.theme().background.opacity(0.8))
            .child(
                div()
                    .max_w(px(820.0))
                    .w_full()
                    .on_drag_move(
                        ctx.listener(move |_, event: &DragMoveEvent<Element>, _, ctx| {
                            let is_outside = controller.borrow_mut().on_outside(event);
                            if is_outside {
                                ctx.notify();
                            }
                        }),
                    )
                    .children(
                        elements
                            .borrow()
                            .iter()
                            .map(|node| div().child(node.element.clone())),
                    ),
            )
    }
}
