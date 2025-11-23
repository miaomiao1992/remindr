use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Window, div};
use gpui_router::{Route, Routes};

use crate::{components::layout::Layout, screens::login_screen::LoginScreen};

pub mod login_screen;
pub mod main_screen;
pub mod parts;

pub struct Router {
    login_screen: Entity<LoginScreen>,
}

impl Router {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let login_screen = cx.new(|cx| LoginScreen::new(cx));
        Self { login_screen }
    }
}

impl Render for Router {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(
            Routes::new()
                .child(
                    Route::new()
                        .path("login")
                        .element(self.login_screen.clone()),
                )
                .child(
                    Route::new()
                        .layout(Layout::new())
                        .child(Route::new().index().element(div().child("Home")))
                        .child(
                            Route::new()
                                .path("{*not_match}")
                                .element(div().child("Not found")),
                        ),
                ),
        )
    }
}
