use std::collections::HashMap;

use gpui::{Entity, Window, div, prelude::*};
use rd_ui::{Column, Table, TableDelegate, TableEvent, TableState};
use uuid::Uuid;

use crate::app::engine::EngineAppExt;

pub struct ArtnetOutputTabView {
    table: Entity<TableState<ArtnetOutputInstanceTable>>,
    uncommitted_project: Entity<rd::Project>,
}

impl ArtnetOutputTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let table = cx.new(|cx| {
            TableState::new(
                ArtnetOutputInstanceTable::new(uncommitted_project.clone(), window, cx),
                cx.focus_handle(),
                window,
                cx,
            )
        });

        cx.observe_in(&uncommitted_project, window, |this, uncommitted_project, window, cx| {
            this.table.update(cx, |table, cx| {
                *table = TableState::new(
                    ArtnetOutputInstanceTable::new(uncommitted_project.clone(), window, cx),
                    cx.focus_handle(),
                    window,
                    cx,
                );
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&table, |this, table, event, cx| match event {
            TableEvent::EditSubmitted => this.uncommitted_project.update(cx, |project, cx| {
                project.output.artnet.instances =
                    table.read(cx).delegate().instances.values().cloned().collect();
                cx.notify();
            }),
        })
        .detach();

        Self { table, uncommitted_project }
    }
}

impl Render for ArtnetOutputTabView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(Table::new(
            "artnet-output-instances",
            self.table.clone(),
            window,
            cx,
        ))
    }
}

struct ArtnetOutputInstanceTable {
    columns: Vec<Column<Self>>,
    instances: HashMap<Uuid, rd::project::artnet::ArtnetOutputInstanceConfig>,
}

impl ArtnetOutputInstanceTable {
    fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Self {
        let instances = uncommitted_project
            .read(cx)
            .output
            .artnet
            .instances
            .iter()
            .map(|instance| (Uuid::new_v4(), instance.clone()))
            .collect();

        let this = cx.entity();
        cx.on_engine_event_in(window, {
            let uncommitted_project = uncommitted_project.clone();
            move |event, window, cx| match event {
                rd::Event::ProjectLoaded => {
                    this.update(cx, |this, cx| {
                        this.selection().update(cx, |selection, cx| {
                            selection.clear();
                            cx.notify();
                        });

                        *this = TableState::new(
                            ArtnetOutputInstanceTable::new(uncommitted_project.clone(), window, cx),
                            cx.focus_handle(),
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
                Column::<Self>::new("name", "Name")
                    .with_sort_handler(|a, b| a.name.cmp(&b.name))
                    .with_auto_enumerable_editor(|row| &mut row.name)
                    .with_cell_builder(|row, _window, _cx| row.name.to_string().into_any_element()),
                Column::<Self>::new("port_address", "Port Address")
                    .with_sort_handler(|a, b| a.port_address.cmp(&b.port_address))
                    .with_auto_enumerable_editor(|row| &mut row.port_address)
                    .with_cell_builder(|row, _window, _cx| {
                        row.port_address.to_string().into_any_element()
                    }),
                Column::<Self>::new("local_universe", "Local Universe")
                    .with_sort_handler(|a, b| a.local_universe.cmp(&b.local_universe))
                    .with_auto_enumerable_editor(|row| &mut row.local_universe)
                    .with_cell_builder(|row, _window, _cx| {
                        row.local_universe.to_string().into_any_element()
                    }),
            ],
            instances,
        }
    }
}

impl TableDelegate for ArtnetOutputInstanceTable {
    type Row = rd::project::artnet::ArtnetOutputInstanceConfig;
    type RowId = Uuid;

    fn columns(&self) -> &[Column<Self>] {
        &self.columns
    }

    fn column(&self, column_id: &str) -> Option<&Column<Self>> {
        self.columns.iter().find(|c| c.id() == column_id)
    }

    fn rows(&self) -> impl Iterator<Item = (&Self::RowId, &Self::Row)> {
        self.instances.iter()
    }

    fn row_count(&self) -> usize {
        self.instances.len()
    }

    fn row(&self, row_id: &Self::RowId) -> Option<&Self::Row> {
        self.instances.get(row_id)
    }

    fn row_mut(&mut self, row_id: &Self::RowId) -> Option<&mut Self::Row> {
        self.instances.get_mut(row_id)
    }

    fn delete_rows<'a>(&mut self, row_ids: impl Iterator<Item = &'a Self::RowId>) {
        for row_id in row_ids {
            self.instances.remove(row_id);
        }
    }
}
