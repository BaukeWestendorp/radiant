use smallvec::SmallVec;
use std::{cmp, ops::Range};

use gpui::{
    point, size, AnyElement, AvailableSpace, Bounds, ContentMask, Element, ElementContext,
    ElementId, Hitbox, InteractiveElement, Interactivity, IntoElement, LayoutId, Pixels, Render,
    Size, StyleRefinement, Styled, View, ViewContext, WindowContext,
};

/// This is a very ad-hoc implementation of a grid layout. It is not intended to
/// be a general-purpose grid layout.
pub fn uniform_grid<I, R, V>(
    view: View<V>,
    id: I,
    cols: usize,
    rows: usize,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut ViewContext<V>) -> Vec<R>,
) -> UniformGrid
where
    I: Into<ElementId>,
    R: IntoElement,
    V: Render,
{
    let render_range = move |range, cx: &mut WindowContext| {
        view.update(cx, |this, cx| {
            f(this, range, cx)
                .into_iter()
                .map(|component| component.into_any_element())
                .collect()
        })
    };

    let id = id.into();

    let base_style = StyleRefinement::default();

    let mut interactivity = Interactivity::default();
    interactivity.element_id = Some(id.clone());
    interactivity.base_style = Box::new(base_style);

    UniformGrid {
        cols,
        rows,
        cell_to_measure_index: 0,
        render_cells: Box::new(render_range),
        interactivity,
    }
}

pub struct UniformGrid {
    cols: usize,
    rows: usize,
    cell_to_measure_index: usize,
    render_cells:
        Box<dyn for<'a> Fn(Range<usize>, &'a mut WindowContext) -> SmallVec<[AnyElement; 64]>>,
    interactivity: Interactivity,
}

impl UniformGrid {
    fn cell_count(&self) -> usize {
        self.cols * self.rows
    }

    fn measure_cell(
        &self,
        cell_width: Option<Pixels>,
        cell_height: Option<Pixels>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        if self.cols == 0 || self.rows == 0 {
            return Size::default();
        }

        let cell_ix = cmp::min(self.cell_to_measure_index, self.cell_count() - 1);
        let mut cells = (self.render_cells)(cell_ix..cell_ix + 1, cx);
        let mut cell_to_measure = cells.pop().unwrap();
        let available_space = size(
            cell_width.map_or(AvailableSpace::MinContent, |width| {
                AvailableSpace::Definite(width)
            }),
            cell_height.map_or(AvailableSpace::MinContent, |height| {
                AvailableSpace::Definite(height)
            }),
        );

        cell_to_measure.measure(available_space, cx)
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct GridState {
    cols: usize,
    rows: usize,
    cells: SmallVec<[AnyElement; 32]>,
    cell_size: Size<Pixels>,
}

impl Element for UniformGrid {
    type BeforeLayout = GridState;
    type AfterLayout = Option<Hitbox>;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let rows = self.rows;
        let cols = self.cols;
        let cell_size = self.measure_cell(None, None, cx);

        let layout_id = self.interactivity.before_layout(cx, |style, cx| {
            cx.request_measured_layout(style, move |known_dimensions, available_space, _cx| {
                let desired_height = cell_size.height * rows;

                let width = known_dimensions
                    .width
                    .unwrap_or(match available_space.width {
                        AvailableSpace::Definite(x) => x,
                        AvailableSpace::MinContent | AvailableSpace::MaxContent => cell_size.width,
                    });

                let height = match available_space.height {
                    AvailableSpace::Definite(height) => desired_height.min(height),
                    AvailableSpace::MinContent | AvailableSpace::MaxContent => desired_height,
                };

                size(width, height)
            })
        });

        let element_state = GridState {
            cols,
            rows,
            cells: SmallVec::new(),
            cell_size,
        };

        (layout_id, element_state)
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Self::AfterLayout {
        let content_size = Size {
            width: before_layout.cell_size.width * before_layout.cols as f32,
            height: before_layout.cell_size.height * before_layout.rows as f32,
        };

        let bounds = Bounds::new(bounds.origin, content_size);

        let style = self.interactivity.compute_style(None, cx);

        let border = style.border_widths.to_pixels(cx.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.lower_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let cell_size = before_layout.cell_size;
        let cell_count = self.cell_count();

        self.interactivity.after_layout(
            bounds,
            content_size,
            cx,
            |_style, _scroll_offset, hitbox, cx| {
                if cell_count == 0 {
                    return hitbox;
                }

                let visible_range = 0..cell_count;

                let cells = (self.render_cells)(visible_range.clone(), cx);
                let content_mask = ContentMask { bounds };
                cx.with_content_mask(Some(content_mask), |cx| {
                    for (mut cell, ix) in cells.into_iter().zip(visible_range) {
                        let cell_origin = padded_bounds.origin
                            + point(
                                cell_size.width * (ix % self.cols) + padding.top,
                                cell_size.height * (ix / self.cols) + padding.left,
                            );

                        let available_space = size(
                            AvailableSpace::Definite(cell_size.width),
                            AvailableSpace::Definite(cell_size.height),
                        );

                        cell.layout(cell_origin, available_space, cx);
                        before_layout.cells.push(cell);
                    }
                });

                hitbox
            },
        )
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        hitbox: &mut Option<Hitbox>,
        cx: &mut ElementContext,
    ) {
        self.interactivity
            .paint(bounds, hitbox.as_ref(), cx, |_, cx| {
                for cell in &mut before_layout.cells {
                    cell.paint(cx);
                }
            })
    }
}

impl IntoElement for UniformGrid {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl InteractiveElement for UniformGrid {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        &mut self.interactivity
    }
}

impl Styled for UniformGrid {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}
