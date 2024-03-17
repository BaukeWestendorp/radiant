use gpui::prelude::FluentBuilder;
use gpui::{
    div, rgb, uniform_list, AnyElement, IntoElement, ParentElement, Pixels, Render, SharedString,
    Styled, ViewContext,
};

pub struct Sheet<D: SheetDelegate> {
    pub delegate: D,

    id: SharedString,
}

impl<D: SheetDelegate> Sheet<D> {
    pub fn new(delegate: D, id: &str) -> Self {
        Self {
            delegate,
            id: id.to_string().into(),
        }
    }
}

impl<D: SheetDelegate + 'static> Render for Sheet<D> {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        uniform_list(
            cx.view().clone(),
            self.id.clone(),
            self.delegate.values(cx).len() + 1,
            move |view, mut visible_range, cx| {
                visible_range.end -= 1;

                let mut rows = vec![];

                let header_row = view.delegate.render_header_row(cx).into_any_element();
                rows.push(header_row);

                for ix in visible_range {
                    let data = view.delegate.values(cx)[ix].clone();
                    let cells = view
                        .delegate
                        .columns(cx)
                        .iter()
                        .map(|column_id| {
                            let content = view.delegate.render_cell_content(column_id, &data, cx);
                            let cell = view.delegate.render_cell(column_id, content, cx);
                            cell
                        })
                        .collect::<Vec<_>>();
                    let row = view.delegate.render_row(ix, cells, cx).into_any_element();
                    rows.push(row);
                }

                rows
            },
        )
    }
}

pub trait SheetDelegate: Sized {
    type Data: Clone;
    type ColumnId: Clone;

    fn columns(&self, cx: &mut ViewContext<Sheet<Self>>) -> Vec<Self::ColumnId>;

    fn values(&self, cx: &mut ViewContext<Sheet<Self>>) -> &Vec<Self::Data>;

    fn header_label(&self, column_id: &Self::ColumnId, cx: &mut ViewContext<Sheet<Self>>)
        -> String;

    fn column_width(&self, column_id: &Self::ColumnId, cx: &mut ViewContext<Sheet<Self>>)
        -> Pixels;

    fn render_cell_content(
        &self,
        column_id: &Self::ColumnId,
        data: &Self::Data,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement;

    fn render_header_cell(
        &self,
        column_id: &Self::ColumnId,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        self.render_cell(column_id, self.header_label(column_id, cx), cx)
    }

    fn render_header_row(&self, cx: &mut ViewContext<Sheet<Self>>) -> AnyElement {
        let header_cells = self
            .columns(cx)
            .iter()
            .map(|column_id| self.render_header_cell(column_id, cx))
            .collect::<Vec<_>>();

        div()
            .flex()
            .flex_row()
            .children(header_cells)
            .border_b()
            .border_color(rgb(0x666666))
            .into_any_element()
    }

    fn render_row(
        &self,
        ix: usize,
        children: impl IntoIterator<Item = impl IntoElement>,
        _cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        div()
            .flex()
            .flex_row()
            .when(ix % 2 == 0, |this| this.bg(rgb(0x343434)))
            .children(children)
            .into_any_element()
    }

    fn render_cell(
        &self,
        column_id: &Self::ColumnId,
        content: impl IntoElement,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        div()
            .w(self.column_width(column_id, cx))
            .px_2()
            .whitespace_nowrap()
            .overflow_hidden()
            .border_r()
            .border_color(rgb(0x666666))
            .child(content)
            .into_any_element()
    }
}
