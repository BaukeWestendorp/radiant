use crate::show::asset::{Asset, AssetPool};
use gpui::{App, Entity, Window, div, prelude::*};
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

    fn rows(&self, cx: &App) -> Vec<Self::Row> {
        let mut rows =
            self.asset_pool.values().map(|asset| Row { asset: asset.clone() }).collect::<Vec<_>>();
        rows.sort_by(|a, b| a.asset.read(cx).id.as_u32().cmp(&b.asset.read(cx).id.as_u32()));
        rows
    }
}

pub struct Row<T> {
    asset: Entity<Asset<T>>,
}

impl<T: 'static> TableRow<AssetTable<T>> for Row<T> {
    fn render_cell(
        &self,
        column: &Column,
        _w: &mut Window,
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
