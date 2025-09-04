use gpui::prelude::*;
use gpui::{App, ElementId, Entity, ReadGlobal, UpdateGlobal, Window, WindowHandle, div, px};
use radiant::builtin::{FixtureId, Patch};
use radiant::cmd::{Command, PatchCommand};
use radiant::engine::event::EngineEvent;
use ui::interactive::input::{Field, FieldEvent};
use ui::interactive::table::{Table, TableColumn, TableDelegate, TableRow};

use crate::engine::EngineManager;

pub struct PatchWindow {
    table: Entity<Table<PatchTable>>,
}

impl PatchWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |window, cx| cx.new(|cx| Self::new(window, cx)))
            .expect("should open patch window")
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let eh = EngineManager::global(cx).event_handler();
        cx.subscribe_in(&eh, window, |this, _, event: &EngineEvent, window, cx| match event {
            EngineEvent::PatchChanged => *this = Self::new(window, cx),
        })
        .detach();

        Self {
            table: cx.new(|cx| {
                let mut table = Table::new(PatchTable::new(window, cx), "patch_table", window, cx);
                table.set_column_width(PatchTableColumn::Name, px(150.0));
                table.set_column_width(PatchTableColumn::FixtureId, px(50.0));
                table.set_column_width(PatchTableColumn::Address, px(100.0));
                table.set_column_width(PatchTableColumn::FixtureType, px(200.0));
                table.set_column_width(PatchTableColumn::DmxMode, px(200.0));
                table
            }),
        }
    }
}

impl Render for PatchWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root(window, cx)
            .child(div().size_full().id("patch_window_root").child(self.table.clone()))
    }
}

struct PatchTable {
    rows: Vec<PatchTableRow>,
}

impl PatchTable {
    pub fn new(window: &mut Window, cx: &mut Context<Table<Self>>) -> Self {
        let rows = EngineManager::global(cx).engine.patch().read(|patch| {
            patch.fixtures().iter().map(|f| PatchTableRow::new(f.fid, patch, window, cx)).collect()
        });

        Self { rows }
    }
}

impl TableDelegate for PatchTable {
    type Row = PatchTableRow;

    type Column = PatchTableColumn;

    fn rows(&mut self, _cx: &mut App) -> Vec<Self::Row> {
        self.rows.clone()
    }
}

#[derive(Clone)]
struct PatchTableRow {
    fid: FixtureId,

    name_field: Entity<Field<String>>,
    fid_field: Entity<Field<FixtureId>>,
    address_field: Entity<Field<dmx::Address>>,
}

impl PatchTableRow {
    pub fn new(fid: FixtureId, patch: &Patch, window: &mut Window, cx: &mut App) -> Self {
        let fixture = patch.fixture(fid);
        Self {
            fid,
            name_field: {
                let field = cx.new(|cx| {
                    Field::new("name_field", cx.focus_handle(), window, cx)
                        .with_placeholder("Name", cx)
                        .with_value(&fixture.map(|f| f.name.to_string()).unwrap_or_default(), cx)
                        .with_styled(false)
                });
                cx.subscribe(&field, move |_, event: &FieldEvent<String>, cx| match event {
                    FieldEvent::Submit(value) => EngineManager::update_global(cx, |state, _| {
                        state.engine.exec_and_log_err(Command::Patch(PatchCommand::SetName {
                            fid,
                            name: value.clone(),
                        }));
                    }),
                    _ => {}
                })
                .detach();
                field
            },
            fid_field: {
                let field = cx.new(|cx| {
                    Field::new("fid_field", cx.focus_handle(), window, cx)
                        .with_placeholder("Fixture Id", cx)
                        .with_value(&fixture.map(|f| f.fid).unwrap_or_default(), cx)
                        .with_styled(false)
                });
                cx.subscribe(&field, move |_, event: &FieldEvent<FixtureId>, cx| match event {
                    FieldEvent::Submit(value) => EngineManager::update_global(cx, |state, _| {
                        state.engine.exec_and_log_err(Command::Patch(PatchCommand::SetFixtureId {
                            fid,
                            new_fid: *value,
                        }));
                    }),
                    _ => {}
                })
                .detach();
                field
            },
            address_field: {
                let field = cx.new(|cx| {
                    Field::new("address_field", cx.focus_handle(), window, cx)
                        .with_placeholder("Address", cx)
                        .with_value(&fixture.map(|f| f.address).unwrap_or_default(), cx)
                        .with_styled(false)
                });
                cx.subscribe(&field, move |_, event: &FieldEvent<dmx::Address>, cx| match event {
                    FieldEvent::Submit(value) => EngineManager::update_global(cx, |state, _| {
                        state.engine.exec_and_log_err(Command::Patch(PatchCommand::SetAddress {
                            fid,
                            address: *value,
                        }));
                    }),
                    _ => {}
                })
                .detach();
                field
            },
        }
    }
}

impl TableRow<PatchTable> for PatchTableRow {
    fn id(&self, _cx: &mut Context<Table<PatchTable>>) -> ElementId {
        ElementId::Integer(u32::from(self.fid) as u64)
    }

    fn render_cell(
        &self,
        column: &PatchTableColumn,
        _window: &mut Window,
        cx: &mut Context<Table<PatchTable>>,
    ) -> impl IntoElement {
        EngineManager::global(cx).engine.patch().read(|patch| {
            let Some(fixture) = patch.fixture(self.fid) else {
                return div();
            };

            match column {
                PatchTableColumn::Name => div().size_full().child(self.name_field.clone()),
                PatchTableColumn::FixtureId => div().size_full().child(self.fid_field.clone()),
                PatchTableColumn::Address => div().size_full().child(self.address_field.clone()),
                PatchTableColumn::FixtureType => div()
                    .px_1()
                    .w_full()
                    .text_ellipsis()
                    .child(fixture.fixture_type(patch).short_name.to_string()),
                PatchTableColumn::DmxMode => {
                    div().px_1().w_full().text_ellipsis().child(fixture.dmx_mode.to_string())
                }
            }
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PatchTableColumn {
    Name,
    FixtureId,
    Address,
    FixtureType,
    DmxMode,
}

impl TableColumn for PatchTableColumn {
    fn label(&self) -> &str {
        match self {
            PatchTableColumn::Name => "Name",
            PatchTableColumn::FixtureId => "Fixture Id",
            PatchTableColumn::Address => "Address",
            PatchTableColumn::FixtureType => "Fixture Type",
            PatchTableColumn::DmxMode => "DMX Mode",
        }
    }

    fn all<'a>() -> &'a [Self] {
        &[Self::Name, Self::FixtureId, Self::Address, Self::FixtureType, Self::DmxMode]
    }
}
