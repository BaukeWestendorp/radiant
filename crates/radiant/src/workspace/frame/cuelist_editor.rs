use gpui::*;
use show::{CueList, Show};
use ui::{container, theme::ActiveTheme, ContainerKind};

use super::{FrameDelegate, FrameView};

pub struct CueListEditorFrameDelegate {
    show: Model<Show>,
    cuelist: Model<CueList>,
}

impl CueListEditorFrameDelegate {
    pub fn new(show: Model<Show>, cuelist: Model<CueList>, cx: &mut WindowContext) -> Self {
        Self { show, cuelist }
    }
}

impl FrameDelegate for CueListEditorFrameDelegate {
    fn init(&mut self, _cx: &mut ViewContext<FrameView<Self>>) {}

    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        format!(
            "Cue Editor ({}) [{}]",
            self.cuelist.read(cx).label,
            self.cuelist.read(cx).id
        )
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        container(ContainerKind::Element, cx)
            .size_full()
            .border_color(cx.theme().frame_header_border)
            .child("editor")
    }
}
