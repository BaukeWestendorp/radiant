use gpui::{App, Entity, Window, div, prelude::*, px};
use rd_engine::project;
use rd_ui::{Column, Table, TableDelegate, TableSelection, TableState};
use uuid::Uuid;

use crate::app::engine::EngineAppExt;

pub struct MidiTabView {
    table: Entity<TableState<MidiMappingTable>>,
}

impl MidiTabView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            table: cx.new(|cx| {
                TableState::new(
                    MidiMappingTable::new(window, cx),
                    cx.new(|_| TableSelection::Multiple(Vec::new())),
                    window,
                    cx,
                )
            }),
        }
    }
}

impl Render for MidiTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(Table::new(self.table.clone()))
    }
}

struct MidiMappingTable {
    columns: Vec<Column>,

    mappings: Vec<(Uuid, project::midi::MidiMapping)>,
}

impl MidiMappingTable {
    fn new(window: &mut Window, cx: &mut Context<TableState<Self>>) -> Self {
        let mut mappings = cx.engine().with_project(|project| {
            project
                .trigger
                .midi
                .iter()
                .map(|mapping| (Uuid::new_v4(), mapping.clone()))
                .collect::<Vec<_>>()
        });
        mappings.sort_by(|(_, a), (_, b)| a.device_name.cmp(&b.device_name));

        let this = cx.entity();
        cx.on_engine_event_in(window, move |event, window, cx| match event {
            rd_engine::Event::ProjectLoaded => {
                cx.update_entity(&this, |this, cx| {
                    this.clear_selection(cx);
                    *this = TableState::new(
                        MidiMappingTable::new(window, cx),
                        this.selection(),
                        window,
                        cx,
                    );
                    cx.notify();
                });
            }
            _ => {}
        })
        .detach();

        Self {
            columns: vec![
                Column::new("device_name", "Device Name").with_min_width(px(250.0)),
                Column::new("device_channel", "Device Channel").with_min_width(px(125.0)),
                Column::new("filter_type", "Filter Type").with_min_width(px(100.0)),
                Column::new("filter_controller", "Filter Controller").with_min_width(px(125.0)),
                Column::new("filter_note", "Filter Note").with_min_width(px(100.0)),
                Column::new("transform_min_output", "Min Output").with_min_width(px(100.0)),
                Column::new("transform_max_output", "Max Output").with_min_width(px(100.0)),
                Column::new("transform_invert", "Invert").with_min_width(px(100.0)),
                Column::new("target", "Target").with_min_width(px(200.0)),
            ],
            mappings,
        }
    }
}

impl TableDelegate for MidiMappingTable {
    type RowId = Uuid;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn root_row_ids(&self, _cx: &App) -> Vec<Self::RowId> {
        self.mappings.iter().map(|(id, _)| *id).collect()
    }

    fn edit_rows(&self, row_ids: &[Self::RowId], cx: &mut App) {
        let mappings = self
            .mappings
            .iter()
            .map(|(id, mapping)| {
                let mut mapping = mapping.clone();
                if row_ids.contains(id) {
                    mapping.device_name = "Edited Device Name".to_string();
                }
                mapping
            })
            .collect::<Vec<_>>();

        if let Err(err) = cx.update_project(|project, _cx| {
            project.trigger.midi = mappings;
        }) {
            log::error!("Failed to update project: {err:#}");
        }
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let Some(mapping) =
            self.mappings.iter().find(|(id, _)| id == row_id).map(|(_, mapping)| mapping)
        else {
            return div().px_1().child("".to_string());
        };

        let content = match self.column(col_ix, cx).id().as_str() {
            "device_name" => mapping.device_name.to_string(),
            "device_channel" => mapping.device_channel.map(|v| v.to_string()).unwrap_or_default(),
            "filter_type" => mapping.filter_type.to_string(),
            "filter_controller" => {
                mapping.filter_controller.map(|v| v.to_string()).unwrap_or_default()
            }
            "filter_note" => mapping.filter_note.map(|v| v.to_string()).unwrap_or_default(),
            "transform_min_output" => mapping.transform_min_output.to_string(),
            "transform_max_output" => mapping.transform_max_output.to_string(),
            "transform_invert" => mapping.transform_invert.to_string(),
            "target" => mapping.target.to_string(),
            _ => "".to_string(),
        };

        div().px_1().child(content)
    }
}
