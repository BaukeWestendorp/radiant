use gpui::*;
use show::{Cue, Show};
use ui::{container, theme::ActiveTheme, ContainerKind};

use super::{FrameDelegate, FrameView};

pub struct CueEditorFrameDelegate {
    show: Model<Show>,
    cue: Model<Cue>,
}

impl CueEditorFrameDelegate {
    pub fn new(show: Model<Show>, cue: Model<Cue>, cx: &mut WindowContext) -> Self {
        Self { show, cue }
    }
}

impl FrameDelegate for CueEditorFrameDelegate {
    fn init(&mut self, _cx: &mut ViewContext<FrameView<Self>>) {}

    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        format!(
            "Cue Editor ({}) [{}]",
            self.cue.read(cx).label,
            self.cue.read(cx).id
        )
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        container(ContainerKind::Element, cx)
            .size_full()
            .border_color(cx.theme().frame_header_border)
            .child("editor")
    }
}
