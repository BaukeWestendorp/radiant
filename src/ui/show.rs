use gpui::{
    div, white, Context, FocusHandle, FocusableView, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::show::{self, Screen, Show};

use super::screen::ScreenView;

pub struct ShowView {
    show: Model<Show>,
    screens: Vec<Model<Screen>>,
    focus_handle: FocusHandle,
}

impl FocusableView for ShowView {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ShowView {
    pub fn build(show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let screens = show
                .read(cx)
                .screens
                .clone()
                .iter()
                .map(|screen| cx.new_model(|_cx| screen.clone()))
                .collect();

            cx.observe(&show, move |this: &mut Self, show, cx| {
                this.screens = show
                    .read(cx)
                    .screens
                    .clone()
                    .iter()
                    .map(|screen| cx.new_model(|_cx| screen.clone()))
                    .collect();
                cx.notify();
                println!("show updated")
            })
            .detach();

            let focus_handle = cx.focus_handle();

            let this = Self {
                show,
                screens,
                focus_handle,
            };

            this
        })
    }

    fn cmd_store(&mut self, _action: &show::cmd::Store, cx: &mut ViewContext<Self>) {
        println!("Store");
        self.show.update(cx, |show, cx| {
            show.screens.push(show::Screen {
                layout: show::Layout {
                    windows: vec![show::Window {}],
                },
            });
            cx.notify();
        })
    }
}

impl Render for ShowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .border()
            .border_color(gpui::red())
            .size_full()
            .track_focus(&self.focus_handle)
            .key_context("Show")
            .on_action(cx.listener(Self::cmd_store))
            .font("Zed Sans")
            .text_color(white())
            .children({
                self.screens
                    .iter()
                    .cloned()
                    .map(|screen| ScreenView::build(screen.clone(), cx))
            })
    }
}
