use gpui::*;
use show::{Cue, Effect, Sequence, SequenceId, Show, Template};
use ui::{Selector, SelectorEvent, StyledExt, Table, TableDelegate, TextField, TextFieldEvent};

use crate::ui::{AssetSelectorDelegate, EffectGraphSelector, GroupSelector};

struct TemplateRow {
    label_field: View<TextField>,
    group_id_selector: View<GroupSelector>,
    effect_id_selector: View<EffectGraphSelector>,
}

impl TemplateRow {
    fn new<V: 'static>(
        template: &Template,
        sequence_id: SequenceId,
        cue_ix: usize,
        template_ix: usize,
        show: Model<Show>,
        cx: &mut ViewContext<V>,
    ) -> Self {
        let label_field = {
            let label = &template.label;

            let field = cx.new_view(|cx| TextField::new("label", label.into(), cx));

            let show = show.clone();
            cx.subscribe(&field, move |_table, _field, event, cx| match event {
                TextFieldEvent::Change(new_label) => {
                    show.update(cx, |show, cx| {
                        show.assets.sequences.update(cx, |sequences, cx| {
                            let sequence = sequences.get_mut(&sequence_id).unwrap();
                            let cue = &mut sequence.cues[cue_ix].templates[template_ix];
                            cue.label = new_label.to_string();
                            cx.notify();
                        });
                        cx.notify();
                    });
                }
                _ => {}
            })
            .detach();

            field
        };

        let group_id_selector = {
            let group_id = &template.group;

            let selector = Selector::build(
                AssetSelectorDelegate::new(show.read(cx).assets.groups.clone()),
                "group-selector",
                Some(group_id),
                cx,
            );

            let show = show.clone();
            cx.subscribe(&selector, move |_table, _field, event, cx| match event {
                SelectorEvent::Change(new_group) => {
                    if let Some(new_group) = new_group {
                        show.update(cx, |show, cx| {
                            show.assets.sequences.update(cx, |sequences, cx| {
                                let sequence = sequences.get_mut(&sequence_id).unwrap();
                                let cue = &mut sequence.cues[cue_ix].templates[template_ix];
                                cue.group = *new_group;
                                cx.notify();
                            });
                            cx.notify();
                        });
                    } else {
                        unimplemented!("Handle empty group selector");
                    }
                }
            })
            .detach();

            selector
        };

        let effect_id_selector = {
            let Effect::Graph(graph_id) = &template.effect;

            let selector = Selector::build(
                AssetSelectorDelegate::new(show.read(cx).assets.effect_graphs.clone()),
                "effect-selector",
                Some(graph_id),
                cx,
            );

            let show = show.clone();
            cx.subscribe(&selector, move |_table, _field, event, cx| match event {
                SelectorEvent::Change(new_graph_id) => {
                    if let Some(new_graph_id) = new_graph_id {
                        show.update(cx, |show, cx| {
                            show.assets.sequences.update(cx, |sequences, cx| {
                                let sequence = sequences.get_mut(&sequence_id).unwrap();
                                let cue = &mut sequence.cues[cue_ix].templates[template_ix];
                                cue.effect = Effect::Graph(*new_graph_id);
                                cx.notify();
                            });
                            cx.notify();
                        });
                    } else {
                        todo!("Handle empty effect selector");
                    }
                }
            })
            .detach();

            selector
        };

        Self {
            label_field,
            group_id_selector,
            effect_id_selector,
        }
    }
}

pub struct TemplateTableDelegate {
    sequence: Model<Sequence>,
    cue_ix: usize,
    row_views: Vec<TemplateRow>,
}

impl TemplateTableDelegate {
    const COLUMN_LABELS: [&'static str; 3] = ["Label", "Group", "Effect"];

    pub fn new(
        sequence: Model<Sequence>,
        cue_ix: usize,
        show: Model<Show>,
        cx: &mut ViewContext<Table<Self>>,
    ) -> Self {
        let cue = &sequence
            .read(cx)
            .cues
            .get(cue_ix)
            .expect("A TemplateTable should always be given a valid cue index");

        let row_views = cue
            .templates
            .clone()
            .iter()
            .enumerate()
            .map(|(ix, template)| {
                TemplateRow::new(template, sequence.read(cx).id, cue_ix, ix, show.clone(), cx)
            })
            .collect();

        Self {
            sequence,
            cue_ix,
            row_views,
        }
    }

    fn cue<'a>(&self, cx: &'a AppContext) -> &'a Cue {
        &self
            .sequence
            .read(cx)
            .cues
            .get(self.cue_ix)
            .expect("A TemplateTable should always be given a valid cue index")
    }
}

impl TableDelegate for TemplateTableDelegate {
    fn column_count(&self) -> usize {
        Self::COLUMN_LABELS.len()
    }

    fn row_count(&self, cx: &AppContext) -> usize {
        self.cue(cx).templates.len()
    }

    fn column_label(&self, col_ix: usize, _cx: &mut ViewContext<Table<Self>>) -> SharedString {
        SharedString::from(Self::COLUMN_LABELS[col_ix])
    }

    fn col_width(&self, col_ix: usize) -> Pixels {
        match col_ix {
            0 => px(160.0),
            1 => px(80.0),
            2 => px(100.0),
            _ => unreachable!(),
        }
    }

    fn render_cell(
        &self,
        row_ix: usize,
        col_ix: usize,
        _cx: &ViewContext<Table<Self>>,
    ) -> impl IntoElement {
        let row_view = &self
            .row_views
            .get(row_ix)
            .expect("Only valid row indicies should be rendered");

        let content = match col_ix {
            0 => row_view.label_field.clone().into_any_element(),
            1 => row_view.group_id_selector.clone().into_any_element(),
            2 => row_view.effect_id_selector.clone().into_any_element(),
            _ => unreachable!(),
        };

        div().h_flex().px_1().w_full().child(content)
    }
}
