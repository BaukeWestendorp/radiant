use gpui::prelude::FluentBuilder;
use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::button::{Button, ButtonStyle};
use ui::selectable::Selectable;

use crate::app::ExecuteCommand;
use crate::layout::{LayoutView, GRID_SIZE};
use crate::showfile::{Layout, ShowfileManager};

pub struct Workspace {
    focus_handle: FocusHandle,
    screen: View<Screen>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            focus_handle: cx.focus_handle(),
            screen: Screen::build(cx),
        })
    }

    fn handle_cmd(&mut self, cmd: &ExecuteCommand, cx: &mut ViewContext<Self>) {
        if let Err(err) =
            ShowfileManager::update(cx, |showfile, _cx| showfile.show.execute_command(&cmd.0))
        {
            log::error!("Failed to execute command '{}': {err}", cmd.0);
        }
        cx.notify();
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("Workspace")
            .size_full()
            .text_color(cx.theme().colors().text)
            .font("Zed Sans")
            .on_action(cx.listener(Self::handle_cmd))
            .track_focus(&self.focus_handle)
            .child(self.screen.clone())
    }
}

pub struct Screen {
    current_layout_view: View<LayoutView>,
}

impl Screen {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let current_layout_view = get_current_layout_view(cx);

        cx.new_view(|cx| {
            cx.observe_global::<ShowfileManager>({
                move |screen: &mut Screen, cx| {
                    screen.current_layout_view = get_current_layout_view(cx);
                    cx.notify();
                }
            })
            .detach();

            Self {
                current_layout_view,
            }
        })
    }

    fn render_sidebar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let layouts = ShowfileManager::layouts(cx).clone();

        // FIXME: For now we are showing 10 layouts, but this should be a
        // inplace-scrollable, just like the pool windows.
        let items = (1..=10).map(|id| {
            let layout = layouts.layouts.iter().find(|layout| layout.id == id);
            self.render_layout_item(layout, id, cx).into_any_element()
        });

        div().w_32().children(items)
    }

    fn render_layout_item(
        &self,
        layout: Option<&Layout>,
        id: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let border_color = match ShowfileManager::layouts(cx).selected_layout_id == id {
            true => cx.theme().colors().border_selected,
            false => match layout.is_some() {
                true => cx.theme().colors().border,
                false => cx.theme().colors().border_disabled,
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
                this.text_color(cx.theme().colors().text_muted)
            })
            .child(id.to_string());

        let content = div()
            .flex()
            .flex_col()
            .h_full()
            .child(display)
            .child(id_element);

        Button::new(ButtonStyle::Primary, id, cx)
            .selected(ShowfileManager::layouts(cx).selected_layout_id == id)
            .border_color(border_color)
            .w_full()
            .h(GRID_SIZE)
            .text_sm()
            .on_click(cx.listener({
                let layout = layout.cloned();
                move |_screen, _event, cx| {
                    if let Some(layout) = &layout {
                        ShowfileManager::update(cx, |showfile, _cx| {
                            showfile.layouts.selected_layout_id = layout.id;
                        });
                        cx.notify();
                    }
                }
            }))
            .child(content)
    }
}

impl Render for Screen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div()
            .size_full()
            .overflow_hidden()
            .child(self.current_layout_view.clone());

        div()
            .size_full()
            .flex()
            .child(content)
            .child(self.render_sidebar(cx))
    }
}

fn get_current_layout_view(cx: &mut WindowContext) -> View<LayoutView> {
    let current_layout_model = cx.new_model(|cx| {
        // FIXME: Handle nonexistent layout (this should not be possible, but lets
        // softerror on this to be sure).
        ShowfileManager::layouts(cx)
            .layouts
            .iter()
            .find(|layout| layout.id == ShowfileManager::layouts(cx).selected_layout_id)
            .unwrap()
            .clone()
    });

    LayoutView::build(current_layout_model, cx)
}
