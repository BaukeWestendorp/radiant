use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, IntoElement, Model, ParentElement,
    Render, Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;

use crate::layout::Layout;
use crate::showfile::Showfile;

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
}

impl Screen {
    pub fn build(layouts: Model<Vec<Layout>>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { layouts })
    }
}

impl Render for Screen {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div().size_full().bg(gpui::red());
        let sidebar = div().w_20().h_full().bg(gpui::green());

        div().size_full().flex().child(content).child(sidebar)
    }
}
