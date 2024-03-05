use gpui::{
    div, rgb, IntoElement, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::show::{ProgrammerState, ShowModel};

use super::layout::Layout;

#[derive(Clone)]
pub struct Screen {
    layout: Layout,
    programmer_state: ProgrammerState,
}

impl Screen {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let show = ShowModel::global(cx).clone();
            let programmer_state = show.inner.read(cx).programmer_state.clone();
            cx.observe(&show.inner, |this: &mut Self, model, cx| {
                this.programmer_state = model.read(cx).programmer_state.clone();
                cx.notify();
            })
            .detach();

            Screen {
                layout: Layout::new(),
                programmer_state,
            }
        })
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
}

impl Render for Screen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let status_bar = div()
            .flex()
            .justify_center()
            .bg(rgb(0x303030))
            .p_1()
            .h_10()
            .text_xs()
            .child(format!(
                "Programmer State: {}",
                self.programmer_state.to_string()
            ));

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(cx.new_view(|_| self.layout.clone()))
            .child(status_bar)
    }
}
