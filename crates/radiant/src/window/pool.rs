use backstage::cmd::{Command, Object};
use gpui::{
    div, prelude::FluentBuilder, Global, InteractiveElement, IntoElement, MouseButton,
    ParentElement, SharedString, Styled, ViewContext, WindowContext,
};

use crate::{
    layout::GRID_SIZE,
    showfile::{Showfile, Window},
};

use super::{WindowDelegate, WindowView};

pub trait PoolDelegate {
    fn title(&self, cx: &mut WindowContext) -> SharedString;

    fn has_content(&self, id: usize, cx: &mut WindowContext) -> bool;

    fn render_pool_item(&mut self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement>;
    fn render_header_item(
        &mut self,
        _id: usize,
        _cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }

    fn on_click_item(&mut self, id: usize, cx: &mut WindowContext);
}

pub struct PoolWindowDelegate<D: PoolDelegate> {
    pub pool_delegate: D,
    window: Window,
}

impl<D: PoolDelegate> PoolWindowDelegate<D> {
    pub fn new(pool_delegate: D, window: Window) -> Self {
        Self {
            pool_delegate,
            window,
        }
    }
}

impl<D: PoolDelegate + 'static> WindowDelegate for PoolWindowDelegate<D> {
    fn title(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some(self.pool_delegate.title(cx))
    }

    fn render_content(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        let line_height = cx.line_height() * 0.5;

        let header_cell = div()
            .size(GRID_SIZE)
            .border()
            .border_color(gpui::blue())
            .bg(gpui::rgb(0x000088))
            .rounded_md()
            .flex()
            .justify_center()
            .items_center()
            .child(self.pool_delegate.title(cx));

        let items = (0..self.window.bounds.area()).map(|id| {
            let has_content = self.pool_delegate.has_content(id, cx);

            let border_color = match has_content {
                true => gpui::white().into(),
                false => gpui::rgb(0x999999),
            };

            let item_header = div()
                .border_b()
                .border_color(border_color)
                .flex()
                .items_center()
                .justify_between()
                .p_1()
                .child(
                    div()
                        .when(!has_content, |this| this.text_color(gpui::rgb(0xaaaaaa)))
                        .child(id.to_string()),
                )
                .children(self.pool_delegate.render_header_item(id, cx));

            div()
                .size(GRID_SIZE)
                .relative()
                .text_sm()
                .child(
                    div()
                        .absolute()
                        .size_full()
                        .border()
                        .border_color(border_color)
                        .rounded_md(),
                )
                .child(
                    div()
                        .absolute()
                        .size_full()
                        .child(item_header)
                        .children(self.pool_delegate.render_pool_item(id, cx)),
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _event, cx| {
                        this.delegate.pool_delegate.on_click_item(id, cx)
                    }),
                )
        });

        div()
            .w(self.window.bounds.size.width as f32 * GRID_SIZE)
            .h(self.window.bounds.size.height as f32 * GRID_SIZE)
            .overflow_hidden()
            .line_height(line_height)
            .flex()
            .flex_wrap()
            .child(header_cell)
            .children(items)
    }

    fn render_header(
        &mut self,
        _cx: &mut ViewContext<WindowView<Self>>,
    ) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }
}

pub struct GroupPoolWindowDelegate {}

impl GroupPoolWindowDelegate {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for GroupPoolWindowDelegate {
    fn title(&self, _cx: &mut WindowContext) -> SharedString {
        "Group".into()
    }

    fn has_content(&self, id: usize, cx: &mut WindowContext) -> bool {
        Showfile::get(cx).show.data().group(id).is_some()
    }

    fn render_pool_item(&mut self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let Some(group) = Showfile::get(cx).show.data().group(id) else {
            return None;
        };

        Some(
            div()
                .size_full()
                .p_1()
                .child(div().my_auto().child(group.label.clone()))
                .overflow_hidden(),
        )
    }

    fn render_header_item(
        &mut self,
        id: usize,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        let Some(group) = Showfile::get(cx).show.data().group(id) else {
            return None;
        };

        Some(
            div()
                .text_color(gpui::rgb(0xaaaaaa))
                .child(format!("{} fxt.", group.fixtures.len())),
        )
    }

    fn on_click_item(&mut self, id: usize, cx: &mut WindowContext) {
        Showfile::update(cx, |showfile, _cx| {
            showfile
                .show
                .execute_command(&Command::Select(Some(Object::Group(Some(id)))))
                .map_err(|err| {
                    log::error!("Failed to execute command when clicking on group pool item: {err}")
                })
                .ok();
        });
    }
}

pub struct PresetPoolWindowDelegate {}

impl PresetPoolWindowDelegate {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for PresetPoolWindowDelegate {
    fn title(&self, _cx: &mut WindowContext) -> SharedString {
        "Preset".into()
    }

    fn has_content(&self, id: usize, cx: &mut WindowContext) -> bool {
        Showfile::get(cx).show.data().preset(id).is_some()
    }

    fn render_pool_item(&mut self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let Some(preset) = Showfile::get(cx).show.data().preset(id) else {
            return None;
        };

        Some(
            div()
                .size_full()
                .p_1()
                .child(div().my_auto().child(preset.label.clone()))
                .overflow_hidden(),
        )
    }

    fn on_click_item(&mut self, id: usize, cx: &mut WindowContext) {
        Showfile::update(cx, |showfile, _cx| {
            showfile
                .show
                .execute_command(&Command::Select(Some(Object::Preset(Some(id)))))
                .map_err(|err| {
                    log::error!(
                        "Failed to execute command when clicking on preset pool item: {err}"
                    )
                })
                .ok();
        });
    }
}
