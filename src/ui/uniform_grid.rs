use smallvec::SmallVec;
use std::{cmp, ops::Range};

use gpui::{
    point, size, AnyElement, AvailableSpace, Bounds, ContentMask, Element, ElementContext,
    ElementId, InteractiveElement, InteractiveElementState, Interactivity, IntoElement, LayoutId,
    Overflow, Pixels, Render, Size, StyleRefinement, View, ViewContext, WindowContext,
};

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

    let mut base_style = StyleRefinement::default();
    base_style.overflow.y = Some(Overflow::Scroll);

    let mut interactivity = Interactivity::default();
    interactivity.element_id = Some(id.clone());
    interactivity.base_style = Box::new(base_style);

    UniformGrid {
        id,
        cols,
        rows,
        item_to_measure_index: 0,
        render_items: Box::new(render_range),
        interactivity,
    }
}

pub struct UniformGrid {
    id: ElementId,
    cols: usize,
    rows: usize,
    item_to_measure_index: usize,
    render_items:
        Box<dyn for<'a> Fn(Range<usize>, &'a mut WindowContext) -> SmallVec<[AnyElement; 64]>>,
    interactivity: Interactivity,
}

impl UniformGrid {
    fn item_count(&self) -> usize {
        self.cols * self.rows
    }

    fn measure_item(
        &self,
        cell_width: Option<Pixels>,
        cell_height: Option<Pixels>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        if self.cols == 0 || self.rows == 0 {
            return Size::default();
        }

        let item_ix = cmp::min(self.item_to_measure_index, self.item_count() - 1);
        let mut items = (self.render_items)(item_ix..item_ix + 1, cx);
        let mut item_to_measure = items.pop().unwrap();
        let available_space = size(
            cell_width.map_or(AvailableSpace::MinContent, |width| {
                AvailableSpace::Definite(width)
            }),
            cell_height.map_or(AvailableSpace::MinContent, |height| {
                AvailableSpace::Definite(height)
            }),
        );
        item_to_measure.measure(available_space, cx)
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct GridState {
    interactive: InteractiveElementState,
    item_size: Size<Pixels>,
}

impl Element for UniformGrid {
    type State = GridState;

    fn request_layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        let rows = self.rows;
        let item_size = state
            .as_ref()
            .map(|s| s.item_size)
            .unwrap_or_else(|| self.measure_item(None, None, cx));

        let (layout_id, interactive) =
            self.interactivity
                .layout(state.map(|s| s.interactive), cx, |style, cx| {
                    cx.request_measured_layout(
                        style,
                        move |_known_dimensions, available_space, _cx| {
                            let desired_height = item_size.height * rows;

                            let width = gpui::px(200.0);

                            let height = match available_space.height {
                                AvailableSpace::Definite(height) => desired_height.min(height),
                                AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                    desired_height
                                }
                            };
                            size(width, height)
                        },
                    )
                });

        let element_state = GridState {
            interactive,
            item_size,
        };

        (layout_id, element_state)
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut ElementContext) {
        let style = self
            .interactivity
            .compute_style(Some(bounds), &mut state.interactive, cx);
        let border = style.border_widths.to_pixels(cx.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.lower_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let content_size = Size {
            width: padded_bounds.size.width,
            height: padded_bounds.size.height,
        };

        let item_size = self.measure_item(
            Some(padded_bounds.size.width),
            Some(padded_bounds.size.height),
            cx,
        );

        let item_count = self.item_count();
        self.interactivity.paint(
            bounds,
            content_size,
            &mut state.interactive,
            cx,
            |_style, mut _scroll_offset, cx| {
                if item_count == 0 {
                    return;
                }

                let visible_range = 0..item_count;

                let mut items = (self.render_items)(visible_range.clone(), cx);
                cx.with_z_index(1, |cx| {
                    let content_mask = ContentMask { bounds };
                    cx.with_content_mask(Some(content_mask), |cx| {
                        for (item, ix) in items.iter_mut().zip(visible_range) {
                            let item_origin = padded_bounds.origin
                                + point(
                                    item_size.width * (ix % self.cols) + padding.top,
                                    item_size.height * (ix / self.cols) + padding.left,
                                );

                            let available_space = size(
                                AvailableSpace::Definite(item_size.width),
                                AvailableSpace::Definite(item_size.height),
                            );

                            item.draw(item_origin, available_space, cx)
                        }
                    })
                })
            },
        );
    }
}

impl IntoElement for UniformGrid {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl InteractiveElement for UniformGrid {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        &mut self.interactivity
    }
}
