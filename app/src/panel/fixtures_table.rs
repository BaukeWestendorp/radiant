use gpui::prelude::*;
use gpui::{App, ElementId, Entity, Subscription, UpdateGlobal, Window, div, px};
use radiant::engine::{Command, EngineEvent};
use radiant::show::Fixture;
use ui::{Table, TableColumn, TableDelegate, TableRow};

use crate::app::{AppState, on_engine_event, with_show};

pub struct FixturesTablePanel {
    table: Entity<Table<FixturesTable>>,
    _event_subscription: Option<Subscription>,
}

impl FixturesTablePanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let event_subscription =
            on_engine_event(cx, window, |panel, event, window, cx| match event {
                EngineEvent::SelectionChanged => {
                    panel._event_subscription.take();
                    *panel = FixturesTablePanel::new(window, cx);
                    cx.notify();
                }
                _ => {}
            });

        Self {
            table: cx.new(|cx| {
                let mut table = Table::new(FixturesTable::new(), "fixtures_table", window, cx);
                table.set_column_width(Column::Id, px(50.0));
                table.set_column_width(Column::Address, px(80.0));
                table.set_column_width(Column::Type, px(200.0));
                table.set_column_width(Column::DmxMode, px(200.0));
                table
            }),
            _event_subscription: Some(event_subscription),
        }
    }
}

impl Render for FixturesTablePanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.table.clone())
    }
}

struct FixturesTable {}

impl FixturesTable {
    pub fn new() -> Self {
        Self {}
    }
}

impl TableDelegate for FixturesTable {
    type Row = Row;

    type Column = Column;

    fn rows(&mut self, cx: &mut App) -> Vec<Self::Row> {
        with_show(cx, |show| {
            show.patch().fixtures().iter().cloned().map(|fixture| Row { fixture }).collect()
        })
    }

    fn handle_on_click_row(
        &mut self,
        row: Self::Row,
        _event: &gpui::ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) {
        AppState::update_global(cx, |state, _| {
            state.engine.exec(Command::SelectFixture { fid: row.fixture.fid() });
        });
    }
}

#[derive(Debug, Clone)]
struct Row {
    fixture: Fixture,
}

impl TableRow<FixturesTable> for Row {
    fn id(&self, _cx: &mut Context<Table<FixturesTable>>) -> ElementId {
        ElementId::Integer(*self.fixture.fid() as u64)
    }

    fn selected(&self, cx: &mut Context<Table<FixturesTable>>) -> bool {
        with_show(cx, |show| show.selected_fixtures().contains(&self.fixture.fid()))
    }

    fn render_cell(
        &self,
        column: &<FixturesTable as TableDelegate>::Column,
        _window: &mut Window,
        cx: &mut Context<Table<FixturesTable>>,
    ) -> impl IntoElement {
        let value = with_show(cx, |show| match column {
            Column::Id => self.fixture.fid().to_string(),
            Column::Address => self.fixture.address().to_string(),
            Column::Type => self
                .fixture
                .fixture_type(show.patch())
                .name
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or("<unnamed fixture type>".to_string()),
            Column::DmxMode => self
                .fixture
                .dmx_mode(show.patch())
                .name
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or("<unnamed dmx mode>".to_string()),
        });

        div().px_1().w_full().overflow_hidden().text_ellipsis().child(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Column {
    Id,
    Address,
    Type,
    DmxMode,
}

impl TableColumn for Column {
    fn label(&self) -> &str {
        match self {
            Column::Id => "Id",
            Column::Address => "Address",
            Column::Type => "Fixture Type",
            Column::DmxMode => "DMX Mode",
        }
    }

    fn all<'a>() -> &'a [Self] {
        &[Self::Id, Self::Address, Self::Type, Self::DmxMode]
    }
}
