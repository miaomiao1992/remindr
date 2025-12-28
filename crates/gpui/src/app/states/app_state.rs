use gpui_nav::Navigator;

pub struct AppState {
    pub navigator: Navigator,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            navigator: Navigator::new(),
        }
    }
}
