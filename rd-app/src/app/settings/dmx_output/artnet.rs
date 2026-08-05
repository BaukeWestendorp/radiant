use gpui::{App, Entity, Window, div, prelude::*};
use rd_ui::{Column, EnumerableValue, Table, TableDelegate, TableState};

pub struct ArtnetOutputTabView {
    table: Entity<TableState<ArtnetOutputInstanceTable>>,
    instances: Entity<Vec<rd::project::artnet::ArtnetOutputInstanceConfig>>,
}

impl ArtnetOutputTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let instances = cx.new(|cx| uncommitted_project.read(cx).output.artnet.instances.clone());

        cx.observe(&uncommitted_project, move |this, uncommitted_project, cx| {
            this.instances.write(cx, uncommitted_project.read(cx).output.artnet.instances.clone());
        })
        .detach();

        cx.observe(&instances, {
            let uncommitted_project = uncommitted_project.clone();
            move |_, instances, cx| {
                let new_instances = instances.read(cx).clone();

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
    instances: Entity<Vec<rd::project::artnet::ArtnetOutputInstanceConfig>>,
}

impl ArtnetOutputInstanceTable {
    fn new(
        instances: Entity<Vec<rd::project::artnet::ArtnetOutputInstanceConfig>>,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> Self {
        Self {
            columns: vec![
                Column::<Self>::new("name", "Name")
                    .with_sort_handler(|a, b| natord::compare(&a.name, &b.name))
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

    fn columns(&self, _cx: &App) -> impl Iterator<Item = &Column<Self>> {
        self.columns.iter()
    }

    fn column(&self, column_id: &str, _cx: &App) -> Option<&Column<Self>> {
        self.columns.iter().find(|c| c.id() == column_id)
    }

    fn rows(&self) -> Entity<Vec<Self::Row>> {
        self.instances.clone()
    }

    fn insert_new_row(&self, last_item_ix: Option<usize>, cx: &mut App) -> Option<usize> {
        let last_item = last_item_ix.and_then(|ix| self.rows().read(cx).get(ix));
        let new_instance = match last_item {
            Some(last_instance) => rd::project::artnet::ArtnetOutputInstanceConfig {
                name: last_instance.name.enumerated_value(1),
                port_address: last_instance.port_address.enumerated_value(1),
                local_universe: last_instance.local_universe.enumerated_value(1),
            },
            _ => rd::project::artnet::ArtnetOutputInstanceConfig {
                name: "Art-Net Output 1".to_string(),
                port_address: Default::default(),
                local_universe: Default::default(),
            },
        };

        let new_ix = self.instances.read(cx).len();
        self.instances.update(cx, |instances, cx| {
            instances.push(new_instance);
            cx.notify();
        });

        Some(new_ix)
    }
}
