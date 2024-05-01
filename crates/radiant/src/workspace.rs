use backstage::show::FixtureId;
use gpui::{
    div, prelude::FluentBuilder, AppContext, Context, FocusHandle, FocusableView, Global,
    InteractiveElement, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::{
    layout::{LayoutView, GRID_SIZE},
    showfile::{Layout, Showfile},
    theme::THEME,
    ui::{Button, Selectable},
};

use self::action::ExecuteCommand;

pub mod action {
    use backstage::cmd::Command;
    use gpui::impl_actions;

    impl_actions!(workspace, [ExecuteCommand]);

    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    pub struct ExecuteCommand(pub Command);
}

pub struct Workspace {
    selected_fixtures: Model<Vec<FixtureId>>,
    current_layout: Model<Layout>,

    layout_view: View<LayoutView>,

    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let selected_fixtures =
                cx.new_model(|cx| Showfile::get(cx).show.selected_fixture_ids().to_vec());

            let current_layout = cx.new_model(|cx| {
                Showfile::get(cx)
                    .layouts
                    .current_layout()
                    .expect("Failed to get current layout")
                    .clone()
            });

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
                workspace.current_layout.update(cx, |current_layout, cx| {
                    let Some(updated_current_layout) = Showfile::get(cx).layouts.current_layout()
                    else {
                        return;
                    };

                    if updated_current_layout.id != current_layout.id {
                        *current_layout = updated_current_layout.clone();
                        cx.notify();
                    }
                });
            })
            .detach();

            cx.observe_global::<Showfile>({
                move |workspace: &mut Self, cx| {
                    let current_layout = workspace.layout_view.read(cx).layout.clone();

                    let updated_layout = Showfile::get(cx).layouts.current_layout();
                    if Some(current_layout.read(cx)) != updated_layout {
                        workspace.layout_view = LayoutView::build(
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
                current_layout: current_layout.clone(),

                layout_view: LayoutView::build(current_layout, selected_fixtures, cx),

                focus_handle: cx.focus_handle(),
            }
        })
    }

    fn render_sidebar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let layouts = Showfile::get(cx).layouts.clone();

        // FIXME: For now we are showing 10 layouts, but this should be a
        // inplace-scrollable, just like the pool windows.
        let items = (0..10).map(|id| {
            let layout = layouts.layouts.iter().find(|layout| layout.id == id);
            self.render_layout_item(layout, id, cx).into_any_element()
        });

        div()
            .w(GRID_SIZE * 1.5)
            .border_l()
            .border_color(THEME.border)
            .bg(THEME.fill)
            .children(items)
    }

    fn render_layout_item(
        &self,
        layout: Option<&Layout>,
        id: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let is_selected = Showfile::get(cx).layouts.selected_layout_id == id;

        let border_color = match is_selected {
            true => THEME.border_selected,
            false => match layout.is_some() {
                true => THEME.border,
                false => THEME.border_secondary,
            },
        };

        let display = div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .border_b()
            .border_color(border_color)
            .children(layout.map(|l| l.label.clone()));

        let id_element = div()
            .h_5()
            .px_1()
            .when(layout.is_none(), |this| {
                this.text_color(THEME.text_secondary)
            })
            .child(id.to_string());

        let content = div()
            .flex()
            .flex_col()
            .h_full()
            .child(display)
            .child(id_element);

        Button::new(id)
            .selected(is_selected)
            .border_color(border_color)
            .bg(THEME.fill)
            .w_full()
            .h(GRID_SIZE)
            .text_sm()
            .on_click(cx.listener({
                let layout = layout.cloned();
                move |_screen, _event, cx| {
                    if let Some(layout) = &layout {
                        Showfile::update(cx, |showfile, _cx| {
                            showfile.layouts.selected_layout_id = layout.id;
                        });
                        cx.notify();
                    }
                }
            }))
            .child(content)
    }

    fn handle_execute_command(&mut self, command: &ExecuteCommand, cx: &mut ViewContext<Self>) {
        Showfile::update(cx, |showfile, _cx| {
            showfile.show.execute_command(&command.0)
        })
        .map_err(|err| log::error!("Failed to execute command: {err}"))
        .ok();
        cx.notify();
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("Workspace")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::handle_execute_command))
            .size_full()
            .text_color(THEME.text)
            .bg(THEME.background)
            .flex()
            .child(self.layout_view.clone())
            .child(self.render_sidebar(cx))
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
