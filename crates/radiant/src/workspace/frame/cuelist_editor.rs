use gpui::*;
use prelude::FluentBuilder;
use show::{Cue, CueList, Show};
use ui::{
    ActiveTheme, Button, ButtonKind, Container, ContainerKind, StyledExt, Table, TableDelegate,
};

use crate::ui::{group_selector::GroupSelector, AssetSelectorEvent};

use super::{FrameDelegate, FrameView, GRID_SIZE};

pub struct CueListEditorFrameDelegate {
    show: Model<Show>,
    cuelist: Model<CueList>,
    selected_cue: Option<usize>,
    table: Option<View<Table<CueTableDelegate>>>,
}

impl CueListEditorFrameDelegate {
    pub fn new(show: Model<Show>, cuelist: Model<CueList>, _cx: &mut WindowContext) -> Self {
        Self {
            show,
            cuelist,
            selected_cue: None,
            table: None,
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
            Table::new(CueTableDelegate::new(
                self.cuelist.read(cx).clone(),
                ix,
                self.show.clone(),
                cx,
            ))
        }));
        cx.notify();
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
                .child(self.render_cue_editor(cx)),
        )
        .size_full()
    }
}

pub struct CueTableDelegate {
    cuelist: CueList,
    cue_index: usize,
    group_id_selectors: Vec<View<GroupSelector>>,
}

impl CueTableDelegate {
    const COLUMN_LABELS: [&'static str; 4] = ["Id", "Label", "Group", "Effect"];

    pub fn new(
        cuelist: CueList,
        cue_index: usize,
        show: Model<Show>,
        cx: &mut ViewContext<Table<Self>>,
    ) -> Self {
        let cue = &cuelist.cues[cue_index];
        let group_id_selectors = cue
            .lines
            .clone()
            .into_iter()
            .enumerate()
            .map(move |(line_ix, _line)| {
                let selector = cx.new_view(|cx| {
                    GroupSelector::new("group-selector", show.read(cx).assets.groups.clone())
                });

                let show = show.clone();
                cx.subscribe(&selector, move |_table, _field, event, cx| match event {
                    AssetSelectorEvent::Change(new_group) => {
                        show.update(cx, |show, cx| {
                            show.assets.cuelists.update(cx, |cuelists, _cx| {
                                cuelists.get_mut(&cuelist.id).unwrap().cues[cue_index].lines
                                    [line_ix]
                                    .group = *new_group;
                                log::debug!("Updated cueline");
                            })
                        });
                    }
                })
                .detach();

                selector
            })
            .collect();

        Self {
            cuelist,
            cue_index,
            group_id_selectors,
        }
    }

    fn cue(&self) -> &Cue {
        &self.cuelist.cues[self.cue_index]
    }
}

impl TableDelegate for CueTableDelegate {
    fn column_count(&self) -> usize {
        Self::COLUMN_LABELS.len()
    }

    fn row_count(&self) -> usize {
        self.cue().lines.len()
    }

    fn column_label(&self, col_ix: usize, _cx: &mut ViewContext<Table<Self>>) -> SharedString {
        SharedString::from(Self::COLUMN_LABELS[col_ix])
    }

    fn col_width(&self, col_ix: usize) -> Pixels {
        match col_ix {
            0 => px(50.0),
            1 => px(160.0),
            2 => px(80.0),
            3 => px(100.0),
            _ => unreachable!(),
        }
    }

    fn render_cell(
        &self,
        row_ix: usize,
        col_ix: usize,
        _cx: &ViewContext<Table<Self>>,
    ) -> impl IntoElement {
        let line = &self.cue().lines[row_ix];

        let content = match col_ix {
            0 => row_ix.to_string().into_any_element(),
            1 => line.label.clone().into_any_element(),
            2 => self
                .group_id_selectors
                .get(row_ix)
                .unwrap()
                .clone()
                .into_any_element(),
            3 => format!("{:?}", line.effect).into_any_element(),
            _ => unreachable!(),
        };

        div().px_1().w_full().child(content)
    }
}
