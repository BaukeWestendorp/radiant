use rd_artnet::PortAddress;
use rd_ui::{
    comp::stateful::{Field, Table, TableCellEditor, TableColumn},
    gpui::{Entity, Window, div, prelude::*},
};

use crate::comp::stateful::{port_address_field, universe_id_field};

pub struct ArtnetOutputTabView {
    table: Entity<Table<rd::project::ArtnetOutputInstanceConfig>>,
}

impl ArtnetOutputTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let instances = cx.new(|cx| uncommitted_project.read(cx).output.artnet.instances.clone());

        cx.observe(&uncommitted_project, {
            let instances = instances.clone();
            move |_, uncommitted_project, cx| {
                instances.write(cx, uncommitted_project.read(cx).output.artnet.instances.clone());
            }
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
            Table::new("artnet-output-instances", instances.clone(), window, cx).with_columns(
                vec![
                    TableColumn::<rd::project::ArtnetOutputInstanceConfig>::new("Name")
                        .with_element(|row, _, _| row.name.to_string().into_any_element())
                        .with_editor(TableCellEditor::new(
                            "Edit Instance Name",
                            |window, cx| {
                                cx.new(|cx| Field::<String>::new("name", window, cx).w_full())
                            },
                            |row: &mut rd::project::ArtnetOutputInstanceConfig, value, i, _| {
                                let Some(value) = value else { return };
                                let base_name = value.clone().trim().to_string();
                                row.name = if i == 0 {
                                    base_name
                                } else {
                                    format!("{} {}", base_name, i + 1)
                                };
                            },
                        )),
                    TableColumn::<rd::project::ArtnetOutputInstanceConfig>::new("Port Address")
                        .with_element(|row, _, _| row.port_address.to_string().into_any_element())
                        .with_editor(TableCellEditor::new(
                            "Edit Port Address",
                            |window, cx| {
                                cx.new(|cx| port_address_field("port_address", window, cx).w_full())
                            },
                            |row: &mut rd::project::ArtnetOutputInstanceConfig, value, i, _| {
                                let Some(value) = value else { return };
                                let base_address = value.clone();
                                if i == 0 {
                                    row.port_address = base_address
                                } else {
                                    if let Ok(new_address) = PortAddress::from_absolute(
                                        base_address.as_u16() + (i as u16),
                                    ) {
                                        row.port_address = new_address;
                                    }
                                };
                            },
                        )),
                    TableColumn::<rd::project::ArtnetOutputInstanceConfig>::new("Local Universe")
                        .with_element(|row, _, _| row.local_universe.to_string().into_any_element())
                        .with_editor(TableCellEditor::new(
                            "Edit Local Universe",
                            |window, cx| {
                                cx.new(|cx| universe_id_field("universe_id", window, cx).w_full())
                            },
                            |row: &mut rd::project::ArtnetOutputInstanceConfig, value, i, _| {
                                let Some(value) = value else { return };
                                let base_universe = value.clone();
                                if i == 0 {
                                    row.local_universe = base_universe
                                } else {
                                    if let Ok(new_universe) =
                                        rd_dmx::UniverseId::new(base_universe.as_u16() + (i as u16))
                                    {
                                        row.local_universe = new_universe;
                                    }
                                };
                            },
                        )),
                ],
                cx,
            )
        });

        Self { table }
    }
}

impl Render for ArtnetOutputTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.table.clone())
    }
}
