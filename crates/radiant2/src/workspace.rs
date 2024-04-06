use gpui::prelude::FluentBuilder;
use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, IntoElement, Model, ParentElement,
    Render, Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::button::{Button, ButtonStyle};
use ui::selectable::Selectable;

use crate::layout::{LayoutView, GRID_SIZE};
use crate::showfile::{Layout, Layouts, ShowfileManager};

pub struct Workspace {
    focus_handle: FocusHandle,
    screen: View<Screen>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let layouts = cx.new_model(|cx| ShowfileManager::layouts(cx).clone());
            cx.observe_global::<ShowfileManager>({
                let layouts = layouts.clone();
                move |_workspace, cx| {
                    layouts.update(cx, |layouts, cx| {
                        *layouts = ShowfileManager::layouts(cx).clone()
                    });
                    cx.notify();
                }
            })
            .detach();

            Self {
                focus_handle: cx.focus_handle(),
                screen: Screen::build(layouts, cx),
            }
        })
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
            .size_full()
            .text_color(cx.theme().colors().text)
            .font("Zed Sans")
            .child(self.screen.clone())
    }
}

pub struct Screen {
    layouts: Model<Layouts>,

    current_layout_view: View<LayoutView>,
}

impl Screen {
    pub fn build(layouts: Model<Layouts>, cx: &mut WindowContext) -> View<Self> {
        let current_layout_view = get_current_layout_view(layouts.clone(), cx);

        cx.new_view(|cx| {
            cx.observe(&layouts, |this: &mut Self, layouts, cx| {
                this.current_layout_view = get_current_layout_view(layouts, cx);
            })
            .detach();

            Self {
                layouts,
                current_layout_view,
            }
        })
    }

    fn render_sidebar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let layouts = self.layouts.read(cx).clone();

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
        let border_color = match self.layouts.read(cx).selected_layout_id == id {
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
            .selected(self.layouts.read(cx).selected_layout_id == id)
            .border_color(border_color)
            .w_full()
            .h(GRID_SIZE)
            .text_sm()
            .on_click(cx.listener({
                let layout = layout.cloned();
                move |screen, _event, cx| {
                    if let Some(layout) = &layout {
                        screen.layouts.update(cx, |layouts, cx| {
                            layouts.selected_layout_id = layout.id;
                            cx.notify();
                        });
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

fn get_current_layout_view(layouts: Model<Layouts>, cx: &mut WindowContext) -> View<LayoutView> {
    let current_layout_model = cx.new_model(|cx| {
        // FIXME: Handle nonexistent layout (this should not be possible, but lets
        // softerror on this to be sure).
        layouts
            .read(cx)
            .layouts
            .iter()
            .find(|layout| layout.id == layouts.read(cx).selected_layout_id)
            .unwrap()
            .clone()
    });

    LayoutView::build(current_layout_model, cx)
}
