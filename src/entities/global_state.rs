use gpui::{
    App, Context, Global, Half, IntoElement, ParentElement, Pixels, Point, Render, RenderOnce,
    Styled, Window, blue, div, px, rgb, white,
};

pub struct GlobalState {
    pub elements: Vec<DragInfo>,
}

impl GlobalState {
    pub fn new() -> Self {
        let mut elements = Vec::<DragInfo>::new();

        elements.push(DragInfo {
            label: "Hello".to_string(),
            ..Default::default()
        });
        elements.push(DragInfo {
            label: "World".to_string(),
            ..Default::default()
        });
        elements.push(DragInfo {
            label: "Hello World".to_string(),
            ..Default::default()
        });

        Self { elements }
    }
}

impl Global for GlobalState {}

#[derive(Clone, IntoElement)]
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
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let size = gpui::size(px(120.), px(50.));
        let label = self.label.clone();

        div()
            .absolute()
            .left_0()
            .flex()
            .bg(white())
            .text_color(gpui::blue())
            .text_xs()
            .shadow_md()
            .child(format!("Item {}", label))
    }
}

impl RenderOnce for DragInfo {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let label = self.label.clone();

        div()
            .w_full()
            .h_12()
            .flex()
            .justify_center()
            .items_center()
            .bg(white())
            .child(format!("Item {}", label))
    }
}
