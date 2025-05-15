use crate::show::asset::{Asset, AssetPool};
use gpui::{Entity, Window, prelude::*};
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

    fn rows(&self) -> Vec<Self::Row> {
        self.asset_pool.values().map(|asset| Row { asset: asset.clone() }).collect::<Vec<_>>()
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
        match column {
            Column::Id => format!("{:?}", self.asset.read(cx).id),
            Column::Label => self.asset.read(cx).label.to_string(),
        }
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
            Column::Label => "Name",
        }
    }

    fn all<'a>() -> &'a [Self] {
        &[Column::Id, Column::Label]
    }
}
