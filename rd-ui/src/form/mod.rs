use gpui::{App, Entity, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::{Button, h_flex, v_flex};

mod delegate;
mod event;
mod state;

pub use delegate::*;
pub use event::*;
pub use state::*;

#[derive(IntoElement)]
pub struct Form<D: FormDelegate + 'static> {
    state: Entity<FormState<D>>,
}

impl<D: FormDelegate + 'static> Form<D> {
    pub fn new(state: Entity<FormState<D>>, _window: &mut Window, _cx: &mut App) -> Self {
        Self { state }
    }
}

impl<D: FormDelegate + 'static> RenderOnce for Form<D> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let fields = self.state.read(cx).delegate().fields().into_iter().map(|field| {
            h_flex()
                .w_full()
                .gap_4()
                .items_center()
                .child(div().w(px(120.0)).child(field.label))
                .child(div().flex_1().child(field.input))
        });

        let submit_button = Button::new("submit").child("Submit").on_click({
            let state = self.state.clone();
            move |_, _, cx| {
                state.update(cx, |state, cx| state.submit(cx));
            }
        });

        v_flex().gap_4().size_full().children(fields).child(submit_button)
    }
}
