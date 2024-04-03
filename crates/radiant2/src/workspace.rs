use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, AppContext, Context, FocusHandle, FocusableView, IntoElement, Model, ParentElement,
    Pixels, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::button::Button;
use ui::selectable::Selectable;

use crate::layout::Layout;
use crate::showfile::Showfile;

pub const GRID_SIZE: Pixels = px(80.0);

pub struct Workspace {
    focus_handle: FocusHandle,
    screen: View<Screen>,
}

impl Workspace {
    pub fn build(showfile: Model<Showfile>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let layouts = cx.new_model(|cx| showfile.read(cx).layouts.clone());
            cx.observe(&showfile, {
                let layouts = layouts.clone();
                move |_workspace, showfile, cx| {
                    layouts.update(cx, |layouts, cx| {
                        *layouts = showfile.read(cx).layouts.clone()
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
    layouts: Model<Vec<Layout>>,
    selected_layout_id: usize,

    current_layout_view: View<LayoutView>,
}

impl Screen {
    pub fn build(layouts: Model<Vec<Layout>>, cx: &mut WindowContext) -> View<Self> {
        // FIXME: Store this in `layout.json`.
        let selected_layout_id = 3;

        let current_layout_model = cx.new_model(|cx| {
            // FIXME: Handle nonexistent layout (this should not be possible, but lets
            // softerror on this to be sure).
            layouts
                .read(cx)
                .iter()
                .find(|layout| layout.id == selected_layout_id)
                .unwrap()
                .clone()
        });
        let current_layout_view = LayoutView::build(current_layout_model, cx);

        cx.new_view(|_cx| Self {
            layouts,
            selected_layout_id,
            current_layout_view,
        })
    }

    fn render_sidebar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let layouts = self.layouts.read(cx).clone();

        // FIXME: For now we are showing 10 layouts, but this should be a
        // inplace-scrollable, just like the pool windows.
        let items = (1..=10).map(|id| {
            let layout = layouts.iter().find(|layout| layout.id == id);
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
        let border_color = match self.selected_layout_id == id {
            true => cx.theme().colors().border_selected,
            false => match layout.is_some() {
                true => cx.theme().colors().border,
                false => cx.theme().colors().border_disabled,
            },
        };

        Button::new(id, cx)
            .selected(self.selected_layout_id == id)
            .border_color(border_color)
            .w_full()
            .h(GRID_SIZE)
            .text_sm()
            .on_click(cx.listener({
                let layout = layout.cloned();
                move |screen, _event, cx| {
                    if let Some(layout) = &layout {
                        screen.selected_layout_id = layout.id;
                        cx.notify();
                    }
                }
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .h_full()
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .justify_center()
                            .items_center()
                            .border_b()
                            .border_color(border_color)
                            .children(layout.map(|l| l.label.clone())),
                    )
                    .child(
                        div()
                            .h_5()
                            .px_1()
                            .when(layout.is_none(), |this| {
                                this.text_color(cx.theme().colors().text_muted)
                            })
                            .child(id.to_string()),
                    ),
            )
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

pub struct LayoutView {
    layout: Model<Layout>,
}

impl LayoutView {
    pub fn build(layout: Model<Layout>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { layout })
    }

    fn render_grid(&self, cx: &mut WindowContext) -> impl IntoElement {
        let grid_size = self.layout.read(cx).size;
        let dot_size = 2.0;

        let mut dots = vec![];
        for x in 0..grid_size.width + 1 {
            for y in 0..grid_size.height + 1 {
                let dot = div()
                    .absolute()
                    .size(px(dot_size))
                    .bg(cx.theme().colors().accent)
                    .top(y as f32 * GRID_SIZE - px(dot_size / 2.0))
                    .left(x as f32 * GRID_SIZE - px(dot_size / 2.0));
                dots.push(dot);
            }
        }

        div()
            .w(grid_size.width as f32 * GRID_SIZE)
            .h(grid_size.height as f32 * GRID_SIZE)
            .children(dots)
    }
}

impl Render for LayoutView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .relative()
            .child(div().absolute().inset_0().child(self.render_grid(cx)))
            .child(div().child("HELLO I AM A LAYOUT WOOOHOOO"))
    }
}
