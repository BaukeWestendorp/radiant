use gpui::*;
use show::{Cue, CueList, Show};
use ui::{ActiveTheme, Button, ButtonKind, Container, ContainerKind, StyledExt};

use super::{FrameDelegate, FrameView, GRID_SIZE};

pub struct CueListEditorFrameDelegate {
    _show: Model<Show>,
    cuelist: Model<CueList>,
    selected_cue: Option<usize>,
}

impl CueListEditorFrameDelegate {
    pub fn new(show: Model<Show>, cuelist: Model<CueList>, _cx: &mut WindowContext) -> Self {
        Self {
            _show: show,
            cuelist,
            selected_cue: None,
        }
    }

    fn render_cue_selector(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        let cues = self.cuelist.read(cx).cues.clone();

        let cues = uniform_list(
            cx.view().clone(),
            "cues",
            cues.len(),
            move |_, visible_range, cx| {
                visible_range
                    .into_iter()
                    .map(|ix| {
                        let cue = cues[ix].clone();
                        let id = ElementId::NamedInteger(cue.label.clone().into(), ix);
                        div()
                            .w_full()
                            .p_px()
                            .border_b_1()
                            .border_color(cx.theme().border_variant)
                            .child(Button::new(ButtonKind::Ghost, cue.label, id).on_click(
                                cx.listener(move |this, _, _cx| {
                                    this.delegate.selected_cue = Some(ix);
                                    log::info!("Selected cue {ix}");
                                }),
                            ))
                    })
                    .collect()
            },
        )
        .with_sizing_behavior(ListSizingBehavior::Infer);

        let add_button = div()
            .w_full()
            .p_px()
            .font_weight(FontWeight::SEMIBOLD)
            .child(
                Button::new(ButtonKind::Ghost, "(+) Add Cue", "add-cue-button").on_click(
                    cx.listener(move |_this, _, _cx| {
                        log::error!("TODO: Add cue");
                    }),
                ),
            );

        Container::new(ContainerKind::Element)
            .inset(px(1.0))
            .min_w(px(GRID_SIZE * 2.0))
            .max_w(px(GRID_SIZE * 2.0))
            .h_full()
            .child(cues)
            .child(add_button)
    }

    fn render_cue_line_editor(
        &mut self,
        cx: &mut ViewContext<FrameView<Self>>,
    ) -> impl IntoElement {
        let cue_lines = self
            .selected_cue(cx)
            .map(|cue| cue.lines.clone())
            .unwrap_or_default();

        let lines = uniform_list(
            cx.view().clone(),
            "cue-lines",
            cue_lines.len(),
            move |_this, visible_range, cx| {
                visible_range
                    .into_iter()
                    .map(|ix| {
                        let line = &cue_lines[ix];
                        div()
                            .w_full()
                            .p_px()
                            .border_b_1()
                            .border_color(cx.theme().border_variant)
                            .h_flex()
                            .child(line.label.clone())
                            .child("GROUP SELECTOR")
                            .child("EFFECT SELECTOR")
                    })
                    .collect()
            },
        )
        .size_full();

        Container::new(ContainerKind::Element)
            .inset(px(1.0))
            .size_full()
            .child(lines)
    }

    fn selected_cue<'a>(&self, cx: &'a AppContext) -> Option<&'a Cue> {
        self.selected_cue
            .and_then(|ix| self.cuelist.read(cx).cues.get(ix))
    }
}

impl FrameDelegate for CueListEditorFrameDelegate {
    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        format!(
            "Cue Editor ({}) [{}]",
            self.cuelist.read(cx).label,
            self.cuelist.read(cx).id
        )
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        Container::new(ContainerKind::Custom {
            bg: ContainerKind::Element.bg(cx),
            border_color: cx.theme().frame_header_border,
        })
        .child(
            div()
                .size_full()
                .h_flex()
                .child(self.render_cue_selector(cx))
                .child(self.render_cue_line_editor(cx)),
        )
        .size_full()
    }
}
