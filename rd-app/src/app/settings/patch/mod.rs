use rd_ui::{
    Emphasis, PopupSize, StyledExt,
    comp::stateful::{Field, Table, TableCellEditor, TableColumn},
    gpui::{Entity, Window, div, prelude::*},
};

use crate::{
    comp::stateful::{FixtureKindPicker, address_field, fixture_id_field},
    engine::EngineAppExt,
    util::Incrementable,
};

pub struct PatchTabView {
    table: Entity<Table<rd::project::FixtureConfig>>,
}

impl PatchTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let fixtures = cx.new(|cx| uncommitted_project.read(cx).patch.fixtures.clone());

        cx.observe(&uncommitted_project, {
            let fixtures = fixtures.clone();
            move |_, uncommitted_project, cx| {
                fixtures.write(cx, uncommitted_project.read(cx).patch.fixtures.clone());
            }
        })
        .detach();

        cx.observe(&fixtures, {
            let uncommitted_project = uncommitted_project.clone();
            move |_, fixtures, cx| {
                let new_fixtures = fixtures.read(cx).clone();

                if new_fixtures == uncommitted_project.read(cx).patch.fixtures {
                    return;
                }

                uncommitted_project.update(cx, |project, _| {
                    project.patch.fixtures = new_fixtures;
                });
            }
        })
        .detach();

        let table = cx.new(move |cx| {
            Table::new("table", fixtures.clone(), window, cx)
                .with_columns(
                    vec![
                        TableColumn::<rd::project::FixtureConfig>::new("Id")
                            .with_element(|row, _, _| row.id.to_string().into_any_element())
                            .with_editor(TableCellEditor::new(
                                "Edit Fixture ID",
                                |window, cx| {
                                    cx.new(|cx| fixture_id_field("fid", window, cx).w_full())
                                },
                                |row: &mut rd::project::FixtureConfig, value, i, _| {
                                    let Some(value) = value else { return };
                                    row.id = value.increment_by(i)
                                },
                            )),
                        TableColumn::<rd::project::FixtureConfig>::new("Name")
                            .with_element(|row, _, _| row.name.to_string().into_any_element())
                            .with_editor(TableCellEditor::new(
                                "Edit Fixture Name",
                                |window, cx| {
                                    cx.new(|cx| Field::<String>::new("name", window, cx).w_full())
                                },
                                |row: &mut rd::project::FixtureConfig, value, i, _| {
                                    let Some(value) = value else { return };
                                    row.name = value.increment_by(i)
                                },
                            )),
                        TableColumn::<rd::project::FixtureConfig>::new("Address")
                            .with_element(|row, _, _| {
                                row.dmx_address.to_string().into_any_element()
                            })
                            .with_editor(TableCellEditor::new(
                                "Edit Fixture Address",
                                |window, cx| {
                                    cx.new(|cx| address_field("address", window, cx).w_full())
                                },
                                |row: &mut rd::project::FixtureConfig, value, i, cx| {
                                    let Some(value) = value else { return };
                                    let channel_count = cx.engine().with_project(|project| {
                                        row.fixture_kind
                                            .dmx_mode(project)
                                            .map(|dmx_mode| dmx_mode.max_channel_offset() + 1)
                                            .unwrap_or(1)
                                    });
                                    row.dmx_address =
                                        value.increment_by(i * channel_count as usize);
                                },
                            )),
                        TableColumn::<rd::project::FixtureConfig>::new("Kind")
                            .with_element(|row, _, cx| {
                                cx.engine().with_project(|project| {
                                    row.fixture_kind.display(project).into_any_element()
                                })
                            })
                            .with_editor(
                                TableCellEditor::new(
                                    "Edit Fixture Kind",
                                    |window, cx| {
                                        cx.new(|cx| {
                                            FixtureKindPicker::new("fixture_kind", window, cx)
                                        })
                                    },
                                    |row: &mut rd::project::FixtureConfig, value, _, _| {
                                        row.fixture_kind = value.clone();
                                    },
                                )
                                .with_popup_size(PopupSize::Max),
                            ),
                    ],
                    cx,
                )
                .with_on_delete(cx, move |row_ixs, _, _, cx| {
                    fixtures.update(cx, |fixtures, cx| {
                        for ix in row_ixs.iter().rev() {
                            fixtures.remove(*ix);
                        }
                        cx.notify();
                    });
                })
        });

        Self { table }
    }
}

impl Render for PatchTabView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(
            div().size_full().emphasis_bordered(Emphasis::Primary, cx).child(self.table.clone()),
        )
    }
}
