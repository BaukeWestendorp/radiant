use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, rgb, uniform_list, AnyElement, IntoElement, ParentElement, Pixels, Render,
    SharedString, Styled, ViewContext,
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
            self.delegate.values().len(),
            move |view, visible_range, cx| {
                let mut rows = vec![];

                let header_row = view.delegate.render_header_row(cx).into_any_element();
                rows.push(header_row);

                for ix in visible_range {
                    let data = view.delegate.values()[ix].clone();
                    let cells = view.delegate.render_row_items(&data, cx);
                    let row = view.delegate.render_row(ix, cells, cx).into_any_element();
                    rows.push(row);
                }

                rows
            },
        )
    }
}

pub trait SheetDelegate: Sized {
    const DEFAULT_COLUMN_WIDTH: Pixels = px(100.0);

    type Data: Clone;

    fn value_labels(&self) -> Vec<String>;

    fn values(&self) -> &Vec<Self::Data>;

    fn column_widths(&self) -> Vec<Pixels> {
        vec![Self::DEFAULT_COLUMN_WIDTH; self.value_labels().len()]
    }

    fn render_header_row(&self, cx: &mut ViewContext<Sheet<Self>>) -> AnyElement {
        let header_cells = self
            .value_labels()
            .iter()
            .zip(self.column_widths())
            .map(|(name, width)| {
                div()
                    .w(width)
                    .h(cx.line_height())
                    .child(self.render_cell(div().child(name.to_string()), cx))
            })
            .collect::<Vec<_>>();

        div()
            .flex()
            .flex_row()
            .w_full()
            .children(header_cells)
            .into_any_element()
    }

    fn render_row(
        &self,
        ix: usize,
        children: impl IntoIterator<Item = impl IntoElement>,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        let children = children
            .into_iter()
            .zip(self.column_widths())
            .map(|(child, width)| div().w(width).h(cx.line_height()).child(child))
            .collect::<Vec<_>>();

        div()
            .flex()
            .flex_row()
            .w_full()
            .when(ix % 2 == 0, |this| this.bg(rgb(0x333333)))
            .children(children)
            .into_any_element()
    }

    fn render_row_items(
        &self,
        _data: &Self::Data,
        _cx: &mut ViewContext<Sheet<Self>>,
    ) -> Vec<AnyElement>;

    fn render_cell(
        &self,
        content: impl IntoElement,
        _cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        div()
            .size_full()
            .whitespace_nowrap()
            .overflow_hidden()
            .border_r()
            .border_color(rgb(0x888888))
            .child(content)
            .into_any_element()
    }
}
