use std::collections::HashMap;

use gpui::{App, Entity, Window, div, prelude::*};

use rd_ui::{Column, Table, TableDelegate, TableState};
use uuid::Uuid;

pub struct MidiTabView {
    table: Entity<TableState<MidiMappingTable>>,
    mappings: Entity<HashMap<Uuid, rd::project::MidiMapping>>,
}

impl MidiTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mappings = cx.new(|cx| {
            uncommitted_project
                .read(cx)
                .trigger
                .midi
                .iter()
                .map(|mapping| (Uuid::new_v4(), mapping.clone()))
                .collect::<HashMap<_, _>>()
        });

        cx.observe(&uncommitted_project, move |this, uncommitted_project, cx| {
            let new_mappings = uncommitted_project
                .read(cx)
                .trigger
                .midi
                .iter()
                .map(|mapping| (Uuid::new_v4(), mapping.clone()))
                .collect::<HashMap<_, _>>();

            this.mappings.write(cx, new_mappings);
        })
        .detach();

        cx.observe(&mappings, {
            let uncommitted_project = uncommitted_project.clone();
            move |_, mappings, cx| {
                let new_mappings = mappings.read(cx).values().cloned().collect();

                if new_mappings == uncommitted_project.read(cx).trigger.midi {
                    return;
                }

                uncommitted_project.update(cx, |project, _| {
                    project.trigger.midi = new_mappings;
                });
            }
        })
        .detach();

        let table = cx.new(|cx| {
            TableState::new(
                MidiMappingTable::new(mappings.clone(), window, cx),
                cx.focus_handle(),
                window,
                cx,
            )
        });

        Self { table, mappings }
    }
}

impl Render for MidiTabView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(Table::new("midi-triggers", self.table.clone(), window, cx))
    }
}

struct MidiMappingTable {
    columns: Vec<Column<Self>>,
    mappings: Entity<HashMap<Uuid, rd::project::MidiMapping>>,
}

impl MidiMappingTable {
    fn new(
        mappings: Entity<HashMap<Uuid, rd::project::MidiMapping>>,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> Self {
        Self {
            columns: vec![
                Column::<Self>::new("device_name", "Device Name")
                    .with_sort_handler(|a, b| {
                        natord::compare_ignore_case(&a.device_name, &b.device_name)
                    })
                    .with_cell_builder(|row, _window, _cx| {
                        row.device_name.to_string().into_any_element()
                    }),
                Column::<Self>::new("device_channel", "Device Channel")
                    .with_sort_handler(|a, b| a.device_channel.cmp(&b.device_channel))
                    .with_auto_editor(|row| &mut row.device_channel)
                    .with_cell_builder(|row, _window, _cx| {
                        row.device_channel.to_string().into_any_element()
                    }),
                Column::<Self>::new("filter", "Filter")
                    .with_sort_handler(|a, b| a.filter.cmp(&b.filter))
                    .with_auto_editor(|row| &mut row.filter)
                    .with_cell_builder(|row, _window, _cx| {
                        row.filter.to_string().into_any_element()
                    }),
                Column::<Self>::new("target", "Target")
                    .with_sort_handler(|a, b| a.target.cmp(&b.target))
                    .with_auto_editor(|row| &mut row.target)
                    .with_cell_builder(|row, _window, _cx| {
                        row.target.to_string().into_any_element()
                    }),
            ],
            mappings,
        }
    }
}

impl TableDelegate for MidiMappingTable {
    type Row = rd::project::midi::MidiMapping;
    type RowId = Uuid;

    fn columns(&self, _cx: &App) -> impl Iterator<Item = &Column<Self>> {
        self.columns.iter()
    }

    fn column(&self, column_id: &str, _cx: &App) -> Option<&Column<Self>> {
        self.columns.iter().find(|c| c.id() == column_id)
    }

    fn rows(&self) -> Entity<HashMap<Self::RowId, Self::Row>> {
        self.mappings.clone()
    }

    fn insert_new_row(
        &self,
        last_item_id: Option<Self::RowId>,
        cx: &mut App,
    ) -> Option<Self::RowId> {
        let last_item = last_item_id.and_then(|id| self.rows().read(cx).get(&id));
        let new_mapping = match last_item {
            Some(last_mapping) => rd::project::midi::MidiMapping {
                device_name: last_mapping.device_name.clone(),
                device_channel: rd::project::MidiChannel::default(),
                filter: rd::project::MidiFilter::default(),
                target: rd::project::TriggerTarget::default(),
            },
            None => rd::project::midi::MidiMapping {
                device_name: "FIXME".to_string(),
                device_channel: rd::project::MidiChannel::default(),
                filter: rd::project::MidiFilter::default(),
                target: rd::project::TriggerTarget::default(),
            },
        };
        let new_id = Uuid::new_v4();
        self.mappings.update(cx, |mappings, cx| {
            mappings.insert(new_id, new_mapping);
            cx.notify();
        });
        Some(new_id)
    }
}
