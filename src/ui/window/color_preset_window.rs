use gpui::{
    div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Rgba, Styled, ViewContext,
};
use serde::{Deserialize, Serialize};

use crate::{show::Show, Select};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPresetWindow {}

impl ColorPresetWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for ColorPresetWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let color_presets = cx.global::<Show>().presets().color_presets();

        div()
            .flex()
            .flex_none()
            .gap_px()
            .children(
                color_presets
                    .map(|(_id, color_preset)| {
                        div()
                            .w_20()
                            .h_20()
                            .rounded_md()
                            .bg::<Rgba>(color_preset.color.clone().into())
                            .on_mouse_down(gpui::MouseButton::Left, |_event, cx| {
                                cx.dispatch_action(Box::new(Select))
                            })
                    })
                    .collect::<Vec<_>>(),
            )
            .size_full()
            .bg(rgb(0x303030))
    }
}
