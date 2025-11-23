use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Window, div};
use gpui_router::{IntoLayout, Outlet, use_location};

#[derive(IntoElement, IntoLayout)]
pub struct Layout {
    outlet: Outlet,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            outlet: Outlet::new(),
        }
    }
}

impl RenderOnce for Layout {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let location = use_location(cx);
        if location.pathname != SharedString::from("/login") {}

        div().child(self.outlet)
    }
}
