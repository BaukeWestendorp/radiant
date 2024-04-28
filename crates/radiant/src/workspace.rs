use backstage::show::FixtureId;
use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, Global, IntoElement, Model,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::{layout::LayoutView, showfile::Showfile};

pub struct Workspace {
    selected_fixtures: Model<Vec<FixtureId>>,

    current_layout_view: View<LayoutView>,

    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let selected_fixtures =
                cx.new_model(|cx| Showfile::get(cx).show.selected_fixture_ids().to_vec());

            cx.observe_global::<Showfile>(|workspace: &mut Self, cx| {
                workspace
                    .selected_fixtures
                    .update(cx, |selected_fixtures, cx| {
                        let updated_selected_fixtures =
                            Showfile::get(cx).show.selected_fixture_ids().to_vec();
                        if *selected_fixtures != updated_selected_fixtures {
                            *selected_fixtures = updated_selected_fixtures;
                            cx.notify();
                        }
                    });
            })
            .detach();

            cx.observe_global::<Showfile>({
                move |workspace: &mut Self, cx| {
                    let current_layout = workspace.current_layout_view.read(cx).layout.clone();

                    let updated_layout = Showfile::get(cx).layouts.current_layout();
                    if Some(current_layout.read(cx)) != updated_layout {
                        workspace.current_layout_view = LayoutView::build(
                            current_layout,
                            workspace.selected_fixtures.clone(),
                            cx,
                        );
                        cx.notify();
                    }
                }
            })
            .detach();

            Self {
                selected_fixtures: selected_fixtures.clone(),
                focus_handle: cx.focus_handle(),
                current_layout_view: LayoutView::build(
                    cx.new_model(|cx| Showfile::get(cx).layouts.current_layout().unwrap().clone()),
                    selected_fixtures,
                    cx,
                ),
            }
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(gpui::white())
            .child(self.current_layout_view.clone())
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
