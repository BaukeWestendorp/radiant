use std::collections::HashMap;

use gpui::{Entity, Window, div, prelude::*};

use rd_ui::{Column, Table, TableDelegate, TableEvent, TableState};
use uuid::Uuid;

use crate::app::engine::EngineAppExt;

pub struct MidiTabView {
    table: Entity<TableState<MidiMappingTable>>,
    uncommitted_project: Entity<rd::Project>,
}

impl MidiTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let table = cx.new(|cx| {
            TableState::new(
                MidiMappingTable::new(uncommitted_project.clone(), window, cx),
                window,
                cx,
            )
        });

        cx.subscribe(&table, |this, table, event, cx| match event {
            TableEvent::EditSubmitted => this.uncommitted_project.update(cx, |project, cx| {
                project.trigger.midi =
                    dbg!(table.read(cx).delegate().mappings.values().cloned().collect());
                cx.notify();
            }),
        })
        .detach();

        Self { table, uncommitted_project }
    }
}

impl Render for MidiTabView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(Table::new("midi-triggers", self.table.clone(), window, cx))
    }
}

struct MidiMappingTable {
    columns: Vec<Column<Self>>,
    mappings: HashMap<Uuid, rd::project::midi::MidiMapping>,
}

impl MidiMappingTable {
    fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Self {
        let mappings = uncommitted_project
            .read(cx)
            .trigger
            .midi
            .iter()
            .map(|mapping| (Uuid::new_v4(), mapping.clone()))
            .collect();

        let this = cx.entity();
        cx.on_engine_event_in(window, {
            let uncommitted_project = uncommitted_project.clone();
            move |event, window, cx| match event {
                rd::Event::ProjectLoaded => {
                    dbg!("project loaded");
                    this.update(cx, |this, cx| {
                        // FIXME: this.clear_selection(cx);
                        *this = TableState::new(
                            MidiMappingTable::new(uncommitted_project.clone(), window, cx),
                            window,
                            cx,
                        );
                        cx.notify();
                    });
                }
                _ => {}
            }
        })
        .detach();

        Self {
            columns: vec![
                Column::<Self>::new("device_name", "Device Name")
                    .with_sort_handler(|a, b| a.device_name.cmp(&b.device_name))
                    .with_auto_enumerable_editor(|row| &mut row.device_name)
                    .with_cell_builder(|row, _window, _cx| {
                        row.device_name.to_string().into_any_element()
                    }),
                Column::<Self>::new("device_channel", "Device Channel")
                    .with_sort_handler(|a, b| a.device_channel.cmp(&b.device_channel))
                    .with_auto_enumerable_editor(|row| &mut row.device_channel)
                    .with_cell_builder(|row, _window, _cx| {
                        row.device_channel
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                            .into_any_element()
                    }),
                Column::<Self>::new("filter_type", "Filter Type")
                    .with_sort_handler(|a, b| {
                        a.filter_type
                            .partial_cmp(&b.filter_type)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .with_auto_editor(|row| &mut row.filter_type)
                    .with_cell_builder(|row, _window, _cx| {
                        row.filter_type.to_string().into_any_element()
                    }),
                Column::<Self>::new("filter_controller", "Filter Controller")
                    .with_sort_handler(|a, b| a.filter_controller.cmp(&b.filter_controller))
                    .with_auto_enumerable_editor(|row| &mut row.filter_controller)
                    .with_cell_builder(|row, _window, _cx| {
                        row.filter_controller
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                            .into_any_element()
                    }),
                Column::<Self>::new("filter_note", "Filter Note")
                    .with_sort_handler(|a, b| a.filter_note.cmp(&b.filter_note))
                    .with_auto_enumerable_editor(|row| &mut row.filter_note)
                    .with_cell_builder(|row, _window, _cx| {
                        row.filter_note
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                            .into_any_element()
                    }),
                Column::<Self>::new("transform_min_output", "Min Output")
                    .with_sort_handler(|a, b| {
                        a.transform_min_output
                            .partial_cmp(&b.transform_min_output)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .with_auto_editor(|row| &mut row.transform_min_output)
                    .with_cell_builder(|row, _window, _cx| {
                        row.transform_min_output.to_string().into_any_element()
                    }),
                Column::<Self>::new("transform_max_output", "Max Output")
                    .with_sort_handler(|a, b| {
                        a.transform_max_output
                            .partial_cmp(&b.transform_max_output)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .with_auto_editor(|row| &mut row.transform_max_output)
                    .with_cell_builder(|row, _window, _cx| {
                        row.transform_max_output.to_string().into_any_element()
                    }),
                Column::<Self>::new("transform_invert", "Invert")
                    .with_sort_handler(|a, b| a.transform_invert.cmp(&b.transform_invert))
                    .with_auto_editor(|row| &mut row.transform_invert)
                    .with_cell_builder(|row, _window, _cx| {
                        row.transform_invert.to_string().into_any_element()
                    }),
                Column::<Self>::new("target", "Target")
                    .with_sort_handler(|a, b| {
                        a.target.partial_cmp(&b.target).unwrap_or(std::cmp::Ordering::Equal)
                    })
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

    fn columns(&self) -> &[Column<Self>] {
        &self.columns
    }

    fn rows(&self) -> impl Iterator<Item = (&Self::RowId, &Self::Row)> {
        self.mappings.iter()
    }

    fn row(&self, row_id: &Self::RowId) -> Option<&Self::Row> {
        self.mappings.get(row_id)
    }

    fn row_mut(&mut self, row_id: &Self::RowId) -> Option<&mut Self::Row> {
        self.mappings.get_mut(row_id)
    }
}
