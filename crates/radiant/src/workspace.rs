use backstage::show::FixtureId;
use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, Global, IntoElement, Model,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::{attribute_editor::AttributeEditor, showfile::Showfile};

pub struct Workspace {
    attribute_editor: View<AttributeEditor>,
    selected_fixtures: Model<Vec<FixtureId>>,
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

            Self {
                focus_handle: cx.focus_handle(),
                attribute_editor: AttributeEditor::build(selected_fixtures.clone(), cx),
                selected_fixtures,
            }
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(gpui::white())
            .child(self.attribute_editor.clone())
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
