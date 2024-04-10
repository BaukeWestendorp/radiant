use gpui::{
    div, px, uniform_list, AnyElement, InteractiveElement, IntoElement, ParentElement, Pixels,
    Render, ScrollHandle, SharedString, StatefulInteractiveElement, Styled, ViewContext,
};
use theme::ActiveTheme;

pub struct Sheet<D: SheetDelegate> {
    pub delegate: D,

    id: SharedString,
    scroll_handle: ScrollHandle,
}

impl<D: SheetDelegate> Sheet<D> {
    pub fn new(delegate: D, id: &str) -> Self {
        Self {
            delegate,
            id: id.to_string().into(),
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl<D: SheetDelegate + 'static> Render for Sheet<D> {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let header_row = self.delegate.render_header_row(cx).into_any_element();

        let total_width = {
            let mut total_width = px(0.0);
            for column_id in self.delegate.columns(cx) {
                total_width += self.delegate.column_width(&column_id, cx);
            }
            total_width
        };

        div()
            .id(self.id.clone())
            .w_full()
            .overflow_x_scroll()
            // .border()
            // .border_color(cx.theme().colors().border)
            // .rounded_md()
            .track_scroll(&self.scroll_handle)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(total_width)
                    .child(header_row)
                    .child(uniform_list(
                        cx.view().clone(),
                        self.id.clone(),
                        self.delegate.values(cx).len(),
                        move |view, visible_range, cx| {
                            let mut rows = vec![];
                            for ix in visible_range {
                                let data = view.delegate.values(cx)[ix].clone();
                                let cells = view
                                    .delegate
                                    .columns(cx)
                                    .iter()
                                    .map(|column_id| {
                                        let content =
                                            view.delegate.render_cell_content(column_id, &data, cx);
                                        view.delegate.render_cell(column_id, content, cx)
                                    })
                                    .collect::<Vec<_>>();
                                let row =
                                    view.delegate.render_row(ix, cells, cx).into_any_element();
                                rows.push(row);
                            }
                            rows
                        },
                    )),
            )
    }
}

pub trait SheetDelegate: Sized {
    type Data: Clone;
    type ColumnId: Clone;

    fn columns(&self, cx: &mut ViewContext<Sheet<Self>>) -> Vec<Self::ColumnId>;

    fn values(&self, cx: &mut ViewContext<Sheet<Self>>) -> &[Self::Data];

    fn selected_rows(&self, _cx: &mut ViewContext<Sheet<Self>>) -> Vec<usize> {
        vec![]
    }

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
            .bg(cx.theme().colors().element_background_secondary)
            .border_b()
            .border_color(cx.theme().colors().border)
            .children(header_cells)
            .into_any_element()
    }

    fn render_row(
        &self,
        ix: usize,
        children: impl IntoIterator<Item = impl IntoElement>,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        let is_selected = self.selected_rows(cx).contains(&ix);

        div()
            .flex()
            .flex_row()
            .bg(match is_selected {
                true => cx.theme().colors().element_background_selected,
                false => match ix % 2 == 0 {
                    true => cx.theme().colors().table_row_background_even,
                    false => cx.theme().colors().table_row_background_odd,
                },
            })
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
            .min_w(self.column_width(column_id, cx))
            .max_w(self.column_width(column_id, cx))
            .whitespace_nowrap()
            .overflow_hidden()
            .border_r()
            .border_color(cx.theme().colors().border)
            .child(content)
            .into_any_element()
    }
}
