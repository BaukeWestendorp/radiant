use gpui::*;
use prelude::FluentBuilder;

use show::{Cue, CueLine, CueList, Effect, Show};
use ui::{
    ActiveTheme, Button, ButtonKind, Container, ContainerKind, Selector, SelectorEvent, StyledExt,
    Table, TableDelegate, TextField, TextFieldEvent,
};

use crate::ui::{AssetSelectorDelegate, EffectGraphSelector, GroupSelector};

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
    label_fields: Vec<View<TextField>>,
    group_id_selectors: Vec<View<GroupSelector>>,
    effect_id_selectors: Vec<View<EffectGraphSelector>>,
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
        let label_fields = cue
            .lines
            .clone()
            .into_iter()
            .map(|cueline| Self::build_label_field(&cuelist, cue_index, &cueline, show.clone(), cx))
            .collect();
        let group_id_selectors = cue
            .lines
            .clone()
            .into_iter()
            .map(|cueline| {
                Self::build_group_selector(&cuelist, cue_index, &cueline, show.clone(), cx)
            })
            .collect();
        let effect_id_selectors = cue
            .lines
            .clone()
            .into_iter()
            .map(|cueline| {
                Self::build_effect_selector(&cuelist, cue_index, &cueline, show.clone(), cx)
            })
            .collect();

        Self {
            cuelist,
            cue_index,
            label_fields,
            group_id_selectors,
            effect_id_selectors,
        }
    }

    fn build_label_field(
        cuelist: &CueList,
        cue_index: usize,
        cueline: &CueLine,
        show: Model<Show>,
        cx: &mut ViewContext<Table<Self>>,
    ) -> View<TextField> {
        let label = &cueline.label;

        let field = cx.new_view(|cx| TextField::new("label", label.into(), cx));

        let show = show.clone();
        let cuelist_id = cuelist.id;
        let cueline_ix = cueline.index;
        cx.subscribe(&field, move |_table, _field, event, cx| match event {
            TextFieldEvent::Change(new_label) => {
                show.update(cx, |show, cx| {
                    show.assets.cuelists.update(cx, |cuelists, _cx| {
                        let cuelist = cuelists.get_mut(&cuelist_id).unwrap();
                        let cue = cuelist.cues[cue_index]
                            .line_at_index_mut(cueline_ix)
                            .unwrap();
                        cue.label = new_label.to_string();
                    })
                });
            }
            _ => {}
        })
        .detach();

        field
    }

    fn build_group_selector(
        cuelist: &CueList,
        cue_index: usize,
        cueline: &CueLine,
        show: Model<Show>,
        cx: &mut ViewContext<Table<Self>>,
    ) -> View<GroupSelector> {
        let group_id = &cueline.group;

        let selector = Selector::build(
            AssetSelectorDelegate::new(show.read(cx).assets.groups.clone()),
            "group-selector",
            Some(group_id),
            cx,
        );

        let show = show.clone();
        let cuelist_id = cuelist.id;
        let line_index = cueline.index;
        cx.subscribe(&selector, move |_table, _field, event, cx| match event {
            SelectorEvent::Change(new_group) => {
                if let Some(new_group) = new_group {
                    show.update(cx, |show, cx| {
                        show.assets.cuelists.update(cx, |cuelists, _cx| {
                            let cuelist = cuelists.get_mut(&cuelist_id).unwrap();
                            let cue = cuelist.cues[cue_index]
                                .line_at_index_mut(line_index)
                                .unwrap();
                            cue.group = *new_group;
                        })
                    });
                } else {
                    todo!("Handle empty group selector");
                }
            }
        })
        .detach();

        selector
    }

    fn build_effect_selector(
        cuelist: &CueList,
        cue_index: usize,
        cueline: &CueLine,
        show: Model<Show>,
        cx: &mut ViewContext<Table<Self>>,
    ) -> View<EffectGraphSelector> {
        let Effect::Graph(graph_id) = &cueline.effect;

        let selector = Selector::build(
            AssetSelectorDelegate::new(show.read(cx).assets.effect_graphs.clone()),
            "effect-selector",
            Some(graph_id),
            cx,
        );

        let show = show.clone();
        let cuelist_id = cuelist.id;
        let line_index = cueline.index;
        cx.subscribe(&selector, move |_table, _field, event, cx| match event {
            SelectorEvent::Change(new_graph_id) => {
                if let Some(new_graph_id) = new_graph_id {
                    show.update(cx, |show, cx| {
                        show.assets.cuelists.update(cx, |cuelists, _cx| {
                            let cuelist = cuelists.get_mut(&cuelist_id).unwrap();
                            let cue = cuelist.cues[cue_index]
                                .line_at_index_mut(line_index)
                                .unwrap();
                            cue.effect = Effect::Graph(*new_graph_id);
                        })
                    });
                } else {
                    todo!("Handle empty effect selector");
                }
            }
        })
        .detach();

        selector
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
        let content = match col_ix {
            0 => row_ix.to_string().into_any_element(),
            1 => self
                .label_fields
                .get(row_ix)
                .unwrap()
                .clone()
                .into_any_element(),
            2 => self
                .group_id_selectors
                .get(row_ix)
                .unwrap()
                .clone()
                .into_any_element(),
            3 => self
                .effect_id_selectors
                .get(row_ix)
                .unwrap()
                .clone()
                .into_any_element(),
            _ => unreachable!(),
        };

        div().h_flex().px_1().w_full().child(content)
    }
}
