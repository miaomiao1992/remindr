use gpui::{
    App, Context, Global, Half, IntoElement, ParentElement, Pixels, Point, Render, RenderOnce,
    Styled, Window, black, blue, div, px, rgb, white,
};

#[derive(Clone)]
pub struct DragInfo {
    pub label: String,
    pub index: usize,
}

impl Default for DragInfo {
    fn default() -> Self {
        Self {
            label: String::new(),
            index: 0,
        }
    }
}

impl Render for DragInfo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let label = self.label.clone();

        div()
            .flex()
            .items_center()
            .text_color(black())
            .text_xs()
            .child(format!("Item {}", label))
    }
}
