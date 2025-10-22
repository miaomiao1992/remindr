use anyhow::{Error, Ok};
use gpui::{
    AppContext, BorrowAppContext, Context, Entity, IntoElement, Render, SharedString, Styled,
    Subscription, Window, transparent_white,
};
use gpui_component::{
    StyledExt,
    divider::Divider,
    input::{InputEvent, InputState, TextInput},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};
use uuid::Uuid;

use crate::{
    Utils,
    controllers::drag_controller::DragElement,
    entities::ui::elements::{ElementNode, ElementNodeParser, RemindrElement},
    states::document_state::ViewState,
};

#[derive(Debug)]
pub struct DividerElement;

impl ElementNodeParser for DividerElement {
    fn parse(_: &Value, _: &mut Window, _: &mut Context<Self>) -> Result<Self, Error> {
        Ok(Self)
    }
}

impl Render for DividerElement {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Divider::horizontal().my_3()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividerElementData {
    pub id: Uuid,
}
