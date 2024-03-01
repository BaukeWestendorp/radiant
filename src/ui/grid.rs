use gpui::{
    div, px, AnyElement, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, ViewContext,
};

pub struct Grid<D: GridDelegate> {
    pub delegate: D,
}

impl<D: GridDelegate> Grid<D> {
    pub fn new(delegate: D) -> Self {
        Self { delegate }
    }

    fn render_rows(&self, cx: &mut ViewContext<Self>) -> Vec<impl IntoElement> {
        (0..self.delegate.rows())
            .map(|r| {
                let row = (0..self.delegate.cols()).map(|c| {
                    let cell = match (r, c) {
                        (0, 0) => self.delegate.render_first_cell(cx),
                        _ => self.delegate.render_cell(r, c, cx),
                    };

                    div()
                        .child(cell)
                        .w(px(self.delegate.cell_size() as f32))
                        .h(px(self.delegate.cell_size() as f32))
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(move |_, _, cx| {
                                cx.stop_propagation();
                                cx.prevent_default();

                                cx.emit(GridEvent::CellClicked { row: r, col: c });
                            }),
                        )
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let rows = self.render_rows(cx);

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
    fn cell_size(&self) -> usize;

    fn grid_gap(&self) -> usize;

    fn rows(&self) -> usize;

    fn cols(&self) -> usize;

    fn render_cell(&self, row: usize, col: usize, cx: &mut ViewContext<Grid<Self>>) -> AnyElement;

    fn render_first_cell(&self, cx: &mut ViewContext<Grid<Self>>) -> AnyElement {
        self.render_cell(0, 0, cx)
    }
}

impl<D: GridDelegate> EventEmitter<GridEvent> for Grid<D> {}

pub enum GridEvent {
    CellClicked { row: usize, col: usize },
}
