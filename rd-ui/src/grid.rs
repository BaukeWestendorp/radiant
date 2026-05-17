use gpui::{Background, Bounds, Canvas, Pixels, Point, canvas, fill, point, px};

/// Creates a canvas that draws a grid of dots.
pub fn dot_grid(spacing_x: Pixels, spacing_y: Pixels, color: impl Into<Background>) -> Canvas<()> {
    let color = color.into();

    canvas(|_, _, _| {}, {
        move |bounds, _, window, _cx| {
            let width = (bounds.size.width / spacing_x).round() as u32;
            let height = (bounds.size.height / spacing_y).round() as u32;

            for x in 0..=width {
                for y in 0..=height {
                    window.paint_quad(fill(
                        Bounds::centered_at(
                            point(
                                px(x as f32 * spacing_x.as_f32()),
                                px(y as f32 * spacing_y.as_f32()),
                            ) + bounds.origin,
                            gpui::size(px(2.0), px(2.0)),
                        ),
                        color,
                    ));
                }
            }
        }
    })
}

/// Creates a canvas that draws a grid of lines.
pub fn line_grid(spacing_x: Pixels, spacing_y: Pixels, color: impl Into<Background>) -> Canvas<()> {
    let color = color.into();

    canvas(|_, _, _| {}, {
        move |bounds, _, window, _cx| {
            let width = (bounds.size.width / spacing_x).round() as u32;
            let height = (bounds.size.height / spacing_y).round() as u32;

            for x in 0..=width {
                window.paint_quad(fill(
                    Bounds::new(
                        point(x as f32 * spacing_x, px(0.0)) + bounds.origin,
                        gpui::size(px(1.0), height as f32 * spacing_y),
                    ),
                    color,
                ));
            }

            for y in 0..=height {
                window.paint_quad(fill(
                    Bounds::new(
                        point(px(0.0), y as f32 * spacing_y) + bounds.origin,
                        gpui::size(width as f32 * spacing_x, px(1.0)),
                    ),
                    color,
                ));
            }
        }
    })
}

/// Creates a canvas that draws a grid of lines with a specified offset.
pub fn scrollable_line_grid(
    offset: &Point<Pixels>,
    spacing_x: Pixels,
    spacing_y: Pixels,
    color: impl Into<Background>,
) -> Canvas<()> {
    let color = color.into();
    let offset = point(offset.x % spacing_x - spacing_x, offset.y % spacing_y - spacing_y);

    canvas(|_, _, _| {}, {
        move |bounds, _, window, _cx| {
            let width = (bounds.size.width / spacing_x).round() as u32 + 2;
            let height = (bounds.size.height / spacing_y).round() as u32 + 2;

            for x in 0..=width {
                window.paint_quad(fill(
                    Bounds::new(
                        point(x as f32 * spacing_x, px(0.0)) + bounds.origin + offset,
                        gpui::size(px(1.0), height as f32 * spacing_y),
                    ),
                    color,
                ));
            }

            for y in 0..=height {
                window.paint_quad(fill(
                    Bounds::new(
                        point(px(0.0), y as f32 * spacing_y) + bounds.origin + offset,
                        gpui::size(width as f32 * spacing_x, px(1.0)),
                    ),
                    color,
                ));
            }
        }
    })
}
