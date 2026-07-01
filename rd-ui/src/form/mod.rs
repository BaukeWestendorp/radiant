use crate::{ActiveTheme, section};
use gpui::{
    AnyElement, App, Entity, FlexDirection, IntoElement, ParentElement, RenderOnce, Styled, Window,
    div, prelude::*,
};

mod delegate;
mod state;

pub use delegate::*;
pub use state::*;

#[derive(IntoElement)]
pub struct Form<D: FormDelegate + 'static> {
    state: Entity<FormState<D>>,
}

impl<D: FormDelegate + 'static> Form<D> {
    pub fn new(state: Entity<FormState<D>>) -> Self {
        Self { state }
    }
}

impl<D: FormDelegate + 'static> RenderOnce for Form<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // We update the state to get mutable context access for listener bindings
        self.state.clone().update(cx, |state, cx| {
            let layout = state.delegate().layout(cx);
            let mut form_container = div().tab_group().flex().flex_col().w_full().gap_4();

            for node in layout {
                form_container = form_container.child(render_node(state, node, window, cx));
            }

            form_container
        })
    }
}

fn render_node<D: FormDelegate + 'static>(
    state: &FormState<D>,
    node: FormNode<D::Id>,
    window: &mut Window,
    cx: &mut Context<FormState<D>>,
) -> AnyElement {
    match node {
        FormNode::Section { title, flex_direction, children } => {
            let mut content = div().flex().flex_col().w_full().gap_2();

            match flex_direction {
                FlexDirection::Row => content = content.flex_row().items_end(),
                FlexDirection::Column => content = content.flex_col(),
                FlexDirection::RowReverse => content = content.flex_row_reverse().items_end(),
                FlexDirection::ColumnReverse => content = content.flex_col_reverse(),
            }

            for child in children {
                content = content.child(render_node(state, child, window, cx));
            }

            match title {
                Some(title) => section(title).child(content).into_any_element(),
                None => content.into_any_element(),
            }
        }
        FormNode::Field { id, label } => {
            let input = state.delegate.render_input(&id, window, cx).into_any_element();

            div()
                .w_full()
                .items_center()
                .gap_2()
                .when_some(label, |e, label| {
                    e.child(div().text_color(cx.theme().fg_secondary).child(label))
                })
                .child(div().child(input.into_any_element()))
                .into_any_element()
        }
        FormNode::Custom { id } => state.delegate.render_input(&id, window, cx).into_any_element(),
    }
}
