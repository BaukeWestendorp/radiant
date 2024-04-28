use backstage::show::FixtureId;
use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, Global, IntoElement, Model,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::{
    geo::{Bounds, Point, Size},
    showfile::Showfile,
    window::{attribute_editor::AttributeEditorWindowDelegate, Window, WindowKind, WindowView},
};

pub struct Workspace {
    selected_fixtures: Model<Vec<FixtureId>>,

    window: View<WindowView<AttributeEditorWindowDelegate>>,

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
                window: WindowView::build(
                    Window {
                        id: 0,
                        bounds: Bounds {
                            size: Size {
                                width: 10,
                                height: 10,
                            },
                            origin: Point { x: 0, y: 0 },
                        },
                        kind: WindowKind::AttributeEditor,
                    },
                    AttributeEditorWindowDelegate::new(selected_fixtures.clone(), cx),
                    cx,
                ),
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
            .child(self.window.clone())
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
