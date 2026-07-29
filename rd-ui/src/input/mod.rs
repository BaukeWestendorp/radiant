use gpui::{App, Entity, Window, div, prelude::*};

mod builtin;
mod delegate;
mod event;
mod state;

pub use builtin::*;
pub use delegate::*;
pub use event::*;
pub use state::*;

pub const INPUT_HEIGHT: gpui::Pixels = gpui::px(26.0);

#[derive(IntoElement)]
pub struct Input<D: InputDelegate> {
    state: Entity<InputState<D>>,
}

impl<D: InputDelegate> Input<D> {
    pub fn new(state: Entity<InputState<D>>) -> Self {
        Self { state }
    }
}

impl<D: InputDelegate + 'static> RenderOnce for Input<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().w_full().child(D::new_element(self.state, window, cx))
    }
}

pub trait AutoInput: Clone + 'static {
    type Delegate: InputDelegate<Value = Self>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>>;
}
