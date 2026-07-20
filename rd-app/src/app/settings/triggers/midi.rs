use gpui::{App, Entity, Window, div, prelude::*, px};

use rd::{Project, project::midi::MidiMapping};
use rd_ui::{Column, Popup, PopupAppExt, Table, TableDelegate, TableSelection, TableState};
use uuid::Uuid;

use crate::{app::engine::EngineAppExt, util::ValueEnumerator};

pub struct MidiTabView {
    table: Entity<TableState<MidiMappingTable>>,
}

impl MidiTabView {
    pub fn new(
        uncommitted_project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            table: cx.new(|cx| {
                TableState::new(MidiMappingTable::new(uncommitted_project, window, cx), window, cx)
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
    columns: Vec<Column<Self>>,

    mappings: Vec<(Uuid, rd::project::midi::MidiMapping)>,

    uncommitted_project: Entity<Project>,
}

impl MidiMappingTable {
    fn new(
        uncommitted_project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Self {
        let mut mappings = uncommitted_project
            .read(cx)
            .trigger
            .midi
            .iter()
            .map(|mapping| (Uuid::new_v4(), mapping.clone()))
            .collect::<Vec<_>>();
        mappings.sort_by(|(_, a), (_, b)| a.device_name.cmp(&b.device_name));

        // let this = cx.entity();
        // cx.on_engine_event_in(window, {
        //     let uncommitted_project = uncommitted_project.clone();
        //     move |event, window, cx| match event {
        //         rd::Event::ProjectLoaded => {
        //             cx.update_entity(&this, |this, cx| {
        //                 this.clear_selection(cx);
        //                 *this = TableState::new(
        //                     MidiMappingTable::new(uncommitted_project.clone(), window, cx),
        //                     this.selection(),
        //                     window,
        //                     cx,
        //                 );
        //                 cx.notify();
        //             });
        //         }
        //         _ => {}
        //     }
        // })
        // .detach();

        // cx.observe_in(&uncommitted_project, window, move |this, project, window, cx| {
        //     this.clear_selection(cx);
        //     *this = TableState::new(
        //         MidiMappingTable::new(project.clone(), window, cx),
        //         this.selection(),
        //         window,
        //         cx,
        //     );
        //     cx.notify();
        // })
        // .detach();

        Self {
            columns: vec![
                Column::<Self>::new("device_name", "Device Name").with_cell_builder(
                    |row, window, cx| row.device_name.to_string().into_any_element(),
                ),
                Column::<Self>::new("device_channel", "Device Channel").with_cell_builder(
                    |row, window, cx| row.device_channel.to_string().into_any_element(),
                ),
                Column::<Self>::new("filter_type", "Filter Type").with_cell_builder(
                    |row, window, cx| row.filter_type.to_string().into_any_element(),
                ),
                Column::<Self>::new("filter_controller", "Filter Controller").with_cell_builder(
                    |row, window, cx| row.filter_controller.to_string().into_any_element(),
                ),
                Column::<Self>::new("filter_note", "Filter Note").with_cell_builder(
                    |row, window, cx| row.filter_note.to_string().into_any_element(),
                ),
                Column::<Self>::new("transform_min_output", "Min Output").with_cell_builder(
                    |row, window, cx| row.transform_min_output.to_string().into_any_element(),
                ),
                Column::<Self>::new("transform_max_output", "Max Output").with_cell_builder(
                    |row, window, cx| row.transform_max_output.to_string().into_any_element(),
                ),
                Column::<Self>::new("transform_invert", "Invert").with_cell_builder(
                    |row, window, cx| row.transform_invert.to_string().into_any_element(),
                ),
                Column::<Self>::new("target", "Target")
                    .with_cell_builder(|row, window, cx| row.target.to_string().into_any_element()),
            ],
            mappings,
            uncommitted_project,
        }
    }
}

impl TableDelegate for MidiMappingTable {
    type Row = MidiMapping;
    type RowId = Uuid;

    // fn column_count(&self, _cx: &App) -> usize {
    //     self.columns.len()
    // }

    // fn column(&self, col_ix: usize, _cx: &App) -> &Column<Self> {
    //     &self.columns[col_ix]
    // }

    // fn root_row_ids(&self, _cx: &App) -> Vec<Self::RowId> {
    //     self.mappings.iter().map(|(id, _)| *id).collect()
    // }

    // let col_id = self.column(col_ix, cx).id().to_string();

    // let mappings = self
    //     .mappings
    //     .iter_mut()
    //     .filter(|(id, _)| row_ids.contains(id))
    //     .map(|(_, mapping)| mapping);

    // match col_id.as_str() {
    //     "device_name" => {
    //         cx.open_popup(window, |window, cx| {

    //         });
    //     }
    //     "device_channel" => ValueEnumerator::enumerate(
    //         mappings.map(|m| &mut m.device_channel).filter_map(|v| v.as_mut()),
    //         1,
    //     ),
    //     "filter_type" => ValueEnumerator::copy_first(mappings.map(|m| &mut m.filter_type)),
    //     "filter_controller" => ValueEnumerator::enumerate(
    //         mappings.map(|m| &mut m.filter_controller).filter_map(|v| v.as_mut()),
    //         1,
    //     ),
    //     "filter_note" => ValueEnumerator::enumerate(
    //         mappings.map(|m| &mut m.filter_note).filter_map(|v| v.as_mut()),
    //         1,
    //     ),
    //     "transform_min_output" => {
    //         ValueEnumerator::enumerate(mappings.map(|m| &mut m.transform_min_output), 1)
    //     }
    //     "transform_max_output" => {
    //         ValueEnumerator::enumerate(mappings.map(|m| &mut m.transform_max_output), 1)
    //     }
    //     "transform_invert" => {
    //         ValueEnumerator::copy_first(mappings.map(|m| &mut m.transform_invert))
    //     }
    //     "target" => ValueEnumerator::copy_first(mappings.map(|m| &mut m.target)),
    //     _ => {}
    // }

    // let updated_midi_state: Vec<_> = self.mappings.iter().map(|(_, m)| m.clone()).collect();

    // self.uncommitted_project.update(cx, move |project, cx| {
    //     project.trigger.midi = updated_midi_state;
    //     cx.notify();
    // });

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
