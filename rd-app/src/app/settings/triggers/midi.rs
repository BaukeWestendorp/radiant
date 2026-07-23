use std::collections::HashMap;

use gpui::{Entity, Window, div, prelude::*};

use rd::Project;
use rd_ui::todo;
use uuid::Uuid;

pub struct MidiTabView {
    // table: Entity<TableState<MidiMappingTable>>,
}

impl MidiTabView {
    pub fn new(
        uncommitted_project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            // table: cx.new(|cx| {
            //     TableState::new(MidiMappingTable::new(uncommitted_project, window, cx), window, cx)
            // }),
        }
    }
}

impl Render for MidiTabView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // div().size_full().child(Table::new("midi-triggers", self.table.clone(), window, cx))
        todo(cx)
    }
}

// struct MidiMappingTable {
//     columns: Vec<Column<Self>>,
//     items: HashMap<Uuid, rd::project::midi::MidiMapping>,

//     uncommitted_project: Entity<Project>,
// }

// impl MidiMappingTable {
//     fn new(
//         uncommitted_project: Entity<Project>,
//         window: &mut Window,
//         cx: &mut Context<TableState<Self>>,
//     ) -> Self {
//         let items = uncommitted_project
//             .read(cx)
//             .trigger
//             .midi
//             .iter()
//             .map(|mapping| (Uuid::new_v4(), mapping.clone()))
//             .collect();

//         // let this = cx.entity();
//         // cx.on_engine_event_in(window, {
//         //     let uncommitted_project = uncommitted_project.clone();
//         //     move |event, window, cx| match event {
//         //         rd::Event::ProjectLoaded => {
//         //             cx.update_entity(&this, |this, cx| {
//         //                 this.clear_selection(cx);
//         //                 *this = TableState::new(
//         //                     MidiMappingTable::new(uncommitted_project.clone(), window, cx),
//         //                     this.selection(),
//         //                     window,
//         //                     cx,
//         //                 );
//         //                 cx.notify();
//         //             });
//         //         }
//         //         _ => {}
//         //     }
//         // })
//         // .detach();

//         Self {
//             columns: vec![
//                 Column::<Self>::new("device_name", "Device Name")
//                     .with_enumerable_field_edit_handler(|row| &mut row.device_name)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.device_name.to_string().into_any_element()
//                     }),
//                 Column::<Self>::new("device_channel", "Device Channel")
//                     .with_enumerable_field_edit_handler(|row| &mut row.device_channel)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.device_channel
//                             .map(|v| v.to_string())
//                             .unwrap_or_default()
//                             .into_any_element()
//                     }),
//                 Column::<Self>::new("filter_type", "Filter Type")
//                     .with_dropdown_edit_handler(|row| &mut row.filter_type)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.filter_type.to_string().into_any_element()
//                     }),
//                 Column::<Self>::new("filter_controller", "Filter Controller")
//                     .with_enumerable_field_edit_handler(|row| &mut row.filter_controller)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.filter_controller
//                             .map(|v| v.to_string())
//                             .unwrap_or_default()
//                             .into_any_element()
//                     }),
//                 Column::<Self>::new("filter_note", "Filter Note")
//                     .with_enumerable_field_edit_handler(|row| &mut row.filter_note)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.filter_note
//                             .map(|v| v.to_string())
//                             .unwrap_or_default()
//                             .into_any_element()
//                     }),
//                 Column::<Self>::new("transform_min_output", "Min Output")
//                     .with_clonable_field_edit_handler(|row| &mut row.transform_min_output)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.transform_min_output.to_string().into_any_element()
//                     }),
//                 Column::<Self>::new("transform_max_output", "Max Output")
//                     .with_clonable_field_edit_handler(|row| &mut row.transform_max_output)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.transform_max_output.to_string().into_any_element()
//                     }),
//                 Column::<Self>::new("transform_invert", "Invert")
//                     .with_clonable_field_edit_handler(|row| &mut row.transform_invert)
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.transform_invert.to_string().into_any_element()
//                     }),
//                 Column::<Self>::new("target", "Target")
//                     .with_popup_edit_handler(
//                         |row| row.target.clone(),
//                         |_target, _state, _row_ids, _window, _cx| {
//                             todo!();
//                         },
//                     )
//                     .with_cell_builder(|row, _window, _cx| {
//                         row.target.to_string().into_any_element()
//                     }),
//             ],
//             items,
//             uncommitted_project,
//         }
//     }
// }

// impl TableDelegate for MidiMappingTable {
//     type Row = rd::project::midi::MidiMapping;
//     type RowId = Uuid;

//     fn columns(&self) -> &[Column<Self>] {
//         &self.columns
//     }

//     fn rows(&self) -> impl Iterator<Item = (&Self::RowId, &Self::Row)> {
//         self.items.iter()
//     }

//     fn row(&self, row_id: &Self::RowId) -> Option<&Self::Row> {
//         self.items.get(row_id)
//     }

//     fn row_mut(&mut self, row_id: &Self::RowId) -> Option<&mut Self::Row> {
//         self.items.get_mut(row_id)
//     }
// }
