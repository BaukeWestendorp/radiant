use rd_ui::{
    Emphasis, StyledExt,
    comp::stateful::{Field, Table, TableColumn},
    gpui::{Entity, Window, div, prelude::*},
};

use crate::{
    comp::stateful::{address_field, fixture_id_field},
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

        let table = cx.new(|cx| {
            Table::new("table", fixtures.clone(), window, cx).with_columns(
                vec![
                    TableColumn::<rd::project::FixtureConfig>::new("Id")
                        .with_element(|row, _, _| row.id.to_string().into_any_element())
                        .with_editor(
                            "Edit Fixture ID",
                            |window, cx| cx.new(|cx| fixture_id_field("fid", window, cx).w_full()),
                            |row, value, n| row.id = value.increment_by(n),
                        ),
                    TableColumn::<rd::project::FixtureConfig>::new("Name")
                        .with_element(|row, _, _| row.name.to_string().into_any_element())
                        .with_editor(
                            "Edit Fixture Name",
                            |window, cx| {
                                cx.new(|cx| Field::<String>::new("name", window, cx).w_full())
                            },
                            |row, value, n| row.name = value.increment_by(n),
                        ),
                    TableColumn::<rd::project::FixtureConfig>::new("Address")
                        .with_element(|row, _, _| row.dmx_address.to_string().into_any_element())
                        .with_editor(
                            "Edit Fixture Address",
                            |window, cx| cx.new(|cx| address_field("address", window, cx).w_full()),
                            |row, value, n| {
                                let channel_count = 1;
                                row.dmx_address = value.increment_by(n * channel_count);
                            },
                        ),
                    TableColumn::<rd::project::FixtureConfig>::new("Kind")
                        .with_element(|row, _, _| row.fixture_kind.to_string().into_any_element()), // FIXME: Add editor
                ],
                cx,
            )
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
