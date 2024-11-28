use gpui::*;
use ui::{theme::ActiveTheme, z_stack};

pub const GRID_SIZE: f32 = 80.0;

pub struct FrameGrid {
    width: u32,
    height: u32,
}

impl FrameGrid {
    pub fn build(cx: &mut WindowContext) -> View<FrameGrid> {
        cx.new_view(|_cx| FrameGrid {
            width: 16,
            height: 10,
        })
    }
}

impl Render for FrameGrid {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        let background = canvas(|_, _| {}, {
            let width = self.width;
            let height = self.height;
            move |_, _, cx| {
                for x in 0..width {
                    for y in 0..height {
                        cx.paint_quad(fill(
                            Bounds::centered_at(
                                point(px(x as f32 * GRID_SIZE), px(y as f32 * GRID_SIZE)),
                                size(px(2.0), px(2.0)),
                            ),
                            cx.theme().accent.opacity(0.5),
                        ));
                    }
                }
            }
        })
        .size_full()
        .into_any_element();

        let frames = div().into_any_element();

        z_stack([background, frames])
    }
}
