use crate::show::{Asset, AssetPool};
use gpui::{App, ElementId, Entity, EventEmitter, Window, div, prelude::*};
use ui::{Table, TableColumn, TableDelegate, TableRow};

pub struct AssetTable<T> {
    asset_pool: AssetPool<T>,
}

impl<T> AssetTable<T> {
    pub fn new(asset_pool: AssetPool<T>) -> Self {
        AssetTable { asset_pool }
    }
}

impl<T: 'static> TableDelegate for AssetTable<T> {
    type Row = Row<T>;

    type Column = Column;

    fn rows(&mut self, cx: &mut App) -> Vec<Self::Row> {
        let mut rows =
            self.asset_pool.values().map(|asset| Row { asset: asset.clone() }).collect::<Vec<_>>();
        rows.sort_by(|a, b| a.asset.read(cx).id.as_u32().cmp(&b.asset.read(cx).id.as_u32()));
        rows
    }

    fn handle_on_click_row(
        &mut self,
        row: Self::Row,
        _event: &gpui::ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) {
        cx.emit(AssetTableEvent::Selected(row.asset));
    }
}

#[derive(Clone)]
pub struct Row<T> {
    asset: Entity<Asset<T>>,
}

impl<T: 'static> TableRow<AssetTable<T>> for Row<T> {
    fn id(&self, cx: &mut Context<Table<AssetTable<T>>>) -> gpui::ElementId {
        ElementId::Integer(self.asset.read(cx).id.as_u32() as u64)
    }

    fn render_cell(
        &self,
        column: &Column,
        _window: &mut Window,
        cx: &mut Context<Table<AssetTable<T>>>,
    ) -> impl IntoElement {
        let text = match column {
            Column::Id => format!("{:?}", self.asset.read(cx).id),
            Column::Label => self.asset.read(cx).label.to_string(),
        };

        div().flex().items_center().px_1().child(text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Column {
    Id,
    Label,
}

impl TableColumn for Column {
    fn label(&self) -> &str {
        match self {
            Column::Id => "Id",
            Column::Label => "Label",
        }
    }

    fn all<'a>() -> &'a [Self] {
        &[Column::Id, Column::Label]
    }
}

pub enum AssetTableEvent<T> {
    Selected(Entity<Asset<T>>),
}

impl<T: 'static> EventEmitter<AssetTableEvent<T>> for Table<AssetTable<T>> {}
