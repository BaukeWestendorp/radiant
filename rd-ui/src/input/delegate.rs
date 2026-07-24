use gpui::Window;
use gpui::{App, Entity, Focusable, prelude::*};

use crate::InputState;

pub trait InputDelegate: Focusable {
    type Value;

    fn new_element(
        state: Entity<InputState<Self>>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement
    where
        Self: Sized;

    fn value_or_default(&self, cx: &App) -> Self::Value
    where
        Self::Value: Default;
}
