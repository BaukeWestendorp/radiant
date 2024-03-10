use gpui::{
    div, rgb, IntoElement, Model, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext,
};

use crate::show::Show;
use crate::workspace::{ProgrammerState, Workspace};

pub struct Screen {
    // pub layout: View<LayoutView>,
    programmer_state: ProgrammerState,
    show_name: SharedString,
}

impl Screen {
    pub fn build(show: Model<Show>, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let workspace_view = cx.view().clone();

        cx.new_view(|cx| {
            // let layout = cx.new_view(|cx| LayoutView::new(cx));

            cx.observe(&workspace_view, |this: &mut Screen, workspace, cx| {
                this.programmer_state = workspace.read(cx).programmer_state;
            })
            .detach();

            cx.observe(&show, |this: &mut Screen, show, cx| {
                this.show_name = show.read(cx).name.clone();
            })
            .detach();

            Self {
                // layout,
                show_name: show.read(cx).name.clone(),
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
            .child(format!("Programmer State: {}", self.programmer_state))
            .child(format!("name: {}", self.show_name));

        div()
            .flex()
            .flex_col()
            .size_full()
            // .child(content)
            .child(status_bar)
    }
}
