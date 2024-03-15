use gpui::{
    div, rgb, AnyView, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::show::layout::WindowKind;
use crate::show::{self, Show};
use crate::ui::grid_div;

use self::color_picker::ColorPickerWindow;
use self::pool_window::PoolWindow;

pub mod color_picker;
pub mod fixture_sheet;
pub mod pool_item;
pub mod pool_window;

pub struct Window {
    window_id: usize,
    content: AnyView,
    show: Model<Show>,
}

impl Window {
    pub fn build(window_id: usize, show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            window_id,
            content: render_content(window_id, show.clone(), cx),
            show,
        })
    }

    fn render_header(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let show_window = show_window(&self.show, self.window_id, cx);

        if !show_window.kind.show_header() {
            return None;
        }

        let window_title = show_window.kind.window_title().to_string();

        let header = div()
            .flex()
            .items_center()
            .px_3()
            .h_10()
            .bg(rgb(0x222280))
            .border_color(rgb(0x0000ff))
            .border_1()
            .rounded_t_md()
            .child(window_title);

        Some(header)
    }
}

fn render_content(window_id: usize, show: Model<Show>, cx: &mut ViewContext<Window>) -> AnyView {
    let show_window = show.read(cx).layout.window(window_id).unwrap();

    match show_window.kind {
        WindowKind::Pool(_) => PoolWindow::build(window_id, show.clone(), cx).into(),
        WindowKind::ColorPicker => ColorPickerWindow::build(cx).into(),
        WindowKind::FixtureSheet => {
            fixture_sheet::FixtureSheetWindow::build(show.clone(), cx).into()
        }
    }
}

impl Render for Window {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let show_window = show_window(&self.show, self.window_id, cx);

        let content = div()
            .bg(rgb(0x202020))
            .size_full()
            .rounded_b_md()
            .child(self.content.clone());

        grid_div(show_window.bounds.size, Some(show_window.bounds.origin))
            .flex()
            .flex_col()
            .children(self.render_header(cx))
            .child(content)
    }
}

pub fn show_window<'a>(
    show: &Model<Show>,
    window_id: usize,
    cx: &'a mut WindowContext,
) -> &'a show::Window {
    match show.read(cx).layout.window(window_id) {
        Some(window) => window,
        None => {
            log::error!(
                "Failed to get window with id '{}'. Window not found",
                window_id
            );
            panic!()
        }
    }
}

pub fn show_pool_window<'a>(
    show: &Model<Show>,
    window_id: usize,
    cx: &'a mut WindowContext,
) -> &'a show::PoolWindow {
    match show.read(cx).layout.pool_window(window_id) {
        Some(pool_window) => pool_window,
        None => {
            log::error!(
                "Failed to get pool window with id '{}'. Pool window not found",
                window_id
            );
            panic!()
        }
    }
}
