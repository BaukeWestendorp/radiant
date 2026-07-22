use gpui::Window;
use gpui::{App, Entity, EventEmitter, Focusable, prelude::*};

use crate::{InputEvent, InputState};

pub trait InputDelegate: Focusable + EventEmitter<InputEvent<Self::Value>> {
    type Value;

    fn new_element(
        state: Entity<InputState<Self>>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement
    where
        Self: Sized;
}
