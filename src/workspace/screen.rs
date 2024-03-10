use gpui::{
    div, rgb, IntoElement, ParentElement, Render, Styled, View, ViewContext, VisualContext,
};

use crate::workspace::{ProgrammerState, Workspace};

pub struct Screen {
    // pub layout: View<LayoutView>,
    programmer_state: ProgrammerState,
}

impl Screen {
    pub fn build(cx: &mut ViewContext<Workspace>) -> View<Self> {
        let workspace_view = cx.view().clone();

        cx.new_view(|cx| {
            // let layout = cx.new_view(|cx| LayoutView::new(cx));

            cx.observe(&workspace_view, |this: &mut Screen, workspace, cx| {
                this.programmer_state = workspace.read(cx).programmer_state;
            })
            .detach();

            Self {
                // layout,
                programmer_state: ProgrammerState::default(),
            }
        })
    }
}

impl Render for Screen {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        // let content = self.layout.clone();

        let status_bar = div()
            .h_10()
            .px_2()
            .border_t()
            .border_color(rgb(0x3a3a3a))
            .flex()
            .items_center()
            .bg(rgb(0x2a2a2a))
            .child(format!("Programmer State: {}", self.programmer_state));

        div()
            .flex()
            .flex_col()
            .size_full()
            // .child(content)
            .child(status_bar)
    }
}
