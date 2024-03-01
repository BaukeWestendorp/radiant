use std::collections::HashMap;

use gpui::{
    div, px, rgb, AnyElement, IntoElement, ParentElement, Render, Rgba, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use palette::Srgb;

use crate::show::{LedGroup, ObjectId, Show};

use super::{
    grid::{Grid, GridDelegate, GridEvent},
    pool_grid::PoolGrid,
};

#[derive(Debug, Clone)]
pub struct Pool {
    pub kind: PoolKind,
    grid: View<Grid<PoolGridDelegate>>,
    x: usize,
    y: usize,
}

impl Pool {
    pub fn new(
        kind: PoolKind,
        rows: usize,
        cols: usize,
        x: usize,
        y: usize,
        cx: &mut WindowContext,
    ) -> View<Self> {
        let grid_delegate = PoolGridDelegate::new(kind, rows, cols);
        let grid = cx.new_view(|_| Grid::new(grid_delegate));

        let this = cx.new_view({
            let grid = grid.clone();
            |_| Pool { kind, grid, x, y }
        });

        cx.subscribe(&grid, {
            let this = this.clone();
            move |grid, event, cx| match event {
                GridEvent::CellClicked { row, col } => match (row, col) {
                    (0, 0) => this.read(cx).open_pool_settings(),
                    _ => {
                        this.update(cx, |this, cx| {
                            let cell_ix = row * grid.read(cx).delegate.cols() + col;
                            let object = match this.kind {
                                PoolKind::Color => {
                                    cx.global_mut::<Show>().add_color(Srgb::default())
                                }
                                PoolKind::Group => {
                                    cx.global_mut::<Show>().add_group(LedGroup::new(vec![]))
                                }
                            };
                            this.insert_cell(cell_ix, object, cx);
                        });
                    }
                },
            }
        })
        .detach();

        this
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn insert_cell(&mut self, cell_ix: usize, object_id: ObjectId, cx: &mut ViewContext<Self>) {
        self.grid.update(cx, |grid, _| {
            grid.delegate.insert_cell(cell_ix, object_id);
        });
    }

    fn open_pool_settings(&self) {
        todo!("Show pool settings");
    }
}

impl Render for Pool {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child(self.grid.clone())
    }
}

struct PoolGridDelegate {
    kind: PoolKind,
    rows: usize,
    cols: usize,
    objects: HashMap<usize, ObjectId>,
}

impl PoolGridDelegate {
    pub fn new(kind: PoolKind, rows: usize, cols: usize) -> Self {
        PoolGridDelegate {
            kind,
            rows,
            cols,
            objects: HashMap::new(),
        }
    }

    pub fn insert_cell(&mut self, cell_ix: usize, object_id: ObjectId) {
        self.objects.insert(cell_ix, object_id);
    }
}

impl GridDelegate for PoolGridDelegate {
    fn cell_size(&self) -> usize {
        PoolGrid::GRID_SIZE
    }

    fn grid_gap(&self) -> usize {
        PoolGrid::GRID_GAP
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn render_cell(&self, row: usize, col: usize, cx: &mut ViewContext<Grid<Self>>) -> AnyElement {
        let cell_ix = row * self.cols + col;
        div()
            .child(cx.new_view(|_| {
                PoolCell::new(self.kind, cell_ix, self.objects.get(&cell_ix).cloned())
            }))
            .size_full()
            .into_any_element()
    }

    fn render_first_cell(&self, _cx: &mut ViewContext<Grid<Self>>) -> AnyElement {
        let border_color = self.kind.color();
        let mut bg_color = self.kind.color();
        bg_color.a = 0.7;

        div()
            .child(
                div()
                    .child(self.kind.label())
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .font_weight(gpui::FontWeight::BOLD),
            )
            .size_full()
            .bg(bg_color)
            .border_color(border_color)
            .border_1()
            .rounded_md()
            .into_any_element()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoolKind {
    Color,
    Group,
}

impl PoolKind {
    pub fn color(&self) -> gpui::Rgba {
        match self {
            PoolKind::Color => rgb(0xCE4D89),
            PoolKind::Group => rgb(0x5BB9C1),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PoolKind::Color => "Colors",
            PoolKind::Group => "Groups",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolCell {
    pool_kind: PoolKind,
    object_id: Option<ObjectId>,
    cell_ix: usize,
}

impl PoolCell {
    pub fn new(kind: PoolKind, cell_ix: usize, object_id: Option<ObjectId>) -> Self {
        PoolCell {
            pool_kind: kind,
            object_id,
            cell_ix,
        }
    }

    pub fn object_id(&self) -> Option<ObjectId> {
        self.object_id
    }

    pub fn cell_ix(&self) -> usize {
        self.cell_ix
    }

    fn render_pool_object(
        &self,
        cx: &mut ViewContext<Self>,
        object_id: ObjectId,
    ) -> impl IntoElement {
        match self.pool_kind {
            PoolKind::Color => {
                let color = cx.global::<Show>().get_color(&object_id).unwrap();
                let color = Rgba {
                    r: color.red,
                    g: color.green,
                    b: color.blue,
                    a: 1.0,
                };

                div()
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(div().bg(color).size_1_2().rounded_sm())
                    .bg(rgb(0x242424))
            }
            PoolKind::Group => {
                let group = cx.global::<Show>().get_group(&object_id).unwrap();
                div()
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .child(format!("{} leds", group.led_ids().len()))
                            .text_sm()
                            .text_color(rgb(0x808080)),
                    )
                    .bg(rgb(0x242424))
            }
        }
    }

    fn render_empty_cell(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0x181818))
    }
}

impl Render for PoolCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let (content, border_color) = if let Some(object_id) = self.object_id {
            let mut border_color = self.pool_kind.color();
            border_color.a = 0.9;
            (
                self.render_pool_object(cx, object_id).into_any_element(),
                border_color,
            )
        } else {
            let mut border_color = self.pool_kind.color();
            border_color.a = 0.4;
            (self.render_empty_cell(cx).into_any_element(), border_color)
        };

        div()
            .relative()
            .child(div().child(content).absolute().size_full())
            .child(
                div()
                    .child(format!("{}", self.cell_ix))
                    .absolute()
                    .pl(px(4.0))
                    .text_xs()
                    .text_color(rgb(0x808080)),
            )
            .size_full()
            .border_1()
            .border_color(border_color)
            .rounded_md()
    }
}
