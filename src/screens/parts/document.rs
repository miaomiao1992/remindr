use gpui::{App, Context, DragMoveEvent, Entity, Window, blue, div, prelude::*, px, rgb};
use gpui_component::{Icon, IconName};

use crate::entities::global_state::DragInfo;

enum MovingElement {
    Top,
    Bottom,
}

pub struct Document {
    elements: Vec<Entity<DragInfo>>,
    dragging_index: Option<usize>,
    is_dragging: bool,
}

impl Document {
    pub fn new(ctx: &mut Context<Document>) -> Self {
        let mut elements = Vec::new();

        for idx in 0..5 {
            let drag_info = ctx.new(|_| DragInfo {
                label: format!("Élément {}", idx + 1),
                ..Default::default()
            });
            elements.push(drag_info);
        }

        Self {
            elements,
            dragging_index: None,
            is_dragging: false,
        }
    }

    fn move_element(&mut self, target_index: usize, position: MovingElement) {
        if let Some(from_index) = self.dragging_index {
            if from_index == target_index {
                self.dragging_index = None;
                self.is_dragging = false;
                return;
            }

            let element = self.elements.remove(from_index);
            let mut to_index = target_index;

            match position {
                MovingElement::Top => {
                    if from_index < target_index {
                        to_index = target_index.saturating_sub(1);
                    }
                }
                MovingElement::Bottom => {
                    if from_index >= target_index {
                        to_index = target_index + 1;
                    }
                }
            }

            let final_index = to_index.clamp(0, self.elements.len());
            self.elements.insert(final_index, element);

            self.dragging_index = None;
            self.is_dragging = false;
        }
    }
}

impl Render for Document {
    fn render(&mut self, _window: &mut Window, ctx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .bg(rgb(0xded3d3))
            .on_drag_move(
                ctx.listener(|this, drag_event: &DragMoveEvent<DragInfo>, _, ctx| {
                    let mouse_position = drag_event.event.position;
                    let bounds = drag_event.bounds;
                    let is_outside = mouse_position.x < bounds.origin.x
                        || mouse_position.y < bounds.origin.y
                        || mouse_position.x > bounds.origin.x + bounds.size.width
                        || mouse_position.y > bounds.origin.y + bounds.size.height;
                    if is_outside {
                        this.dragging_index = None;
                        this.is_dragging = false;
                        ctx.notify();
                    }
                }),
            )
            .on_drop(ctx.listener(|this, _: &DragInfo, _, ctx| {
                println!("Drop");
                this.dragging_index = None;
                this.is_dragging = false;
                ctx.notify();
            }))
            .children(
                self.elements
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(index, element)| {
                        div()
                            .w_full()
                            .h_12()
                            .flex()
                            .justify_center()
                            .items_center()
                            .hover(|this| this.bg(blue().opacity(0.5)))
                            .drag_over(|style, _: &DragInfo, _, _| style.cursor_not_allowed())
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                ctx.listener(move |this, _, _, ctx| {
                                    println!("Mouse down {}", index);
                                    this.dragging_index = Some(index);
                                    this.is_dragging = true;

                                    ctx.notify();
                                }),
                            )
                            .child(
                                div()
                                    .id(("item", index))
                                    .child(Icon::from(IconName::ArrowDown))
                                    .size_8()
                                    .mr(px(8.0))
                                    .when(self.is_dragging.clone(), |this| this.cursor_move())
                                    .on_drag(
                                        element.clone(), // Passe l'Entity<DragInfo> clonée comme données de glissement
                                        move |element, _, _window: &mut Window, _: &mut App| {
                                            element.clone()
                                        },
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(format!("Item ({})", element.read(ctx).label)),
                            )
                            .when(self.is_dragging, |this| {
                                let drop_zone_index = index;

                                let top_dropable_zone_element = div()
                                    .absolute()
                                    .tab_index(2)
                                    .w_full()
                                    .h_1_2()
                                    .top_0()
                                    .on_drop(ctx.listener(
                                        move |this, _: &Entity<DragInfo>, _, ctx| {
                                            this.is_dragging = false;
                                            this.move_element(drop_zone_index, MovingElement::Top);

                                            ctx.notify();
                                        },
                                    ));

                                let bottom_dropable_zone_element = div()
                                    .absolute()
                                    .tab_index(2)
                                    .w_full()
                                    .h_1_2()
                                    .bottom_0()
                                    .on_drop(ctx.listener(
                                        move |this, _: &Entity<DragInfo>, _, ctx| {
                                            this.is_dragging = false;
                                            this.move_element(
                                                drop_zone_index,
                                                MovingElement::Bottom,
                                            );

                                            ctx.notify();
                                        },
                                    ));

                                this.child(top_dropable_zone_element)
                                    .child(bottom_dropable_zone_element)
                                    .child(
                                        div()
                                            .absolute()
                                            .tab_index(1)
                                            .bg(rgb(0xE5EFFC))
                                            .w_full()
                                            .h_1()
                                            .bottom_neg_0p5(),
                                    )
                            })
                    }),
            )
    }
}
