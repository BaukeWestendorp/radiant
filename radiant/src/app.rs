use frames::FrameContainer;
use gpui::*;
use ui::theme::ActiveTheme;

use crate::frame::{EffectGraphEditor, MainFrame};

pub struct RadiantApp {
    frame_container: Entity<FrameContainer<MainFrame>>,
}

impl RadiantApp {
    pub fn new(cx: &mut App) -> Self {
        Self {
            frame_container: cx.new(|cx| {
                let mut container = FrameContainer::new(size(20, 12), px(80.0));
                container.add_frame(
                    MainFrame::EffectGraphEditor(cx.new(|_cx| EffectGraphEditor::new())),
                    bounds(point(0, 0), size(19, 12)),
                    cx,
                );
                container
            }),
        }
    }
}

impl Render for RadiantApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .child(self.frame_container.clone())
    }
}
