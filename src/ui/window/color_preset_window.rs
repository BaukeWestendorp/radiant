use gpui::{div, IntoElement, ParentElement, Render, ViewContext};

#[derive(Clone)]
pub struct ColorPresetWindow {}

impl ColorPresetWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for ColorPresetWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        // let color_presets = self
        //     .show
        //     .upgrade()
        //     .unwrap()
        //     .read(cx)
        //     .presets
        //     .color_presets();

        // div()
        //     .flex()
        //     .flex_none()
        //     .gap_px()
        //     .children(
        //         color_presets
        //             .map(|(_id, color_preset)| {
        //                 div()
        //                     .w_20()
        //                     .h_20()
        //                     .rounded_md()
        //                     .bg::<Rgba>(color_preset.color.clone().into())
        //                     .on_mouse_down(gpui::MouseButton::Left, |_event, cx| {
        //                         cx.dispatch_action(Box::new(show::cmd::Select))
        //                     })
        //             })
        //             .collect::<Vec<_>>(),
        //     )
        //     .size_full()
        //     .bg(rgb(0x303030))

        div().child("COLOR PRESETS")
    }
}
