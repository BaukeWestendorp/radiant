use std::collections::HashMap;

use gpui::{App, Entity, Window, div, prelude::*};
use rd_ui::{Column, Table, TableDelegate, TableState};
use uuid::Uuid;

pub struct ArtnetOutputTabView {
    table: Entity<TableState<ArtnetOutputInstanceTable>>,
    instances: Entity<HashMap<Uuid, rd::project::artnet::ArtnetOutputInstanceConfig>>,
}

impl ArtnetOutputTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let instances = cx.new(|cx| {
            uncommitted_project
                .read(cx)
                .output
                .artnet
                .instances
                .iter()
                .map(|instance| (Uuid::new_v4(), instance.clone()))
                .collect::<HashMap<_, _>>()
        });

        cx.observe(&uncommitted_project, move |this, uncommitted_project, cx| {
            let new_instances = uncommitted_project
                .read(cx)
                .output
                .artnet
                .instances
                .iter()
                .map(|instance| (Uuid::new_v4(), instance.clone()))
                .collect::<HashMap<_, _>>();

            this.instances.write(cx, new_instances);
        })
        .detach();

        cx.observe(&instances, {
            let uncommitted_project = uncommitted_project.clone();
            move |_, instances, cx| {
                let new_instances = instances.read(cx).values().cloned().collect();

                if new_instances == uncommitted_project.read(cx).output.artnet.instances {
                    return;
                }

                uncommitted_project.update(cx, |project, _| {
                    project.output.artnet.instances = new_instances;
                });
            }
        })
        .detach();

        let table = cx.new(|cx| {
            TableState::new(
                ArtnetOutputInstanceTable::new(instances.clone(), window, cx),
                cx.focus_handle(),
                window,
                cx,
            )
        });

        Self { table, instances }
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
    instances: Entity<HashMap<Uuid, rd::project::artnet::ArtnetOutputInstanceConfig>>,
}

impl ArtnetOutputInstanceTable {
    fn new(
        instances: Entity<HashMap<Uuid, rd::project::artnet::ArtnetOutputInstanceConfig>>,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> Self {
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

    fn columns(&self, _cx: &App) -> impl Iterator<Item = &Column<Self>> {
        self.columns.iter()
    }

    fn column(&self, column_id: &str, _cx: &App) -> Option<&Column<Self>> {
        self.columns.iter().find(|c| c.id() == column_id)
    }

    fn rows(&self) -> Entity<HashMap<Self::RowId, Self::Row>> {
        self.instances.clone()
    }

    fn insert_new_row(&self, cx: &mut App) -> Option<Self::RowId> {
        let new_instance = rd::project::artnet::ArtnetOutputInstanceConfig {
            name: "FIXME".to_string(),
            port_address: Default::default(),
            local_universe: Default::default(),
        };
        let new_id = Uuid::new_v4();

        self.instances.update(cx, |instances, cx| {
            instances.insert(new_id, new_instance);
            cx.notify();
        });

        Some(new_id)
    }
}
