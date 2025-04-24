use gpui::*;

/// Creates a canvas that draws a grid of dots.
pub fn dot_grid(spacing: Pixels, color: impl Into<Background>) -> Canvas<()> {
    let color = color.into();

    canvas(|_, _, _| {}, {
        move |bounds, _, window, _cx| {
            let width = f32::round(bounds.size.width / spacing) as u32;
            let height = f32::round(bounds.size.height / spacing) as u32;

            for x in 0..(width + 1) {
                for y in 0..(height + 1) {
                    window.paint_quad(fill(
                        Bounds::centered_at(
                            point(x as f32 * spacing, y as f32 * spacing) + bounds.origin,
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
pub fn line_grid(spacing: Pixels, color: impl Into<Background>) -> Canvas<()> {
    let color = color.into();

    canvas(|_, _, _| {}, {
        move |bounds, _, window, _cx| {
            let width = f32::round(bounds.size.width / spacing) as u32;
            let height = f32::round(bounds.size.height / spacing) as u32;

            for x in 0..(width + 1) {
                window.paint_quad(fill(
                    Bounds::new(
                        point(x as f32 * spacing, px(0.0)) + bounds.origin,
                        gpui::size(px(1.0), height as f32 * spacing),
                    ),
                    color,
                ));
            }

            for y in 0..(height + 1) {
                window.paint_quad(fill(
                    Bounds::new(
                        point(px(0.0), y as f32 * spacing) + bounds.origin,
                        gpui::size(width as f32 * spacing, px(1.0)),
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
    spacing: Pixels,
    color: impl Into<Background>,
) -> Canvas<()> {
    let color = color.into();
    let offset = point(offset.x % spacing - spacing, offset.y % spacing - spacing);

    canvas(|_, _, _| {}, {
        move |bounds, _, window, _cx| {
            let width = f32::round(bounds.size.width / spacing) as u32 + 2;
            let height = f32::round(bounds.size.height / spacing) as u32 + 2;

            for x in 0..(width + 1) {
                window.paint_quad(fill(
                    Bounds::new(
                        point(x as f32 * spacing, px(0.0)) + bounds.origin + offset,
                        gpui::size(px(1.0), height as f32 * spacing),
                    ),
                    color,
                ));
            }

            for y in 0..(height + 1) {
                window.paint_quad(fill(
                    Bounds::new(
                        point(px(0.0), y as f32 * spacing) + bounds.origin + offset,
                        gpui::size(width as f32 * spacing, px(1.0)),
                    ),
                    color,
                ));
            }
        }
    })
}
