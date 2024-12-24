use gpui::*;
use prelude::FluentBuilder;

use show::{Sequence, Show};
use template_table::TemplateTableDelegate;
use ui::{ActiveTheme, Button, ButtonKind, Container, ContainerKind, StyledExt, Table};

use super::{FrameDelegate, FrameView, GRID_SIZE};

pub mod template_table;

pub struct SequenceEditorFrameDelegate {
    show: Model<Show>,
    sequence: Model<Sequence>,
    selected_cue: Option<usize>,
    table: Option<View<Table<TemplateTableDelegate>>>,
}

impl SequenceEditorFrameDelegate {
    pub fn new(show: Model<Show>, sequence: Model<Sequence>) -> Self {
        Self {
            show,
            sequence,
            selected_cue: None,
            table: None,
        }
    }

    fn render_cue_selector(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        let cues = self.sequence.read(cx).cues.clone();

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
                                cx.listener(move |this, _, cx| this.delegate.select_cue(ix, cx)),
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

    fn render_cue_editor(&mut self, _cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        Container::new(ContainerKind::Element)
            .inset(px(1.0))
            .size_full()
            .when_some(self.table.clone(), |this, table| {
                this.size_full().child(table)
            })
    }

    fn select_cue(&mut self, ix: usize, cx: &mut ViewContext<FrameView<Self>>) {
        self.selected_cue = Some(ix);
        log::info!("Selected cue {ix}");
        self.table = Some(cx.new_view(|cx| {
            Table::new(TemplateTableDelegate::new(
                self.sequence.clone(),
                ix,
                self.show.clone(),
                cx,
            ))
        }));
        cx.notify();
    }
}

impl FrameDelegate for SequenceEditorFrameDelegate {
    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        format!(
            "Sequence Editor ({}) [{}]",
            self.sequence.read(cx).label,
            self.sequence.read(cx).id
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
                .child(self.render_cue_editor(cx)),
        )
        .size_full()
    }
}
