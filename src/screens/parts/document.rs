use std::cell::RefCell;
use std::rc::Rc;

use gpui::{App, Context, DragMoveEvent, Entity, Window, blue, div, prelude::*, px, rgb, white};
use gpui_component::{Icon, IconName};

use crate::{
    controllers::drag_controller::{DragController, DragElement, MovingElement},
    entities::global_state::DragInfo,
};

pub struct Document {
    elements: Rc<RefCell<Vec<Entity<DragElement<DragInfo>>>>>,
    drag_controller: Rc<RefCell<DragController>>,
}

impl Document {
    pub fn new(ctx: &mut Context<Document>) -> Self {
        let drag_controller = Rc::new(RefCell::new(DragController::default()));
        let elements = Rc::new(RefCell::new(Vec::new()));

        for idx in 0..5 {
            let controller_clone = Rc::clone(&drag_controller);
            let drag_info = ctx.new(|ctx| DragElement {
                index: idx,
                controller: Rc::clone(&controller_clone), // stocke le Rc directement
                on_drop: {
                    let controller_clone2 = Rc::clone(&controller_clone);
                    let elements_clone2 = Rc::clone(&elements);
                    Box::new(move |index, direction| {
                        controller_clone2.borrow_mut().drop_element(
                            &mut elements_clone2.borrow_mut(),
                            index,
                            direction,
                        );
                    })
                },
                element: ctx.new(|_| DragInfo {
                    label: format!("Élément {}", idx + 1),
                    ..Default::default()
                }),
            });

            elements.borrow_mut().push(drag_info);
        }

        Self {
            elements,
            drag_controller,
        }
    }
}

impl Render for Document {
    fn render(&mut self, window: &mut Window, ctx: &mut Context<Self>) -> impl IntoElement {
        let elements = Rc::clone(&self.elements);
        let drag_controller = Rc::clone(&self.drag_controller);

        div()
            .flex_1()
            .bg(rgb(0xded3d3))
            .on_drag_move(
                ctx.listener(move |this, event: &DragMoveEvent<DragInfo>, _, ctx| {
                    if drag_controller.borrow_mut().on_outside(event) {
                        ctx.notify();
                    }
                }),
            )
            .children(
                elements
                    .borrow()
                    .iter()
                    .enumerate()
                    .map(|(index, element)| div().child(element.clone())),
            )
    }
}
