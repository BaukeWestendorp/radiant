use gpui::{div, rgb, IntoElement, ParentElement, Styled, VisualContext, WindowContext};

pub mod color_preset_window;

pub use color_preset_window::ColorPresetWindow;

#[derive(Clone)]
pub struct Window {
    pub kind: WindowKind,
}

impl Window {
    pub fn new(kind: WindowKind) -> Self {
        Self { kind }
    }

    pub fn render_header(&self) -> impl IntoElement {
        div()
            .child(self.kind.window_title().to_string())
            .flex()
            .items_center()
            .w_full()
            .px_3()
            .h_10()
            .bg(rgb(0x222280))
            .border_color(rgb(0x0000ff))
            .border_1()
            .rounded_md()
    }

    pub fn get_view(&self, cx: &mut WindowContext) -> impl IntoElement {
        let header = self.render_header();
        let content = match &self.kind {
            WindowKind::ColorPreset(window) => div().child(cx.new_view(|_| window.clone())),
        };

        div()
            .child(header)
            .child(content.size_full())
            .flex()
            .flex_col()
            .size_full()
    }
}

#[derive(Clone)]
pub enum WindowKind {
    ColorPreset(ColorPresetWindow),
}

impl WindowKind {
    pub fn window_title(&self) -> &str {
        match self {
            WindowKind::ColorPreset(_) => "Color Preset",
        }
    }
}
