use gpui::{AnyElement, App, SharedString, Window};

use crate::TableDelegate;

pub struct Column<D: TableDelegate> {
    id: SharedString,
    name: SharedString,

    pub(crate) cell_builder: Option<Box<dyn Fn(&D::Row, &Window, &App) -> AnyElement>>,
    pub(crate) edit_handler: Option<Box<dyn Fn(&[&mut D::Row])>>,
    pub(crate) sort_handler: Option<Box<dyn Fn(&D::Row, &D::Row) -> std::cmp::Ordering>>,
}

impl<D: TableDelegate> Column<D> {
    pub fn new(id: impl Into<SharedString>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            sort_handler: None,
            edit_handler: None,
            cell_builder: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn editable(&self) -> bool {
        self.edit_handler.is_some()
    }

    pub fn sortable(&self) -> bool {
        self.sort_handler.is_some()
    }

    pub fn with_cell_builder(
        mut self,
        cell_builder: impl Fn(&D::Row, &Window, &App) -> AnyElement + 'static,
    ) -> Self {
        self.cell_builder = Some(Box::new(cell_builder));
        self
    }

    pub fn with_edit_handler(mut self, edit_handler: impl Fn(&[&mut D::Row]) + 'static) -> Self {
        self.edit_handler = Some(Box::new(edit_handler));
        self
    }

    pub fn with_sort_handler(
        mut self,
        sort_handler: impl Fn(&D::Row, &D::Row) -> std::cmp::Ordering + 'static,
    ) -> Self {
        self.sort_handler = Some(Box::new(sort_handler));
        self
    }
}
