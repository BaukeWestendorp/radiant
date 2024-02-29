use gpui::{div, px, IntoElement, ParentElement, Render, Styled, ViewContext};

pub struct Grid<D: GridDelegate> {
    pub delegate: D,
}

impl<D: GridDelegate> Grid<D> {
    pub fn new(delegate: D) -> Self {
        Self { delegate }
    }

    fn render_rows(&self) -> Vec<impl IntoElement> {
        (0..self.delegate.rows())
            .map(|r| {
                let row = (0..self.delegate.cols()).map(|c| {
                    let cell = self.delegate.render_cell(r, c);

                    div()
                        .child(cell)
                        .w(px(self.delegate.cell_size() as f32))
                        .h(px(self.delegate.cell_size() as f32))
                });
                div()
                    .children(row)
                    .flex()
                    .flex_row()
                    .gap(px(self.delegate.grid_gap() as f32))
            })
            .collect()
    }
}

impl<D: GridDelegate> Render for Grid<D> {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let rows = self.render_rows();

        div().child(
            div()
                .children(rows)
                .flex()
                .flex_col()
                .gap(px(self.delegate.grid_gap() as f32))
                .absolute(),
        )
    }
}

pub trait GridDelegate: Sized + 'static {
    type Cell: IntoElement;

    fn cell_size(&self) -> usize;

    fn grid_gap(&self) -> usize;

    fn rows(&self) -> usize;

    fn cols(&self) -> usize;

    fn render_cell(&self, row: usize, col: usize) -> Self::Cell;
}
