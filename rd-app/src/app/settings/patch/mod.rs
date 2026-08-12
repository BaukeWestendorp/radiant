use rd_ui::{
    InputPopup, PopupAppExt,
    comp::stateful::{Field, Table, TableColumn},
    gpui::{Entity, Window, div, prelude::*},
};

use crate::comp::stateful::{AddressField, FixtureIdField};

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
            Table::new("table", fixtures.clone(), window, cx).with_columns(vec![
                TableColumn::<rd::project::FixtureConfig>::new("Id")
                    .with_element(|row, _, _| row.id.to_string().into_any_element())
                    .with_on_edit({
                        let fixtures = fixtures.clone();
                        move |row_ixs, window, cx| {
                            let popup = cx.new(|cx| {
                                let row_ixs = row_ixs.to_vec();
                                let fixtures = fixtures.clone();
                                InputPopup::new(
                                    cx.new(|cx| FixtureIdField::new("fid", window, cx).w_full()),
                                    window,
                                    cx,
                                )
                                .with_on_submit(
                                    window,
                                    cx,
                                    move |value, _, cx| {
                                        let Some(value) = value else { return };
                                        fixtures.update(cx, |fixtures, cx| {
                                            for row_ix in &row_ixs {
                                                if let Some(row) = fixtures.get_mut(*row_ix) {
                                                    row.id = *value;
                                                }
                                                cx.notify();
                                            }
                                        });
                                        cx.dismiss_popup();
                                    },
                                )
                            });
                            cx.set_popup("Edit Fixture ID", popup);
                        }
                    }),
                TableColumn::<rd::project::FixtureConfig>::new("Name")
                    .with_element(|row, _, _| row.name.to_string().into_any_element())
                    .with_on_edit({
                        let fixtures = fixtures.clone();
                        move |row_ixs, window, cx| {
                            let popup = cx.new(|cx| {
                                let row_ixs = row_ixs.to_vec();
                                let fixtures = fixtures.clone();
                                InputPopup::new(
                                    cx.new(|cx| Field::new("name", window, cx).w_full()),
                                    window,
                                    cx,
                                )
                                .with_on_submit(
                                    window,
                                    cx,
                                    move |value, _, cx| {
                                        fixtures.update(cx, |fixtures, cx| {
                                            let name = value.to_string().trim().to_string();
                                            for row_ix in &row_ixs {
                                                if let Some(row) = fixtures.get_mut(*row_ix) {
                                                    row.name = name.clone();
                                                }
                                                cx.notify();
                                            }
                                        });
                                        cx.dismiss_popup();
                                    },
                                )
                            });
                            cx.set_popup("Edit Fixture Name", popup);
                        }
                    }),
                TableColumn::<rd::project::FixtureConfig>::new("Address")
                    .with_element(|row, _, _| row.dmx_address.to_string().into_any_element())
                    .with_on_edit({
                        let fixtures = fixtures.clone();
                        move |row_ixs, window, cx| {
                            let popup = cx.new(|cx| {
                                let row_ixs = row_ixs.to_vec();
                                let fixtures = fixtures.clone();
                                InputPopup::new(
                                    cx.new(|cx| AddressField::new("address", window, cx).w_full()),
                                    window,
                                    cx,
                                )
                                .with_on_submit(
                                    window,
                                    cx,
                                    move |value, _, cx| {
                                        let Some(value) = value else { return };
                                        fixtures.update(cx, |fixtures, cx| {
                                            for row_ix in &row_ixs {
                                                if let Some(row) = fixtures.get_mut(*row_ix) {
                                                    row.dmx_address = *value;
                                                }
                                                cx.notify();
                                            }
                                        });
                                        cx.dismiss_popup();
                                    },
                                )
                            });
                            cx.set_popup("Edit Fixture Address", popup);
                        }
                    }),
                TableColumn::<rd::project::FixtureConfig>::new("Kind")
                    .with_element(|row, _, _| row.fixture_kind.to_string().into_any_element()),
            ])
        });

        Self { table }
    }
}

impl Render for PatchTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.table.clone())
    }
}
